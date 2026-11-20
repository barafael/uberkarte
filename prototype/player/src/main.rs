//! The player is the editor. Browse mode sends messages; edit mode moves parts and edits scripts.
//! Every change is journaled and written to the stack file at once: there is no Save.
use dioxus::prelude::*;
use stack_core::{demo, files::Fs, Kind, Obj, Rt, Shape, SoundCmd, StackFile, Value, HOME};

mod highlight;
use std::cell::RefCell;
use std::rc::Rc;

/// Native fetches finished on other threads: (request id, text or error). The frame loop delivers them.
#[cfg(not(target_arch = "wasm32"))]
static FETCHED: std::sync::Mutex<Vec<(u64, Result<String, String>)>> = std::sync::Mutex::new(Vec::new());

#[cfg(not(target_arch = "wasm32"))]
thread_local! {
    static FETCH_SEQ: RefCell<u64> = const { RefCell::new(0) };
    static FETCH_TARGETS: RefCell<std::collections::HashMap<u64, (std::rc::Weak<Rt>, String, u32)>> = RefCell::new(Default::default());
}

/// Deliver finished native fetches to the stacks that asked (urlLoaded, or urlLoaded … "error").
#[cfg(not(target_arch = "wasm32"))]
fn deliver_fetches() -> bool {
    let done: Vec<(u64, Result<String, String>)> = std::mem::take(&mut *FETCHED.lock().unwrap());
    let any = !done.is_empty();
    for (id, r) in done {
        let Some((target, url, to)) = FETCH_TARGETS.with(|t| t.borrow_mut().remove(&id)) else { continue };
        let Some(target) = target.upgrade() else { continue };
        match r {
            Ok(text) => target.url_loaded(&url, &text, to),
            Err(e) => { target.w.borrow_mut().log.push(format!("load url {url}: {e}")); target.url_failed(&url, &e, to); }
        }
    }
    if any { handle_requests(); save(); }
    any
}

/// One open stack: a tab.
struct Tab { path: String, rt: Rc<Rt> }

thread_local! {
    /// The open stacks and the one showing. RT and PATH are the showing one's.
    static TABS: RefCell<Vec<Tab>> = const { RefCell::new(vec![]) };
    static ACTIVE: RefCell<usize> = const { RefCell::new(0) };
    /// One file system for every open stack (the real one on desktop, the browser-storage mirror on the web).
    static FS: Rc<RefCell<Box<dyn Fs>>> = Rc::new(RefCell::new(stack_core::files::default_fs()));
    static RT: RefCell<Option<Rc<Rt>>> = const { RefCell::new(None) };
    /// On the web, the first stack's opening messages wait until browser storage is restored.
    static DEFERRED_OPEN: RefCell<bool> = const { RefCell::new(false) };
    /// Set when something finished in the background (a fetched URL): the frame loop redraws.
    static DIRTY: RefCell<bool> = const { RefCell::new(false) };
    static PATH: RefCell<String> = const { RefCell::new(String::new()) };
    /// data: URLs of assets, by name, with the byte length they were made from.
    static URLS: RefCell<std::collections::HashMap<String, (usize, String)>> = RefCell::new(Default::default());
    /// Stacks left during this session: opening one again sends resumeStack.
    static SUSPENDED: RefCell<std::collections::HashSet<String>> = RefCell::new(Default::default());
    /// The field being edited and its text when editing began (closeField if it changed, exitField if not),
    /// and the last mouse press (part, ms) to recognize the second click of a double-click.
    static FOCUS_TEXT: RefCell<Option<(u32, String)>> = const { RefCell::new(None) };
    static LAST_DOWN: RefCell<(u32, f64)> = const { RefCell::new((0, 0.0)) };
    static DOUBLE: RefCell<bool> = const { RefCell::new(false) };
    /// Set when a script quit HyperCard (doMenu "Quit HyperCard").
    static QUIT: RefCell<bool> = const { RefCell::new(false) };
    /// Sounds whose data the browser side already holds.
    static SENT: RefCell<std::collections::HashSet<String>> = RefCell::new(Default::default());
}

fn asset_url(name: &str) -> Option<String> {
    let r = rt();
    let w = r.w.borrow();
    let a = w.file.asset(name)?;
    let bytes = w.file.blobs.get(&a.name)?;
    Some(URLS.with(|u| {
        let mut u = u.borrow_mut();
        if let Some((len, url)) = u.get(&a.name) { if *len == bytes.len() { return url.clone(); } }
        use base64::Engine as _;
        let url = format!("data:{};base64,{}", a.mime, base64::engine::general_purpose::STANDARD.encode(bytes));
        u.insert(a.name.clone(), (bytes.len(), url.clone()));
        url
    }))
}

/// Hand queued `play` / `play stop` commands to the browser side.
fn flush_sound() {
    let cmds = std::mem::take(&mut rt().w.borrow_mut().sound_cmds);
    for c in cmds {
        match c {
            SoundCmd::Stop => { let _ = document::eval("window.__hcStop && window.__hcStop()"); }
            SoundCmd::Clip(key, bytes) => {
                use base64::Engine as _;
                let url = format!("data:audio/wav;base64,{}", base64::engine::general_purpose::STANDARD.encode(&bytes));
                let _ = document::eval(&format!("window.__hcPlay && window.__hcPlay({key:?}, {url:?})"));
            }
            SoundCmd::Play(name) => {
                let first = SENT.with(|s| s.borrow_mut().insert(name.clone()));
                let url = if first { asset_url(&name).map(|u| format!("{u:?}")).unwrap_or("null".into()) } else { "null".into() };
                let _ = document::eval(&format!("window.__hcPlay && window.__hcPlay({name:?}, {url})"));
            }
        }
    }
}

/// Browser-side audio: one sound at a time; reports `ended` so scripts get `soundDone`.
const SOUND_JS: &str = r#"
window.__hc = { urls: {}, cur: null };
window.__hcStop = () => { const a = window.__hc.cur; if (a) { a.onended = null; a.pause(); window.__hc.cur = null; } };
window.__hcPlay = (name, url) => {
  if (url) window.__hc.urls[name] = url;
  window.__hcStop();
  const a = new Audio(window.__hc.urls[name]);
  window.__hc.cur = a;
  a.onended = () => { if (window.__hc.cur === a) { window.__hc.cur = null; dioxus.send("ended"); } };
  a.play().catch(e => { if (window.__hc.cur === a) { window.__hc.cur = null; dioxus.send("error: " + e.message); } });
};
await new Promise(() => {});
"#;
fn rt() -> Rc<Rt> { RT.with(|r| r.borrow().clone().expect("runtime")) }

const DEMOS: [(&str, &str); 5] = [("Address Book", "stacks/addressbook.stack"), ("Self Test", "stacks/selftest.stack"), ("Breakout", "stacks/breakout.stack"), ("Pillars", "stacks/pillars.stack"), ("Untitled", "stacks/untitled.stack")];

fn demo_for(path: &str) -> StackFile {
    if path.contains("selftest") { demo::self_test() }
    else if path.contains("breakout") { demo::breakout() } else if path.contains("pillars") { demo::pillars() }
    else if path.contains("addressbook") { demo::address_book() }
    else {
        // The stack is named after its file, capitalized ("untitled.stack" → "Untitled"), as in the stack menu.
        let stem = std::path::Path::new(path).file_stem().and_then(|s| s.to_str()).unwrap_or("untitled");
        let mut c = stem.chars();
        let name = c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default();
        demo::blank(&name)
    }
}

/// Open a stack file; a missing file starts from the matching demo stack.
#[cfg(not(target_arch = "wasm32"))]
fn load(path: &str) -> StackFile {
    match std::fs::read(path) {
        Ok(bytes) => {
            // A format-1 (plain TOML) stack is rewritten as a zip on the next save; keep the original once.
            if !bytes.starts_with(b"PK") { let bak = format!("{path}.v1.bak"); if !std::path::Path::new(&bak).exists() { let _ = std::fs::write(&bak, &bytes); } }
            StackFile::from_bytes(&bytes).unwrap_or_else(|e| { eprintln!("{path}: {e}; opening the demo stack"); demo_for(path) })
        }
        Err(_) => demo_for(path),
    }
}
#[cfg(target_arch = "wasm32")]
fn load(path: &str) -> StackFile {
    // A stack a script made this session (create stack) lives in the in-memory file system.
    let made = shared_fs().borrow().read_bytes(path).ok().and_then(|b| StackFile::from_bytes(&b).ok());
    made.unwrap_or_else(|| demo_for(path))
}

/// Home lives next to the stacks, in its own file, and stays loaded when another stack opens.
#[cfg(not(target_arch = "wasm32"))]
fn home_path() -> String {
    PATH.with(|p| std::path::Path::new(p.borrow().as_str()).parent().map(|d| d.join("home.stack").to_string_lossy().into_owned()).unwrap_or_else(|| "home.stack".into()))
}

fn load_home() -> Obj {
    #[cfg(not(target_arch = "wasm32"))]
    if let Ok(bytes) = std::fs::read(home_path()) {
        if let Some(o) = StackFile::from_bytes(&bytes).ok().and_then(|f| f.objs.into_iter().next()) { return o; }
    }
    demo::home()
}

/// The folder searched for stacks by name, which also holds the demo stacks.
fn stacks_dir() -> String { if cfg!(target_arch = "wasm32") { "/".into() } else { "stacks".into() } }

/// A stack path from the core: `demo:Name` is that demo's file in the stacks folder.
fn real_path(p: &str) -> String {
    match p.strip_prefix("demo:") {
        Some(n) => DEMOS.iter().find(|(d, _)| d.eq_ignore_ascii_case(n.trim().trim_end_matches(".stack"))).map(|(_, path)| path.to_string()).unwrap_or_else(|| format!("stacks/{n}.stack")),
        None => p.to_string(),
    }
}

fn shared_fs() -> Rc<RefCell<Box<dyn Fs>>> { FS.with(|f| f.clone()) }

/// The stack menu: switch to the stack if it is open, otherwise open it in a new tab.
fn open(path: &str) {
    let existing = TABS.with(|t| t.borrow().iter().position(|tab| tab.path == path));
    match existing { Some(i) => activate(i), None => open_in(path, true, false) }
}

/// Open a stack in a new tab, or in place of the showing one (`go to stack`, as HyperCard did).
/// It gets openStack, openBackground, openCard (with startup first when the player starts).
fn open_in(path: &str, new_tab: bool, startup: bool) {
    let file = load(path);
    let home = RT.with(|r| r.borrow().as_ref().and_then(|rt| rt.w.borrow().home.clone())).unwrap_or_else(load_home);
    let rt = Rc::new(Rt::with_home(file, Some(home)));
    {
        let mut w = rt.w.borrow_mut();
        w.fs = shared_fs();
        // Relative file paths start at the stack's own folder.
        w.fs_base = if cfg!(target_arch = "wasm32") { "/".into() } else {
            std::path::Path::new(path).parent().map(|d| d.to_string_lossy().into_owned()).filter(|d| !d.is_empty()).unwrap_or_else(|| ".".into())
        };
        w.stacks_dir = stacks_dir();
    }
    let replace = !new_tab && TABS.with(|t| !t.borrow().is_empty());
    if replace {
        // The stack being replaced is closed.
        let old = rt_opt();
        if let Some(old) = old { let c = old.w.borrow().current_card(); old.send_with(c, "closeStack", vec![]); }
        save();
        let i = ACTIVE.with(|a| *a.borrow());
        TABS.with(|t| t.borrow_mut()[i] = Tab { path: path.to_string(), rt: rt.clone() });
        activate_quiet(i);
    } else {
        let i = TABS.with(|t| { let mut t = t.borrow_mut(); t.push(Tab { path: path.to_string(), rt: rt.clone() }); t.len() - 1 });
        if let Some(old) = rt_opt() { let c = old.w.borrow().current_card(); old.send_with(c, "suspendStack", vec![]); save(); }
        activate_quiet(i);
    }
    if cfg!(target_arch = "wasm32") && startup { DEFERRED_OPEN.with(|d| *d.borrow_mut() = true); } else { rt.open_stack(startup); }
    if SUSPENDED.with(|s| s.borrow_mut().remove(path)) { let c = rt.w.borrow().current_card(); rt.send_with(c, "resumeStack", vec![]); }
    save();
}

fn rt_opt() -> Option<Rc<Rt>> { RT.with(|r| r.borrow().clone()) }

/// Show tab i: suspendStack to the one leaving, resumeStack to the one coming back.
fn activate(i: usize) {
    if ACTIVE.with(|a| *a.borrow()) == i && rt_opt().is_some() { return; }
    if let Some(old) = rt_opt() { let c = old.w.borrow().current_card(); old.send_with(c, "suspendStack", vec![]); save(); }
    activate_quiet(i);
    let rt = rt();
    let c = rt.w.borrow().current_card();
    rt.send_with(c, "resumeStack", vec![]);
    save();
}

/// Make tab i the showing one. Home travels with it, so a Home edit made in one tab applies in all.
fn activate_quiet(i: usize) {
    let home = rt_opt().and_then(|r| r.w.borrow().home.clone());
    let (path, rt) = TABS.with(|t| { let t = t.borrow(); (t[i].path.clone(), t[i].rt.clone()) });
    if let Some(h) = home { rt.w.borrow_mut().home = Some(h); rt.compile(HOME); }
    ACTIVE.with(|a| *a.borrow_mut() = i);
    RT.with(|r| *r.borrow_mut() = Some(rt.clone()));
    PATH.with(|p| *p.borrow_mut() = path);
    URLS.with(|u| u.borrow_mut().clear());
    SENT.with(|s| s.borrow_mut().clear());
    // Every stack can ask `the stacks`.
    let names: Vec<String> = TABS.with(|t| t.borrow().iter().map(|tab| tab.rt.w.borrow().file.name.clone()).collect());
    TABS.with(|t| for tab in t.borrow().iter() { tab.rt.w.borrow_mut().open_stacks = names.clone(); });
}

/// Close tab i (closeStack is sent); the last tab cannot be closed.
fn close_tab(i: usize) {
    let n = TABS.with(|t| t.borrow().len());
    if n <= 1 { return; }
    let was_active = ACTIVE.with(|a| *a.borrow()) == i;
    if was_active { save(); }
    let rt_i = TABS.with(|t| t.borrow()[i].rt.clone());
    let c = rt_i.w.borrow().current_card();
    rt_i.send_with(c, "closeStack", vec![]);
    TABS.with(|t| { t.borrow_mut().remove(i); });
    let active = ACTIVE.with(|a| *a.borrow());
    let next = if was_active { i.min(n - 2) } else if active > i { active - 1 } else { active };
    if was_active { RT.with(|r| *r.borrow_mut() = None); }
    activate_quiet(next);
}

/// Things scripts asked the player to do: show another stack, fetch URLs, persist web files.
fn handle_requests() {
    // Built-in menu items: Undo, Message (focus the message box), Quit HyperCard.
    let (undo, message, quit) = { let r = rt(); let mut w = r.w.borrow_mut(); (std::mem::take(&mut w.undo_request), std::mem::take(&mut w.message_request), std::mem::take(&mut w.quit_request)) };
    if undo { rt().undo(); }
    if message { let _ = document::eval("setTimeout(() => document.querySelector('.foot input')?.focus(), 0)"); }
    if quit {
        save();
        QUIT.with(|q| *q.borrow_mut() = true);
        #[cfg(feature = "desktop")]
        dioxus::desktop::window().close();
    }
    let go = rt().w.borrow_mut().go_stack.take();
    if let Some(g) = go {
        let path = real_path(&g.path);
        let current = PATH.with(|p| p.borrow().clone());
        if path != current {
            let open_tab = TABS.with(|t| t.borrow().iter().position(|tab| tab.path == path));
            match (g.new_window, open_tab) {
                (_, Some(i)) => activate(i),
                (true, None) => open_in(&path, true, false),
                (false, None) => open_in(&path, false, false),
            }
        }
        let fx = g.effect.map(|e| format!(" with {}", e.replace('-', " "))).unwrap_or_default();
        match g.card {
            Some(Value::Int(n)) => rt().exec(&format!("go to card {n}{fx};")),
            Some(v) => rt().exec(&format!("go to card {:?}{fx};", v.text())),
            None => {}
        }
    }
    let urls = std::mem::take(&mut rt().w.borrow_mut().url_requests);
    #[cfg(not(target_arch = "wasm32"))]
    for (url, to) in urls {
        // On the desktop, fetch natively: no browser cross-origin rule ("anything goes").
        let target = Rc::downgrade(&rt());
        let id = FETCH_SEQ.with(|s| { let mut s = s.borrow_mut(); *s += 1; *s });
        FETCH_TARGETS.with(|t| t.borrow_mut().insert(id, (target, url.clone(), to)));
        std::thread::spawn(move || {
            let r = ureq::get(&url).timeout(std::time::Duration::from_secs(20)).call()
                .map_err(|e| e.to_string())
                .and_then(|resp| resp.into_string().map_err(|e| e.to_string()));
            FETCHED.lock().unwrap().push((id, r));
        });
    }
    #[cfg(target_arch = "wasm32")]
    for (url, to) in urls {
        let target = rt();
        spawn(async move {
            let js = format!("fetch({url:?}).then(r => r.ok ? r.text() : Promise.reject(r.status + ' ' + r.statusText)).then(t => dioxus.send([true, t])).catch(e => dioxus.send([false, e instanceof TypeError ? String(e) + ' (the site may not allow pages on other sites to read it: a browser rule; the desktop player has no such limit)' : String(e)]))");
            let mut ch = document::eval(&js);
            if let Ok((ok, text)) = ch.recv::<(bool, String)>().await {
                if ok { target.url_loaded(&url, &text, to); } else { target.w.borrow_mut().log.push(format!("load url {url}: {text}")); target.url_failed(&url, &text, to); }
                handle_requests();
                save();
                DIRTY.with(|d| *d.borrow_mut() = true);
            }
        });
    }
    // On the web, file changes go to the browser's private file system.
    #[cfg(target_arch = "wasm32")]
    {
        let changes = shared_fs().borrow_mut().take_changes();
        if !changes.is_empty() {
            let json = format!("[{}]", changes.iter().map(|(op, p, t)| format!("[{op:?},{p:?},{t:?}]")).collect::<Vec<_>>().join(","));
            let _ = document::eval(&format!(r#"(async () => {{
                const root = await navigator.storage.getDirectory();
                for (const [op, path, text] of {json}) {{
                    const parts = path.split('/').filter(Boolean); const last = parts.pop(); let dir = root;
                    for (const p of parts) dir = await dir.getDirectoryHandle(p, {{ create: true }});
                    if (op === 'write') {{ const f = await dir.getFileHandle(last, {{ create: true }}); const w = await f.createWritable(); await w.write(text); await w.close(); }}
                    else if (op === 'delete') {{ await dir.removeEntry(last); }}
                    else if (op === 'folder') {{ await dir.getDirectoryHandle(last, {{ create: true }}); }}
                }}
            }})()"#));
        }
    }
}

/// Immediate persistence: called after every action.
fn save() {
    #[cfg(not(target_arch = "wasm32"))]
    PATH.with(|p| {
        let path = p.borrow();
        if path.is_empty() { return; }
        let Some(rt) = rt_opt() else { return };
        if let Some(dir) = std::path::Path::new(path.as_str()).parent() { let _ = std::fs::create_dir_all(dir); }
        let write = |path: &str, bytes: Vec<u8>| { let tmp = format!("{path}.tmp"); if std::fs::write(&tmp, bytes).is_ok() { let _ = std::fs::rename(&tmp, path); } };
        write(path.as_str(), rt.bytes());
        if let Some(home) = rt.home_text().and_then(|t| StackFile::from_text(&t).ok()) { write(&home_path(), home.to_zip()); }
    });
}

/// Microseconds spent in `f`. web-time uses performance.now() in the browser (often coarsened to 5–100 µs).
fn timed(f: impl FnOnce()) -> u128 { let t = web_time::Instant::now(); f(); t.elapsed().as_micros() }

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let path = std::env::args().nth(1).unwrap_or_else(|| DEMOS[0].1.into());
    #[cfg(target_arch = "wasm32")]
    // On the web, the page address can name the first stack: index.html#selftest or #Self%20Test.
    let path = {
        let hash = web_sys::window().and_then(|w| w.location().hash().ok()).unwrap_or_default();
        let want: String = hash.trim_start_matches('#').replace("%20", " ").chars().filter(|c| !c.is_whitespace()).collect::<String>().to_lowercase();
        DEMOS.iter().find(|(n, _)| !want.is_empty() && n.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_lowercase() == want).map(|(_, p)| p.to_string()).unwrap_or_else(|| DEMOS[0].1.to_string())
    };
    open_in(&path, true, true);
    dioxus::launch(App);
}

/// requestAnimationFrame drives the frame loop on web and desktop alike.
const RAF: &str = "function f(t){ dioxus.send(t); requestAnimationFrame(f); } requestAnimationFrame(f);";

#[derive(Clone, PartialEq)]
struct PartView { style: String, id: u32, kind: Kind, name: String, text: String, x: i64, y: i64, w: i64, h: i64, locked: bool, visible: bool, transparent: bool, picture: String, enabled: bool, hilite: bool, show_name: bool, dont_wrap: bool, icon: String, auto_hilite: bool, bg: bool }

fn part_views() -> Vec<PartView> {
    let r = rt();
    let w = r.w.borrow();
    w.parts_on(w.current_card()).into_iter().filter_map(|id| {
        let o = w.obj(id)?;
        let n = |k: &str, d: i64| o.props.get(k).map(|v| v.int()).unwrap_or(d);
        Some(PartView { style: o.props.get("style").map(|v| v.text().to_lowercase()).unwrap_or_default(), id, kind: o.kind, name: o.name.clone(), text: w.get(id, "text").text(), x: n("left", 20), y: n("top", 20), w: n("width", 100), h: n("height", 32),
            locked: o.props.get("locked").map(|v| v.truthy()).unwrap_or(false), visible: o.props.get("visible").map(|v| v.truthy()).unwrap_or(true),
            transparent: o.props.get("style").map(|v| v.text() == "transparent").unwrap_or(false), picture: o.props.get("picture").map(|v| v.text()).unwrap_or_default(),
            enabled: w.get(id, "enabled").truthy(), hilite: w.get(id, "hilite").truthy(), show_name: w.get(id, "showName").truthy(), dont_wrap: w.get(id, "dontWrap").truthy(),
            icon: o.props.get("icon").map(|v| v.text()).unwrap_or_default(), auto_hilite: w.get(id, "autoHilite").truthy(),
            bg: o.parent.and_then(|p| w.obj(p)).map(|p| p.kind == Kind::Background).unwrap_or(false) })
    })
    // showPict false on a card or background hides the pictures that belong to it.
    .map(|mut p| { if p.kind == Kind::Picture { let owner = w.obj(p.id).and_then(|o| o.parent); if owner.map(|o| !w.get(o, "showPict").truthy()).unwrap_or(false) { p.visible = false; } } p })
    .collect()
}

const CSS: &str = r#"
:root { --bg:#e9ecef; --ink:#1b2530; --muted:#5d6975; --line:#aab4be; --card:#ffffff; --sel:#1b5cc0; --bad:#b3261e; --panel:#f5f6f8; }
* { box-sizing: border-box; }
html, body { margin:0; height:100%; background:var(--bg); color:var(--ink); font:14px/1.4 system-ui, sans-serif; }
.app { display:flex; flex-direction:column; height:100vh; }
.bar { display:flex; gap:8px; align-items:center; padding:8px 12px; background:var(--panel); border-bottom:1px solid var(--line); flex-wrap:wrap; }
.bar b { margin-right:8px; } .bar .sp { flex:1; }
button.t { font:inherit; padding:4px 10px; border:1px solid var(--line); background:#fff; border-radius:4px; cursor:pointer; }
button.t.on { background:var(--ink); color:#fff; border-color:var(--ink); }
.main { display:flex; flex:1; min-height:0; min-width:0; overflow:hidden; }
.stage { flex:1 1 0; min-width:160px; overflow:auto; padding:24px; }
.card { position:relative; background:var(--card); border:1px solid var(--ink); box-shadow:3px 3px 0 var(--ink); user-select:none; }
.part { position:absolute; }
.part > button, .part > textarea { width:100%; height:100%; font:inherit; margin:0; }
.part > button { border:1.5px solid var(--ink); border-radius:7px; background:#fff; cursor:pointer; }
.part > button:active { background:var(--ink); color:#fff; }
.part > textarea { border:1px solid var(--line); padding:6px 8px; resize:none; background:#fff; color:var(--ink); }
.part > textarea[readonly] { border-color:transparent; background:transparent; color:var(--muted); }
.part > svg { display:block; width:100%; height:100%; background:#f7f4ea; border:1px solid var(--ink); }
.browse .part.canvas, .browse .part.picture { pointer-events:none; }
.checks { display:flex; flex-wrap:wrap; gap:4px 14px; }
.checks label { display:flex; gap:6px; align-items:center; text-transform:none; letter-spacing:0; font-size:13px; color:var(--ink); margin:0; }
.checks input { width:auto; }
.stats { font:12px ui-monospace, Menlo, monospace; color:var(--muted); }
select.t { font:inherit; padding:4px 6px; border:1px solid var(--line); border-radius:4px; background:#fff; }
.edit .part { cursor:move; outline:1px dashed var(--line); }
.edit .part > * { pointer-events:none; }
.edit .part.sel { outline:2px solid var(--sel); }
.edit .part.sel.also { outline:2px dashed var(--sel); }
.bar.sub { padding:6px 12px; gap:6px; font-size:13px; }
.tabs { display:flex; gap:2px; padding:6px 12px 0; background:var(--bg); border-bottom:1px solid var(--line); overflow-x:auto; }
.tab { display:flex; align-items:center; border:1px solid var(--line); border-bottom:0; border-radius:6px 6px 0 0; background:var(--panel); }
.tab.on { background:#fff; font-weight:600; }
.tab button { font:inherit; border:0; background:transparent; padding:5px 10px; cursor:pointer; color:var(--ink); }
.tab .tabclose { padding:5px 8px 5px 0; color:var(--muted); }
.filelist { display:flex; flex-direction:column; max-height:240px; overflow:auto; width:380px; max-width:70vw; border:1px solid var(--line); align-self:stretch; }
.filelist button { font:inherit; text-align:left; border:0; border-bottom:1px solid var(--line); background:#fff; padding:5px 8px; cursor:pointer; color:var(--ink); }
.filelist button:hover { background:var(--panel); }
.filedir { font:12px ui-monospace, Menlo, monospace; color:var(--muted); align-self:stretch; overflow-wrap:anywhere; }
input.t { font:inherit; padding:4px 8px; border:1px solid var(--line); border-radius:4px; background:#fff; color:var(--ink); }
/* The inspector never takes more than the window leaves after a strip of the card area. */
.insp { flex:none; min-width:260px; max-width:calc(100vw - 200px); box-sizing:border-box; border-left:1px solid var(--line); background:var(--panel); padding:12px; overflow:auto; display:flex; flex-direction:column; gap:8px; }
.insp label { font-size:11px; letter-spacing:.06em; text-transform:uppercase; color:var(--muted); display:block; margin-bottom:2px; }
.insp input, .insp textarea { width:100%; font:inherit; padding:4px 6px; border:1px solid var(--line); border-radius:3px; background:#fff; color:var(--ink); }
.grip { flex:none; width:7px; margin-left:-3px; margin-right:-4px; cursor:col-resize; z-index:2; }
.grip:hover, .resizing .grip { background:var(--sel); opacity:.35; }
.resizing, .resizing * { cursor:col-resize !important; user-select:none !important; }
.code { position:relative; flex:1; min-height:220px; overflow:auto; border:1px solid var(--line); border-radius:3px; background:#fff; }
.code pre, .insp .code textarea { margin:0; padding:8px; border:0; border-radius:0; font:13px/1.5 ui-monospace, Menlo, monospace; white-space:pre; tab-size:4; letter-spacing:0; }
.code pre { position:absolute; top:0; left:0; min-width:100%; pointer-events:none; color:var(--ink); }
.insp .code textarea { position:relative; display:block; min-height:100%; resize:none; overflow:hidden; background:transparent; color:transparent; caret-color:var(--ink); outline:none; }
.code textarea::selection { background:rgba(27,92,192,.22); color:transparent; }
.hl-kw { color:#8a3ab9; font-weight:600; } .hl-cmd { color:#1b5cc0; } .hl-obj { color:#0b7a75; } .hl-chunk { color:#0b7a75; font-style:italic; }
.hl-msg { color:#b3261e; font-weight:600; } .hl-str { color:#a35a00; } .hl-num { color:#a35a00; } .hl-com { color:#7c8995; font-style:italic; }
.hl-fun { color:#5b4bb7; } .hl-punct { color:#7c8995; } .hl-op { color:#5d6975; } .hl-id { color:var(--ink); }
.row { display:flex; gap:6px; } .row > div { flex:1; min-width:0; }
.ok { color:var(--muted); font-size:12px; } .bad { color:var(--bad); font-size:12px; white-space:pre-wrap; }
.foot { border-top:1px solid var(--line); background:var(--panel); padding:8px 12px; display:flex; flex-direction:column; gap:4px; }
.foot input { font:13px ui-monospace, Menlo, monospace; padding:5px 8px; border:1px solid var(--ink); border-radius:3px; width:100%; }
.log { font:12px ui-monospace, Menlo, monospace; color:var(--muted); white-space:pre-wrap; max-height:54px; overflow:auto; }
.veil { position:fixed; inset:0; background:rgba(20,30,40,.35); display:flex; align-items:center; justify-content:center; }
.dlg { background:#fff; border:2px solid var(--ink); box-shadow:4px 4px 0 var(--ink); padding:20px 24px; min-width:260px; display:flex; flex-direction:column; gap:14px; align-items:flex-end; }
.dlg p { margin:0; align-self:flex-start; }
.part > button.hilite { background:var(--ink); color:#fff; }
.part > button.noname { color:transparent; }
.part > button.noauto:active { background:#fff; color:var(--ink); }
.part > button img.icon { display:block; margin:0 auto 2px; max-height:60%; max-width:100%; pointer-events:none; }
.part > button:disabled { opacity:.45; cursor:default; }
.part > img { display:block; width:100%; height:100%; pointer-events:none; user-select:none; }
.part.missing { outline:1px dashed var(--bad); }
.part > button.transparent { background:transparent; border:0; border-radius:0; color:transparent; }
.part > button.style-opaque { border:0; border-radius:0; }
.part > button.style-rectangle { border-radius:0; }
.part > button.style-shadow { border-radius:0; box-shadow:2px 2px 0 var(--ink); }
.part > button.style-roundrect { border-radius:12px; }
.part > button.style-checkbox, .part > button.style-radiobutton { border:0; background:transparent; text-align:left; display:flex; align-items:center; gap:6px; }
.part > button.style-checkbox.hilite, .part > button.style-radiobutton.hilite { background:transparent; color:var(--ink); }
.part > button .mark { font-size:16px; line-height:1; order:-1; }
.part > textarea.style-transparent { background:transparent; border-color:transparent; }
.part > textarea.style-opaque { border-color:transparent; }
.part > textarea.style-shadow { box-shadow:2px 2px 0 var(--ink); border-color:var(--ink); }
.part > textarea.style-rectangle { border-color:var(--ink); }
.part > textarea.style-scrolling { overflow-y:scroll; border-color:var(--ink); }
.part > button.transparent:active { background:rgba(255,255,255,.08); color:transparent; }
.edit .part > button.transparent { outline:1px dashed var(--sel); }
.edit .part.hidden { opacity:.35; }
.fx-dissolve { animation: fx-dissolve .6s ease backwards; }
@keyframes fx-dissolve { from { opacity:0; } }
.fx-wipe-left { animation: fx-wipe-left .55s ease backwards; }
@keyframes fx-wipe-left { from { clip-path: inset(0 0 0 100%); } to { clip-path: inset(0 0 0 0); } }
.fx-wipe-right { animation: fx-wipe-right .55s ease backwards; }
@keyframes fx-wipe-right { from { clip-path: inset(0 100% 0 0); } to { clip-path: inset(0 0 0 0); } }
.fx-iris-open { animation: fx-iris .6s ease backwards; }
@keyframes fx-iris { from { clip-path: circle(0% at 50% 50%); } to { clip-path: circle(75% at 50% 50%); } }
.fx-wipe-up { animation: fx-wipe-up .55s ease backwards; }
@keyframes fx-wipe-up { from { clip-path: inset(100% 0 0 0); } to { clip-path: inset(0 0 0 0); } }
.fx-wipe-down { animation: fx-wipe-down .55s ease backwards; }
@keyframes fx-wipe-down { from { clip-path: inset(0 0 100% 0); } to { clip-path: inset(0 0 0 0); } }
.fx-iris-close { animation: fx-iris-close .6s ease backwards; }
@keyframes fx-iris-close { from { clip-path: inset(0 0 0 0 round 0); opacity:.2; } to { clip-path: inset(0 0 0 0); opacity:1; } }
.fx-scroll-left, .fx-push-left { animation: fx-scroll-left .5s ease backwards; }
@keyframes fx-scroll-left { from { transform: translateX(100%); } }
.fx-scroll-right, .fx-push-right { animation: fx-scroll-right .5s ease backwards; }
@keyframes fx-scroll-right { from { transform: translateX(-100%); } }
.fx-scroll-up, .fx-push-up { animation: fx-scroll-up .5s ease backwards; }
@keyframes fx-scroll-up { from { transform: translateY(100%); } }
.fx-scroll-down, .fx-push-down { animation: fx-scroll-down .5s ease backwards; }
@keyframes fx-scroll-down { from { transform: translateY(-100%); } }
.fx-zoom-in, .fx-zoom-open { animation: fx-zoom-open .5s ease backwards; }
@keyframes fx-zoom-open { from { transform: scale(.1); opacity:0; } }
.fx-zoom-out, .fx-zoom-close { animation: fx-zoom-close .5s ease backwards; }
@keyframes fx-zoom-close { from { transform: scale(1.6); opacity:0; } }
.fx-barn-door-open { animation: fx-barn-open .6s ease backwards; }
@keyframes fx-barn-open { from { clip-path: inset(0 50% 0 50%); } to { clip-path: inset(0 0 0 0); } }
.fx-barn-door-close { animation: fx-barn-close .6s ease backwards; }
@keyframes fx-barn-close { from { clip-path: polygon(0 0, 50% 0, 50% 0, 50% 100%, 50% 100%, 0 100%); } to { clip-path: polygon(0 0, 100% 0, 100% 0, 100% 100%, 100% 100%, 0 100%); } }
.fx-venetian-blinds { animation: fx-blinds .6s steps(6) backwards; }
@keyframes fx-blinds { from { mask-image: repeating-linear-gradient(#000 0 2px, transparent 2px 24px); -webkit-mask-image: repeating-linear-gradient(#000 0 2px, transparent 2px 24px); } to { mask-image: repeating-linear-gradient(#000 0 24px, transparent 24px 24px); -webkit-mask-image: repeating-linear-gradient(#000 0 24px, transparent 24px 24px); } }
.fx-checkerboard { animation: fx-dissolve .6s steps(4) backwards; }
.fx-shrink-to-center, .fx-stretch-from-center { animation: fx-zoom-open .5s ease backwards; }
.fx-shrink-to-top, .fx-stretch-from-top { animation: fx-wipe-down .5s ease backwards; }
.fx-shrink-to-bottom, .fx-stretch-from-bottom { animation: fx-wipe-up .5s ease backwards; }
@media (prefers-reduced-motion: reduce) { [class*="fx-"] { animation:none; } }
.menubar { display:flex; align-items:center; gap:2px; padding:2px 8px; border-bottom:1px solid var(--line); background:#fff; font-size:13px; position:relative; z-index:20; }
.menubar .menu { position:relative; }
.menubar .mname { font:inherit; border:0; background:transparent; padding:3px 10px; border-radius:3px; cursor:pointer; }
.menubar .mname.on, .menubar .mname:hover { background:var(--ink); color:#fff; }
.menubar .drop { position:absolute; left:0; top:100%; min-width:180px; background:#fff; border:1px solid var(--ink); box-shadow:2px 2px 0 var(--ink); display:flex; flex-direction:column; padding:3px 0; }
.menubar .item { font:inherit; text-align:left; border:0; background:transparent; padding:4px 14px; cursor:pointer; white-space:nowrap; }
.menubar .item:hover:not(.off) { background:var(--ink); color:#fff; }
.menubar .item.off { color:#aaa; cursor:default; }
.menubar .sep { border-top:1px dotted #999; margin:3px 0; }
.menubar .toolpick { display:flex; gap:6px; align-items:center; color:var(--muted); }
.menubar .sp { flex:1; }
.paintlayer { position:absolute; inset:0; pointer-events:none; z-index:2; }
.paintlayer svg { display:block; }
.paintcatch { position:absolute; inset:0; z-index:40; cursor:crosshair; }
label.t { display:inline-flex; align-items:center; font:inherit; padding:4px 10px; border:1px solid var(--line); background:#fff; border-radius:4px; cursor:pointer; }
label.t input { display:none; }
.assets { display:flex; flex-direction:column; gap:4px; font-size:13px; }
.assets div { display:flex; gap:8px; align-items:center; } .assets span { flex:1; min-width:0; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; }
"#;

#[component]
fn App() -> Element {
    let mut tick = use_signal(|| 0u32);
    let mut edit = use_signal(|| false);
    let mut sel = use_signal(|| None::<u32>);
    // All selected parts (shift-click adds or removes); `sel` is the first, which the inspector shows.
    let mut multi = use_signal(Vec::<u32>::new);
    // A drag in progress: start point, current offset. Positions are written once, on release.
    let mut drag = use_signal(|| None::<(f64, f64, f64, f64)>);
    let mut new_card_form = use_signal(|| None::<(String, String)>);
    let mut msg = use_signal(String::new);
    // The open menu in the menu bar, and the stroke being painted (card coordinates).
    let mut open_menu = use_signal(|| None::<usize>);
    let mut stroke = use_signal(Vec::<(f64, f64)>::new);
    let mut ask_text = use_signal(String::new);
    let mut file_dir = use_signal(|| None::<String>);
    let mut compile_us = use_signal(|| 0u128);
    let mut stats = use_signal(|| (0.0f64, 0u128));
    let mut insp_w = use_signal(|| 420.0f64);
    let mut resize = use_signal(|| None::<(f64, f64)>);
    let _ = tick();

    // Frame loop: in browse mode, a card with a canvas part gets `frame` once per display frame.
    use_future(move || async move {
        let mut raf = document::eval(RAF);
        let mut times: std::collections::VecDeque<f64> = Default::default();
        let mut cost = 0u128;
        let mut last_idle = 0.0f64;
        while let Ok(t) = raf.recv::<f64>().await {
            // `edit the script of …` opens the editor on that object.
            if let Some(id) = rt().w.borrow_mut().edit_request.take() { rt().w.borrow_mut().tool = "button".into(); edit.set(true); sel.set(Some(id)); multi.set(vec![]); tick += 1; }
            // `choose button tool` / `choose browse tool` from a script switch the player's mode.
            {
                let tool = rt().w.borrow().tool.clone();
                let wants_edit = tool == "button" || tool == "field";
                if wants_edit != edit() { edit.set(wants_edit); tick += 1; }
            }
            if DIRTY.with(|d| std::mem::take(&mut *d.borrow_mut())) { flush_sound(); tick += 1; }
            #[cfg(not(target_arch = "wasm32"))]
            if deliver_fetches() { flush_sound(); tick += 1; }
            // A paused script (answer, ask, wait): end its wait when the time comes, check `wait until` again.
            let now = web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).map(|d| d.as_secs_f64() * 1000.0).unwrap_or(0.0);
            if rt().tick(now) { handle_requests(); save(); flush_sound(); tick += 1; }
            if edit() { times.clear(); continue; }
            // `select …` from a script: show it in the field, after the redraw that the command caused.
            if let Some((f, a, b)) = rt().w.borrow_mut().select_request.take() {
                let _ = document::eval(&format!("setTimeout(() => {{ const t = document.querySelector('textarea[data-id=\"{f}\"]'); if (t) {{ t.focus({{ preventScroll: true }}); t.setSelectionRange({a}, {b}); }} }}, 80)"));
            }
            // About ten times a second: idle, for stacks that handle it.
            if now - last_idle >= 100.0 { last_idle = now; if rt().idle() { handle_requests(); flush_sound(); tick += 1; } }
            // Browse mode: due `send … in` messages, then mouseStillDown / mouseWithin, then frame.
            // Nothing of that while a script is paused: the stack is modal until it finishes.
            if rt().is_paused() { times.clear(); continue; }
            // Save only when something was written: these messages can arrive every frame.
            let before = { let r = rt(); let w = r.w.borrow(); (w.file.journal.len(), w.file.journal.last().map(|c| format!("{:?}", c.new))) };
            let mut changed = rt().run_timers(now) > 0;
            let (pressed, hover, down) = { let r = rt(); let w = r.w.borrow(); (w.pressed, w.hover, w.mouse_down) };
            if let (Some(p), true) = (pressed, down) { rt().send_with(p, "mouseStillDown", vec![]); changed = true; }
            if let Some(h) = hover { rt().send_with(h, "mouseWithin", vec![]); changed = true; }
            if changed {
                let after = { let r = rt(); let w = r.w.borrow(); (w.file.journal.len(), w.file.journal.last().map(|c| format!("{:?}", c.new))) };
                handle_requests();
                if after != before { save(); }
                flush_sound();
                tick += 1;
            }
            let mut ran = false;
            cost = (cost * 7 + timed(|| ran = rt().frame())) / 8;
            flush_sound();
            if !ran { times.clear(); continue; }
            times.push_back(t);
            if times.len() > 60 { times.pop_front(); }
            if times.len() > 1 {
                let fps = (times.len() - 1) as f64 * 1000.0 / (times.back().unwrap() - times.front().unwrap());
                stats.set((fps, cost));
            }
            tick += 1;
        }
    });

    // Text selection in fields (for the selection, selectedText, type) and window size (sizeWindow).
    use_future(move || async move {
        let mut ch = document::eval(r#"
            document.addEventListener('selectionchange', () => { const a = document.activeElement; if (a && a.dataset && a.dataset.id) dioxus.send(['sel', +a.dataset.id, a.selectionStart, a.selectionEnd]); });
            let t; window.addEventListener('resize', () => { clearTimeout(t); t = setTimeout(() => dioxus.send(['size', innerWidth, innerHeight, 0]), 150); });
            dioxus.send(['size', innerWidth, innerHeight, 1]);
            document.addEventListener('visibilitychange', () => dioxus.send(['vis', document.hidden ? 1 : 0, 0, 0]));
            let sx = screenX, sy = screenY;
            dioxus.send(['move', sx, sy, 1]);
            setInterval(() => { if (screenX !== sx || screenY !== sy) { sx = screenX; sy = screenY; dioxus.send(['move', sx, sy, 0]); } }, 500);
            await new Promise(() => {});"#);
        while let Ok((kind, a, b, c)) = ch.recv::<(String, f64, f64, f64)>().await {
            let r = rt();
            let card = r.w.borrow().current_card();
            match kind.as_str() {
                "sel" => r.w.borrow_mut().selection = Some((a as u32, b as usize, c as usize)),
                // The window's size and place, for the screenRect; sizeWindow and moveWindow when they change.
                "size" => {
                    TABS.with(|t| for tab in t.borrow().iter() { tab.rt.w.borrow_mut().screen = (a, b); });
                    if c == 0.0 && !edit() { r.send_with(card, "sizeWindow", vec![]); save(); tick += 1; }
                }
                "move" => {
                    TABS.with(|t| for tab in t.borrow().iter() { tab.rt.w.borrow_mut().window_loc = (a, b); });
                    if c == 0.0 && !edit() { r.send_with(card, "moveWindow", vec![]); save(); tick += 1; }
                }
                // The player hidden or shown again (another window or tab in front): suspend / resume.
                "vis" => { r.send_with(card, if a == 1.0 { "suspend" } else { "resume" }, vec![]); handle_requests(); save(); tick += 1; }
                _ => {}
            }
        }
    });

    // On the web, the file system is restored from the browser's private storage at start.
    #[cfg(target_arch = "wasm32")]
    use_future(move || async move {
        let mut ch = document::eval(r#"
            const out = [];
            async function walk(dir, prefix) {
                for await (const [name, h] of dir.entries()) {
                    if (h.kind === 'file') out.push([prefix + '/' + name, await (await h.getFile()).text(), true]);
                    else { out.push([prefix + '/' + name, '', false]); await walk(h, prefix + '/' + name); }
                }
            }
            try { await walk(await navigator.storage.getDirectory(), ''); } catch (e) {}
            dioxus.send(out);"#);
        if let Ok(entries) = ch.recv::<Vec<(String, String, bool)>>().await {
            let fs = shared_fs();
            for (p, t, is_file) in entries { fs.borrow_mut().preload(&p, if is_file { Some(&t) } else { None }); }
        }
        // Now the stack can open: its openStack handler sees the restored files.
        if DEFERRED_OPEN.with(|d| std::mem::take(&mut *d.borrow_mut())) {
            rt().open_stack(true);
            handle_requests();
            save();
            flush_sound();
            tick += 1;
        }
    });

    // The local time zone, for the date and the time.
    use_future(move || async move {
        let mut ch = document::eval("dioxus.send(-new Date().getTimezoneOffset())");
        if let Ok(min) = ch.recv::<f64>().await { rt().w.borrow_mut().tz_offset_min = min as i64; }
    });

    // Test hook (desktop): HC_AUTODRIVE names a JavaScript file of UI actions to run inside this window;
    // what it sends back is appended to HC_AUTODRIVE_OUT. Without the variable this does nothing.
    #[cfg(not(target_arch = "wasm32"))]
    use_future(move || async move {
        let Ok(script) = std::env::var("HC_AUTODRIVE") else { return };
        let out = std::env::var("HC_AUTODRIVE_OUT").unwrap_or_else(|_| "autodrive.log".into());
        let js = std::fs::read_to_string(&script).unwrap_or_default();
        let mut ch = document::eval(&js);
        while let Ok(line) = ch.recv::<String>().await {
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&out) { let _ = writeln!(f, "{line}"); }
        }
    });

    // Keyboard messages go to the player, so it takes focus when it starts.
    use_effect(|| { let _ = document::eval("setTimeout(() => document.querySelector('.app')?.focus({ preventScroll: true }), 0)"); });

    // Browser audio: reports when a sound ends, so `the sound` clears and scripts get `soundDone`.
    use_future(move || async move {
        let mut ch = document::eval(SOUND_JS);
        while let Ok(m) = ch.recv::<String>().await {
            if m != "ended" { rt().w.borrow_mut().log.push(format!("sound: {m}")); }
            rt().sound_finished();
            flush_sound();
            save();
            tick += 1;
        }
    });

    // After any action: write the file, start or stop sounds, redraw.
    let mut done = move || { handle_requests(); save(); flush_sound(); tick += 1; };

    let r = rt();
    let (name, width, height, card, bg, stack, ncards, cardno, dialog, error, log, fx, fx_seq, assets) = {
        let w = r.w.borrow();
        let c = w.current_card();
        (w.file.name.clone(), w.file.width, w.file.height, c, w.background_of(c).unwrap_or(0), w.stack_id(), w.cards().len(), w.card_number(c),
         w.dialog.clone(), w.last_error.clone(), w.log.iter().rev().take(4).rev().cloned().collect::<Vec<_>>().join("\n"),
         w.effect.clone().unwrap_or_default(), w.effect_seq, w.file.assets.clone())
    };
    let (dialog_buttons, dialog_ask, dialog_file) = { let w = r.w.borrow(); (w.dialog_buttons.clone(), w.dialog_ask.clone(), w.dialog_file.clone()) };
    // An ask dialog starts with its default answer; the box is cleared when no dialog is open.
    if let Some(d) = dialog_ask.clone() { if ask_text.peek().is_empty() && !d.is_empty() { ask_text.set(d); } }
    if dialog.is_none() && !ask_text.peek().is_empty() { ask_text.set(String::new()); }
    let pictures: Vec<String> = assets.iter().filter(|a| a.kind == "picture").map(|a| a.name.clone()).collect();
    let parts = part_views();
    let draws = r.w.borrow().draw.clone();
    let animating = !edit() && parts.iter().any(|p| p.kind == Kind::Canvas);
    let (fps, frame_us) = stats();
    // The user level (HyperCard's five): below 4 there is no editing, at 4 no script editing, at 1 no typing.
    let user_level = r.w.borrow().user_level;
    let (tool, menus, message_visible, paint) = { let w = r.w.borrow(); (w.tool.clone(), w.menus.clone(), w.message_visible, w.get(card, "paint").text()) };
    let painting = !edit() && user_level >= 3 && !matches!(tool.as_str(), "browse" | "button" | "field");
    let paint_svg_markup = paint_svg(&paint, if painting && !stroke().is_empty() { Some((tool.as_str(), r.w.borrow().line_size, r.w.borrow().filled, stroke())) } else { None }, width, height);
    if edit() && user_level < 4 { edit.set(false); r.w.borrow_mut().tool = "browse".into(); }
    let editing = edit() && user_level >= 4;
    let target = sel().filter(|id| r.w.borrow().obj(*id).is_some()).unwrap_or(card);
    let (t_kind, t_name, t_script, t_text, t_rect, t_err) = {
        let w = r.w.borrow();
        let o = w.obj(target).cloned();
        let n = |k: &str| w.get(target, k).int();
        (o.as_ref().map(|o| o.kind).unwrap_or(Kind::Card), o.as_ref().map(|o| o.name.clone()).unwrap_or_default(), o.map(|o| o.script).unwrap_or_default(),
         w.get(target, "text").text(), [n("left"), n("top"), n("width"), n("height")], r.script_error(target))
    };
    let is_part = matches!(t_kind, Kind::Button | Kind::Field | Kind::Canvas | Kind::Picture);
    let (t_picture, t_transparent) = { let w = r.w.borrow(); (w.get(target, "picture").text(), w.get(target, "style").text() == "transparent") };
    let t_flags = { let w = r.w.borrow(); let o = w.obj(target); let f = |k: &str, d: bool| o.and_then(|o| o.props.get(k)).map(|v| v.truthy()).unwrap_or(d); (f("visible", true), f("locked", false), f("sharedText", false)) };
    let script_lines = t_script.split('\n').count();
    let script_cols = t_script.split('\n').map(|l| l.chars().map(|c| if c == '\t' { 4 } else { 1 }).sum::<usize>()).max().unwrap_or(0);
    let toks = highlight::tokens(&t_script);
    let kind_label = format!("{t_kind:?}").to_lowercase();

    let new_part = move |kind: Kind| {
        let r = rt();
        let id = {
            let mut w = r.w.borrow_mut();
            w.begin();
            let card = w.current_card();
            let label = match kind { Kind::Button => "New Button", Kind::Canvas => "new canvas", _ => "new field" };
            let id = w.create(kind, label, Some(card), None);
            for (k, v) in [("left", 40), ("top", 40), ("width", 130), ("height", 34)] { w.set(id, k, Value::Int(v), false); }
            if kind == Kind::Button { w.set(id, "text", label.into(), false); w.set(id, "script", "on mouseUp {\n    \n}".into(), false); }
            id
        };
        r.compile(id);
        match kind { Kind::Button => { r.send_with(id, "newButton", vec![]); } Kind::Field => { r.send_with(id, "newField", vec![]); } _ => {} }
        sel.set(Some(id));
        multi.set(vec![id]);
        done();
    };

    // Duplicate the selected parts (copies become the selection), or the current card.
    let duplicate = move || {
        let r = rt();
        let ids = multi();
        let new: Vec<u32> = {
            let mut w = r.w.borrow_mut();
            w.begin();
            if ids.is_empty() || sel() == Some(w.current_card()) {
                let c = w.current_card();
                w.duplicate(c).into_iter().collect()
            } else {
                ids.iter().filter_map(|id| w.duplicate(*id)).collect()
            }
        };
        for id in &new { r.compile(*id); }
        let is_card = new.first().map(|id| r.w.borrow().obj(*id).map(|o| o.kind == Kind::Card).unwrap_or(false)).unwrap_or(false);
        if is_card {
            let c = new[0];
            for id in r.w.borrow().file.objs.iter().filter(|o| o.parent == Some(c)).map(|o| o.id).collect::<Vec<_>>() { r.compile(id); }
            r.exec(&format!("go to card {};", r.w.borrow().card_number(c)));
            sel.set(Some(c));
            multi.set(vec![]);
        } else {
            sel.set(new.first().copied());
            multi.set(new);
        }
        save(); flush_sound(); tick += 1;
    };
    let mut arrange = move |how: &'static str| {
        let r = rt();
        { let mut w = r.w.borrow_mut(); w.begin(); w.arrange(&multi(), how); }
        done();
    };
    let create_card = move || {
        let Some((n, b)) = new_card_form() else { return };
        let r = rt();
        let c = { let mut w = r.w.borrow_mut(); w.begin(); let bg = b.parse::<u32>().ok(); w.new_card_named(&n, bg) };
        r.exec(&format!("go to card {};", r.w.borrow().card_number(c)));
        new_card_form.set(None);
        sel.set(Some(c));
        multi.set(vec![]);
        save(); flush_sound(); tick += 1;
    };
    let backgrounds: Vec<(u32, String)> = { let w = r.w.borrow(); w.backgrounds().into_iter().map(|b| (b, w.get(b, "name").text())).collect() };
    let n_sel = multi().len();

    rsx! {
        style { dangerous_inner_html: CSS }
        div { class: if resize().is_some() { "app resizing" } else { "app" }, tabindex: "0",
            onmousemove: move |e| { if let Some((sx, sw)) = resize() { insp_w.set((sw + sx - e.client_coordinates().x).clamp(260.0, 1100.0)); } },
            onmouseup: move |_| { if resize().is_some() { resize.set(None); } { let r = rt(); let mut w = r.w.borrow_mut(); w.mouse_down = false; w.pressed = None; } },
            onkeydown: move |e| {
                if editing && (e.modifiers().ctrl() || e.modifiers().meta()) && e.key() == Key::Character("d".into()) {
                    e.prevent_default();
                    let mut f = duplicate; f();
                    return;
                }
                let key = e.key().to_string();
                rt().w.borrow_mut().keys.insert(key.clone());
                // F1 … F15: functionKey <n>.
                if !editing { if let Some(n) = key.strip_prefix('F').and_then(|n| n.parse::<i64>().ok()).filter(|n| (1..=15).contains(n)) {
                    let card = rt().w.borrow().current_card();
                    if rt().send_with(card, "functionKey", vec![Value::Int(n)]) { e.prevent_default(); }
                    done();
                    return;
                } }
                if !editing && (e.modifiers().ctrl() || e.modifiers().meta()) && key.chars().count() == 1 {
                    let card = rt().w.borrow().current_card();
                    if rt().send_with(card, "commandKeyDown", vec![Value::Str(key.clone())]) { e.prevent_default(); }
                    done();
                    return;
                }
                if !editing && !e.is_auto_repeating() {
                    let r = rt();
                    let card = r.w.borrow().current_card();
                    let arrow = match key.as_str() { "ArrowLeft" => Some("left"), "ArrowRight" => Some("right"), "ArrowUp" => Some("up"), "ArrowDown" => Some("down"), _ => None };
                    if let Some(dir) = arrow {
                        // arrowKey: left/right go to the previous/next card unless a handler takes it.
                        if !r.send_with(card, "arrowKey", vec![Value::Str(dir.into())]) && r.w.borrow().last_error.is_none() {
                            if dir == "left" { r.exec("go prev;"); } else if dir == "right" { r.exec("go next;"); }
                        }
                    } else if key.chars().count() == 1 || key == "Enter" || key == "Tab" || key == "Backspace" {
                        r.send_with(card, "keyDown", vec![Value::Str(key.clone())]);
                    }
                    done();
                }
            },
            onkeyup: move |e| { rt().w.borrow_mut().keys.remove(&e.key().to_string()); },
            if QUIT.with(|q| *q.borrow()) {
                div { class: "veil", style: "z-index:1000",
                    div { class: "dlg", p { "HyperCard has quit. Your stacks are saved." }
                        div { class: "row", button { class: "t on", onclick: move |_| { let _ = document::eval("location.reload()"); }, "Start again" } }
                    }
                }
            }
            div { class: "tabs",
                for (i, (tname, active)) in TABS.with(|t| t.borrow().iter().enumerate().map(|(i, tab)| (tab.rt.w.borrow().file.name.clone(), i == ACTIVE.with(|a| *a.borrow()))).collect::<Vec<_>>()).into_iter().enumerate() {
                    div { key: "{i}", class: if active { "tab on" } else { "tab" },
                        button { class: "tabname", onclick: move |_| { activate(i); sel.set(None); multi.set(vec![]); tick += 1; }, "{tname}" }
                        if TABS.with(|t| t.borrow().len()) > 1 {
                            button { class: "tabclose", title: "Close {tname}", onclick: move |e| { e.stop_propagation(); close_tab(i); sel.set(None); multi.set(vec![]); tick += 1; }, "×" }
                        }
                    }
                }
            }
            div { class: "menubar", onmouseleave: move |_| open_menu.set(None),
                for (mi, m) in menus.into_iter().enumerate() {
                    div { key: "{mi}", class: "menu",
                        button { class: if open_menu() == Some(mi) { "mname on" } else { "mname" },
                            onclick: move |_| open_menu.set(if open_menu() == Some(mi) { None } else { Some(mi) }),
                            onmouseenter: move |_| { if open_menu().is_some() { open_menu.set(Some(mi)); } },
                            "{m.name}" }
                        if open_menu() == Some(mi) {
                            div { class: "drop",
                                for (ii, it) in m.items.clone().into_iter().enumerate() {
                                    if it.text == "-" { div { key: "{ii}", class: "sep" } }
                                    else {
                                        { let (text, mname) = (it.text.clone(), m.name.clone());
                                          rsx! { button { key: "{ii}", class: if it.enabled { "item" } else { "item off" }, disabled: !it.enabled,
                                              onclick: move |_| { open_menu.set(None); rt().do_menu(&text, &mname); done(); },
                                              if it.checked { "✓ " } "{it.text}" } } }
                                    }
                                }
                            }
                        }
                    }
                }
                if user_level >= 3 {
                    span { class: "sp" }
                    label { class: "toolpick", "Tool "
                        select { class: "t", value: "{tool}",
                            onchange: move |e| { rt().exec(&format!("choose {} tool;", e.value())); done(); },
                            for t in stack_core::TOOLS.iter().filter(|t| user_level >= 4 || !matches!(**t, "button" | "field")).filter(|t| !matches!(**t, "select" | "lasso" | "bucket" | "text")) {
                                option { value: "{t}", selected: *t == tool, "{t}" }
                            }
                        }
                    }
                }
            }
            div { class: "bar",
                b { "{name}" }
                button { class: if !editing { "t on" } else { "t" }, onclick: move |_| { edit.set(false); drag.set(None); rt().w.borrow_mut().tool = "browse".into(); }, "Browse" }
                if user_level >= 4 { button { class: if editing { "t on" } else { "t" }, onclick: move |_| { edit.set(true); rt().w.borrow_mut().tool = "button".into(); }, "Edit" } }
                if editing {
                    button { class: "t", onclick: move |_| { let mut f = new_part; f(Kind::Button) }, "+ Button" }
                    button { class: "t", onclick: move |_| { let mut f = new_part; f(Kind::Field) }, "+ Field" }
                    button { class: "t", onclick: move |_| { let mut f = new_part; f(Kind::Canvas) }, "+ Canvas" }
                    label { class: "t", title: "Add pictures to the stack and place them on this card",
                        "+ Picture"
                        input { r#type: "file", accept: "image/*", multiple: true, onchange: move |e| async move {
                            for f in e.files() {
                                let Ok(bytes) = f.read_bytes().await else { continue };
                                let (name, mime) = asset_name_and_type(&f.name());
                                let r = rt();
                                let id = {
                                    let mut w = r.w.borrow_mut();
                                    w.begin();
                                    w.add_asset(&name, &mime, bytes.to_vec());
                                    let card = w.current_card();
                                    let id = w.create(Kind::Picture, &name, Some(card), None);
                                    let (cw, ch) = (w.file.width, w.file.height);
                                    for (k, v) in [("left", 0), ("top", 0), ("width", cw), ("height", ch - 60)] { w.set(id, k, Value::Int(v), false); }
                                    w.set(id, "picture", Value::Str(name.clone()), false);
                                    id
                                };
                                sel.set(Some(id));
                                multi.set(vec![id]);
                            }
                            done();
                        } }
                    }
                    label { class: "t", title: "Add sounds to the stack; play them with: play \"name\";",
                        "+ Sound"
                        input { r#type: "file", accept: "audio/*", multiple: true, onchange: move |e| async move {
                            for f in e.files() {
                                let Ok(bytes) = f.read_bytes().await else { continue };
                                let (name, mime) = asset_name_and_type(&f.name());
                                let r = rt();
                                { let mut w = r.w.borrow_mut(); w.begin(); w.add_asset(&name, &mime, bytes.to_vec()); }
                                r.w.borrow_mut().log.push(format!("added sound \"{name}\": play it with  play \"{name}\";"));
                                SENT.with(|s| s.borrow_mut().remove(&name));
                            }
                            done();
                        } }
                    }
                    button { class: "t", title: "New card after this one, with a name and a background", onclick: move |_| {
                        let bg = rt().w.borrow().background_of(rt().w.borrow().current_card()).map(|b| b.to_string()).unwrap_or_default();
                        new_card_form.set(Some((String::new(), bg)));
                    }, "+ Card" }
                    button { class: "t", title: "Duplicate the selected parts, or the card (Ctrl+D)", onclick: move |_| { let mut f = duplicate; f(); }, "Duplicate" }
                    button { class: "t", onclick: move |_| {
                        let r = rt();
                        let ids = if multi().len() > 1 { multi() } else { vec![sel().unwrap_or(r.w.borrow().current_card())] };
                        let mut deleted = false;
                        r.w.borrow_mut().begin();
                        for id in ids {
                            let (is_stack_or_bg, last_card) = { let w = r.w.borrow(); let k = w.obj(id).map(|o| o.kind); (matches!(k, Some(Kind::Stack) | Some(Kind::Background) | None), k == Some(Kind::Card) && w.cards().len() <= 1) };
                            if !is_stack_or_bg && !last_card { r.w.borrow_mut().delete(id); r.compile(id); deleted = true; }
                        }
                        if deleted { sel.set(None); multi.set(vec![]); done(); }
                    }, "Delete" }
                }
                button { class: "t", onclick: move |_| { rt().undo(); done(); }, "Undo" }
                select { class: "t", value: "{name}",
                    onchange: move |e| { if let Some((_, path)) = DEMOS.iter().find(|(n, _)| *n == e.value()) {
                        save(); open(path); sel.set(None); multi.set(vec![]); tick += 1;
                    } },
                    for (n, _) in DEMOS { option { value: "{n}", "{n}" } }
                }
                span { class: "sp" }
                if animating { span { class: "stats", "{fps:.0} fps · frame script {frame_us} µs" } }
                button { class: "t", onclick: move |_| { rt().exec("go prev;"); done(); }, "◀" }
                span { "card {cardno} of {ncards}" }
                button { class: "t", onclick: move |_| { rt().exec("go next;"); done(); }, "▶" }
            }
            if editing && n_sel > 1 {
                div { class: "bar sub",
                    span { class: "stats", "{n_sel} parts selected · first one is the reference" }
                    span { "Align" }
                    for (label, how) in [("left", "left"), ("top", "top"), ("right", "right"), ("bottom", "bottom")] {
                        button { key: "{how}", class: "t", onclick: move |_| arrange(how), "{label}" }
                    }
                    span { "Same" }
                    for (label, how) in [("width", "width"), ("height", "height"), ("size", "size")] {
                        button { key: "{how}", class: "t", onclick: move |_| arrange(how), "{label}" }
                    }
                    span { "Spread" }
                    button { class: "t", title: "Even gaps, left to right (3 or more parts)", onclick: move |_| arrange("across"), "across" }
                    button { class: "t", title: "Even gaps, top to bottom (3 or more parts)", onclick: move |_| arrange("down"), "down" }
                }
            }
            if let Some((cname, cbg)) = new_card_form() {
                div { class: "bar sub",
                    span { "New card named" }
                    input { class: "t", value: "{cname}", placeholder: "card name", autofocus: true,
                        oninput: move |e| { let b = new_card_form().map(|(_, b)| b).unwrap_or_default(); new_card_form.set(Some((e.value(), b))); },
                        onkeydown: move |e| { if e.key() == Key::Enter { let mut f = create_card; f(); } if e.key() == Key::Escape { new_card_form.set(None); } } }
                    span { "on" }
                    select { class: "t",
                        onchange: move |e| { let (n, _) = new_card_form().unwrap_or_default(); new_card_form.set(Some((n, e.value()))); },
                        for (b, bname) in backgrounds.clone() { option { value: "{b}", selected: cbg == b.to_string(), "{bname}" } }
                        option { value: "new", selected: cbg == "new", "a new background" }
                    }
                    button { class: "t on", onclick: move |_| { let mut f = create_card; f(); }, "Create" }
                    button { class: "t", onclick: move |_| new_card_form.set(None), "Cancel" }
                }
            }
            div { class: "main",
                div { class: "stage",
                    div {
                        key: "{card}-{fx_seq}",
                        class: if editing { "card edit".to_string() } else if fx.is_empty() { "card browse".to_string() } else { format!("card browse fx-{fx}") },
                        style: "width:{width}px;height:{height}px;",
                        onmousedown: move |e| {
                            if editing { sel.set(None); multi.set(vec![]); return; }
                            let p = e.element_coordinates();
                            { let r = rt(); let mut w = r.w.borrow_mut(); w.mouse_down = true; w.click_loc = (p.x, p.y); }
                            let card = rt().w.borrow().current_card();
                            rt().send_with(card, "mouseDown", vec![]);
                            done();
                        },
                        onclick: move |_| { if !editing { rt().w.borrow_mut().mouse_click = true; let card = rt().w.borrow().current_card(); rt().send(card, "mouseUp"); done(); } },
                        onmousemove: move |e| {
                            if !editing { let p = e.element_coordinates(); rt().w.borrow_mut().mouse = (p.x, p.y); }
                            if let Some((sx, sy, _, _)) = drag() {
                                let p = e.client_coordinates();
                                drag.set(Some((sx, sy, p.x - sx, p.y - sy)));
                            }
                        },
                        onmouseup: move |_| {
                            rt().w.borrow_mut().mouse_down = false;
                            if let Some((_, _, dx, dy)) = drag() {
                                drag.set(None);
                                let (dx, dy) = (dx.round() as i64, dy.round() as i64);
                                if dx != 0 || dy != 0 {
                                    let r = rt();
                                    let mut w = r.w.borrow_mut();
                                    w.begin();
                                    for id in multi() {
                                        let (l, t) = (w.get(id, "left").int(), w.get(id, "top").int());
                                        w.set(id, "left", Value::Int(l + dx), false);
                                        w.set(id, "top", Value::Int(t + dy), false);
                                    }
                                }
                                done();
                            }
                        },
                        div { class: "paintlayer", dangerous_inner_html: "{paint_svg_markup}" }
                        // With a paint tool, the card is for painting: drags become strokes.
                        if painting {
                            div { class: "paintcatch",
                                onmousedown: move |e| { e.stop_propagation(); let p = e.element_coordinates(); stroke.set(vec![(p.x, p.y)]); },
                                onmousemove: move |e| { if !stroke().is_empty() { let p = e.element_coordinates(); stroke.write().push((p.x, p.y)); } },
                                onmouseup: move |e| { e.stop_propagation(); let pts = stroke(); stroke.set(vec![]); if !pts.is_empty() { rt().paint(&pts); done(); } },
                                onmouseleave: move |_| { let pts = stroke(); stroke.set(vec![]); if !pts.is_empty() { rt().paint(&pts); done(); } },
                                onclick: move |e| e.stop_propagation(),
                            }
                        }
                        for p in parts.into_iter().filter(|p| p.visible || editing) {
                            div {
                                key: "{p.id}",
                                class: match (editing && multi().contains(&p.id), p.kind, p.visible) { (true, _, _) => if sel() == Some(p.id) { "part sel" } else { "part sel also" }, (_, _, false) => "part hidden", (false, Kind::Canvas, _) => "part canvas", (false, Kind::Picture, _) => "part picture", _ => "part" },
                                style: {
                                    let (ox, oy) = match drag() { Some((_, _, dx, dy)) if multi().contains(&p.id) => (dx.round() as i64, dy.round() as i64), _ => (0, 0) };
                                    // HyperCard's layers: background parts, then the card's paint, then card parts.
                                    format!("left:{}px;top:{}px;width:{}px;height:{}px;z-index:{};", p.x + ox, p.y + oy, p.w, p.h, if p.bg { 1 } else { 3 })
                                },
                                title: "{p.name}",
                                onmouseenter: move |_| { if !editing && p.kind != Kind::Picture { rt().w.borrow_mut().hover = Some(p.id); rt().send_with(p.id, "mouseEnter", vec![]); done(); } },
                                onmouseleave: move |_| { if !editing && p.kind != Kind::Picture { { let r = rt(); let mut w = r.w.borrow_mut(); if w.hover == Some(p.id) { w.hover = None; } } rt().send_with(p.id, "mouseLeave", vec![]); done(); } },
                                onmousedown: move |e| {
                                    if !editing && matches!(p.kind, Kind::Button | Kind::Field) {
                                        e.stop_propagation();
                                        let c = e.element_coordinates();
                                        { let r = rt(); let mut w = r.w.borrow_mut(); w.mouse_down = true; w.pressed = Some(p.id); w.click_loc = (p.x as f64 + c.x, p.y as f64 + c.y); }
                                        // A second press within 400 ms on the same part is the second click of a double-click:
                                        // HyperCard sends only mouseDoubleClick for it, on release.
                                        let now = web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).map(|d| d.as_secs_f64() * 1000.0).unwrap_or(0.0);
                                        let double = LAST_DOWN.with(|l| { let (id, t) = *l.borrow(); id == p.id && now - t < 400.0 });
                                        LAST_DOWN.with(|l| *l.borrow_mut() = if double { (0, 0.0) } else { (p.id, now) });
                                        DOUBLE.with(|d| *d.borrow_mut() = double);
                                        if p.enabled && !double { rt().send_with(p.id, "mouseDown", vec![]); done(); }
                                    }
                                    if editing {
                                        e.stop_propagation();
                                        let mut m = multi();
                                        if e.modifiers().shift() {
                                            // Shift-click adds or removes a part; no drag.
                                            if let Some(i) = m.iter().position(|x| *x == p.id) { m.remove(i); } else { m.push(p.id); }
                                            sel.set(m.first().copied());
                                            multi.set(m);
                                            return;
                                        }
                                        if !m.contains(&p.id) { m = vec![p.id]; multi.set(m); sel.set(Some(p.id)); }
                                        let c = e.client_coordinates();
                                        drag.set(Some((c.x, c.y, 0.0, 0.0)));
                                    }
                                },
                                if p.kind == Kind::Picture {
                                    if let Some(url) = asset_url(&p.picture) { img { src: "{url}", alt: "{p.name}", draggable: "false" } }
                                    else { div { class: "part missing", style: "position:static;width:100%;height:100%;", "missing picture \"{p.picture}\"" } }
                                } else if p.kind == Kind::Canvas {
                                    svg { view_box: "0 0 {p.w} {p.h}",
                                        for (i, s) in draws.get(&p.id).cloned().unwrap_or_default().into_iter().enumerate() {
                                            match s {
                                                Shape::Rect { x, y, w, h, color } => rsx! { rect { key: "{i}", x: "{x}", y: "{y}", width: "{w}", height: "{h}", fill: "{color}" } },
                                                Shape::Circle { x, y, r, color } => rsx! { circle { key: "{i}", cx: "{x}", cy: "{y}", r: "{r}", fill: "{color}" } },
                                                Shape::Text { x, y, text, color } => rsx! { text { key: "{i}", x: "{x}", y: "{y}", fill: "{color}", font_size: "16", "{text}" } },
                                            }
                                        }
                                    }
                                } else if p.kind == Kind::Button {
                                    button {
                                        class: format!("{}{}{}{} style-{}", if p.transparent { "transparent" } else { "" }, if p.hilite { " hilite" } else { "" }, if p.show_name { "" } else { " noname" }, if p.auto_hilite { "" } else { " noauto" }, p.style),
                                        disabled: !p.enabled,
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            if editing || !p.enabled { return; }
                                            rt().w.borrow_mut().mouse_click = true;
                                            if DOUBLE.with(|d| std::mem::take(&mut *d.borrow_mut())) { rt().send(p.id, "mouseDoubleClick"); } else { rt().click(p.id); }
                                            done();
                                        },
                                        if !p.icon.is_empty() { if let Some(url) = asset_url(&p.icon) { img { class: "icon", src: "{url}", alt: "", draggable: "false" } } }
                                        if p.style == "checkbox" || p.style == "radiobutton" { span { class: "mark", aria_hidden: "true", if p.hilite { if p.style == "checkbox" { "☑" } else { "◉" } } else if p.style == "checkbox" { "☐" } else { "○" } } }
                                        "{p.text}"
                                    }
                                } else {
                                    textarea {
                                        value: "{p.text}",
                                        readonly: p.locked || user_level < 2,
                                        class: "style-{p.style}",
                                        "data-id": "{p.id}",
                                        wrap: if p.dont_wrap { "off" } else { "soft" },
                                        // A locked field acts like a button; an unlocked one is for typing.
                                        onclick: move |e| async move {
                                            e.stop_propagation();
                                            if editing || !p.locked { return; }
                                            // Where in the text the click landed: a read-only textarea still places the caret.
                                            let mut ch = document::eval(&format!("const t = document.querySelector('textarea[data-id=\"{}\"]'); dioxus.send(t ? t.selectionStart : -1)", p.id));
                                            let at = ch.recv::<f64>().await.ok().filter(|a| *a >= 0.0).map(|a| a as usize);
                                            rt().click_field(p.id, at);
                                            done();
                                        },
                                        onfocus: move |_| {
                                            rt().w.borrow_mut().focused = Some(p.id);
                                            if !editing && !p.locked { FOCUS_TEXT.with(|f| *f.borrow_mut() = Some((p.id, p.text.clone()))); rt().send_with(p.id, "openField", vec![]); done(); }
                                        },
                                        onkeydown: move |e| {
                                            e.stop_propagation();
                                            if !editing && e.key() == Key::Enter {
                                                let msg = if e.code() == Code::NumpadEnter { "enterInField" } else { "returnKey" };
                                                // A handler that takes the key stops the newline.
                                                if rt().send_with(p.id, msg, vec![]) { e.prevent_default(); }
                                                done();
                                            }
                                        },
                                        oninput: move |e| { let r = rt(); { let mut w = r.w.borrow_mut(); w.begin(); w.set(p.id, "text", Value::Str(e.value()), true); } done(); },
                                        onblur: move |_| {
                                            if editing { return; }
                                            // closeField when the text changed while editing, exitField when it did not.
                                            let before = FOCUS_TEXT.with(|f| f.borrow_mut().take()).filter(|(id, _)| *id == p.id).map(|(_, t)| t);
                                            let now = rt().w.borrow().get(p.id, "text").text();
                                            let msg = if before.as_deref() == Some(now.as_str()) { "exitField" } else { "closeField" };
                                            rt().send(p.id, msg);
                                            done();
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
                if editing {
                    div { class: "grip", title: "Drag to resize",
                        onmousedown: move |e| { e.prevent_default(); resize.set(Some((e.client_coordinates().x, insp_w()))); } }
                    div { class: "insp", style: "width:{insp_w}px;",
                        div { class: "row",
                            button { class: if target == card { "t on" } else { "t" }, onclick: move |_| { sel.set(Some(card)); multi.set(vec![]); }, "Card" }
                            button { class: if target == bg { "t on" } else { "t" }, onclick: move |_| { sel.set(Some(bg)); multi.set(vec![]); }, "Background" }
                            button { class: if target == stack { "t on" } else { "t" }, onclick: move |_| { sel.set(Some(stack)); multi.set(vec![]); }, "Stack" }
                            button { class: if target == HOME { "t on" } else { "t" }, title: "Handlers here run for every stack", onclick: move |_| { sel.set(Some(HOME)); multi.set(vec![]); }, "Home" }
                        }
                        if t_kind == Kind::Card {
                            div { label { "background of this card" }
                                select { class: "t", style: "width:100%",
                                    onchange: move |e| { if let Ok(b) = e.value().parse::<u32>() { let r = rt(); { let mut w = r.w.borrow_mut(); w.begin(); w.set_background(target, b); } done(); } },
                                    for (b, bname) in backgrounds.clone() { option { value: "{b}", selected: b == bg, "{bname}" } }
                                }
                            }
                        }
                        div { label { "{kind_label} name" }
                            input { value: "{t_name}", oninput: move |e| { let r = rt(); { let mut w = r.w.borrow_mut(); w.begin(); w.set(target, "name", Value::Str(e.value()), true); } done(); } }
                        }
                        if is_part {
                            if t_kind == Kind::Picture {
                                div { label { "picture" }
                                    select { class: "t", style: "width:100%", value: "{t_picture}",
                                        onchange: move |e| { let r = rt(); { let mut w = r.w.borrow_mut(); w.begin(); w.set(target, "picture", Value::Str(e.value()), false); } done(); },
                                        for n in pictures.clone() { option { value: "{n}", selected: n == t_picture, "{n}" } }
                                    }
                                }
                            }
                            div { class: "checks",
                                label { title: "Hidden parts show faintly in edit mode",
                                    input { r#type: "checkbox", checked: t_flags.0, onchange: move |e| { let r = rt(); { let mut w = r.w.borrow_mut(); w.begin(); w.set(target, "visible", Value::Bool(e.checked()), false); } done(); } }
                                    "visible" }
                                if t_kind == Kind::Button {
                                    label { title: "A hotspot over a picture",
                                        input { r#type: "checkbox", checked: t_transparent, onchange: move |e| { let r = rt(); { let mut w = r.w.borrow_mut(); w.begin(); w.set(target, "style", Value::Str(if e.checked() { "transparent".into() } else { "standard".into() }), false); } done(); } }
                                        "transparent" }
                                }
                                if t_kind == Kind::Field {
                                    label { title: "Locked text cannot be typed into while browsing",
                                        input { r#type: "checkbox", checked: t_flags.1, onchange: move |e| { let r = rt(); { let mut w = r.w.borrow_mut(); w.begin(); w.set(target, "locked", Value::Bool(e.checked()), false); } done(); } }
                                        "locked text" }
                                    label { title: "On a background field: the same text on every card",
                                        input { r#type: "checkbox", checked: t_flags.2, onchange: move |e| { let r = rt(); { let mut w = r.w.borrow_mut(); w.begin(); w.set(target, "sharedText", Value::Bool(e.checked()), false); } done(); } }
                                        "shared text" }
                                }
                            }
                            if !matches!(t_kind, Kind::Canvas | Kind::Picture) { div { label { if t_kind == Kind::Button { "label" } else { "text on this card" } }
                                input { value: "{t_text}", oninput: move |e| { let r = rt(); { let mut w = r.w.borrow_mut(); w.begin(); w.set(target, "text", Value::Str(e.value()), true); } done(); } }
                            } }
                            div { class: "row",
                                for (i, key) in ["left", "top", "width", "height"].into_iter().enumerate() {
                                    div { key: "{key}", label { "{key}" }
                                        input { r#type: "number", value: "{t_rect[i]}", oninput: move |e| {
                                            if let Ok(v) = e.value().parse::<i64>() { let r = rt(); { let mut w = r.w.borrow_mut(); w.begin(); w.set(target, key, Value::Int(v), true); } done(); }
                                        } }
                                    }
                                }
                            }
                        }
                        if t_kind == Kind::Stack && target != HOME && !assets.is_empty() {
                            div { label { "pictures and sounds in this stack" }
                                div { class: "assets",
                                    for a in assets.clone() {
                                        div { key: "{a.name}",
                                            span { title: "{a.file}", "{a.kind}: {a.name}" }
                                            if a.kind == "sound" {
                                                button { class: "t", onclick: move |_| { rt().exec(&format!("play {:?};", a.name)); done(); }, "▶" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if user_level < 5 { p { class: "ok", "Scripts are shown and edited at user level 5 (set the userLevel to 5)." } }
                        if user_level >= 5 {
                        label { if target == HOME { "script of Home (every stack)" } else { "script of this {kind_label}" } }
                        div { class: "code",
                            pre { aria_hidden: "true",
                                for (c, t) in toks { span { class: "hl-{c}", "{t}" } }
                                "\n "
                            }
                            textarea { class: "script", spellcheck: "false", wrap: "off", value: "{t_script}",
                                rows: "{script_lines + 1}",
                                style: "width:max(100%, calc({script_cols + 2}ch + 16px));",
                                oninput: move |e| { let us = timed(|| rt().set_script(target, &e.value())); compile_us.set(us); done(); },
                                onkeydown: move |e| { e.stop_propagation(); if e.key() == Key::Tab { e.prevent_default(); let _ = document::eval("document.execCommand('insertText', false, '    ')"); } },
                            }
                        }
                        if let Some(err) = t_err {
                            div { class: "bad", "{err}\n(the last working version is still running)" }
                        } else {
                            div { class: "ok", "live · last edit applied in {compile_us} µs" }
                        }
                        }
                    }
                }
            }
            div { class: "foot",
                if message_visible { input { placeholder: "message box: type a statement and press Enter, e.g.  go next;   or   put \"hi\" into field \"notes\";", value: "{msg}",
                    oninput: move |e| msg.set(e.value()),
                    onkeydown: move |e| { e.stop_propagation(); if e.key() == Key::Enter { rt().exec(&msg()); msg.set(String::new()); done(); } }
                } }
                if let Some(err) = error { div { class: "bad", "{err}" } }
                if !log.is_empty() { div { class: "log", "{log}" } }
            }
            if let Some(text) = dialog {
                div { class: "veil",
                    div { class: "dlg", p { "{text}" }
                        if let Some((saving, start)) = dialog_file.clone() {
                            {
                                // answer file / ask file: browse folders, pick a file or name one.
                                let dir = file_dir().unwrap_or(start);
                                let (files, folders) = shared_fs().borrow().list(&dir).unwrap_or_default();
                                let up = stack_core::files::resolve_path(&dir, "..");
                                rsx! {
                                    div { class: "filedir", "{dir}" }
                                    div { class: "filelist",
                                        button { onclick: move |_| file_dir.set(Some(up.clone())), "⬑ .." }
                                        for f in folders { { let d = stack_core::files::resolve_path(&dir, &f); rsx! { button { key: "d{f}", onclick: move |_| file_dir.set(Some(d.clone())), "▸ {f}/" } } } }
                                        for f in files {
                                            { let full = stack_core::files::resolve_path(&dir, &f); let name = f.clone();
                                              rsx! { button { key: "f{f}", onclick: move |_| { if saving { ask_text.set(name.clone()); } else { file_dir.set(None); rt().reply(&full); done(); } }, "{f}" } } }
                                        }
                                    }
                                    if saving {
                                        input { class: "t", style: "width:100%", value: "{ask_text}", placeholder: "file name",
                                            oninput: move |e| ask_text.set(e.value()) }
                                    }
                                    div { class: "row",
                                        button { class: "t", onclick: move |_| { file_dir.set(None); rt().cancel(); done(); }, "Cancel" }
                                        if saving {
                                            { let d2 = dir.clone(); rsx! { button { class: "t on", onclick: move |_| { let p = stack_core::files::resolve_path(&d2, &ask_text()); file_dir.set(None); rt().reply(&p); done(); }, "Save" } } }
                                        }
                                    }
                                }
                            }
                        } else if dialog_ask.is_some() {
                            input { class: "t", style: "width:100%", value: "{ask_text}", autofocus: true,
                                oninput: move |e| ask_text.set(e.value()),
                                onkeydown: move |e| { e.stop_propagation(); if e.key() == Key::Enter { rt().reply(&ask_text()); done(); } } }
                            div { class: "row",
                                button { class: "t", onclick: move |_| { rt().cancel(); done(); }, "Cancel" }
                                button { class: "t on", onclick: move |_| { rt().reply(&ask_text()); done(); }, "OK" }
                            }
                        } else {
                            div { class: "row",
                                for (i, b) in (if dialog_buttons.is_empty() { vec!["OK".to_string()] } else { dialog_buttons.clone() }).into_iter().enumerate() {
                                    button { key: "{i}", class: if i == 0 { "t on" } else { "t" }, onclick: move |_| { rt().reply(&b); done(); }, "{b}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// The card's paint as SVG: one shape per stroke ("<tool> <lineSize> <filled> h,v h,v …"), plus the stroke
/// being painted. Black ink on the card, as in HyperCard; the eraser paints the card colour.
fn paint_svg(paint: &str, live: Option<(&str, i64, bool, Vec<(f64, f64)>)>, w: i64, h: i64) -> String {
    let mut out = format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">"#);
    let mut shape = |tool: &str, size: i64, filled: bool, pts: &[(f64, f64)]| {
        if pts.is_empty() { return; }
        let ink = "#111";
        let (a, b) = (pts[0], *pts.last().unwrap());
        let poly = |pts: &[(f64, f64)]| pts.iter().map(|(x, y)| format!("{x:.0},{y:.0}")).collect::<Vec<_>>().join(" ");
        let fill = if filled { ink } else { "none" };
        let (x, y, bw, bh) = (a.0.min(b.0), a.1.min(b.1), (a.0 - b.0).abs(), (a.1 - b.1).abs());
        let line = |width: f64, color: &str, extra: &str| if pts.len() == 1 {
            format!(r#"<circle cx="{:.0}" cy="{:.0}" r="{}" fill="{color}"/>"#, a.0, a.1, (width / 2.0).max(0.5))
        } else {
            format!(r#"<polyline points="{}" fill="none" stroke="{color}" stroke-width="{width}" stroke-linecap="round" stroke-linejoin="round" {extra}/>"#, poly(pts))
        };
        out.push_str(&match tool {
            "pencil" => line(1.0, ink, ""),
            "brush" => line(6.0, ink, ""),
            "eraser" => line(14.0, "#fff", ""),
            "spray" => line(8.0, ink, r#"stroke-dasharray="0.5 4" opacity="0.7""#),
            "curve" | "polygon" => line(size as f64, ink, ""),
            "line" => format!(r#"<line x1="{:.0}" y1="{:.0}" x2="{:.0}" y2="{:.0}" stroke="{ink}" stroke-width="{size}" stroke-linecap="round"/>"#, a.0, a.1, b.0, b.1),
            "rectangle" => format!(r#"<rect x="{x:.0}" y="{y:.0}" width="{bw:.0}" height="{bh:.0}" fill="{fill}" stroke="{ink}" stroke-width="{size}"/>"#),
            "round_rect" | "round rect" => format!(r#"<rect x="{x:.0}" y="{y:.0}" width="{bw:.0}" height="{bh:.0}" rx="10" fill="{fill}" stroke="{ink}" stroke-width="{size}"/>"#),
            "oval" => format!(r#"<ellipse cx="{:.0}" cy="{:.0}" rx="{:.0}" ry="{:.0}" fill="{fill}" stroke="{ink}" stroke-width="{size}"/>"#, x + bw / 2.0, y + bh / 2.0, bw / 2.0, bh / 2.0),
            "regular_polygon" | "regular polygon" => {
                // Drawn from its centre, as in HyperCard: six sides through the end point.
                let r = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
                let t0 = (b.1 - a.1).atan2(b.0 - a.0);
                let v: Vec<(f64, f64)> = (0..6).map(|k| { let t = t0 + k as f64 * std::f64::consts::PI / 3.0; (a.0 + r * t.cos(), a.1 + r * t.sin()) }).collect();
                format!(r#"<polygon points="{}" fill="{fill}" stroke="{ink}" stroke-width="{size}"/>"#, poly(&v))
            }
            _ => String::new(),
        });
    };
    for l in paint.lines() {
        let mut it = l.split_whitespace();
        let (Some(tool), Some(size), Some(filled)) = (it.next(), it.next(), it.next()) else { continue };
        let pts: Vec<(f64, f64)> = it.filter_map(|p| p.split_once(',')).filter_map(|(x, y)| Some((x.parse().ok()?, y.parse().ok()?))).collect();
        shape(tool, size.parse().unwrap_or(1), filled == "1", &pts);
    }
    if let Some((tool, size, filled, pts)) = live { shape(tool, size, filled, &pts); }
    out.push_str("</svg>");
    out
}

/// Asset name from a file name (without extension) and its media type from the extension.
fn asset_name_and_type(file: &str) -> (String, String) {
    let (stem, ext) = file.rsplit_once('.').unwrap_or((file, ""));
    let mime = match ext.to_lowercase().as_str() {
        "png" => "image/png", "jpg" | "jpeg" => "image/jpeg", "gif" => "image/gif", "webp" => "image/webp", "svg" => "image/svg+xml",
        "wav" => "audio/wav", "mp3" => "audio/mpeg", "ogg" => "audio/ogg", "m4a" => "audio/mp4", "flac" => "audio/flac",
        _ => "application/octet-stream",
    };
    (stem.to_string(), mime.to_string())
}

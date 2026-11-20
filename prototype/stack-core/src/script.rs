//! Script engine: stock Rhai plus English-shaped commands, and the message path.
//! Path: part -> card -> background -> stack -> built-in. Commands are messages first.
use crate::model::{Stroke, canon_prop, default_menus, AssetKind, GoStack, Kind, Menu, MenuItem, Obj, OpenFile, PlayerRequest, Shape, SoundCmd, StackFile, Tool, Value, World, HOME, LIB0, RECENT_MAX, VISIT0};
use rhai::{Dynamic, Engine, EvalAltResult, ImmutableString, Position, Scope, AST};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type R<T> = Result<T, Box<EvalAltResult>>;
pub type W = Rc<RefCell<World>>;

/// One handler invocation. `obj` is `me` (the object whose script runs); `target` is where the message started.
struct Frame { obj: u32, target: u32, msg: String, args: Vec<Dynamic>, handled: bool, passed: bool, globals: Vec<String>, ret: Option<Dynamic> }

/// Function handlers (`function f a, b`) answer function calls, not messages: their frames carry this prefix.
const FN_PREFIX: char = '\u{1}';

/// HyperTalk words that are also Rhai method names; registered capitalized, see hypertalk_words.
/// Also `break` and `continue`, so they work inside `repeat` (custom syntax, which Rhai's parser does not see as a loop).
const COLLIDE: [&str; 6] = ["sort", "push", "pop", "contains", "break", "continue"];
const CHUNKS: [&str; 5] = ["word", "line", "item", "char", "character"];
const ORDINALS: [&str; 13] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "last", "middle", "any"];
/// Plural words after `the number of`.
const PLURALS: [&str; 10] = ["cards", "buttons", "fields", "pictures", "backgrounds", "lines", "words", "items", "chars", "characters"];
const RESERVED: [&str; 16] = ["field", "button", "card", "canvas", "picture", "me", "target", "the", "word", "line", "item", "char", "true", "false", "file", "url"];

/// Parse state for `put`: into/before/after, the chunks named (outermost last), and the target.
#[derive(Clone, Default)]
struct PutSt { mode: String, kinds: Vec<String>, ranged: Vec<bool>, field: bool, file: bool, selection: bool, phase: u8 }

/// Parse state for `go`: how (next, prev, first, last, to) and an optional visual effect.
#[derive(Clone, Default)]
struct GoSt { how: String, effect: Vec<String>, phase: u8, card: bool, stack: bool, new_window: bool }

/// Parse state for an object reference:
/// me | target | this card|background|stack | card [name] | background [name] | stack | field|button|picture|canvas <name>.
/// A name is optional after card/background; it is taken when the next token is a literal or `(`.
#[derive(Clone, Default)]
struct RefSt { kind: String, has_key: bool, by_id: bool, phase: u8 }

impl RefSt {
    /// The kind as `resolve` takes it: "button", or "button#id" for `button id 7`.
    fn k(&self) -> String { if self.by_id { format!("{}#id", self.kind) } else { self.kind.clone() } }

    /// Next symbol for the reference, or None when it is complete and `look` belongs to the caller.
    fn step(&mut self, look: &str) -> Option<ImmutableString> {
        match self.phase {
            0 => match look {
                "me" | "target" => { self.kind = look.into(); self.phase = 9; Some(look.into()) }
                // `stack` alone is this stack; `stack "name"` another one.
                "stack" => { self.kind = look.into(); self.phase = 2; Some(look.into()) }
                "this" => { self.phase = 1; Some("this".into()) }
                // `the target`, as HyperTalk writes it
                "the" => { self.phase = 7; Some("the".into()) }
                "card" | "background" => { self.kind = look.into(); self.phase = 2; Some(look.into()) }
                _ => { self.kind = look.into(); self.phase = 3; Some(look.into()) }
            },
            1 => { self.kind = look.into(); self.phase = 9; Some(look.into()) }
            7 => { self.kind = "target".into(); self.phase = 9; Some("target".into()) }
            2 | 3 if look == "id" && !self.by_id => { self.by_id = true; self.phase = 3; Some("id".into()) }
            2 => if look.starts_with('"') || look.starts_with('(') || look.chars().next().is_some_and(|c| c.is_ascii_digit()) { self.has_key = true; self.phase = 9; Some(slot(look)) } else { None },
            3 => { self.has_key = true; self.phase = 9; Some(slot(look)) }
            _ => None,
        }
    }
}

/// `set [the] <prop> [of <ref>] to <value>`
#[derive(Clone, Default)]
struct SetSt { r: RefSt, has_ref: bool, phase: u8 }

/// `the <prop> [of <ref>]`, `the <ordinal> <chunk> of <container>`, `the number of <plural> [of <container>]`.
#[derive(Clone, Default)]
struct TheSt { mode: u8, chunk: String, r: RefSt, var: bool, cont: i64, phase: u8 }

/// `<ordinal> <chunk> of <container>`, `there is [not] a <ref>`, `add/subtract/multiply/divide`.
#[derive(Clone, Default)]
struct OrdSt { chunk: String, var: bool, cont: i64 }
#[derive(Clone, Default)]
struct ThereSt { not: bool, r: RefSt, phase: u8 }
#[derive(Clone, Default)]
struct ArithSt { field: bool, phase: u8 }

#[derive(Default)]
struct Live { asts: HashMap<u32, Rc<AST>>, errors: HashMap<u32, String>, frames: Vec<Frame>,
    /// HyperTalk-style globals: runtime state shared by all handlers, not saved with the stack.
    globals: Rc<HashMap<String, Dynamic>>,
    /// Canvases named by enclosing `with canvas` blocks; drawing goes to the innermost.
    canvas: Vec<u32>,
    /// Variables each handler sets with `put … into <name>`, by object and message. They are
    /// declared when the handler starts, so a `put` inside an `if` block sets a handler-wide local.
    locals: HashMap<u32, HashMap<String, Vec<String>>>,
    /// What the running script has seen from outside, for running it again after a pause.
    tape: Rc<RefCell<Tape>>,
    /// What a paused script waits for.
    waiting: Option<Waiting>,
    /// Where the player's queues stood when a paused script started again: what the script queues
    /// again before it reaches its pause point (sounds, fetches, …) already happened, and is dropped.
    settle: Option<Settle> }
type L = Rc<RefCell<Live>>;

pub struct Rt { pub engine: Engine, pub w: W, live: L, paused: RefCell<Option<Paused>>, later: RefCell<std::collections::VecDeque<Later>> }

// ---- Pausing: answer, ask and wait ------------------------------------------------------------------
//
// A Rhai call cannot be suspended. A script that has to wait for the user (answer, ask) or for time
// (wait) stops instead, and when the wait is over it runs again from the start of what set it going
// (a click, a key, a message box line). Before that, everything it changed is rolled back with the
// journal, and the runtime state it touched is restored. On the second run, everything it reads from
// outside the stack (the time, the mouse, keys, files, the reply it waited for) comes from the tape
// recorded on the first run, so it takes the same path; what it did outside the stack (file writes,
// sounds, fetches) is not done again. When it reaches the pause point it is live again.
// While a script is paused the stack is modal, as in HyperCard: no other messages are delivered.

/// What a paused script waits for.
#[derive(Clone, Debug, PartialEq)]
pub enum Waiting {
    /// A dialog (`answer`, `ask`, `answer file`, `ask file`): the player calls `reply` or `cancel`.
    Dialog,
    /// `wait N ticks|seconds`: until this time (ms since 1970).
    Until(f64),
    /// `wait until …` / `wait while …`: the condition is checked again at this time (ms since 1970).
    Poll(f64),
}

/// Inputs from outside, in the order a script read them.
#[derive(Default)]
struct Tape {
    /// False outside a run that may pause (frame, errorDialog): pausing there is an error.
    on: bool,
    entries: Vec<(String, Dynamic)>,
    pos: usize,
    replaying: bool,
    /// Set when a run going again got past what the tape holds.
    went_live: bool,
    /// The value the pause point gets when the run going again reaches it (the dialog reply).
    resume: Option<Dynamic>,
    /// Set if the run went a different way the second time (it read something not on the tape).
    diverged: Option<String>,
}

impl Tape {
    /// Read an outside input: from the tape when running again, otherwise for real (and recorded).
    fn io(&mut self, kind: &str, f: impl FnOnce() -> Dynamic) -> Dynamic {
        if self.replaying {
            if let Some((k, v)) = self.entries.get(self.pos) {
                if k == kind { self.pos += 1; return v.clone(); }
                self.diverged.get_or_insert(format!("it read {kind} where it read {k} before"));
            }
            self.entries.truncate(self.pos);
            self.replaying = false;
            self.went_live = true;
        }
        let v = f();
        if self.on { self.entries.push((kind.to_string(), v.clone())); self.pos = self.entries.len(); }
        v
    }
    /// At a pause point, past the end of the tape: the run is live again. True when that just happened.
    fn go_live(&mut self) -> bool {
        if self.replaying && self.pos >= self.entries.len() { self.replaying = false; self.went_live = true; return true; }
        false
    }
}

#[derive(Clone)]
struct Settle { sounds: usize, urls: usize, go: Option<GoStack>, edit: Option<u32>, requests: usize }

/// The run reached live again: drop what it queued for the player a second time.
fn settle(w: &W, live: &L) {
    let Some(s) = live.borrow_mut().settle.take() else { return };
    let mut wb = w.borrow_mut();
    wb.sound_cmds.truncate(s.sounds);
    wb.url_requests.truncate(s.urls);
    wb.go_stack = s.go;
    wb.edit_request = s.edit;
    wb.player_requests.truncate(s.requests);
}

/// Why a run stops early. Carried in Rhai's `ErrorTerminated`, which `try` cannot catch, so it unwinds the whole run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Halt {
    /// A pause point (answer, ask, wait): the run goes on later.
    Pause,
    /// `exit to HyperCard`: the run ends quietly.
    ExitAll,
}
fn halt(h: Halt) -> Box<EvalAltResult> { Box::new(EvalAltResult::ErrorTerminated(Dynamic::from(h), Position::NONE)) }
fn halted(e: &EvalAltResult) -> Option<Halt> { match e { EvalAltResult::ErrorTerminated(v, _) => v.read_lock::<Halt>().map(|h| *h), _ => None } }
fn is_exit_all(e: &EvalAltResult) -> bool { halted(e) == Some(Halt::ExitAll) }
fn pause(live: &L, why: Waiting) -> Box<EvalAltResult> {
    live.borrow_mut().waiting = Some(why);
    halt(Halt::Pause)
}
fn is_pause(e: &EvalAltResult) -> bool { halted(e) == Some(Halt::Pause) }

/// A pause point. Running again after the pause, it returns the reply (Some); the first time, None:
/// the caller shows its dialog (or sets its timer) and returns `pause(…)`.
fn pause_point(w: &W, live: &L, kind: &str) -> R<Option<Dynamic>> {
    let tape = live.borrow().tape.clone();
    let mut t = tape.borrow_mut();
    if !t.on { return Err(err("answer, ask and wait cannot pause here: frame and errorDialog handlers run to the end")); }
    if t.replaying {
        if let Some((k, v)) = t.entries.get(t.pos).cloned() {
            if k == kind { t.pos += 1; return Ok(Some(v)); }
            t.diverged.get_or_insert(format!("it reached {kind} where it reached {k} before"));
            let p = t.pos;
            t.entries.truncate(p);
        }
    }
    let now_live = t.go_live();
    let r = t.resume.take();
    if let Some(v) = &r { t.entries.push((kind.to_string(), v.clone())); t.pos = t.entries.len(); }
    drop(t);
    if now_live { settle(w, live); }
    Ok(r)
}

/// A dialog reply goes into `it`; Cancel leaves `it` empty and sets the result to "Cancel".
fn reply_into_it(ctx: &mut rhai::EvalContext, w: &W, r: Dynamic) -> R<Dynamic> {
    if r.is_unit() { ctx.scope_mut().set_value("it", Dynamic::from(String::new())); w.borrow_mut().result = "Cancel".into(); }
    else { ctx.scope_mut().set_value("it", r); w.borrow_mut().result.clear(); }
    Ok(Dynamic::UNIT)
}

fn now_ms() -> f64 { web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).map(|d| d.as_secs_f64() * 1000.0).unwrap_or(0.0) }

/// The file system as a script sees it during a run: every call goes through the tape, so running again
/// after a pause reads what it read the first time and writes nothing twice.
struct TapeFs { inner: Rc<RefCell<Box<dyn crate::files::Fs>>>, tape: Rc<RefCell<Tape>> }
impl TapeFs {
    fn rec<T: Clone + 'static>(&self, kind: String, f: impl FnOnce() -> T) -> T {
        self.tape.borrow_mut().io(&kind, || Dynamic::from(f())).cast::<T>()
    }
}
impl crate::files::Fs for TapeFs {
    fn read(&self, path: &str) -> Result<String, String> { self.rec(format!("read {path}"), || self.inner.borrow().read(path)) }
    fn read_bytes(&self, path: &str) -> Result<Vec<u8>, String> { self.rec(format!("read bytes {path}"), || self.inner.borrow().read_bytes(path)) }
    fn write(&mut self, path: &str, text: &str) -> Result<(), String> { self.rec(format!("write {path}"), || self.inner.borrow_mut().write(path, text)) }
    fn write_bytes(&mut self, path: &str, bytes: &[u8]) -> Result<(), String> { self.rec(format!("write bytes {path}"), || self.inner.borrow_mut().write_bytes(path, bytes)) }
    fn delete(&mut self, path: &str) -> Result<(), String> { self.rec(format!("delete {path}"), || self.inner.borrow_mut().delete(path)) }
    fn rename(&mut self, from: &str, to: &str) -> Result<(), String> { self.rec(format!("rename {from}"), || self.inner.borrow_mut().rename(from, to)) }
    fn create_folder(&mut self, path: &str) -> Result<(), String> { self.rec(format!("create folder {path}"), || self.inner.borrow_mut().create_folder(path)) }
    fn is_file(&self, path: &str) -> bool { self.rec(format!("is file {path}"), || self.inner.borrow().is_file(path)) }
    fn is_folder(&self, path: &str) -> bool { self.rec(format!("is folder {path}"), || self.inner.borrow().is_folder(path)) }
    fn list(&self, path: &str) -> Result<(Vec<String>, Vec<String>), String> { self.rec(format!("list {path}"), || self.inner.borrow().list(path)) }
    fn take_changes(&mut self) -> Vec<crate::files::FsChange> { self.inner.borrow_mut().take_changes() }
    fn preload(&mut self, path: &str, text: Option<&str>) { self.inner.borrow_mut().preload(path, text) }
}

/// What set a run going, so it can run again after a pause.
enum Entry {
    /// Messages sent in order to one object (stopping at an error). `auto`: system messages, whose
    /// changes Undo skips.
    Send { obj: u32, msgs: Vec<(String, Vec<Dynamic>)>, auto: bool },
    /// A message box line.
    Exec(String),
}

/// Runtime state a run may change outside the journal, taken when it starts.
/// The runtime state a run may change outside the journal, named once: `Snap` holds a copy, taken when a
/// run starts and put back when a paused run goes on. The bulky fields are `Rc`s, so a copy is cheap and
/// only a run that writes one pays for cloning it.
macro_rules! run_state {
    ($($f:ident: $t:ty),* $(,)?) => {
        struct Snap { mark: usize, txn: u64, next_id: u32, current_card: u32, log_len: usize, globals: Rc<HashMap<String, Dynamic>>, seed: u64, $($f: $t),* }
        fn take_snap(wb: &World, live: &Live) -> Snap {
            Snap { mark: wb.file.journal.len(), txn: wb.file.txn, next_id: wb.file.next_id, current_card: wb.file.current_card,
                   log_len: wb.log.len(), globals: live.globals.clone(), seed: rand_seed(), $($f: wb.$f.clone()),* }
        }
        /// Put the state back (the journal is rolled back separately); returns the journal mark.
        fn put_snap(wb: &mut World, live: &mut Live, s: Snap) -> usize {
            (wb.file.txn, wb.file.next_id, wb.file.current_card) = (s.txn, s.next_id, s.current_card);
            wb.log.truncate(s.log_len);
            live.globals = s.globals;
            set_rand_seed(s.seed);
            $(wb.$f = s.$f;)*
            s.mark
        }
    };
}
run_state! {
    result: String, item_delim: String, timers: Vec<(f64, u32, String)>, push_list: Vec<u32>, selection: Option<(u32, usize, usize)>,
    focused: Option<u32>, user_level: i64, open_files: Rc<HashMap<String, OpenFile>>, effect: Option<String>, effect_seq: u64,
    sound: Option<String>, draw: Rc<HashMap<u32, Vec<Shape>>>, mouse_click: bool, click_loc: (f64, f64), libs: Rc<Vec<Obj>>,
    visited: Rc<Vec<Obj>>, tool: Tool, line_size: i64, filled: bool, menus: Rc<Vec<Menu>>, recent: std::collections::VecDeque<u32>,
    lock_recent: bool,
}

struct Paused { entry: Entry, snap: Snap, tape: Vec<(String, Dynamic)> }

/// Work that arrived while a script was paused (a fetch result, soundDone), done when it finishes.
type Later = Box<dyn FnOnce(&Rt)>;

/// Read outside state through the tape (the time, the mouse, keys).
fn taped(live: &L, kind: &str, f: impl FnOnce() -> Dynamic) -> Dynamic {
    let tape = live.borrow().tape.clone();
    let mut t = tape.borrow_mut();
    t.io(kind, f)
}

/// `the …` properties that come from outside the stack and change on their own.
const VOLATILE: &[&str] = &["mouseh", "mousev", "mouse", "mouseloc", "sound", "commandkey", "shiftkey", "optionkey", "seconds", "ticks",
    "date", "short date", "shortdate", "long date", "longdate", "abbreviated date", "abbrev date", "abbreviateddate", "time", "short time",
    "shorttime", "long time", "longtime", "mouseclick"];

fn err(s: &str) -> Box<EvalAltResult> { Box::new(EvalAltResult::ErrorRuntime(s.into(), Position::NONE)) }

fn to_dyn(v: Value) -> Dynamic {
    match v { Value::Bool(b) => b.into(), Value::Int(i) => i.into(), Value::Num(n) => n.into(), Value::Str(s) => s.into() }
}
fn from_dyn(d: &Dynamic) -> Value {
    if let Ok(b) = d.as_bool() { Value::Bool(b) }
    else if let Ok(i) = d.as_int() { Value::Int(i) }
    else if let Ok(f) = d.as_float() { Value::Num(f) }
    else { Value::Str(d.to_string()) }
}

fn dispatch(engine: &Engine, w: &W, live: &L, start: u32, msg: &str, args: Vec<Dynamic>) -> R<bool> {
    dispatch_ex(engine, w, live, start, msg, args).map(|(h, _)| h)
}

/// Send a message (or, with FN_PREFIX, call a function handler) along the path. Returns whether a
/// handler took it, and the value its `return` gave.
fn dispatch_ex(engine: &Engine, w: &W, live: &L, start: u32, msg: &str, args: Vec<Dynamic>) -> R<(bool, Option<Dynamic>)> {
    let mut cur = Some(start);
    let mut hops = 0;
    while let Some(id) = cur {
        hops += 1;
        if hops > 16 { break; }
        if id >= VISIT0 && !live.borrow().asts.contains_key(&id) && !live.borrow().errors.contains_key(&id) {
            let src = w.borrow().obj(id).map(|o| o.script.clone()).unwrap_or_default();
            compile_into(engine, live, id, &src);
        }
        let ast = live.borrow().asts.get(&id).cloned();
        let parent = w.borrow().next_on_path(id);
        if let Some(ast) = ast {
            if live.borrow().frames.len() > 64 { return Err(err("too many nested messages")); }
            live.borrow_mut().frames.push(Frame { obj: id, target: start, msg: msg.to_lowercase(), args: args.clone(), handled: false, passed: false, globals: vec![], ret: None });
            let res = engine.run_ast_with_scope(&mut Scope::new(), &ast);
            let f = live.borrow_mut().frames.pop().unwrap();
            res?;
            if f.handled && !f.passed { return Ok((true, f.ret)); }
        }
        cur = parent;
    }
    Ok((false, None))
}

fn me(w: &W, live: &L) -> u32 { live.borrow().frames.last().map(|f| f.obj).unwrap_or_else(|| w.borrow().current_card()) }

fn navigate(engine: &Engine, w: &W, live: &L, target: u32) -> R<()> {
    let from = w.borrow().current_card();
    if from == target { return Ok(()); }
    // Handlers for system messages write "automatic" changes, which Undo skips.
    // With lockMessages set, the card changes silently.
    let quiet = w.borrow().lock_messages;
    { let mut wb = w.borrow_mut(); if !wb.lock_recent { wb.recent.push_back(from); if wb.recent.len() > RECENT_MAX { wb.recent.pop_front(); } } }
    let (bg_from, bg_to) = { let wb = w.borrow(); (wb.background_of(from), wb.background_of(target)) };
    w.borrow_mut().auto_depth += 1;
    let send = |id: u32, m: &str| -> R<bool> { if quiet { Ok(false) } else { dispatch(engine, w, live, id, m, vec![]) } };
    let r = send(from, "closeCard")
        .and_then(|_| if bg_from != bg_to { send(from, "closeBackground") } else { Ok(false) })
        .and_then(|_| {
            w.borrow_mut().file.current_card = target;
            if bg_from != bg_to { send(target, "openBackground") } else { Ok(false) }
        })
        .and_then(|_| send(target, "openCard"));
    w.borrow_mut().auto_depth -= 1;
    r.map(|_| ())
}

fn resolve(w: &W, live: &L, kind: &str, key: Option<&Dynamic>) -> R<u32> {
    if let Some(base) = kind.strip_suffix("#id") {
        let want = match base { "card" => Kind::Card, "background" => Kind::Background, "field" => Kind::Field, "button" => Kind::Button, "picture" => Kind::Picture, "canvas" => Kind::Canvas, _ => return Err(err(&format!("{base} id: not an object kind"))) };
        let id = key.map(from_dyn).map(|v| v.int()).unwrap_or(-1);
        let wb = w.borrow();
        return match wb.obj(id as u32) { Some(o) if id > 0 && o.kind == want => Ok(o.id), _ => Err(err(&format!("no {base} id {id}"))) };
    }
    match kind {
        "me" => Ok(me(w, live)),
        "target" => Ok(live.borrow().frames.last().map(|f| f.target).unwrap_or_else(|| w.borrow().current_card())),
        "card" => match key {
            None => Ok(w.borrow().current_card()),
            Some(k) => {
                let wb = w.borrow();
                let cards = wb.cards();
                match from_dyn(k) {
                    Value::Int(i) => cards.get((i as usize).wrapping_sub(1)).copied(),
                    v => { let n = v.text().to_lowercase(); cards.iter().copied().find(|c| wb.obj(*c).map(|o| o.name.to_lowercase() == n).unwrap_or(false)) }
                }.ok_or_else(|| err(&format!("no card \"{k}\"")))
            }
        },
        "background" => match key {
            None => { let wb = w.borrow(); wb.background_of(wb.current_card()).ok_or_else(|| err("this card has no background")) }
            Some(k) => {
                let wb = w.borrow();
                let bgs = wb.backgrounds();
                match from_dyn(k) {
                    Value::Int(i) => bgs.get((i as usize).wrapping_sub(1)).copied(),
                    v => { let n = v.text().to_lowercase(); bgs.iter().copied().find(|c| wb.obj(*c).map(|o| o.name.to_lowercase() == n).unwrap_or(false)) }
                }.ok_or_else(|| err(&format!("no background \"{k}\"")))
            }
        },
        "stack" => match key {
            None => Ok(w.borrow().stack_id()),
            Some(k) => other_stack(w, &k.to_string()),
        },
        "part" => {
            let key = from_dyn(key.ok_or_else(|| err("part what? write part 3 or part \"name\""))?);
            let wb = w.borrow();
            let parts: Vec<u32> = wb.parts_on(wb.current_card()).into_iter().filter(|id| wb.obj(*id).map(|o| matches!(o.kind, Kind::Button | Kind::Field)).unwrap_or(false)).collect();
            match &key {
                Value::Int(n) => parts.get((*n as usize).wrapping_sub(1)).copied(),
                v => { let n = v.text().to_lowercase(); parts.iter().rev().copied().find(|p| wb.obj(*p).map(|o| o.name.to_lowercase() == n).unwrap_or(false)) }
            }.ok_or_else(|| err(&format!("no part \"{}\" on this card", key.text())))
        }
        "field" | "button" | "canvas" | "picture" => {
            let k = match kind { "field" => Kind::Field, "button" => Kind::Button, "canvas" => Kind::Canvas, _ => Kind::Picture };
            let key = from_dyn(key.ok_or_else(|| err("missing name"))?);
            w.borrow().find_part(k, &key).ok_or_else(|| err(&format!("no {kind} \"{}\" on this card", key.text())))
        }
        other => Err(err(&format!("cannot refer to '{other}'; use me, target, this card, card, background, stack, field, button, picture or canvas"))),
    }
}

/// The name slot after `field`, `button`, `canvas`, `card`. A literal name or number ends the
/// reference, so `the left of canvas "play" - 40` subtracts 40 from the left. A computed name
/// still takes a whole expression: write `(field (n + 1))` to stop it early.
fn slot(look: &str) -> ImmutableString {
    if look.starts_with('"') { "$string$".into() }
    else if !look.is_empty() && look.chars().all(|c| c.is_ascii_digit()) { "$int$".into() }
    else { "$expr$".into() }
}

fn is_direction(look: &str) -> bool { matches!(look, "left" | "right" | "up" | "down" | "open" | "close" | "in" | "out") }

fn effect_name(inputs: &[rhai::Expression]) -> String {
    effect_from_words(inputs.iter().filter_map(|i| i.get_string_value()))
}

/// A visual effect's name from its words (`barn door open slowly to black` → `barn-door-open`): speed words
/// and a `to <colour>` ending do not change the effect here.
fn effect_from_words<'a>(words: impl IntoIterator<Item = &'a str>) -> String {
    words.into_iter().map(str::to_lowercase).take_while(|w| w != "to").filter(|w| !matches!(w.as_str(), "very" | "slow" | "slowly" | "fast")).collect::<Vec<_>>().join("-")
}

/// A value as a number: numbers, and text that reads as one.
fn num(d: &Dynamic) -> Option<f64> {
    d.as_float().ok().or_else(|| d.as_int().ok().map(|i| i as f64)).or_else(|| d.clone().into_string().ok().and_then(|s| s.trim().parse().ok()))
}

/// A point written "h,v".
fn point(d: &Dynamic, what: &str) -> R<(f64, f64)> {
    let p: Vec<f64> = d.to_string().split(',').filter_map(|x| x.trim().parse().ok()).collect();
    if p.len() == 2 { Ok((p[0], p[1])) } else { Err(err(&format!("{what} needs a point \"h,v\""))) }
}

/// `n <unit>` in milliseconds (wait, send … in): ticks are 1/60 s. None for a unit that is not one.
fn duration_ms(n: f64, unit: &str) -> Option<f64> {
    match unit { "seconds" | "second" | "secs" | "sec" => Some(n * 1000.0), "ticks" | "tick" => Some(n * 1000.0 / 60.0),
                 "milliseconds" | "millisecond" | "ms" => Some(n), _ => None }
}

/// A text as a HyperTalk expression: a literal, with `quote` for the quote marks it holds.
fn hypertalk_literal(text: &str) -> String {
    text.split('"').map(|p| format!("\"{p}\"")).collect::<Vec<_>>().join(" & quote & ")
}

fn canvas_target(w: &W, live: &L) -> Option<u32> {
    if let Some(id) = live.borrow().canvas.last() { return Some(*id); }
    let wb = w.borrow();
    let me = live.borrow().frames.last().map(|f| f.obj);
    if let Some(m) = me { if wb.obj(m).map(|o| o.kind == Kind::Canvas).unwrap_or(false) { return Some(m); } }
    wb.parts_on(wb.current_card()).into_iter().find(|id| wb.obj(*id).map(|o| o.kind == Kind::Canvas).unwrap_or(false))
}

/// A container written as `field "x"`, `file "x"` or `url "x"` after `of` ends at its name, so
/// `the number of lines of url "…" > 3` compares the count. Codes: 2 field, 3 file, 4 url.
fn container_code(look: &str) -> Option<i64> { match look { "field" => Some(2), "file" => Some(3), "url" => Some(4), _ => None } }

fn container_text(w: &W, live: &L, code: i64, key: Dynamic) -> R<String> {
    match code {
        2 => { let f = resolve(w, live, "field", Some(&key))?; Ok(w.borrow().get(f, "text").text()) }
        3 => { let wb = w.borrow(); let p = wb.path(&key.to_string()); let r = wb.fs.borrow().read(&p); Ok(r.unwrap_or_default()) }
        _ => Ok(w.borrow().url_cache.get(&key.to_string()).cloned().unwrap_or_default()),
    }
}

/// A bare variable name (not a keyword), which ends the expression after `of`.
fn plain_ident(look: &str) -> bool {
    let mut c = look.chars();
    matches!(c.next(), Some(x) if x.is_alphabetic() || x == '_') && look.chars().all(|x| x.is_alphanumeric() || x == '_') && !RESERVED.contains(&look)
}

fn index(d: &Dynamic) -> R<i64> {
    d.as_int().ok().or_else(|| d.as_float().ok().map(|f| f as i64)).or_else(|| d.clone().into_string().ok().and_then(|s| s.trim().parse().ok()))
        .ok_or_else(|| err("a chunk position must be a number"))
}

/// `put val into|before|after <chunks> of text`. Chunks are listed as written (innermost first).
fn put_chunk(text: &str, kinds: &[String], idx: &[(i64, i64)], mode: &str, val: &str) -> String {
    match kinds.len() {
        0 => match mode { "before" => format!("{val}{text}"), "after" => format!("{text}{val}"), _ => val.to_string() },
        n => {
            let k = match kinds[n - 1].as_str() { "character" => "char", k => k };
            let (i, j) = idx[n - 1];
            if j != i {
                // A range: replace the characters from the start of chunk i to the end of chunk j.
                let inner = chunk_range(k, i, j, text);
                let new = put_chunk(&inner, &kinds[..n - 1], &idx[..n - 1], mode, val);
                let chars: Vec<char> = text.chars().collect();
                let (a, _) = chunk_span(k, i, text);
                let (_, b) = chunk_span(k, j, text);
                if a >= chars.len() { return replace_chunk(text, k, i, &new); }
                return chars[..a].iter().copied().chain(new.chars()).chain(chars[b.max(a)..].iter().copied()).collect();
            }
            let inner = chunk(k, i, text);
            let new = put_chunk(&inner, &kinds[..n - 1], &idx[..n - 1], mode, val);
            replace_chunk(text, k, i, &new)
        }
    }
}

/// Replace one chunk. Missing items and lines are added as empty ones, as in HyperTalk.
fn replace_chunk(text: &str, kind: &str, n: i64, new: &str) -> String {
    let i = (n.max(1) - 1) as usize;
    match kind {
        "item" | "line" => {
            let dl = delim();
            let d = if kind == "item" { dl.as_str() } else { "\n" };
            let mut parts: Vec<String> = if text.is_empty() { vec![] } else { text.split(d).map(String::from).collect() };
            while parts.len() <= i { parts.push(String::new()); }
            parts[i] = new.to_string();
            parts.join(d)
        }
        "word" => {
            let mut spans: Vec<(usize, usize)> = vec![];
            let mut start = None;
            for (b, c) in text.char_indices() {
                match (c.is_whitespace(), start) { (false, None) => start = Some(b), (true, Some(s)) => { spans.push((s, b)); start = None; } _ => {} }
            }
            if let Some(s) = start { spans.push((s, text.len())); }
            match spans.get(i) {
                Some((a, b)) => format!("{}{}{}", &text[..*a], new, &text[*b..]),
                None => { let mut s = text.to_string(); if !s.is_empty() && !s.ends_with(char::is_whitespace) { s.push(' '); } s.push_str(new); s }
            }
        }
        _ => {
            let c: Vec<char> = text.chars().collect();
            if i < c.len() { let mut s: String = c[..i].iter().collect(); s.push_str(new); s.extend(&c[i + 1..]); s } else { format!("{text}{new}") }
        }
    }
}

thread_local! {
    /// The item delimiter in effect for chunk helpers (HyperTalk's itemDelimiter property).
    static DELIM: RefCell<String> = RefCell::new(",".into());
}
fn delim() -> String { DELIM.with(|d| d.borrow().clone()) }

/// The pieces of `text` for a chunk kind.
fn pieces(kind: &str, text: &str) -> Vec<String> {
    match kind {
        "word" => text.split_whitespace().map(String::from).collect(),
        "line" => text.lines().map(String::from).collect(),
        "item" => if text.is_empty() { vec![] } else { text.split(delim().as_str()).map(String::from).collect() },
        _ => text.chars().map(|c| c.to_string()).collect(),
    }
}

fn chunk(kind: &str, n: i64, text: &str) -> String {
    let i = (n.max(1) - 1) as usize;
    pieces(kind, text).get(i).cloned().unwrap_or_default()
}

/// Chunks n to m, joined by their delimiter.
fn chunk_range(kind: &str, n: i64, m: i64, text: &str) -> String {
    let p = pieces(kind, text);
    let (a, b) = ((n.max(1) - 1) as usize, (m.max(0) as usize).min(p.len()));
    if a >= b { return String::new(); }
    let sep = match kind { "word" => " ".to_string(), "line" => "\n".to_string(), "item" => delim(), _ => String::new() };
    p[a..b].join(&sep)
}

/// 1-based position for an ordinal word over `count` pieces.
fn ordinal(word: &str, count: usize) -> i64 {
    match word {
        "last" => count as i64,
        "middle" => (count / 2 + 1) as i64,
        "any" => if count == 0 { 0 } else { 1 + (rand_u64() % count as u64) as i64 },
        w => ORDINALS.iter().position(|o| *o == w).map(|i| i as i64 + 1).unwrap_or(1),
    }
}

thread_local! { static SEED: std::cell::Cell<u64> = std::cell::Cell::new(0x9E3779B97F4A7C15 ^ web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(1)); }
fn rand_u64() -> u64 { SEED.with(|s| { let mut x = s.get(); x ^= x << 13; x ^= x >> 7; x ^= x << 17; s.set(x); x }) }
/// A paused script running again draws the same random numbers.
fn rand_seed() -> u64 { SEED.with(|s| s.get()) }
fn set_rand_seed(v: u64) { SEED.with(|s| s.set(v)) }

fn build_engine(w: &W, live: &L) -> Engine {
    let mut e = Engine::new();
    e.set_max_operations(2_000_000);
    register_coercion(&mut e);
    register_vocabulary(&mut e, live);
    e.set_max_call_levels(48);
    { let w = w.clone(); e.on_print(move |s| w.borrow_mut().log.push(s.to_string())); }
    e.register_fn("num", |s: &str| -> Dynamic { let t = s.trim(); t.parse::<i64>().map(Dynamic::from).or_else(|_| t.parse::<f64>().map(Dynamic::from)).unwrap_or(Dynamic::from(0_i64)) });
    e.register_fn("num", |i: i64| i);

    // on <message> [params...] { ... }   and   function <name> [params...] { ... }
    // A function handler answers `name(…)` calls that reach it on the message path; `return x` gives its value.
    for (keyword, prefix) in [("on", ""), ("function", "\u{1}")] {
      let live = live.clone();
      e.register_custom_syntax_with_state_raw(keyword,
        move |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            let last = syms.last().map(|s| s.as_str()).unwrap_or("");
            Ok(if last == "$block$" { None }
               else if syms.len() == 1 { *state = Dynamic::from(format!("{prefix}{}", look.to_lowercase())); Some(look.into()) }
               else if look == "{" { Some("$block$".into()) }
               else { Some("$ident$".into()) })
        }, true,
        move |ctx, inputs, state| {
            let name = state.to_string();
            let args = {
                let mut lb = live.borrow_mut();
                let Some(f) = lb.frames.last_mut() else { return Ok(Dynamic::UNIT) };
                if f.handled || f.msg != name { return Ok(Dynamic::UNIT) }
                f.handled = true;
                f.args.clone()
            };
            let n = inputs.len();
            let base = ctx.scope().len();
            for (i, p) in inputs[..n - 1].iter().enumerate() {
                let pname = p.get_string_value().unwrap_or("_").to_string();
                ctx.scope_mut().push(pname, args.get(i).cloned().unwrap_or(Dynamic::from(String::new())));
            }
            let declared = { let lb = live.borrow(); lb.frames.last().and_then(|f| lb.locals.get(&f.obj)).and_then(|m| m.get(&name)).cloned().unwrap_or_default() };
            for v in declared.into_iter().chain(["it".to_string()]) { if !ctx.scope().contains(&v) { ctx.scope_mut().push(v, Dynamic::from(String::new())); } }
            // Evaluate without rewinding the scope, so variables declared `global` can be written back.
            #[allow(deprecated)]
            let r = ctx.eval_expression_tree_raw(&inputs[n - 1], false);
            let names = live.borrow().frames.last().map(|f| f.globals.clone()).unwrap_or_default();
            for name in names {
                if let Some(v) = ctx.scope().get_value::<Dynamic>(&name) { Rc::make_mut(&mut live.borrow_mut().globals).insert(name, v); }
            }
            ctx.scope_mut().rewind(base);
            match r {
                Err(_) if live.borrow().frames.last().map(|f| f.passed).unwrap_or(false) => Ok(Dynamic::UNIT),
                // `return x` ends the handler; for a function handler, x is the function's value.
                Err(e) if matches!(*e, EvalAltResult::Return(..)) => {
                    if let EvalAltResult::Return(v, _) = *e { if let Some(f) = live.borrow_mut().frames.last_mut() { f.ret = Some(v); } }
                    Ok(Dynamic::UNIT)
                }
                r => r.map(|_| Dynamic::UNIT),
            }
        });
    }

    // A call to a function this script does not define goes along the message path to a function handler
    // (card, background, stack, libraries, Home), as in HyperTalk.
    { let (w, live) = (w.clone(), live.clone());
      #[allow(deprecated)]
      e.on_missing_function(move |name, args, is_method, ctx| {
          if is_method || name.starts_with("__") { return Ok(None); }
          let start = me(&w, &live);
          let a: Vec<Dynamic> = args.iter().map(|x| (**x).clone()).collect();
          let (handled, ret) = dispatch_ex(ctx.engine(), &w, &live, start, &format!("{FN_PREFIX}{}", name.to_lowercase()), a)?;
          Ok(if handled { Some(ret.unwrap_or_else(|| Dynamic::from(""))) } else { None })
      });
    }

    // __cmd(name, [args]): a line like `check "x", true` that names no built-in command sends the message
    // `check` with those parameters, as HyperTalk does. Nothing handling it is an error, as in HyperTalk.
    { let (w, live) = (w.clone(), live.clone());
      e.register_fn("__cmd", move |ctx: rhai::NativeCallContext, name: &str, args: rhai::Array| -> R<()> {
          let start = me(&w, &live);
          let (handled, _) = dispatch_ex(ctx.engine(), &w, &live, start, name, args)?;
          if handled { Ok(()) } else { Err(err(&format!("Can't understand \"{name}\": no handler for it on the message path"))) }
      });
    }

    // global <name>, <name>, ...   -- load shared variables into this handler; written back when it ends
    { let live = live.clone();
      e.register_custom_syntax_with_state_raw("global",
        |syms: &[ImmutableString], look: &str, _| {
            // global, name, ",", name, ... : odd length means a name comes next.
            Ok(if syms.len() % 2 == 1 { Some("$ident$".into()) } else if look == "," { Some(",".into()) } else { None })
        }, true,
        move |ctx, inputs, _| {
            for i in inputs {
                let name = i.get_string_value().unwrap_or("_").to_string();
                let v = live.borrow().globals.get(&name).cloned().unwrap_or(Dynamic::from(String::new()));
                ctx.scope_mut().push(name.clone(), v);
                if let Some(f) = live.borrow_mut().frames.last_mut() { if !f.globals.contains(&name) { f.globals.push(name); } }
            }
            Ok(Dynamic::UNIT)
        });
    }

    // pass
    { let live = live.clone();
      e.register_custom_syntax_with_state_raw("pass",
        // `pass` or `pass <message>`: the name is optional and must be the handler's own.
        |syms: &[ImmutableString], look: &str, _| Ok(if syms.len() == 1 && look != ";" && look != "}" && plain_ident(look) { Some("$ident$".into()) } else { None }),
        false,
        move |_, _, _| {
            if let Some(f) = live.borrow_mut().frames.last_mut() { f.passed = true; }
            Err(err("__pass__"))
        });
    }

    // field <name>  -> its text
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("field",
        |syms: &[ImmutableString], look: &str, _| Ok(if syms.len() == 1 { Some(slot(look)) } else { None }),
        false,
        move |ctx, inputs, _| {
            let key = ctx.eval_expression_tree(&inputs[0])?;
            let id = resolve(&w, &live, "field", Some(&key))?;
            Ok(to_dyn(w.borrow().get(id, "text")))
        });
    }

    // word|line|item|char[acter] <n> [to <m>] of <container>. A bare variable name ends the
    // expression, so `word 1 of name == "x"` compares the word. Other containers take a whole expression.
    for kind in CHUNKS {
        let (w, live) = (w.clone(), live.clone());
        // A chain of chunks is one expression: `word 1 of line 2 of field "f"`, each with an optional `to <m>`.
        // It ends where its container ends (a variable, `field|file|url <name>`, a literal), so
        // `word 1 of line i of field "f" is "x"` compares the word.
        #[derive(Clone, Default)]
        struct ChunkSt { kinds: Vec<String>, ranged: Vec<bool>, cont: i64, phase: u8 }
        e.register_custom_syntax_with_state_raw(kind,
            |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
                let mut st = if syms.len() == 1 { ChunkSt { kinds: vec![syms[0].to_string()], ranged: vec![false], ..Default::default() } } else { state.clone_cast::<ChunkSt>() };
                let next: Option<ImmutableString> = match st.phase {
                    0 => { st.phase = 1; Some("$expr$".into()) }
                    1 => if look == "to" && !st.ranged.last().copied().unwrap_or(true) { if let Some(r) = st.ranged.last_mut() { *r = true; } st.phase = 0; Some("to".into()) } else { st.phase = 5; Some("of".into()) },
                    5 => if CHUNKS.contains(&look) { st.kinds.push(look.into()); st.ranged.push(false); st.phase = 0; Some(look.into()) }
                         else if let Some(c) = container_code(look) { st.cont = c; st.phase = 7; Some(look.into()) }
                         else if look.starts_with('"') { st.cont = 5; st.phase = 9; Some("$string$".into()) }
                         else if plain_ident(look) { st.cont = 1; st.phase = 9; Some("$ident$".into()) }
                         else { st.cont = 0; st.phase = 9; Some("$expr$".into()) },
                    7 => { st.phase = 9; Some(slot(look)) }
                    _ => None,
                };
                *state = Dynamic::from(st);
                Ok(next)
            }, false,
            move |ctx, inputs, state| {
                let st = state.clone_cast::<ChunkSt>();
                let mut k = 0;
                let mut idx = vec![];
                for r in &st.ranged {
                    let a = index(&ctx.eval_expression_tree(&inputs[k])?)?; k += 1;
                    let b = if *r { let b = index(&ctx.eval_expression_tree(&inputs[k])?)?; k += 1; Some(b) } else { None };
                    idx.push((a, b));
                }
                let src = &inputs[k];
                let mut text = match st.cont {
                    1 => {
                        let name = src.get_string_value().unwrap_or("").to_string();
                        ctx.scope().get_value::<Dynamic>(&name).ok_or_else(|| err(&format!("there is no variable named {name}")))?.to_string()
                    }
                    2..=4 => { let key = ctx.eval_expression_tree(src)?; container_text(&w, &live, st.cont, key)? }
                    _ => ctx.eval_expression_tree(src)?.to_string(),
                };
                // Apply the outermost chunk first: in `word 1 of line 2 of x`, line 2, then word 1.
                for (kind, (a, b)) in st.kinds.iter().zip(idx).rev() {
                    let kind = if kind == "character" { "char" } else { kind.as_str() };
                    text = match b { Some(b) => chunk_range(kind, a, b, &text), None => chunk(kind, a, &text) };
                }
                Ok(Dynamic::from(text))
            });
    }

    // first|second|…|tenth|last|middle|any <chunk> of <container>
    for ord in ORDINALS {
        let (w, live) = (w.clone(), live.clone());
        e.register_custom_syntax_with_state_raw(ord,
            |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
                Ok(match syms.len() {
                    1 => { *state = Dynamic::from(OrdSt { chunk: look.into(), var: false, cont: 0 }); Some(look.into()) }
                    2 => Some("of".into()),
                    3 => {
                        let mut st = state.clone_cast::<OrdSt>();
                        let r: ImmutableString = if let Some(c) = container_code(look) { st.cont = c; look.into() }
                            else if look.starts_with('"') { "$string$".into() }
                            else { st.var = plain_ident(look); if st.var { "$ident$".into() } else { "$expr$".into() } };
                        *state = Dynamic::from(st);
                        Some(r)
                    }
                    4 if state.clone_cast::<OrdSt>().cont >= 2 => Some(slot(look)),
                    _ => None,
                })
            }, false,
            move |ctx, inputs, state| {
                let st = state.clone_cast::<OrdSt>();
                let text = if st.cont >= 2 {
                    let key = ctx.eval_expression_tree(&inputs[0])?;
                    container_text(&w, &live, st.cont, key)?
                } else if st.var {
                    let name = inputs[0].get_string_value().unwrap_or("").to_string();
                    ctx.scope().get_value::<Dynamic>(&name).ok_or_else(|| err(&format!("there is no variable named {name}")))?.to_string()
                } else { ctx.eval_expression_tree(&inputs[0])?.to_string() };
                let kind = if st.chunk == "character" { "char" } else { st.chunk.as_str() };
                if !CHUNKS.contains(&st.chunk.as_str()) { return Err(err(&format!("{ord} what? use {ord} word|line|item|char of …"))); }
                let n = ordinal(ord, pieces(kind, &text).len());
                Ok(Dynamic::from(chunk(kind, n, &text)))
            });
    }

    // put <expr> into|before|after [<chunk> <n> of ...] <variable | field <name>>
    // put <expr>   (no destination): shows it in the message box, as in HyperCard
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("put",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            if syms.len() == 1 { *state = Dynamic::from(PutSt::default()); return Ok(Some("$expr$".into())); }
            let mut st = state.clone_cast::<PutSt>();
            let next: Option<ImmutableString> = match st.phase {
                0 => if matches!(look, "into" | "before" | "after") { st.phase = 1; st.mode = look.to_string(); Some(look.into()) } else { None },
                1 => if CHUNKS.contains(&look) { st.kinds.push(look.into()); st.ranged.push(false); st.phase = 2; Some(look.into()) }
                     else if look == "field" { st.field = true; st.phase = 4; Some("field".into()) }
                     else if look == "file" { st.file = true; st.phase = 4; Some("file".into()) }
                     else if look == "the" { st.selection = true; st.phase = 6; Some("the".into()) }
                     else { st.phase = 5; Some("$ident$".into()) },
                2 => { st.phase = 3; Some("$expr$".into()) }
                3 => if look == "to" && !st.ranged.last().copied().unwrap_or(true) { if let Some(r) = st.ranged.last_mut() { *r = true; } st.phase = 2; Some("to".into()) } else { st.phase = 1; Some("of".into()) },
                6 => { st.phase = 5; Some("selection".into()) }
                4 => { st.phase = 5; Some(slot(look)) }
                _ => None,
            };
            *state = Dynamic::from(st);
            Ok(next)
        }, true,
        move |ctx, inputs, state| {
            let st = state.clone_cast::<PutSt>();
            let v = ctx.eval_expression_tree(&inputs[0])?;
            if st.mode.is_empty() { w.borrow_mut().log.push(v.to_string()); return Ok(Dynamic::UNIT); }
            if st.selection {
                // put … into|before|after the selection
                let Some((f, a, b)) = w.borrow().selection else { return Err(err("there is no selection")) };
                let old: Vec<char> = w.borrow().get(f, "text").text().chars().collect();
                let (a, b) = (a.min(old.len()), b.min(old.len()));
                let t: Vec<char> = v.to_string().chars().collect();
                let (a, b) = match st.mode.as_str() { "before" => (a, a), "after" => (b, b), _ => (a, b) };
                let new: String = old[..a].iter().chain(t.iter()).chain(old[b..].iter()).collect();
                let mut wb = w.borrow_mut();
                wb.selection = Some((f, a, a + t.len()));
                wb.set(f, "text", Value::Str(new), false);
                return Ok(Dynamic::UNIT);
            }
            let n = st.kinds.len();
            let mut idx = vec![];
            let mut k = 1;
            for i in 0..n {
                let a = index(&ctx.eval_expression_tree(&inputs[k])?)?; k += 1;
                let b = if st.ranged[i] { let b = index(&ctx.eval_expression_tree(&inputs[k])?)?; k += 1; b } else { a };
                idx.push((a, b));
            }
            let tgt = &inputs[k];
            if st.file {
                // a whole file as a container: put … into|before|after [line 2 of] file "notes.txt"
                let p = { let wb = w.borrow(); wb.path(&ctx.eval_expression_tree(tgt)?.to_string()) };
                let fs = w.borrow().fs.clone();
                let old = fs.borrow().read(&p).unwrap_or_default();
                let new = put_chunk(&old, &st.kinds, &idx, &st.mode, &v.to_string());
                fs.borrow_mut().write(&p, &new).map_err(|e| err(&e))?;
                return Ok(Dynamic::UNIT);
            }
            if st.field {
                let key = ctx.eval_expression_tree(tgt)?;
                let id = resolve(&w, &live, "field", Some(&key))?;
                let old = w.borrow().get(id, "text").text();
                let new = put_chunk(&old, &st.kinds, &idx, &st.mode, &v.to_string());
                w.borrow_mut().set(id, "text", Value::Str(new), false);
            } else {
                let name = tgt.get_string_value().unwrap_or("it").to_string();
                if n == 0 && st.mode == "into" {
                    ctx.scope_mut().set_value(name, v);
                } else {
                    let old = ctx.scope().get_value::<Dynamic>(&name).map(|d| d.to_string()).unwrap_or_default();
                    let new = put_chunk(&old, &st.kinds, &idx, &st.mode, &v.to_string());
                    ctx.scope_mut().set_value(name, Dynamic::from(new));
                }
            }
            Ok(Dynamic::UNIT)
        });
    }

    // set [the] <prop> [of <ref>] to <expr>   -- without `of`: a global property (itemDelimiter, lockScreen, lockMessages)
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("set",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            if syms.len() == 1 { *state = Dynamic::from(SetSt::default()); return Ok(Some(if look == "the" { "the".into() } else { "$ident$".into() })); }
            let mut st = state.clone_cast::<SetSt>();
            let next: Option<ImmutableString> = match st.phase {
                0 => if syms.len() == 2 && syms[1] == "the" { Some("$ident$".into()) } else if look == "of" { st.phase = 1; st.has_ref = true; Some("of".into()) } else { st.phase = 5; Some("to".into()) },
                1 => match st.r.step(look) { Some(x) => Some(x), None => { st.phase = 5; Some("to".into()) } },
                5 => { st.phase = 6; Some("$expr$".into()) }
                _ => None,
            };
            *state = Dynamic::from(st);
            Ok(next)
        }, false,
        move |ctx, inputs, state| {
            let st = state.clone_cast::<SetSt>();
            let prop = canon_prop(inputs[0].get_string_value().unwrap_or(""));
            let val = ctx.eval_expression_tree(&inputs[inputs.len() - 1])?;
            if !st.has_ref {
                let mut wb = w.borrow_mut();
                return match prop.as_str() {
                    "searchPaths" | "searchpaths" => {
                        // Folders to search for stacks, one per line, kept in Home.
                        let v = val.to_string();
                        if let Some(h) = wb.home.as_mut() { h.props.insert("searchPaths".into(), Value::Str(v)); }
                        Ok(Dynamic::UNIT)
                    }
                    "userLevel" => {
                        let n = from_dyn(&val).int();
                        if !(1..=5).contains(&n) { return Err(err("userLevel is 1 to 5")) }
                        wb.user_level = n;
                        // As in HyperCard, the user level is kept in Home, so it lasts between sessions.
                        if let Some(h) = wb.home.as_mut() { h.props.insert("userLevel".into(), Value::Int(n)); }
                        Ok(Dynamic::UNIT)
                    }
                    "itemDelimiter" => { let d = val.to_string(); if d.is_empty() { return Err(err("itemDelimiter cannot be empty")) } DELIM.with(|x| *x.borrow_mut() = d.clone()); wb.item_delim = d; Ok(Dynamic::UNIT) }
                    "lockScreen" => { wb.locked = from_dyn(&val).truthy(); Ok(Dynamic::UNIT) }
                    "lineSize" => { let n = from_dyn(&val).int(); if !(1..=8).contains(&n) { return Err(err("lineSize is 1 to 8")) } wb.line_size = n; Ok(Dynamic::UNIT) }
                    "filled" => { wb.filled = from_dyn(&val).truthy(); Ok(Dynamic::UNIT) }
                    "lockRecent" => { wb.lock_recent = from_dyn(&val).truthy(); Ok(Dynamic::UNIT) }
                    "lockMessages" => { wb.lock_messages = from_dyn(&val).truthy(); Ok(Dynamic::UNIT) }
                    other => Err(err(&format!("set the {other} of what? write: set the {other} of <object> to …"))),
                };
            }
            let key = if st.r.has_key { Some(ctx.eval_expression_tree(&inputs[1])?) } else { None };
            let id = resolve(&w, &live, &st.r.k(), key.as_ref())?;
            match prop.as_str() {
                "id" | "number" | "owner" => Err(err(&format!("'{prop}' cannot be set from a script"))),
                "script" => {
                    // As in HyperCard, a script can write a script. It is journaled (Undo restores it) and compiled now;
                    // if it does not compile, the object keeps running its last working script.
                    let src = val.to_string();
                    w.borrow_mut().set(id, "script", Value::Str(src.clone()), false);
                    compile_into(ctx.engine(), &live, id, &src);
                    Ok(Dynamic::UNIT)
                }
                "rect" | "loc" => {
                    let nums: Vec<i64> = val.to_string().split(',').map(|x| x.trim().parse::<f64>().unwrap_or(0.0).round() as i64).collect();
                    let mut wb = w.borrow_mut();
                    if prop == "rect" && nums.len() == 4 {
                        for (k, v) in [("left", nums[0]), ("top", nums[1]), ("width", nums[2] - nums[0]), ("height", nums[3] - nums[1])] { wb.set(id, k, Value::Int(v), false); }
                    } else if prop == "loc" && nums.len() == 2 {
                        let (wd, ht) = (wb.get(id, "width").int(), wb.get(id, "height").int());
                        wb.set(id, "left", Value::Int(nums[0] - wd / 2), false);
                        wb.set(id, "top", Value::Int(nums[1] - ht / 2), false);
                    } else { return Err(err(&format!("{prop} needs {} numbers separated by commas", if prop == "rect" { 4 } else { 2 }))); }
                    Ok(Dynamic::UNIT)
                }
                _ => { w.borrow_mut().set(id, &prop, from_dyn(&val), false); Ok(Dynamic::UNIT) }
            }
        });
    }

    // the <prop> [of <ref>]  |  the <ordinal> <chunk> of <container>  |  the number of <plural> [of <container>]
    // Global reads: mouseH, mouseV, mouse, mouseLoc, clickLoc, number (of cards), sound, target, date, time,
    // seconds, ticks, result, version, itemDelimiter, lockScreen, lockMessages, commandKey, shiftKey, optionKey.
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("the",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            if syms.len() == 1 { *state = Dynamic::from(TheSt::default()); return Ok(Some("$ident$".into())); }
            let mut st = state.clone_cast::<TheSt>();
            let next: Option<ImmutableString> = match st.phase {
                0 => {
                    let p = syms[1].to_lowercase();
                    if matches!(p.as_str(), "long" | "short" | "abbreviated" | "abbrev") && matches!(look, "date" | "time") { st.chunk = format!("{p} {look}"); st.phase = 30; Some("$ident$".into()) }
                    else if ORDINALS.contains(&p.as_str()) && CHUNKS.contains(&look) { st.mode = 1; st.chunk = look.into(); st.phase = 10; Some(look.into()) }
                    else if look == "of" { st.phase = 1; Some("of".into()) }
                    else { None }
                }
                1 => {
                    if syms[1].eq_ignore_ascii_case("number") && PLURALS.contains(&look) { st.mode = 2; st.chunk = look.into(); st.phase = 20; Some(look.into()) }
                    else { st.mode = 3; st.phase = 2; st.r.step(look) }
                }
                2 => st.r.step(look),
                10 => { st.phase = 11; Some("of".into()) }
                11 | 21 => if let Some(c) = container_code(look) { st.cont = c; st.phase = 50; Some(look.into()) }
                    else if look.starts_with('"') { st.phase += 1; Some("$string$".into()) }
                    else { st.var = plain_ident(look); st.phase += 1; Some(if st.var { "$ident$".into() } else { "$expr$".into() }) },
                50 => { st.phase = 51; Some(slot(look)) }
                20 => if matches!(st.chunk.as_str(), "lines" | "words" | "items" | "chars" | "characters") && look == "of" { st.phase = 21; Some("of".into()) } else { None },
                _ => None,
            };
            *state = Dynamic::from(st);
            Ok(next)
        }, false,
        move |ctx, inputs, state| {
            let st = state.clone_cast::<TheSt>();
            let prop = if st.phase == 30 { st.chunk.clone() } else { inputs[0].get_string_value().unwrap_or("").to_string() };
            let container = |ctx: &mut rhai::EvalContext, i: usize| -> R<String> {
                if st.cont >= 2 { let key = ctx.eval_expression_tree(&inputs[i])?; return container_text(&w, &live, st.cont, key); }
                if st.var {
                    let name = inputs[i].get_string_value().unwrap_or("").to_string();
                    Ok(ctx.scope().get_value::<Dynamic>(&name).ok_or_else(|| err(&format!("there is no variable named {name}")))?.to_string())
                } else { Ok(ctx.eval_expression_tree(&inputs[i])?.to_string()) }
            };
            match st.mode {
                1 => {
                    let text = container(ctx, 1)?;
                    let kind = if st.chunk == "character" { "char" } else { st.chunk.as_str() };
                    let n = ordinal(&prop.to_lowercase(), pieces(kind, &text).len());
                    return Ok(Dynamic::from(chunk(kind, n, &text)));
                }
                2 => {
                    let wb = w.borrow();
                    let card = wb.current_card();
                    let count_parts = |k: Kind| wb.parts_on(card).iter().filter(|id| wb.obj(**id).map(|o| o.kind == k).unwrap_or(false)).count();
                    let n = match st.chunk.as_str() {
                        "cards" => wb.cards().len(),
                        "backgrounds" => wb.backgrounds().len(),
                        "buttons" => count_parts(Kind::Button),
                        "fields" => count_parts(Kind::Field),
                        "pictures" => count_parts(Kind::Picture),
                        plural => { drop(wb); let text = container(ctx, 1)?; let kind = match plural { "lines" => "line", "words" => "word", "items" => "item", _ => "char" }; pieces(kind, &text).len() }
                    };
                    return Ok(Dynamic::from(n as i64));
                }
                3 => {
                    let key = if st.r.has_key { Some(ctx.eval_expression_tree(&inputs[1])?) } else { None };
                    if st.r.kind == "folder" {
                        // the files / the folders of folder "…": one name per line
                        let wb = w.borrow();
                        let p = wb.path(&key.map(|k| k.to_string()).unwrap_or_default());
                        let (files, folders) = wb.fs.borrow().list(&p).map_err(|e| err(&e))?;
                        return Ok(Dynamic::from(if prop.eq_ignore_ascii_case("folders") { folders } else { files }.join("\n")));
                    }
                    let id = resolve(&w, &live, &st.r.k(), key.as_ref())?;
                    return Ok(to_dyn(w.borrow().get(id, &canon_prop(&prop))));
                }
                _ => {}
            }
            let read = || -> R<Dynamic> {
            let wb = w.borrow();
            let key_state = |names: &[&str]| if names.iter().any(|n| wb.keys.contains(*n)) { "down" } else { "up" };
            let now = web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0);
            Ok(match prop.to_lowercase().as_str() {
                "mouseh" => Dynamic::from(wb.mouse.0),
                "mousev" => Dynamic::from(wb.mouse.1),
                "mouse" => Dynamic::from(if wb.mouse_down { "down" } else { "up" }),
                "mouseloc" => Dynamic::from(format!("{},{}", wb.mouse.0.round(), wb.mouse.1.round())),
                "clickloc" => Dynamic::from(format!("{},{}", wb.click_loc.0.round(), wb.click_loc.1.round())),
                "number" => Dynamic::from(wb.cards().len() as i64),
                "sound" => Dynamic::from(wb.sound.clone().unwrap_or_else(|| "done".into())),
                "target" => { drop(wb); let t = resolve(&w, &live, "target", None)?; to_dyn(w.borrow().get(t, "name")) }
                "result" => Dynamic::from(wb.result.clone()),
                "version" => Dynamic::from("0.1 (prototype)"),
                "userlevel" => Dynamic::from(wb.user_level),
                "stacks" => Dynamic::from(if wb.open_stacks.is_empty() { wb.file.name.clone() } else { wb.open_stacks.join("\n") }),
                "stacksinuse" => Dynamic::from(wb.libs.iter().map(|l| l.name.clone()).collect::<Vec<_>>().join("\n")),
                "searchpaths" => Dynamic::from(wb.home.as_ref().and_then(|h| h.props.get("searchPaths")).map(|v| v.text()).unwrap_or_default()),
                "files" | "folders" => {
                    let p = wb.path(".");
                    let (files, folders) = wb.fs.borrow().list(&p).map_err(|e| err(&e))?;
                    Dynamic::from(if prop.eq_ignore_ascii_case("folders") { folders } else { files }.join("\n"))
                }
                "selection" | "selectedtext" => Dynamic::from(wb.selection.map(|(f, a, b)| wb.get(f, "text").text().chars().skip(a).take(b.saturating_sub(a)).collect::<String>()).unwrap_or_default()),
                "selectedfield" => Dynamic::from(wb.selection.map(|(f, _, _)| wb.get(f, "name").text()).unwrap_or_default()),
                "selectedchunk" => Dynamic::from(wb.selection.map(|(f, a, b)| format!("char {} to {} of field \"{}\"", a + 1, b, wb.get(f, "name").text())).unwrap_or_default()),
                "itemdelimiter" => Dynamic::from(wb.item_delim.clone()),
                "tool" => Dynamic::from(format!("{} tool", wb.tool)),
                "linesize" => Dynamic::from(wb.line_size),
                "filled" => Dynamic::from(wb.filled),
                "lockrecent" => Dynamic::from(wb.lock_recent),
                "menus" => Dynamic::from(wb.menus.iter().map(|m| m.name.clone()).collect::<Vec<_>>().join("\n")),
                // The card window and the message box, then the other stacks open in the player.
                "windows" => Dynamic::from(std::iter::once("card window".to_string()).chain(std::iter::once("message window".to_string()))
                    .chain(wb.open_stacks.iter().filter(|s| **s != wb.file.name).cloned()).collect::<Vec<_>>().join("\n")),
                "screenrect" => Dynamic::from(format!("0,0,{},{}", wb.screen.0.round(), wb.screen.1.round())),
                "windowloc" => Dynamic::from(format!("{},{}", wb.window_loc.0.round(), wb.window_loc.1.round())),
                // Memory and disk are not limits here; these report HyperCard-sized room to spare, not a measurement.
                "diskspace" | "heapspace" | "stackspace" | "freesize" => Dynamic::from(i32::MAX as i64),
                "systemversion" => Dynamic::from(format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)),
                "lockscreen" => Dynamic::from(wb.locked),
                "lockmessages" => Dynamic::from(wb.lock_messages),
                "commandkey" => Dynamic::from(key_state(&["Meta", "Control"])),
                "shiftkey" => Dynamic::from(key_state(&["Shift"])),
                "optionkey" => Dynamic::from(key_state(&["Alt"])),
                // HyperCard counts seconds from 1 January 1904, local time.
                "seconds" => Dynamic::from(now as i64 + wb.tz_offset_min * 60 + EPOCH_1904),
                "ticks" => Dynamic::from((now * 60.0) as i64),
                "date" | "short date" | "shortdate" => Dynamic::from(format_when(now as i64 + wb.tz_offset_min * 60, "short date").unwrap_or_default()),
                "long date" | "longdate" => Dynamic::from(format_when(now as i64 + wb.tz_offset_min * 60, "long date").unwrap_or_default()),
                "abbreviated date" | "abbrev date" | "abbreviateddate" => Dynamic::from(format_when(now as i64 + wb.tz_offset_min * 60, "abbreviated date").unwrap_or_default()),
                "time" | "short time" | "shorttime" => Dynamic::from(format_when(now as i64 + wb.tz_offset_min * 60, "time").unwrap_or_default()),
                "long time" | "longtime" => Dynamic::from(format_when(now as i64 + wb.tz_offset_min * 60, "long time").unwrap_or_default()),
                "mouseclick" => { drop(wb); let mut wb = w.borrow_mut(); let c = wb.mouse_click; wb.mouse_click = false; Dynamic::from(c) }
                "selectedline" => Dynamic::from(wb.selection.map(|(f, a, _)| { let t = wb.get(f, "text").text(); let n = t.chars().take(a).filter(|c| *c == '\n').count() + 1; format!("line {n} of field \"{}\"", wb.get(f, "name").text()) }).unwrap_or_default()),
                "clicktext" | "clickline" | "clickchunk" => {
                    let Some((f, off)) = wb.click_text_at else { return Ok(Dynamic::from("")) };
                    let t = wb.get(f, "text").text();
                    let chars: Vec<char> = t.chars().collect();
                    let off = off.min(chars.len());
                    let name = wb.get(f, "name").text();
                    match prop.to_lowercase().as_str() {
                        "clickline" => { let n = chars[..off].iter().filter(|c| **c == '\n').count() + 1; Dynamic::from(format!("line {n} of field \"{name}\"")) }
                        k => {
                            let is_w = |c: char| !c.is_whitespace();
                            let mut a = off; while a > 0 && is_w(chars[a - 1]) { a -= 1; }
                            let mut b = off; while b < chars.len() && is_w(chars[b]) { b += 1; }
                            if k == "clicktext" { Dynamic::from(chars[a..b].iter().collect::<String>()) } else { Dynamic::from(format!("char {} to {} of field \"{name}\"", a + 1, b)) }
                        }
                    }
                }
                other => return Err(err(&format!("unknown: the {other}"))),
            })
            };
            let lower = prop.to_lowercase();
            if VOLATILE.contains(&lower.as_str()) {
                let mut failed = None;
                let v = taped(&live, &format!("the {lower}"), || read().unwrap_or_else(|e| { failed = Some(e); Dynamic::UNIT }));
                if let Some(e) = failed { return Err(e); }
                Ok(v)
            } else { read() }
        });
    }

    // hide|show|enable|disable|mark|unmark <ref>   (<ref> may also be `part 3`)  -- sugar for the visible / enabled / marked properties
    for (word, prop, value) in [("hide", "visible", false), ("show", "visible", true), ("enable", "enabled", true), ("disable", "enabled", false)] {
        let (w, live) = (w.clone(), live.clone());
        e.register_custom_syntax_with_state_raw(word,
            |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
                let mut st = if syms.len() == 1 { RefSt::default() } else { state.clone_cast::<RefSt>() };
                let next = st.step(look);
                *state = Dynamic::from(st);
                Ok(next)
            }, false,
            move |ctx, inputs, state| {
                let st = state.clone_cast::<RefSt>();
                let key = if st.has_key { Some(ctx.eval_expression_tree(&inputs[0])?) } else { None };
                let id = resolve(&w, &live, &st.k(), key.as_ref())?;
                w.borrow_mut().set(id, prop, Value::Bool(value), false);
                Ok(Dynamic::UNIT)
            });
    }

    // mark|unmark <card ref> | mark|unmark all cards | mark|unmark cards where <expr> | mark|unmark cards by finding <text>
    for (word, value) in [("mark", true), ("unmark", false)] {
        let (w, live) = (w.clone(), live.clone());
        #[derive(Clone, Default)]
        struct MarkSt { mode: String, r: RefSt }
        e.register_custom_syntax_with_state_raw(word,
            |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
                let mut st = if syms.len() == 1 { MarkSt::default() } else { state.clone_cast::<MarkSt>() };
                let last = syms.last().map(|s| s.as_str()).unwrap_or("");
                let next: Option<ImmutableString> = if syms.len() == 1 {
                    match look { "all" => { st.mode = "all".into(); Some("all".into()) } "cards" => { st.mode = "cards".into(); Some("cards".into()) } _ => { st.mode = "ref".into(); st.r.step(look) } }
                } else {
                    match (st.mode.as_str(), last) {
                        ("all", "all") => Some("cards".into()),
                        ("cards", "cards") => match look { "where" => Some("where".into()), "by" => { st.mode = "finding".into(); Some("by".into()) } _ => None },
                        ("cards", "where") => Some("$expr$".into()),
                        ("finding", "by") => Some("finding".into()),
                        ("finding", "finding") => Some("$expr$".into()),
                        ("ref", _) => st.r.step(look),
                        _ => None,
                    }
                };
                *state = Dynamic::from(st);
                Ok(next)
            }, false,
            move |ctx, inputs, state| {
                let st = state.clone_cast::<MarkSt>();
                let (cards, here) = { let wb = w.borrow(); (wb.cards(), wb.current_card()) };
                let mut hits = vec![];
                match st.mode.as_str() {
                    "ref" => { let key = if st.r.has_key { Some(ctx.eval_expression_tree(&inputs[0])?) } else { None }; hits.push(resolve(&w, &live, &st.r.k(), key.as_ref())?); }
                    "all" => hits = cards.clone(),
                    "cards" | "finding" => {
                        let needle = if st.mode == "finding" { Some(ctx.eval_expression_tree(&inputs[0])?.to_string().to_lowercase()) } else { None };
                        for c in &cards {
                            let yes = match &needle {
                                Some(n) => card_texts(&w.borrow(), *c).iter().any(|(_, t)| t.to_lowercase().contains(n.as_str())),
                                None => {
                                    // Evaluate the condition with each card current, without messages.
                                    w.borrow_mut().file.current_card = *c;
                                    let r = ctx.eval_expression_tree(&inputs[0]);
                                    w.borrow_mut().file.current_card = here;
                                    r?.as_bool().map_err(|_| err("mark cards where … needs true or false"))?
                                }
                            };
                            if yes { hits.push(*c); }
                        }
                    }
                    _ => {}
                }
                let mut wb = w.borrow_mut();
                for c in hits { wb.set(c, "marked", Value::Bool(value), false); }
                Ok(Dynamic::UNIT)
            });
    }

    // there is [not] a|an <ref>   -- true when the object exists
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("there",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            let mut st = if syms.len() == 1 { ThereSt::default() } else { state.clone_cast::<ThereSt>() };
            let next: Option<ImmutableString> = match st.phase {
                0 => { st.phase = 1; Some("is".into()) }
                1 => if look == "not" { st.not = true; Some("not".into()) } else { st.phase = 2; Some(if look == "an" { "an".into() } else { "a".into() }) },
                2 => { st.phase = 3; st.r.step(look) }
                _ => st.r.step(look),
            };
            *state = Dynamic::from(st);
            Ok(next)
        }, false,
        move |ctx, inputs, state| {
            let st = state.clone_cast::<ThereSt>();
            let key = if st.r.has_key { Some(ctx.eval_expression_tree(&inputs[0])?) } else { None };
            let exists = match st.r.kind.as_str() {
                "menu" => { let n = key.map(|k| k.to_string()).unwrap_or_default(); w.borrow().menus.iter().any(|m| m.name.eq_ignore_ascii_case(&n)) }
                "file" | "folder" => {
                    let wb = w.borrow();
                    let p = wb.path(&key.map(|k| k.to_string()).unwrap_or_default());
                    let fs = wb.fs.borrow();
                    if st.r.kind == "file" { fs.is_file(&p) } else { fs.is_folder(&p) }
                }
                _ => resolve(&w, &live, &st.r.k(), key.as_ref()).is_ok(),
            };
            Ok(Dynamic::from(exists != st.not))
        });
    }

    // get <expr>  -- puts the value into `it`
    e.register_custom_syntax(["get", "$expr$"], false, |ctx, inputs| {
        let v = ctx.eval_expression_tree(&inputs[0])?;
        ctx.scope_mut().set_value("it", v);
        Ok(Dynamic::UNIT)
    }).unwrap();

    // add <n> to <c> | subtract <n> from <c> | multiply <c> by <n> | divide <c> by <n>   (c: variable or field)
    for (word, joiner, target_first) in [("add", "to", false), ("subtract", "from", false), ("multiply", "by", true), ("divide", "by", true)] {
        let (w, live) = (w.clone(), live.clone());
        e.register_custom_syntax_with_state_raw(word,
            move |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
                let mut st = if syms.len() == 1 { ArithSt::default() } else { state.clone_cast::<ArithSt>() };
                // phase 0: first operand; 1: joiner; 2: second operand; then maybe a field name.
                let container = |st: &mut ArithSt, look: &str| -> ImmutableString { if look == "field" { st.field = true; "field".into() } else { "$ident$".into() } };
                let next: Option<ImmutableString> = match (st.phase, target_first) {
                    (0, false) | (2, true) => { st.phase += 1; Some("$expr$".into()) }
                    (0, true) | (2, false) => { st.phase += 1; let c = container(&mut st, look); if st.field { st.phase = 10 + st.phase; } Some(c) }
                    (1, _) => { st.phase = 2; Some(joiner.into()) }
                    (11, true) => { st.phase = 1; Some(slot(look)) }
                    (13, false) => { st.phase = 4; Some(slot(look)) }
                    _ => None,
                };
                *state = Dynamic::from(st);
                Ok(next)
            }, true,
            move |ctx, inputs, state| {
                let st = state.clone_cast::<ArithSt>();
                let (cont, num) = if target_first { (0, inputs.len() - 1) } else { (1, 0) };
                let n = ctx.eval_expression_tree(&inputs[num])?;
                let (old, set_field) = if st.field {
                    let key = ctx.eval_expression_tree(&inputs[cont])?;
                    let id = resolve(&w, &live, "field", Some(&key))?;
                    (to_dyn(w.borrow().get(id, "text")), Some(id))
                } else {
                    let name = inputs[cont].get_string_value().unwrap_or("it").to_string();
                    (ctx.scope().get_value::<Dynamic>(&name).unwrap_or(Dynamic::from(0_i64)), None)
                };
                let op = match word { "add" => "+", "subtract" => "-", "multiply" => "*", _ => "/" };
                let r = ctx.engine().eval_with_scope::<Dynamic>(&mut { let mut s = Scope::new(); s.push("a", if old.to_string().trim().is_empty() { Dynamic::from(0_i64) } else { old }); s.push("b", n); s }, &format!("a {op} b"))?;
                match set_field {
                    Some(id) => w.borrow_mut().set(id, "text", Value::Str(r.to_string()), false),
                    None => { let name = inputs[cont].get_string_value().unwrap_or("it").to_string(); ctx.scope_mut().set_value(name, r); }
                }
                Ok(Dynamic::UNIT)
            });
    }

    // do <expr>  -- run text as statements, in this handler's context.
    // Rhai's own `do { } while` loop is switched off to free the word.
    e.disable_symbol("do");
    e.register_custom_syntax(["do", "$expr$"], false, |ctx, inputs| {
        let src = ctx.eval_expression_tree(&inputs[0])?.to_string();
        let engine = ctx.engine();
        let ast = engine.compile(&hypertalk_words(&hypertalk_blocks(&src))).map_err(|e| err(&format!("do: {e}")))?;
        let _ = engine.eval_ast_with_scope::<Dynamic>(ctx.scope_mut(), &ast)?;
        Ok(Dynamic::UNIT)
    }).unwrap();

    // play "<sound>" | play stop     (sound is always allowed)
    { let w = w.clone();
      e.register_custom_syntax_with_state_raw("play",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            Ok(match syms.len() {
                1 => if look == "stop" { *state = Dynamic::from(true); Some("stop".into()) } else { Some(if look.starts_with('"') { "$string$".into() } else { "$expr$".into() }) },
                _ => {
                    let last = syms.last().map(|s| s.as_str()).unwrap_or("");
                    if state.as_bool().unwrap_or(false) { None }
                    else if last == "tempo" { Some("$expr$".into()) }
                    else if syms.len() == 2 && look == "tempo" { Some("tempo".into()) }
                    else if !syms.iter().skip(2).any(|s| s.as_str() != "tempo" && s.as_str() != "$expr$") && syms.len() <= 4 && look.starts_with('"') { *state = Dynamic::from(2_i64); Some("$string$".into()) }
                    else { None }
                }
            })
        }, false,
        move |ctx, inputs, state| {
            if state.as_int().ok() == Some(2) {
                // play "<instrument>" [tempo n] "<notes>": a clip synthesized from the notes.
                let instrument = ctx.eval_expression_tree(&inputs[0])?.to_string();
                let tempo = if inputs.len() == 3 { index(&ctx.eval_expression_tree(&inputs[1])?)? as f32 } else { 120.0 };
                let notes = ctx.eval_expression_tree(&inputs[inputs.len() - 1])?.to_string();
                let clip = synth_notes(&instrument, tempo, &notes).map_err(|e| err(&format!("play: {e}")))?;
                let mut wb = w.borrow_mut();
                let key = format!("{instrument} {}", wb.sound_cmds.len() + wb.effect_seq as usize + notes.len());
                wb.sound = Some(instrument);
                wb.sound_cmds.push(SoundCmd::Clip(key, clip));
                return Ok(Dynamic::UNIT);
            }
            if state.as_bool().unwrap_or(false) {
                let mut wb = w.borrow_mut();
                wb.sound = None;
                wb.sound_cmds.push(SoundCmd::Stop);
                return Ok(Dynamic::UNIT);
            }
            let name = ctx.eval_expression_tree(&inputs[0])?.to_string();
            let mut wb = w.borrow_mut();
            let Some(a) = wb.file.asset(&name).filter(|a| a.kind == AssetKind::Sound).cloned() else { return Err(err(&format!("there is no sound \"{name}\" in this stack"))) };
            wb.sound = Some(a.name.clone());
            wb.sound_cmds.push(SoundCmd::Play(a.name));
            Ok(Dynamic::UNIT)
        });
    }

    // lock screen | unlock screen [with <effect> [<direction>]]
    { let w = w.clone(); e.register_custom_syntax(["lock", "screen"], false, move |_, _| { w.borrow_mut().locked = true; Ok(Dynamic::UNIT) }).unwrap(); }
    { let w = w.clone();
      e.register_custom_syntax_with_state_raw("unlock",
        |syms: &[ImmutableString], look: &str, _| {
            Ok(match syms.len() {
                1 => Some("screen".into()),
                2 => if look == "with" { Some("with".into()) } else { None },
                3 => Some("$ident$".into()),
                4 => if is_direction(look) { Some("$ident$".into()) } else { None },
                _ => None,
            })
        }, false,
        move |_, inputs, _| {
            let mut wb = w.borrow_mut();
            wb.locked = false;
            if !inputs.is_empty() { wb.set_effect(&effect_name(inputs)); }
            Ok(Dynamic::UNIT)
        });
    }

    // choose <tool name> tool | choose tool <number>   (HyperCard's order: 1 browse, 2 button, 3 field, 6 pencil, …)
    { let w = w.clone();
      e.register_custom_syntax_with_state_raw("choose",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            if syms.len() == 1 { *state = Dynamic::from(String::new()); if look == "tool" { return Ok(Some("tool".into())); } }
            let last = syms.last().map(|s| s.as_str()).unwrap_or("");
            if syms.len() == 2 && last == "tool" { return Ok(Some("$expr$".into())); }
            if last == "tool" || last == "$expr$" { return Ok(None); }
            if look == "tool" { return Ok(Some("tool".into())); }
            if !look.chars().next().is_some_and(|c| c.is_alphabetic()) { return Ok(None); }
            *state = Dynamic::from(format!("{} {}", state, look.to_lowercase()));
            Ok(Some(look.into()))
        }, false,
        move |ctx, inputs, state| {
            let tool = if inputs.is_empty() {
                let name = state.to_string().split_whitespace().filter(|w| *w != "can").collect::<Vec<_>>().join(" ");
                name.parse::<Tool>().map_err(|_| err(&format!("choose what? no {name} tool")))?
            } else {
                Tool::from_number(from_dyn(&ctx.eval_expression_tree(&inputs[0])?).int()).ok_or_else(|| err("choose tool 1 to 18"))?
            };
            let need = tool.user_level();
            if w.borrow().user_level < need { return Err(err(&format!("the {tool} tool needs userLevel {need} or more"))); }
            w.borrow_mut().tool = tool;
            Ok(Dynamic::UNIT)
        });
    }

    // drag from <point> to <point> [with shiftKey|optionKey|commandKey, …]
    // With the browse tool: mouseDown, mouseStillDown along the way, mouseUp (if it ends where it started).
    // With the button or field tool: moves the button or field under the start point. With a paint tool: paints.
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("drag",
        |syms: &[ImmutableString], look: &str, _| Ok(match (syms.len(), syms.last().map(|s| s.as_str()).unwrap_or("")) {
            (1, _) => Some("from".into()),
            (2, _) => Some("$expr$".into()),
            (3, _) => Some("to".into()),
            (4, _) => Some("$expr$".into()),
            (5, _) => if look == "with" { Some("with".into()) } else { None },
            (_, "with") | (_, ",") => Some("$ident$".into()),
            (_, "$ident$") => if look == "," { Some(",".into()) } else { None },
            _ => None,
        }), false,
        move |ctx, inputs, _| {
            let a = point(&ctx.eval_expression_tree(&inputs[0])?, "drag")?;
            let b = point(&ctx.eval_expression_tree(&inputs[1])?, "drag")?;
            let keys: Vec<String> = inputs[2..].iter().filter_map(|i| i.get_string_value()).map(|k| match k.to_lowercase().as_str() { "shiftkey" => "Shift", "optionkey" => "Alt", "commandkey" | "cmdkey" => "Meta", _ => "" }.to_string()).filter(|k| !k.is_empty()).collect();
            let tool = w.borrow().tool;
            if tool.is_paint() {
                if !tool.strokes() { w.borrow_mut().result = format!("The {tool} tool does not paint here."); return Ok(Dynamic::UNIT); }
                let mut wb = w.borrow_mut();
                let card = wb.current_card();
                add_stroke(&mut wb, card, &[a, b]);
                return Ok(Dynamic::UNIT);
            }
            if tool.edits_parts() {
                let kind = if tool == Tool::Button { Kind::Button } else { Kind::Field };
                let mut wb = w.borrow_mut();
                if let Some(id) = part_at(&wb, a, &[kind]) {
                    let (l, t) = (wb.get(id, "left").int(), wb.get(id, "top").int());
                    wb.set(id, "left", Value::Int(l + (b.0 - a.0).round() as i64), false);
                    wb.set(id, "top", Value::Int(t + (b.1 - a.1).round() as i64), false);
                }
                return Ok(Dynamic::UNIT);
            }
            let (target, rect) = {
                let wb = w.borrow();
                match part_at(&wb, a, &[Kind::Button, Kind::Field, Kind::Canvas]) {
                    Some(id) => (id, Some(wb.rect(id).map(|v| v as f64))),
                    None => (wb.current_card(), None),
                }
            };
            { let mut wb = w.borrow_mut(); for k in &keys { wb.keys.insert(k.clone()); } wb.mouse = a; wb.mouse_down = true; wb.click_loc = a; wb.pressed = Some(target); }
            let mut r = dispatch(ctx.engine(), &w, &live, target, "mouseDown", vec![]).map(|_| ());
            for i in 1..=8 {
                if r.is_err() { break; }
                let f = i as f64 / 8.0;
                w.borrow_mut().mouse = (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f);
                r = dispatch(ctx.engine(), &w, &live, target, "mouseStillDown", vec![]).map(|_| ());
            }
            { let mut wb = w.borrow_mut(); wb.mouse = b; wb.mouse_down = false; wb.pressed = None; }
            let inside = rect.map(|q| b.0 >= q[0] && b.0 < q[0] + q[2] && b.1 >= q[1] && b.1 < q[1] + q[3]).unwrap_or(true);
            if r.is_ok() && inside { r = dispatch(ctx.engine(), &w, &live, target, "mouseUp", vec![]).map(|_| ()); }
            { let mut wb = w.borrow_mut(); for k in &keys { wb.keys.remove(k); } }
            r.map(|_| Dynamic::UNIT)
        });
    }

    // reset paint: lineSize 1, not filled
    { let w = w.clone();
      e.register_custom_syntax_with_state_raw("reset",
        |syms: &[ImmutableString], look: &str, _| Ok(if syms.len() == 1 { Some(look.into()) } else { None }), false,
        move |_, _, _| { let mut wb = w.borrow_mut(); wb.line_size = 1; wb.filled = false; Ok(Dynamic::UNIT) }); }

    // menu "<name>": its items, one per line
    { let w = w.clone();
      e.register_custom_syntax_with_state_raw("menu",
        |syms: &[ImmutableString], look: &str, _| Ok(if syms.len() == 1 { Some(slot(look)) } else { None }), false,
        move |ctx, inputs, _| {
            let n = ctx.eval_expression_tree(&inputs[0])?.to_string();
            let mut wb = w.borrow_mut();
            let m = wb.menu_mut(&n).map_err(|e| err(&e))?;
            Ok(Dynamic::from(m.items.iter().map(|i| i.text.clone()).collect::<Vec<_>>().join("\n")))
        }); }

    // Menus (statements rewritten by `menu_statement`).
    { let w = w.clone(); e.register_fn("__messageBox", move |on: bool| { let mut wb = w.borrow_mut(); wb.message_visible = on; if on { wb.player_requests.push(PlayerRequest::FocusMessageBox); } }); }
    { let w = w.clone(); e.register_fn("__menuReset", move || { w.borrow_mut().menus = Rc::new(default_menus()); }); }
    { let w = w.clone(); e.register_fn("__menuDelete", move |name: Dynamic| -> R<()> {
        let n = name.to_string();
        let mut wb = w.borrow_mut();
        let i = wb.menus.iter().position(|m| m.name.eq_ignore_ascii_case(&n)).ok_or_else(|| err(&format!("no menu \"{n}\"")))?;
        Rc::make_mut(&mut wb.menus).remove(i);
        Ok(())
    }); }
    { let w = w.clone(); e.register_fn("__menuPut", move |value: Dynamic, how: &str, name: Dynamic, msgs: Dynamic| -> R<()> {
        let mut wb = w.borrow_mut();
        let m = wb.menu_mut(&name.to_string()).map_err(|e| err(&e))?;
        // Items are separated by returns or commas; menuMessages likewise, one per item.
        let split = |s: String| -> Vec<String> { if s.is_empty() { vec![] } else { s.split(['\n', ',']).map(|x| x.trim().to_string()).collect() } };
        let msgs = split(msgs.to_string());
        let items: Vec<MenuItem> = split(value.to_string()).into_iter().enumerate().map(|(i, t)| { let mut it = MenuItem::new(&t); it.message = msgs.get(i).cloned().unwrap_or_default(); it }).collect();
        match how { "into" => m.items = items, "before" => { m.items.splice(0..0, items); } _ => m.items.extend(items) }
        Ok(())
    }); }
    { let w = w.clone(); e.register_fn("__menuItem", move |op: &str, item: Dynamic, name: Dynamic| -> R<()> {
        let (it, n) = (item.to_string(), name.to_string());
        let mut wb = w.borrow_mut();
        let m = wb.menu_mut(&n).map_err(|e| err(&e))?;
        if it.is_empty() { for i in &mut m.items { i.enabled = op == "enable"; } return Ok(()); }
        let k = match it.trim().parse::<usize>() { Ok(k) if k >= 1 => Some(k - 1), _ => m.items.iter().position(|i| i.text.eq_ignore_ascii_case(&it)) }
            .filter(|k| *k < m.items.len()).ok_or_else(|| err(&format!("no menu item \"{it}\" in menu \"{n}\"")))?;
        match op { "delete" => { m.items.remove(k); } "enable" => m.items[k].enabled = true, "disable" => m.items[k].enabled = false, "check" => m.items[k].checked = true, "uncheck" => m.items[k].checked = false, _ => {} }
        Ok(())
    }); }

    // doMenu <item> [, <menu>]: an item with a menuMessage sends that message; otherwise `doMenu item, menu`
    // goes to the current card, and when no handler takes it the item does its built-in action.
    for args in [1, 2] {
        let (w, live) = (w.clone(), live.clone());
        let f = move |ctx: rhai::NativeCallContext, item: String, menu: String| -> R<()> {
            let found = {
                let wb = w.borrow();
                wb.menus.iter().filter(|m| menu.is_empty() || m.name.eq_ignore_ascii_case(&menu))
                    .find_map(|m| m.items.iter().find(|i| i.text.eq_ignore_ascii_case(&item)).map(|i| (m.name.clone(), i.clone())))
            };
            let card = w.borrow().current_card();
            if let Some((_, i)) = &found {
                if !i.enabled { return Ok(()); }
                if !i.message.is_empty() { dispatch(ctx.engine(), &w, &live, card, &i.message, vec![])?; return Ok(()); }
            }
            let menu_name = found.as_ref().map(|f| f.0.clone()).unwrap_or(menu);
            if dispatch(ctx.engine(), &w, &live, card, "doMenu", vec![Dynamic::from(item.clone()), Dynamic::from(menu_name)])? { return Ok(()); }
            let key = item.to_lowercase().replace("...", "…");
            let script = match key.as_str() {
                "first" | "prev" | "previous" | "next" | "last" | "back" => format!("go {key};"),
                "new card" => "new card;".into(),
                "delete card" => "delete this card;".into(),
                "find…" | "find" => "ask \"Find:\"; if it != \"\" { find it; }".into(),
                "new stack…" | "new stack" => "ask \"Name of the new stack:\"; if it != \"\" { create stack it; go to stack it in a new window; }".into(),
                "undo" => { w.borrow_mut().player_requests.push(PlayerRequest::Undo); return Ok(()); }
                "message" => { let mut wb = w.borrow_mut(); wb.message_visible = true; wb.player_requests.push(PlayerRequest::FocusMessageBox); return Ok(()); }
                "quit hypercard" | "quit" => { dispatch(ctx.engine(), &w, &live, card, "quit", vec![])?; w.borrow_mut().player_requests.push(PlayerRequest::Quit); return Ok(()); }
                k if k.parse::<Tool>().is_ok() => format!("choose {k} tool;"),
                _ if found.is_some() => return Ok(()),
                _ => return Err(err(&format!("doMenu: no menu item \"{item}\""))),
            };
            let ast = ctx.engine().compile(&hypertalk_words(&hypertalk_blocks(&script)))?;
            let _ = ctx.engine().eval_ast_with_scope::<Dynamic>(&mut Scope::new(), &ast)?;
            Ok(())
        };
        if args == 1 { let f = f.clone(); e.register_fn("__doMenu", move |ctx: rhai::NativeCallContext, item: Dynamic| f(ctx, item.to_string(), String::new())); }
        else { e.register_fn("__doMenu", move |ctx: rhai::NativeCallContext, item: Dynamic, menu: Dynamic| f(ctx, item.to_string(), menu.to_string())); }
    }

    // visual [effect] <name> [direction] [very slow|slow|fast|very fast] [to black|white|gray|inverse|card]:
    // the effect for the next `go` in this handler run.
    { let w = w.clone();
      e.register_custom_syntax_with_state_raw("visual",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            if syms.len() == 1 { *state = Dynamic::from(String::new()); }
            if syms.len() == 1 && look == "effect" { return Ok(Some("effect".into())); }
            // Words until the statement ends; `in` (zoom in) is a keyword, so words are kept in the state.
            if !look.chars().next().is_some_and(|c| c.is_alphabetic()) { return Ok(None); }
            *state = Dynamic::from(format!("{} {}", state, look.to_lowercase()));
            Ok(Some(if look == "in" { "in".into() } else { "$ident$".into() }))
        }, false,
        move |_, _, state| {
            let name = effect_from_words(state.to_string().split_whitespace().collect::<Vec<_>>());
            if name.is_empty() { return Err(err("visual effect what? e.g. visual effect dissolve")); }
            w.borrow_mut().visual = Some(name);
            Ok(Dynamic::UNIT)
        });
    }

    // Drawing. The target canvas is the innermost `with canvas` block, else `me` if it is a canvas,
    // else the first canvas on the current card. Coordinates are canvas-relative.
    fn f(d: &Dynamic) -> f64 { d.as_float().ok().or_else(|| d.as_int().ok().map(|i| i as f64)).or_else(|| d.clone().into_string().ok().and_then(|s| s.trim().parse().ok())).unwrap_or(0.0) }
    fn draw(w: &W, live: &L, s: Shape) -> R<()> {
        let id = canvas_target(w, live).ok_or_else(|| err("there is no canvas on this card to draw on"))?;
        Rc::make_mut(&mut w.borrow_mut().draw).entry(id).or_default().push(s);
        Ok(())
    }
    { let (w, live) = (w.clone(), live.clone()); e.register_fn("clear", move || -> R<()> {
        let id = canvas_target(&w, &live).ok_or_else(|| err("there is no canvas on this card to clear"))?;
        Rc::make_mut(&mut w.borrow_mut().draw).remove(&id); Ok(()) }); }
    { let (w, live) = (w.clone(), live.clone()); e.register_fn("rect", move |x: Dynamic, y: Dynamic, wd: Dynamic, h: Dynamic, c: &str| {
        draw(&w, &live, Shape::Rect { x: f(&x), y: f(&y), w: f(&wd), h: f(&h), color: c.into() }) }); }
    { let (w, live) = (w.clone(), live.clone()); e.register_fn("circle", move |x: Dynamic, y: Dynamic, r: Dynamic, c: &str| {
        draw(&w, &live, Shape::Circle { x: f(&x), y: f(&y), r: f(&r), color: c.into() }) }); }
    { let (w, live) = (w.clone(), live.clone()); e.register_fn("text", move |x: Dynamic, y: Dynamic, t: &str, c: &str| {
        draw(&w, &live, Shape::Text { x: f(&x), y: f(&y), text: t.into(), color: c.into() }) }); }

    // with canvas <name> { ... }   -- draw on a named canvas inside the block
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("with",
        |syms: &[ImmutableString], look: &str, _| {
            Ok(match syms.len() { 1 => Some("canvas".into()), 2 => Some(slot(look)), 3 => Some("$block$".into()), _ => None })
        }, false,
        move |ctx, inputs, _| {
            let key = ctx.eval_expression_tree(&inputs[0])?;
            let id = resolve(&w, &live, "canvas", Some(&key))?;
            live.borrow_mut().canvas.push(id);
            let r = ctx.eval_expression_tree(&inputs[1]);
            live.borrow_mut().canvas.pop();
            r.map(|_| Dynamic::UNIT)
        });
    }
    { let (w, live) = (w.clone(), live.clone()); e.register_fn("isKeyDown", move |k: &str| taped(&live, &format!("isKeyDown {k}"), || Dynamic::from(w.borrow().keys.contains(k))).as_bool().unwrap_or(false)); }

    // go next | prev | first | last | to card <name or number>  [with <effect> [<direction>]]   (a command)
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("go",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            // go [to] next|prev|first|last [card] | go [to] card <name> [of stack <name>] | go [to] stack <name>
            //   … [in a new window] [with <effect> [<direction>]]
            let mut st = if syms.len() == 1 { GoSt::default() } else { state.clone_cast::<GoSt>() };
            fn after_target(st: &mut GoSt, look: &str) -> Option<ImmutableString> {
                match look {
                    "with" => { st.phase = 4; Some("with".into()) }
                    "in" if st.stack => { st.phase = 10; Some("in".into()) }
                    "card" if matches!(st.how.as_str(), "next" | "prev" | "previous" | "first" | "last") && !st.card => { st.card = true; Some("card".into()) }
                    _ => None,
                }
            }
            let next: Option<ImmutableString> = match st.phase {
                0 | 1 => match look {
                    "to" if st.phase == 0 => { st.phase = 1; Some("to".into()) }
                    "card" => { st.how = "to".into(); st.phase = 2; Some("card".into()) }
                    "stack" => { st.how = "to".into(); st.stack = true; st.phase = 7; Some("stack".into()) }
                    _ => { st.how = look.into(); st.phase = 3; Some(look.into()) }
                },
                2 => { st.card = true; st.phase = 21; Some(slot(look)) }
                21 => if look == "of" { st.phase = 22; Some("of".into()) } else { st.phase = 3; after_target(&mut st, look) },
                22 => { st.stack = true; st.phase = 7; Some("stack".into()) }
                7 => { st.phase = 3; Some(slot(look)) }
                3 => after_target(&mut st, look),
                4 => { st.phase = 5; st.effect.push(look.to_string()); Some("$ident$".into()) }
                5 => if is_direction(look) { st.phase = 6; st.effect.push(look.to_string()); Some("$ident$".into()) } else { None },
                10 => { st.phase = 11; Some("a".into()) }
                11 => { st.phase = 12; Some("new".into()) }
                12 => { st.new_window = true; st.phase = 3; Some("window".into()) }
                _ => None,
            };
            *state = Dynamic::from(st);
            Ok(next)
        }, false,
        move |ctx, inputs, state| {
            let st = state.clone_cast::<GoSt>();
            let how = st.how.clone();
            let mut k = 0;
            let card_key = if st.card && how == "to" { k += 1; Some(from_dyn(&ctx.eval_expression_tree(&inputs[0])?)) } else { None };
            if st.stack {
                // Another stack: the player opens it (in this tab, or a new one), then goes to the card.
                let name = ctx.eval_expression_tree(&inputs[k])?.to_string();
                let found = w.borrow().find_stack(&name);
                let Some(path) = found else { w.borrow_mut().result = "No such stack.".into(); return Ok(Dynamic::UNIT) };
                let mut wb = w.borrow_mut();
                wb.result.clear();
                // `go … with <effect>`, else the effect a `visual effect` command set earlier in this run.
                let pending = wb.visual.take();
                let effect = if st.effect.is_empty() { pending } else { Some(effect_from_words(st.effect.iter().map(String::as_str))) };
                wb.go_stack = Some(GoStack { path, card: card_key, new_window: st.new_window, effect });
                return Ok(Dynamic::UNIT);
            }
            // For `go back`: where the card gone back to sits in the recent list (it and what follows are dropped once there).
            let mut back_at = None;
            let target = {
                let wb = w.borrow();
                let cards = wb.cards();
                let pos = cards.iter().position(|c| *c == wb.current_card()).unwrap_or(0);
                let n = cards.len().max(1);
                match (how.as_str(), card_key) {
                    ("next", _) => cards.get((pos + 1) % n).copied(),
                    ("prev", _) | ("previous", _) => cards.get((pos + n - 1) % n).copied(),
                    ("first", _) => cards.first().copied(),
                    ("last", _) => cards.last().copied(),
                    // The card before this one in the recent list (go back walks back through it).
                    ("back", _) => { back_at = wb.recent.iter().rposition(|c| wb.obj(*c).is_some() && *c != wb.current_card()); back_at.map(|i| wb.recent[i]) }
                    ("to", Some(Value::Int(i))) => cards.get((i as usize).wrapping_sub(1)).copied(),
                    ("to", Some(v)) => { let name = v.text().to_lowercase(); cards.iter().copied().find(|c| wb.obj(*c).map(|o| o.name.to_lowercase() == name).unwrap_or(false)) }
                    _ => return Err(err("go where? use next, prev, first, last, card <name> or stack <name>")),
                }
            };
            let Some(target) = target else { w.borrow_mut().result = "No such card.".into(); return Ok(Dynamic::UNIT) };
            w.borrow_mut().result.clear();
            let from = me(&w, &live);
            let name = w.borrow().get(target, "name").text();
            if !dispatch(ctx.engine(), &w, &live, from, "go", vec![name.into()])? {
                // `go … with <effect>`, else the effect a `visual effect` command set earlier in this run.
                let pending = w.borrow_mut().visual.take();
                if !st.effect.is_empty() { w.borrow_mut().set_effect(&effect_from_words(st.effect.iter().map(String::as_str))); }
                else if let Some(v) = pending { w.borrow_mut().set_effect(&v); }
                navigate(ctx.engine(), &w, &live, target)?;
                // Going back does not add to the recent list; the card gone back to (and anything after it) leaves it.
                if let Some(i) = back_at { w.borrow_mut().recent.truncate(i); }
            }
            Ok(Dynamic::UNIT)
        });
    }

    // new card      (a command)
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax(["new", "card"], false, move |ctx, _| {
        let id = w.borrow_mut().new_card();
        navigate(ctx.engine(), &w, &live, id)?;
        // As in HyperCard, newCard goes to the new card once it exists.
        w.borrow_mut().auto_depth += 1;
        let r = if w.borrow().lock_messages { Ok(false) } else { dispatch(ctx.engine(), &w, &live, id, "newCard", vec![]) };
        w.borrow_mut().auto_depth -= 1;
        r?;
        Ok(Dynamic::UNIT)
      }).unwrap();
    }

    // find [normal|whole|word|chars|string] <text> [in field <name>]   (a command)
    // Searches the cards after this one (wrapping round), shows the match as the selection, sets the result.
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("find",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            if syms.len() == 1 {
                if matches!(look, "normal" | "whole" | "word" | "chars" | "string") { *state = Dynamic::from(look.to_string()); return Ok(Some(look.into())); }
                *state = Dynamic::from("normal".to_string());
                // A literal ends at the literal, so `find "x" in field "y"` is not Rhai's `in`.
                return Ok(Some(if look.starts_with('"') { "$string$".into() } else { "$expr$".into() }));
            }
            let last = syms.last().map(|s| s.as_str()).unwrap_or("");
            let after_text = syms.len() == 2 && !matches!(syms[1].as_str(), "normal" | "whole" | "word" | "chars" | "string") || syms.len() == 3;
            // … of marked cards: search only the marked cards.
            if last == "of" { return Ok(Some("marked".into())); }
            if last == "marked" { return Ok(Some("cards".into())); }
            if last == "cards" { return Ok(None); }
            if look == "of" && (after_text || syms.len() >= 2 && syms[syms.len() - 2] == "field") { *state = Dynamic::from(format!("{} marked", state)); return Ok(Some("of".into())); }
            Ok(match last {
                "normal" | "whole" | "word" | "chars" | "string" if syms.len() == 2 => Some(if look.starts_with('"') { "$string$".into() } else { "$expr$".into() }),
                _ if after_text && !syms.iter().any(|s| s == "in") => if look == "in" { Some("in".into()) } else { None },
                "in" => Some("field".into()),
                "field" => Some(slot(look)),
                _ => None,
            })
        }, false,
        move |ctx, inputs, state| {
            let flags = state.to_string();
            let marked_only = flags.ends_with(" marked");
            let form = flags.trim_end_matches(" marked").to_string();
            let needle = ctx.eval_expression_tree(&inputs[0])?.to_string();
            let only = if inputs.len() > 1 { let k = ctx.eval_expression_tree(&inputs[1])?; Some(from_dyn(&k).text().to_lowercase()) } else { None };
            let from = me(&w, &live);
            if dispatch(ctx.engine(), &w, &live, from, "find", vec![needle.clone().into()])? { return Ok(Dynamic::UNIT); }
            let found = {
                let wb = w.borrow();
                let cards = wb.cards();
                let pos = cards.iter().position(|c| *c == wb.current_card()).unwrap_or(0);
                (1..=cards.len()).map(|k| cards[(pos + k) % cards.len()]).filter(|c| !marked_only || wb.get(*c, "marked").truthy()).find_map(|c| {
                    card_texts(&wb, c).into_iter()
                        // Shared-text fields read the same on every card, so they cannot tell cards apart; dontSearch opts out.
                        .filter(|(f, _)| only.is_some() || !(wb.get(*f, "sharedText").truthy() || wb.get(*f, "dontSearch").truthy()))
                        .filter(|(f, _)| only.as_ref().map(|n| wb.obj(*f).map(|o| o.name.to_lowercase() == *n).unwrap_or(false)).unwrap_or(true))
                        .find_map(|(f, t)| find_in(&form, &needle, &t).map(|(a, b)| (c, f, a, b)))
                })
            };
            match found {
                Some((c, f, a, b)) if !needle.trim().is_empty() => {
                    w.borrow_mut().result.clear();
                    navigate(ctx.engine(), &w, &live, c)?;
                    let mut wb = w.borrow_mut();
                    wb.selection = Some((f, a, b));
                    wb.select_request = Some((f, a, b));
                }
                _ => { let mut wb = w.borrow_mut(); wb.result = "not found".into(); wb.log.push(format!("find: \"{needle}\" not found")); }
            }
            Ok(Dynamic::UNIT)
        });
    }

    // answer <expr>   (a command)
    { let (w, live) = (w.clone(), live.clone());
      // answer <expr> [with "A" [or "B" [or "C"]]]: the script waits; the clicked button goes into `it`.
      e.register_custom_syntax_with_state_raw("answer",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            if syms.len() == 1 && look == "file" { *state = Dynamic::from(true); return Ok(Some("file".into())); }
            if state.as_bool().unwrap_or(false) { return Ok(if syms.len() == 2 { Some("$expr$".into()) } else { None }); }
            Ok(match syms.len() {
                1 => Some("$expr$".into()),
                2 => if look == "with" { Some("with".into()) } else { None },
                3 | 5 | 7 => Some("$string$".into()),
                4 | 6 => if look == "or" { Some("or".into()) } else { None },
                _ => None,
            })
        }, true,
        move |ctx, inputs, state| {
            let v = ctx.eval_expression_tree(&inputs[0])?;
            if state.as_bool().unwrap_or(false) {
                // answer file "prompt": the chosen path goes into `it` (empty, and the result "Cancel", if cancelled)
                if let Some(r) = pause_point(&w, &live, "answer file")? { return reply_into_it(ctx, &w, r); }
                let mut wb = w.borrow_mut();
                let folder = wb.path(".");
                wb.dialog = Some(v.to_string());
                wb.dialog_buttons = vec![];
                wb.dialog_ask = None;
                wb.dialog_file = Some((false, folder));
                return Err(pause(&live, Waiting::Dialog));
            }
            let buttons: Vec<String> = inputs[1..].iter().filter_map(|i| i.get_string_value().map(String::from)).collect();
            let from = me(&w, &live);
            // A handler for `answer` on the path can take it instead of the dialog.
            if dispatch(ctx.engine(), &w, &live, from, "answer", vec![v.clone()])? { return Ok(Dynamic::UNIT); }
            if let Some(r) = pause_point(&w, &live, "answer")? { return reply_into_it(ctx, &w, r); }
            let mut wb = w.borrow_mut();
            wb.dialog = Some(v.to_string());
            wb.dialog_buttons = buttons;
            wb.dialog_ask = None;
            Err(pause(&live, Waiting::Dialog))
        });
    }

    // ask [file] <expr> [with <default>]: the script waits; the typed text (or path) goes into `it`.
    // Cancel leaves `it` empty and sets the result to "Cancel".
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("ask",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            // ask [file] <prompt> [with <default>]
            if syms.len() == 1 && look == "file" { *state = Dynamic::from(true); return Ok(Some("file".into())); }
            let k = if state.as_bool().unwrap_or(false) { syms.len() - 1 } else { syms.len() };
            Ok(match k { 1 => Some("$expr$".into()), 2 => if look == "with" { Some("with".into()) } else { None }, 3 => Some("$expr$".into()), _ => None })
        }, true,
        move |ctx, inputs, state| {
            let file = state.as_bool().unwrap_or(false);
            let q = ctx.eval_expression_tree(&inputs[0])?.to_string();
            let def = if inputs.len() > 1 { ctx.eval_expression_tree(&inputs[1])?.to_string() } else { String::new() };
            if let Some(r) = pause_point(&w, &live, if file { "ask file" } else { "ask" })? { return reply_into_it(ctx, &w, r); }
            let mut wb = w.borrow_mut();
            wb.dialog = Some(q);
            wb.dialog_buttons = vec![];
            wb.dialog_ask = Some(def);
            wb.dialog_file = if file { Some((true, wb.path("."))) } else { None };
            Err(pause(&live, Waiting::Dialog))
        });
    }

    // wait [for] <n> [ticks|seconds|milliseconds]   (ticks if no unit, as in HyperCard)
    // wait until <condition>   /   wait while <condition>
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("wait",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            let last = syms.last().map(|s| s.as_str()).unwrap_or("");
            Ok(match (syms.len(), last) {
                (1, _) => match look {
                    "until" | "while" => { *state = Dynamic::from(look.to_string()); Some(look.into()) }
                    "for" => Some("for".into()),
                    _ => { *state = Dynamic::from("ticks".to_string()); Some("$expr$".into()) }
                },
                (2, "for") => { *state = Dynamic::from("ticks".to_string()); Some("$expr$".into()) }
                (_, "$expr$") if !matches!(state.to_string().as_str(), "until" | "while")
                    && matches!(look, "ticks" | "tick" | "seconds" | "second" | "secs" | "sec" | "milliseconds" | "millisecond" | "ms") => {
                    *state = Dynamic::from(look.to_string()); Some(look.into())
                }
                (2, "until" | "while") => Some("$expr$".into()),
                _ => None,
            })
        }, false,
        move |ctx, inputs, state| {
            let how = state.to_string();
            if how == "until" || how == "while" {
                let tape = live.borrow().tape.clone();
                if !tape.borrow().on { return Err(err("wait cannot pause here: frame and errorDialog handlers run to the end")); }
                let now_live = tape.borrow_mut().go_live();
                if now_live { settle(&w, &live); }
                let at = tape.borrow().entries.len();
                let c = ctx.eval_expression_tree(&inputs[0])?;
                let c = c.as_bool().unwrap_or_else(|_| { let t = c.to_string(); t.eq_ignore_ascii_case("true") });
                if c == (how == "until") || tape.borrow().replaying { return Ok(Dynamic::UNIT); }
                // Not yet: forget this check's reads (they are read afresh next time), and look again shortly.
                { let mut t = tape.borrow_mut(); t.entries.truncate(at); t.pos = at; }
                return Err(pause(&live, Waiting::Poll(now_ms() + 30.0)));
            }
            let n = num(&ctx.eval_expression_tree(&inputs[0])?).ok_or_else(|| err("wait needs a number"))?;
            let ms = duration_ms(n, &how).unwrap_or_else(|| n * 1000.0 / 60.0);
            if ms <= 0.0 { return Ok(Dynamic::UNIT); }
            if pause_point(&w, &live, "wait")?.is_some() { return Ok(Dynamic::UNIT); }
            Err(pause(&live, Waiting::Until(now_ms() + ms)))
        });
    }

    // sort [cards] [ascending|descending] [numeric|text] by <expr>  -- <expr> is evaluated on each card
    { let w = w.clone();
      e.register_custom_syntax_with_state_raw("Sort",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            if syms.len() == 1 { *state = Dynamic::from(String::new()); }
            let last = syms.last().map(|s| s.as_str()).unwrap_or("");
            if last == "by" { return Ok(Some("$expr$".into())); }
            if last == "$expr$" { return Ok(None); }
            if matches!(look, "descending" | "numeric") { *state = Dynamic::from(format!("{} {look}", state)); }
            Ok(Some(if matches!(look, "cards" | "ascending" | "descending" | "numeric" | "text" | "by") { look.into() } else { "by".into() }))
        }, false,
        move |ctx, inputs, state| {
            let flags = state.to_string();
            let (cards, here) = { let wb = w.borrow(); (wb.cards(), wb.current_card()) };
            let mut keyed = vec![];
            for c in &cards {
                // Evaluate the key with each card current, without sending any messages.
                w.borrow_mut().file.current_card = *c;
                let k = ctx.eval_expression_tree(&inputs[0]);
                w.borrow_mut().file.current_card = here;
                keyed.push((k?.to_string(), *c));
            }
            sort_keyed(&mut keyed, &flags);
            let order: Vec<u32> = keyed.into_iter().map(|(_, c)| c).collect();
            w.borrow_mut().reorder_cards(&order);
            Ok(Dynamic::UNIT)
        });
    }

    // push card | push this card | pop card
    { let w = w.clone(); e.register_custom_syntax_with_state_raw("Push",
        |syms: &[ImmutableString], look: &str, _| Ok(match syms.len() { 1 => Some(if look == "this" { "this".into() } else { "card".into() }), 2 if syms[1] == "this" => Some("card".into()), _ => None }),
        false, move |_, _, _| { let mut wb = w.borrow_mut(); let c = wb.current_card(); wb.push_list.push(c); Ok(Dynamic::UNIT) }); }
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax(["Pop", "card"], false, move |ctx, _| {
        let target = w.borrow_mut().push_list.pop();
        match target {
            Some(t) if w.borrow().obj(t).is_some() => { w.borrow_mut().result.clear(); navigate(ctx.engine(), &w, &live, t)?; }
            _ => w.borrow_mut().result = "No pushed card.".into(),
        }
        Ok(Dynamic::UNIT)
      }).unwrap();
    }

    // edit the script of <ref>  -- the player opens that script in the editor
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("edit",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            Ok(match syms.len() {
                1 => Some("the".into()),
                2 => Some("script".into()),
                3 => Some("of".into()),
                _ => { let mut st = if syms.len() == 4 { RefSt::default() } else { state.clone_cast::<RefSt>() }; let n = st.step(look); *state = Dynamic::from(st); n }
            })
        }, false,
        move |ctx, inputs, state| {
            let st = state.clone_cast::<RefSt>();
            let key = if st.has_key { Some(ctx.eval_expression_tree(&inputs[0])?) } else { None };
            let id = resolve(&w, &live, &st.k(), key.as_ref())?;
            w.borrow_mut().edit_request = Some(id);
            Ok(Dynamic::UNIT)
        });
    }

    // select [before|after] <chunk> <n> [to <m>] [of <chunk> <n> …] of field <name>
    // select [before|after] text of field <name>   |   select empty
    { let (w, live) = (w.clone(), live.clone());
      #[derive(Clone, Default)]
      struct SelSt { pos: String, chunk: String, phase: u8, levels: Vec<(String, bool)> }
      e.register_custom_syntax_with_state_raw("select",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            let mut st = if syms.len() == 1 { SelSt::default() } else { state.clone_cast::<SelSt>() };
            let next: Option<ImmutableString> = match st.phase {
                0 => if look == "empty" { st.phase = 9; Some("empty".into()) }
                     else if matches!(look, "before" | "after") && st.pos.is_empty() { st.pos = look.into(); Some(look.into()) }
                     else if look == "text" { st.chunk = "text".into(); st.phase = 3; Some("text".into()) }
                     else { st.chunk = look.into(); st.levels.push((look.into(), false)); st.phase = 1; Some(look.into()) },
                1 => { st.phase = 2; Some("$expr$".into()) }
                2 => if look == "to" && !st.levels.last().map(|l| l.1).unwrap_or(true) { if let Some(l) = st.levels.last_mut() { l.1 = true; } st.phase = 1; Some("to".into()) } else { st.phase = 6; Some("of".into()) },
                // after `of`: another chunk (item 2 of line 3 of …) or the field
                6 => if matches!(look, "line" | "item" | "word" | "char" | "character") { st.levels.push((look.into(), false)); st.phase = 1; Some(look.into()) } else { st.phase = 5; Some("field".into()) },
                3 => { st.phase = 4; Some("of".into()) }
                4 => { st.phase = 5; Some("field".into()) }
                5 => { st.phase = 9; Some(slot(look)) }
                _ => None,
            };
            *state = Dynamic::from(st);
            Ok(next)
        }, false,
        move |ctx, inputs, state| {
            let st = state.clone_cast::<SelSt>();
            if st.chunk.is_empty() { let mut wb = w.borrow_mut(); wb.selection = None; wb.select_request = None; return Ok(Dynamic::UNIT); }
            let field_key = ctx.eval_expression_tree(&inputs[inputs.len() - 1])?;
            let f = resolve(&w, &live, "field", Some(&field_key))?;
            let text = w.borrow().get(f, "text").text();
            let (a, b) = if st.chunk == "text" { (0, text.chars().count()) } else {
                // Inputs come innermost first (item 2 of line 3); apply them outermost first, each within the last.
                let mut at = 0;
                let mut bounds = vec![];
                for (chunk, range) in &st.levels {
                    let kind = match chunk.as_str() { "character" => "char", k => k };
                    if !CHUNKS.contains(&kind) { return Err(err(&format!("select what? use select line 2 of field …, not {chunk}"))); }
                    let n = index(&ctx.eval_expression_tree(&inputs[at])?)?;
                    let m = if *range { at += 1; index(&ctx.eval_expression_tree(&inputs[at])?)? } else { n };
                    at += 1;
                    bounds.push((kind, n, m));
                }
                let chars: Vec<char> = text.chars().collect();
                let (mut a, mut b) = (0, chars.len());
                for (kind, n, m) in bounds.into_iter().rev() {
                    let inner: String = chars[a..b].iter().collect();
                    let (x, y) = (chunk_span(kind, n, &inner).0, chunk_span(kind, m, &inner).1);
                    (a, b) = (a + x, a + y.max(x));
                }
                (a, b)
            };
            let r = match st.pos.as_str() { "before" => (f, a, a), "after" => (f, b, b), _ => (f, a, b) };
            let mut wb = w.borrow_mut();
            wb.selection = Some(r);
            wb.select_request = Some(r);
            Ok(Dynamic::UNIT)
        });
    }

    // click at "<h>,<v>"  -- as if the user clicked the button or field there (card coordinates)
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax(["click", "at", "$expr$"], false, move |ctx, inputs| {
        let p = point(&ctx.eval_expression_tree(&inputs[0])?, "click at")?;
        // With a paint tool a click paints a dot (or does nothing, for a tool that leaves no stroke).
        let tool = w.borrow().tool;
        if tool.is_paint() { if tool.strokes() { let mut wb = w.borrow_mut(); let card = wb.current_card(); add_stroke(&mut wb, card, &[p]); } return Ok(Dynamic::UNIT); }
        let target = { let wb = w.borrow(); part_at(&wb, p, &[Kind::Button, Kind::Field]).unwrap_or_else(|| wb.current_card()) };
        w.borrow_mut().click_loc = p;
        dispatch(ctx.engine(), &w, &live, target, "mouseDown", vec![])?;
        dispatch(ctx.engine(), &w, &live, target, "mouseUp", vec![])?;
        Ok(Dynamic::UNIT)
      }).unwrap();
    }

    // type <expr>  -- as if typed into the field being edited (or the first unlocked field on the card)
    { let w = w.clone();
      e.register_custom_syntax(["type", "$expr$"], false, move |ctx, inputs| {
        let t = ctx.eval_expression_tree(&inputs[0])?.to_string();
        let f = {
            let wb = w.borrow();
            // Where the selection is, else the field being edited, else the first unlocked field.
            wb.selection.map(|(f, _, _)| f).or(wb.focused).filter(|f| wb.obj(*f).is_some()).or_else(|| wb.parts_on(wb.current_card()).into_iter()
                .find(|id| wb.obj(*id).map(|o| o.kind == Kind::Field).unwrap_or(false) && !wb.get(*id, "locked").truthy()))
        }.ok_or_else(|| err("type: there is no field to type into"))?;
        let mut wb = w.borrow_mut();
        let old: Vec<char> = wb.get(f, "text").text().chars().collect();
        // Typing replaces the selection in that field, or goes at the end.
        let (a, b) = match wb.selection { Some((sf, a, b)) if sf == f => (a.min(old.len()), b.min(old.len())), _ => (old.len(), old.len()) };
        let new: String = old[..a].iter().copied().chain(t.chars()).chain(old[b..].iter().copied()).collect();
        let caret = a + t.chars().count();
        wb.selection = Some((f, caret, caret));
        wb.set(f, "text", Value::Str(new), false);
        Ok(Dynamic::UNIT)
      }).unwrap();
    }

    // create button|field [<name>]  -- a new part on this card; newButton / newField are sent to it
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("create",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            Ok(match syms.len() {
                1 => { *state = Dynamic::from(look.to_string()); Some(look.into()) }
                2 => if look.starts_with('"') || look == "(" { Some(slot(look)) } else { None },
                _ => None,
            })
        }, false,
        move |ctx, inputs, state| {
            if state.to_string() == "menu" {
                let name = ctx.eval_expression_tree(inputs.first().ok_or_else(|| err("create menu needs a name"))?)?.to_string();
                let mut wb = w.borrow_mut();
                if wb.menus.iter().any(|m| m.name.eq_ignore_ascii_case(&name)) { return Err(err(&format!("there is already a menu \"{name}\""))); }
                Rc::make_mut(&mut wb.menus).push(Menu { name, items: vec![] });
                return Ok(Dynamic::UNIT);
            }
            if state.to_string() == "stack" {
                // create stack "<name>": a new stack file, one card on one background, in this stack's folder.
                let name = ctx.eval_expression_tree(inputs.first().ok_or_else(|| err("create stack needs a name"))?)?.to_string();
                let name = name.trim().trim_end_matches(".stack").to_string();
                if name.is_empty() { return Err(err("create stack needs a name")); }
                let (width, height, path) = { let wb = w.borrow(); (wb.file.width, wb.file.height, wb.path(&format!("{name}.stack"))) };
                if w.borrow().fs.borrow().is_file(&path) { return Err(err(&format!("there is already a stack \"{name}\""))); }
                let mut file = crate::demo::blank(&name);
                (file.width, file.height) = (width, height);
                let fs = w.borrow().fs.clone();
                fs.borrow_mut().write_bytes(&path, &file.to_zip()).map_err(|e| err(&e))?;
                w.borrow_mut().result.clear();
                return Ok(Dynamic::UNIT);
            }
            if state.to_string() == "folder" {
                let p = ctx.eval_expression_tree(inputs.first().ok_or_else(|| err("create folder needs a path"))?)?.to_string();
                let wb = w.borrow();
                let p = wb.path(&p);
                wb.fs.borrow_mut().create_folder(&p).map_err(|e| err(&e))?;
                return Ok(Dynamic::UNIT);
            }
            let kind = match state.to_string().as_str() { "button" => Kind::Button, "field" => Kind::Field, k => return Err(err(&format!("create what? create button, field, menu, stack or folder, not {k}"))) };
            let name = if inputs.is_empty() { None } else { Some(ctx.eval_expression_tree(&inputs[0])?.to_string()) };
            let id = {
                let mut wb = w.borrow_mut();
                let card = wb.current_card();
                let n = wb.parts_on(card).iter().filter(|p| wb.obj(**p).map(|o| o.kind == kind).unwrap_or(false)).count() + 1;
                let name = name.unwrap_or_else(|| format!("{} {n}", if kind == Kind::Button { "button" } else { "field" }));
                let id = wb.create(kind, &name, Some(card), None);
                for (k, v) in [("left", 20 + 10 * n as i64), ("top", 20 + 10 * n as i64), ("width", 130), ("height", 34)] { wb.set(id, k, Value::Int(v), false); }
                if kind == Kind::Button { wb.set(id, "text", Value::Str(name), false); }
                id
            };
            dispatch(ctx.engine(), &w, &live, id, if kind == Kind::Button { "newButton" } else { "newField" }, vec![])?;
            Ok(Dynamic::UNIT)
        });
    }

    // convert <container> to seconds|dateItems|date|short date|long date|time   (UTC)
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("convert",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            let mut st = if syms.len() == 1 { ArithSt::default() } else { state.clone_cast::<ArithSt>() };
            let next: Option<ImmutableString> = match st.phase {
                0 => if look == "field" { st.field = true; st.phase = 1; Some("field".into()) } else { st.phase = 2; Some("$ident$".into()) },
                1 => { st.phase = 2; Some(slot(look)) }
                2 => { st.phase = 3; Some("to".into()) }
                3 => { st.phase = 4; Some("$ident$".into()) }
                4 => if look == "date" { st.phase = 5; Some("date".into()) } else { None },
                _ => None,
            };
            *state = Dynamic::from(st);
            Ok(next)
        }, true,
        move |ctx, inputs, state| {
            let st = state.clone_cast::<ArithSt>();
            let fmt = inputs[1].get_string_value().unwrap_or("").to_lowercase() + if st.phase == 5 { " date" } else { "" };
            let (old, field) = if st.field {
                let key = ctx.eval_expression_tree(&inputs[0])?;
                let id = resolve(&w, &live, "field", Some(&key))?;
                (w.borrow().get(id, "text").text(), Some(id))
            } else {
                let name = inputs[0].get_string_value().unwrap_or("it").to_string();
                (ctx.scope().get_value::<Dynamic>(&name).map(|d| d.to_string()).unwrap_or_default(), None)
            };
            let secs = parse_when(&old).ok_or_else(|| err(&format!("convert: \"{old}\" is not a date or time")))?;
            let new = format_when(secs, &fmt).ok_or_else(|| err(&format!("convert to {fmt}? use seconds, dateItems, date, short date, long date or time")))?;
            match field {
                Some(id) => w.borrow_mut().set(id, "text", Value::Str(new), false),
                None => { let name = inputs[0].get_string_value().unwrap_or("it").to_string(); ctx.scope_mut().set_value(name, Dynamic::from(new)); }
            }
            Ok(Dynamic::UNIT)
        });
    }

    // file <path>  -- the whole text of a file (empty, and `the result` set, if there is none)
    { let w = w.clone();
      e.register_custom_syntax_with_state_raw("file",
        |syms: &[ImmutableString], look: &str, _| Ok(if syms.len() == 1 { Some(slot(look)) } else { None }), false,
        move |ctx, inputs, _| {
            let p = { let wb = w.borrow(); wb.path(&ctx.eval_expression_tree(&inputs[0])?.to_string()) };
            let r = w.borrow().fs.borrow().read(&p);
            let mut wb = w.borrow_mut();
            match r { Ok(t) => { wb.result.clear(); Ok(Dynamic::from(t)) } Err(e) => { wb.result = e; Ok(Dynamic::from("")) } }
        });
    }

    // open file <path>  -- created if missing; close file <path>  -- writes what was written
    { let w = w.clone();
      e.register_custom_syntax_with_state_raw("open",
        |syms: &[ImmutableString], look: &str, _| Ok(match syms.len() { 1 => Some("file".into()), 2 => Some(slot(look)), _ => None }), false,
        move |ctx, inputs, _| {
            let p = { let wb = w.borrow(); wb.path(&ctx.eval_expression_tree(&inputs[0])?.to_string()) };
            let text = w.borrow().fs.borrow().read(&p).unwrap_or_default();
            let mut wb = w.borrow_mut();
            if !wb.fs.borrow().is_file(&p) { wb.fs.borrow_mut().write(&p, "").map_err(|e| err(&e))?; }
            Rc::make_mut(&mut wb.open_files).insert(p, crate::model::OpenFile { text, read_pos: 0, written: false });
            wb.result.clear();
            Ok(Dynamic::UNIT)
        });
    }
    { let w = w.clone();
      e.register_custom_syntax_with_state_raw("close",
        |syms: &[ImmutableString], look: &str, _| Ok(match syms.len() { 1 => Some("file".into()), 2 => Some(slot(look)), _ => None }), false,
        move |ctx, inputs, _| {
            let p = { let wb = w.borrow(); wb.path(&ctx.eval_expression_tree(&inputs[0])?.to_string()) };
            let f = Rc::make_mut(&mut w.borrow_mut().open_files).remove(&p).ok_or_else(|| err(&format!("file \"{p}\" is not open")))?;
            if f.written { let fs = w.borrow().fs.clone(); fs.borrow_mut().write(&p, &f.text).map_err(|e| err(&e))?; }
            Ok(Dynamic::UNIT)
        });
    }

    // read from file <path> until <char or eof> | for <n>  -- into `it`; the result is "eof" at the end
    { let w = w.clone();
      e.register_custom_syntax_with_state_raw("read",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            Ok(match syms.len() {
                1 => Some("from".into()),
                2 => Some("file".into()),
                3 => Some(slot(look)),
                4 => { *state = Dynamic::from(look == "for"); Some(if look == "for" { "for".into() } else { "until".into() }) }
                5 => Some("$expr$".into()),
                _ => None,
            })
        }, false,
        move |ctx, inputs, state| {
            let p = { let wb = w.borrow(); wb.path(&ctx.eval_expression_tree(&inputs[0])?.to_string()) };
            let arg = ctx.eval_expression_tree(&inputs[1])?;
            let by_count = state.as_bool().unwrap_or(false);
            let (got, at_end) = {
                let mut wb = w.borrow_mut();
                let f = Rc::make_mut(&mut wb.open_files).get_mut(&p).ok_or_else(|| err(&format!("file \"{p}\" is not open; use open file first")))?;
                let chars: Vec<char> = f.text.chars().collect();
                let start = f.read_pos.min(chars.len());
                let end = if by_count { (start + index(&arg)?.max(0) as usize).min(chars.len()) } else {
                    let stop = arg.to_string();
                    if stop == "\u{4}" || stop.is_empty() { chars.len() }
                    else { let sc: Vec<char> = stop.chars().collect(); (start..chars.len()).find(|i| chars[*i..].starts_with(&sc)).map(|i| i + sc.len()).unwrap_or(chars.len()) }
                };
                f.read_pos = end;
                (chars[start..end].iter().collect::<String>(), end >= chars.len())
            };
            w.borrow_mut().result = if at_end { "eof".into() } else { String::new() };
            ctx.scope_mut().set_value("it", got);
            Ok(Dynamic::UNIT)
        });
    }

    // write <text> to file <path> [at eof|end|<n>]  -- the first write after open replaces the file, later ones append
    { let w = w.clone();
      e.register_custom_syntax_with_state_raw("write",
        |syms: &[ImmutableString], look: &str, _| {
            Ok(match syms.len() {
                1 => Some("$expr$".into()),
                2 => Some("to".into()),
                3 => Some("file".into()),
                4 => Some(slot(look)),
                5 => if look == "at" { Some("at".into()) } else { None },
                6 => Some("$expr$".into()),
                _ => None,
            })
        }, false,
        move |ctx, inputs, _| {
            let text = ctx.eval_expression_tree(&inputs[0])?.to_string();
            let p = { let wb = w.borrow(); wb.path(&ctx.eval_expression_tree(&inputs[1])?.to_string()) };
            let at = if inputs.len() > 2 { Some(ctx.eval_expression_tree(&inputs[2])?) } else { None };
            let mut wb = w.borrow_mut();
            let f = Rc::make_mut(&mut wb.open_files).get_mut(&p).ok_or_else(|| err(&format!("file \"{p}\" is not open; use open file first")))?;
            match at {
                None => { if !f.written { f.text.clear(); } f.text.push_str(&text); }
                Some(a) if a.to_string() == "\u{4}" || a.to_string() == "end" => f.text.push_str(&text),
                Some(a) => {
                    let n = (index(&a)?.max(1) - 1) as usize;
                    let mut chars: Vec<char> = f.text.chars().collect();
                    while chars.len() < n { chars.push(' '); }
                    let new: Vec<char> = text.chars().collect();
                    for (k, c) in new.iter().enumerate() { if n + k < chars.len() { chars[n + k] = *c } else { chars.push(*c) } }
                    f.text = chars.into_iter().collect();
                }
            }
            f.written = true;
            Ok(Dynamic::UNIT)
        });
    }

    // delete file|folder <path> | delete <chunk> <n> of <variable | field <name>>  -- removes the chunk and its delimiter
    { let (w, live) = (w.clone(), live.clone());
      #[derive(Clone, Default)]
      struct DelSt { what: String, field: bool, var: bool, by_id: bool, phase: u8 }
      e.register_custom_syntax_with_state_raw("delete",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            let mut st = if syms.len() == 1 { DelSt::default() } else { state.clone_cast::<DelSt>() };
            let next: Option<ImmutableString> = match st.phase {
                0 if look == "this" => { st.what = "card".into(); st.phase = 30; Some("this".into()) }
                30 => { st.phase = 10; Some("card".into()) }
                0 if look == "card" => { st.what = "card".into(); st.phase = 31; Some("card".into()) }
                // `delete card` alone is this card; `delete card <name, number or variable>`, `delete card id <n>`.
                31 if look == "id" => { st.by_id = true; st.phase = 9; Some("id".into()) }
                31 => if look.starts_with(['"', '(']) || look.chars().next().is_some_and(|c| c.is_alphanumeric() || c == '_') { st.phase = 10; Some(slot(look)) } else { None },
                0 => { st.what = look.into(); st.phase = if matches!(look, "file" | "folder") { 9 } else { 1 }; Some(look.into()) }
                9 => { st.phase = 10; Some(slot(look)) }
                1 => { st.phase = 2; Some("$expr$".into()) }
                2 => { st.phase = 3; Some("of".into()) }
                3 => if look == "field" { st.field = true; st.phase = 4; Some("field".into()) } else { st.var = true; st.phase = 10; Some("$ident$".into()) },
                4 => { st.phase = 10; Some(slot(look)) }
                _ => None,
            };
            *state = Dynamic::from(st);
            Ok(next)
        }, true,
        move |ctx, inputs, state| {
            let st = state.clone_cast::<DelSt>();
            if st.what == "card" {
                // delete [this] card [<name>]: deleteCard goes to it, then the next card shows.
                let key = if inputs.is_empty() { None } else { Some(ctx.eval_expression_tree(&inputs[0])?) };
                let card = resolve(&w, &live, if st.by_id { "card#id" } else { "card" }, key.as_ref())?;
                let (n, here) = { let wb = w.borrow(); (wb.cards().len(), wb.current_card()) };
                if n <= 1 { return Err(err("can't delete the last card")); }
                dispatch(ctx.engine(), &w, &live, card, "deleteCard", vec![])?;
                if card == here {
                    let next = { let wb = w.borrow(); let cards = wb.cards(); let i = cards.iter().position(|c| *c == card).unwrap_or(0); cards[(i + 1) % cards.len()] };
                    navigate(ctx.engine(), &w, &live, next)?;
                }
                w.borrow_mut().delete(card);
                return Ok(Dynamic::UNIT);
            }
            if st.what == "file" || st.what == "folder" {
                let p = { let wb = w.borrow(); wb.path(&ctx.eval_expression_tree(&inputs[0])?.to_string()) };
                let fs = w.borrow().fs.clone();
                return fs.borrow_mut().delete(&p).map(|_| Dynamic::UNIT).map_err(|e| err(&e));
            }
            let kind = match st.what.as_str() { "character" => "char", k => k };
            if !CHUNKS.contains(&kind) { return Err(err(&format!("delete what? delete line 2 of x, or delete file \"…\", not {}", st.what))); }
            let n = index(&ctx.eval_expression_tree(&inputs[0])?)?;
            let (old, field) = if st.field {
                let key = ctx.eval_expression_tree(&inputs[1])?;
                let id = resolve(&w, &live, "field", Some(&key))?;
                (w.borrow().get(id, "text").text(), Some(id))
            } else {
                let name = inputs[1].get_string_value().unwrap_or("it").to_string();
                (ctx.scope().get_value::<Dynamic>(&name).map(|d| d.to_string()).unwrap_or_default(), None)
            };
            let new = delete_chunk(kind, n, &old);
            match field {
                Some(id) => w.borrow_mut().set(id, "text", Value::Str(new), false),
                None => { let name = inputs[1].get_string_value().unwrap_or("it").to_string(); ctx.scope_mut().set_value(name, Dynamic::from(new)); }
            }
            Ok(Dynamic::UNIT)
        });
    }

    // rename file <path> to <path>
    { let w = w.clone();
      e.register_custom_syntax_with_state_raw("rename",
        |syms: &[ImmutableString], look: &str, _| Ok(match syms.len() { 1 => Some(if look == "folder" { "folder".into() } else { "file".into() }), 2 => Some(slot(look)), 3 => Some("to".into()), 4 => Some("$expr$".into()), _ => None }), false,
        move |ctx, inputs, _| {
            let (a, b) = { let wb = w.borrow(); (wb.path(&ctx.eval_expression_tree(&inputs[0])?.to_string()), wb.path(&ctx.eval_expression_tree(&inputs[1])?.to_string())) };
            let fs = w.borrow().fs.clone();
            let r = fs.borrow_mut().rename(&a, &b);
            r.map(|_| Dynamic::UNIT).map_err(|e| err(&e))
        });
    }

    // load url <url>  -- fetched in the background; `urlLoaded <url>` comes back when it is there.
    // url <url>  -- what was loaded (empty, and the result "not loaded", before that)
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax_with_state_raw("load",
        |syms: &[ImmutableString], look: &str, _| Ok(match syms.len() { 1 => Some("url".into()), 2 => Some(slot(look)), _ => None }), false,
        move |ctx, inputs, _| {
            let u = ctx.eval_expression_tree(&inputs[0])?.to_string();
            let from = me(&w, &live);
            w.borrow_mut().url_requests.push((u, from));
            Ok(Dynamic::UNIT)
        });
    }
    { let w = w.clone();
      e.register_custom_syntax_with_state_raw("url",
        |syms: &[ImmutableString], look: &str, _| Ok(if syms.len() == 1 { Some(slot(look)) } else { None }), false,
        move |ctx, inputs, _| {
            let u = ctx.eval_expression_tree(&inputs[0])?.to_string();
            let mut wb = w.borrow_mut();
            match wb.url_cache.get(&u).cloned() { Some(t) => { wb.result.clear(); Ok(Dynamic::from(t)) } None => { wb.result = "not loaded".into(); Ok(Dynamic::from("")) } }
        });
    }

    // start using stack <name> | stop using stack <name>: library stacks in the message path, after this stack
    for (word, start) in [("start", true), ("stop", false)] {
        let (w, live) = (w.clone(), live.clone());
        e.register_custom_syntax_with_state_raw(word,
            |syms: &[ImmutableString], look: &str, _| Ok(match syms.len() { 1 => Some("using".into()), 2 => Some("stack".into()), 3 => Some(slot(look)), _ => None }), false,
            move |ctx, inputs, _| {
                let name = ctx.eval_expression_tree(&inputs[0])?.to_string();
                if start {
                    let found = w.borrow().find_stack(&name);
                    let Some(path) = found else { w.borrow_mut().result = "No such stack.".into(); return Ok(Dynamic::UNIT) };
                    let mut obj = load_stack_obj(&w, &path)?;
                    if w.borrow().libs.iter().any(|l| l.name.eq_ignore_ascii_case(&obj.name)) { return Ok(Dynamic::UNIT); }
                    let id = LIB0 + w.borrow().libs.len() as u32;
                    obj.id = id; obj.parent = None;
                    let src = obj.script.clone();
                    Rc::make_mut(&mut w.borrow_mut().libs).push(obj);
                    compile_into(ctx.engine(), &live, id, &src);
                    w.borrow_mut().result.clear();
                    dispatch(ctx.engine(), &w, &live, id, "libraryStack", vec![])?;
                } else {
                    let pos = w.borrow().libs.iter().position(|l| l.name.eq_ignore_ascii_case(name.trim().trim_end_matches(".stack")));
                    let Some(pos) = pos else { return Ok(Dynamic::UNIT) };
                    dispatch(ctx.engine(), &w, &live, LIB0 + pos as u32, "releaseStack", vec![])?;
                    Rc::make_mut(&mut w.borrow_mut().libs).remove(pos);
                    // Library ids are positions: renumber and recompile the ones after it.
                    let libs: Vec<(u32, String)> = { let mut wb = w.borrow_mut(); Rc::make_mut(&mut wb.libs).iter_mut().enumerate().map(|(i, l)| { l.id = LIB0 + i as u32; (l.id, l.script.clone()) }).collect() };
                    { let mut lb = live.borrow_mut(); lb.asts.retain(|k, _| *k < LIB0); lb.errors.retain(|k, _| *k < LIB0); }
                    for (id, src) in libs { compile_into(ctx.engine(), &live, id, &src); }
                }
                Ok(Dynamic::UNIT)
            });
    }

    // beep   (a command; sound is always allowed)
    { let (w, live) = (w.clone(), live.clone());
      e.register_custom_syntax(["beep"], false, move |ctx, _| {
        let from = me(&w, &live);
        if !dispatch(ctx.engine(), &w, &live, from, "beep", vec![])? { w.borrow_mut().log.push("BEEP".into()); }
        Ok(Dynamic::UNIT)
      }).unwrap();
    }

    // send <expr> [to <ref>] [in <n> seconds|ticks]  -- send a message now, or later (instead of `wait`)
    { let (w, live) = (w.clone(), live.clone());
      #[derive(Clone, Default)]
      struct SendSt { r: RefSt, to: bool, later: bool, phase: u8 }
      e.register_custom_syntax_with_state_raw("send",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            // A literal message name ends at the literal, so `send "x" in 2 seconds` is not Rhai's `in`.
            if syms.len() == 1 { *state = Dynamic::from(SendSt::default()); return Ok(Some(if look.starts_with('"') { "$string$".into() } else { "$expr$".into() })); }
            let mut st = state.clone_cast::<SendSt>();
            let next: Option<ImmutableString> = match st.phase {
                0 => match look { "to" => { st.to = true; st.phase = 1; Some("to".into()) } "in" => { st.later = true; st.phase = 2; Some("in".into()) } _ => None },
                1 => match st.r.step(look) { Some(x) => Some(x), None => if look == "in" { st.later = true; st.phase = 2; Some("in".into()) } else { None } },
                2 => { st.phase = 3; Some("$expr$".into()) }
                3 => { st.phase = 4; Some("$ident$".into()) }
                _ => None,
            };
            *state = Dynamic::from(st);
            Ok(next)
        }, false,
        move |ctx, inputs, state| {
            let st = state.clone_cast::<SendSt>();
            let name = ctx.eval_expression_tree(&inputs[0])?.to_string();
            let mut i = 1;
            let start = if st.to {
                let key = if st.r.has_key { i += 1; Some(ctx.eval_expression_tree(&inputs[1])?) } else { None };
                resolve(&w, &live, &st.r.k(), key.as_ref())?
            } else { me(&w, &live) };
            if st.later {
                let n = num(&ctx.eval_expression_tree(&inputs[i])?).ok_or_else(|| err("send … in needs a number"))?;
                let unit = inputs[i + 1].get_string_value().unwrap_or("seconds").to_lowercase();
                let ms = duration_ms(n, &unit).ok_or_else(|| err(&format!("send … in {n} {unit}: use seconds or ticks")))?;
                w.borrow_mut().timers.push((now_ms() + ms, start, name));
                return Ok(Dynamic::UNIT);
            }
            dispatch(ctx.engine(), &w, &live, start, &name, vec![])?;
            Ok(Dynamic::UNIT)
        });
    }

    // Break / Continue (from `break` / `continue`, see hypertalk_words), exit repeat, next repeat, exit <handler>.
    // They raise the same signals as Rhai's own break, continue and return, so they work in Rhai loops too.
    e.register_custom_syntax(["Break"], false, |_, _| Err(Box::new(EvalAltResult::LoopBreak(true, Dynamic::UNIT, Position::NONE)))).unwrap();
    e.register_custom_syntax(["Continue"], false, |_, _| Err(Box::new(EvalAltResult::LoopBreak(false, Dynamic::UNIT, Position::NONE)))).unwrap();
    e.register_custom_syntax(["next", "repeat"], false, |_, _| Err(Box::new(EvalAltResult::LoopBreak(false, Dynamic::UNIT, Position::NONE)))).unwrap();
    // exit repeat | exit <handler> | exit to HyperCard (ends every handler of the run)
    e.register_custom_syntax_with_state_raw("exit",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| Ok(match syms.len() {
            1 => { *state = Dynamic::from(look.to_string()); Some(match look { "repeat" => "repeat".into(), "to" => "to".into(), _ => "$ident$".into() }) }
            2 if syms[1] == "to" => Some("$ident$".into()),
            _ => None,
        }),
        false,
        |_, inputs, state| match state.to_string().as_str() {
            "repeat" => Err(Box::new(EvalAltResult::LoopBreak(true, Dynamic::UNIT, Position::NONE))),
            "to" if inputs.first().and_then(|i| i.get_string_value()).is_some_and(|t| t.eq_ignore_ascii_case("hypercard")) => Err(halt(Halt::ExitAll)),
            "to" => Err(err("exit to what? write exit to HyperCard")),
            _ => Err(Box::new(EvalAltResult::Return(Dynamic::UNIT, Position::NONE))),
        });

    // repeat [forever] { } | repeat <n> [times] { } | repeat while|until <cond> { } | repeat with <v> = <a> [down] to <b> { }
    // `break` and `continue` (exit repeat / next repeat) work inside.
    e.register_custom_syntax_with_state_raw("repeat",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            let last = syms.last().map(|s| s.as_str()).unwrap_or("");
            if last == "$block$" { return Ok(None); }
            if syms.len() == 1 {
                let mode = match look { "forever" | "while" | "until" | "with" => look, "{" => "forever0", _ => "count" };
                *state = Dynamic::from(mode.to_string());
                return Ok(Some(match mode { "count" => "$expr$".into(), "forever0" => "$block$".into(), m => m.into() }));
            }
            let mode = state.to_string();
            Ok(Some(match (mode.as_str(), last) {
                ("forever", _) => "$block$".into(),
                ("count", "$expr$") => if look == "times" { "times".into() } else { "$block$".into() },
                ("count", "times") => "$block$".into(),
                ("while" | "until", "while" | "until") => "$expr$".into(),
                ("while" | "until", _) => "$block$".into(),
                ("with", "with") => "$ident$".into(),
                ("with", "=") => "$expr$".into(),
                ("with", "down") => "to".into(),
                ("with", "to") => "$expr$".into(),
                ("with", "$expr$") => if syms.iter().any(|s| s == "to") { "$block$".into() } else if look == "down" { "down".into() } else { "to".into() },
                ("with", _) => "=".into(),
                _ => return Ok(None),
            }))
        }, true,
        |ctx, inputs, state| {
            use rhai::EvalAltResult as E;
            // Run the body once; Ok(true) means stop the loop.
            fn body(ctx: &mut rhai::EvalContext, b: &rhai::Expression) -> R<bool> {
                match ctx.eval_expression_tree(b) {
                    Ok(_) => Ok(false),
                    Err(e) => match *e { E::LoopBreak(true, ..) => Ok(true), E::LoopBreak(false, ..) => Ok(false), _ => Err(e) },
                }
            }
            let mode = state.to_string();
            let block = &inputs[inputs.len() - 1];
            match mode.as_str() {
                "forever" | "forever0" => loop { if body(ctx, block)? { break } },
                "count" => {
                    let n = index(&ctx.eval_expression_tree(&inputs[0])?)?;
                    for _ in 0..n { if body(ctx, block)? { break } }
                }
                "while" | "until" => loop {
                    let c = ctx.eval_expression_tree(&inputs[0])?;
                    let c = c.as_bool().map_err(|_| err("repeat while/until needs true or false"))?;
                    if c != (mode == "while") { break }
                    if body(ctx, block)? { break }
                },
                _ => {
                    let var = inputs[0].get_string_value().unwrap_or("i").to_string();
                    let a = index(&ctx.eval_expression_tree(&inputs[1])?)?;
                    let b = index(&ctx.eval_expression_tree(&inputs[2])?)?;
                    let step: i64 = if b < a { -1 } else { 1 };
                    let mut i = a;
                    loop {
                        if (step > 0 && i > b) || (step < 0 && i < b) { break }
                        ctx.scope_mut().set_value(var.clone(), i);
                        if body(ctx, block)? { break }
                        i += step;
                    }
                }
            }
            Ok(Dynamic::UNIT)
        });
    e
}

impl Rt {
    pub fn new(file: StackFile) -> Rt { Rt::with_home(file, None) }

    /// Open a stack with a Home stack object behind it on the message path.
    pub fn with_home(file: StackFile, home: Option<Obj>) -> Rt {
        let mut world = World::new(file);
        world.home = home.map(|mut h| { h.id = HOME; h.parent = None; h.kind = Kind::Stack; h });
        if let Some(l) = world.home.as_ref().and_then(|h| h.props.get("userLevel")).map(|v| v.int()).filter(|l| (1..=5).contains(l)) { world.user_level = l; }
        let w: W = Rc::new(RefCell::new(world));
        let live: L = Rc::new(RefCell::new(Live::default()));
        let engine = build_engine(&w, &live);
        let rt = Rt { engine, w, live, paused: RefCell::new(None), later: RefCell::new(Default::default()) };
        let mut ids: Vec<u32> = rt.w.borrow().file.objs.iter().map(|o| o.id).collect();
        if rt.w.borrow().home.is_some() { ids.push(HOME); }
        for id in ids { rt.compile(id); }
        rt
    }

    /// Compile an object's script. On error the previous working version stays live.
    pub fn compile(&self, id: u32) {
        let src = self.w.borrow().obj(id).map(|o| o.script.clone()).unwrap_or_default();
        compile_into(&self.engine, &self.live, id, &src);
    }

    /// Run one animation frame: send `frame` to the current card if it has a canvas part.
    /// Returns false when there is nothing to animate.
    pub fn frame(&self) -> bool {
        if self.is_paused() { return false; }
        let card = self.w.borrow().current_card();
        let has_canvas = { let w = self.w.borrow(); w.parts_on(card).iter().any(|id| w.obj(*id).map(|o| o.kind == Kind::Canvas).unwrap_or(false)) };
        if !has_canvas { return false; }
        self.w.borrow_mut().auto_depth += 1;
        let r = dispatch(&self.engine, &self.w, &self.live, card, "frame", vec![]);
        self.w.borrow_mut().auto_depth -= 1;
        match r { Err(e) if is_exit_all(&e) => self.live.borrow_mut().frames.clear(), Err(e) => self.fail(card, e), Ok(_) => {} }
        true
    }

    pub fn script_error(&self, id: u32) -> Option<String> { self.live.borrow().errors.get(&id).cloned() }

    /// Live edit: journal the new text and recompile at once.
    pub fn set_script(&self, id: u32, text: &str) {
        { let mut wb = self.w.borrow_mut(); wb.begin(); wb.set(id, "script", Value::Str(text.into()), true); }
        self.compile(id);
    }

    /// Send a message with parameters (keyDown "a", arrowKey "left", …), as an automatic change.
    pub fn send_with(&self, obj: u32, msg: &str, args: Vec<Value>) -> bool {
        self.run(Entry::Send { obj, msgs: vec![(msg.to_string(), args.into_iter().map(to_dyn).collect())], auto: true })
    }

    /// Send a message into the path, starting at `obj`. Errors land in `world.last_error`.
    pub fn send(&self, obj: u32, msg: &str) -> bool {
        self.run(Entry::Send { obj, msgs: vec![(msg.to_string(), vec![])], auto: false })
    }

    /// Run statements as the message box does: in the context of the current card.
    pub fn exec(&self, text: &str) { self.run(Entry::Exec(text.to_string())); }

    /// Opening a stack: openStack, openBackground, openCard to the current card (and startup first, if asked).
    pub fn open_stack(&self, startup: bool) {
        let card = self.w.borrow().current_card();
        let msgs = ["startup", "openStack", "openBackground", "openCard"].into_iter().filter(|m| startup || *m != "startup").map(|m| (m.to_string(), vec![])).collect();
        self.run(Entry::Send { obj: card, msgs, auto: true });
    }

    /// A click on a button: a button in a family (radio group) lights up and the others in its family go out;
    /// then mouseUp is sent.
    pub fn click(&self, id: u32) -> bool {
        if self.is_paused() { return false; }
        // A checkBox flips its hilite when clicked, a radioButton turns it on (both only with autoHilite, as in HyperCard).
        {
            let style = self.w.borrow().get(id, "style").text().to_lowercase();
            let auto = self.w.borrow().get(id, "autoHilite").truthy();
            if auto && (style == "checkbox" || style == "radiobutton") {
                let mut wb = self.w.borrow_mut();
                wb.begin();
                let on = if style == "checkbox" { !wb.get(id, "hilite").truthy() } else { true };
                wb.set(id, "hilite", Value::Bool(on), false);
            }
        }
        let family = self.w.borrow().get(id, "family").int();
        if family > 0 {
            let mut wb = self.w.borrow_mut();
            wb.begin();
            let card = wb.current_card();
            let members: Vec<u32> = wb.parts_on(card).into_iter().filter(|p| wb.obj(*p).map(|o| o.kind == Kind::Button).unwrap_or(false) && wb.get(*p, "family").int() == family).collect();
            for m in members { wb.set(m, "hilite", Value::Bool(m == id), false); }
        }
        self.send(id, "mouseUp")
    }

    /// The user closed a dialog: the paused script goes on, with the button or text in `it`.
    pub fn reply(&self, text: &str) {
        if self.waiting() == Some(Waiting::Dialog) { self.resume(Some(Dynamic::from(text.to_string()))); }
    }

    /// The user cancelled a dialog: `it` is empty and the result is "Cancel".
    pub fn cancel(&self) {
        if self.waiting() == Some(Waiting::Dialog) { self.resume(Some(Dynamic::UNIT)); }
    }

    /// `idle`: the player calls this a few times a second in browse mode. The current card gets `idle`
    /// when a script on its path handles it (so stacks without one cost nothing).
    pub fn idle(&self) -> bool {
        let card = self.w.borrow().current_card();
        !self.is_paused() && self.handles(card, "idle") && self.send_with(card, "idle", vec![])
    }

    /// Whether a handler for `msg` sits on the message path from `obj` (so the player sends messages that
    /// come every frame, such as mouseWithin, only to stacks that handle them).
    pub fn handles(&self, obj: u32, msg: &str) -> bool {
        let msg = msg.to_lowercase();
        let (wb, lb) = (self.w.borrow(), self.live.borrow());
        std::iter::successors(Some(obj), |id| wb.next_on_path(*id)).take(16).any(|id| lb.asts.get(&id).is_some_and(|a| handles(a, &msg)))
    }

    /// A click in a locked field at a character offset: for clickText / clickLine / clickChunk, and with
    /// autoSelect (a list field) the clicked line is selected. Then mouseUp goes to the field.
    pub fn click_field(&self, f: u32, at: Option<usize>) -> bool {
        if self.is_paused() { return false; }
        {
            let mut wb = self.w.borrow_mut();
            wb.mouse_click = true;
            if let Some(at) = at {
                wb.click_text_at = Some((f, at));
                if wb.get(f, "autoSelect").truthy() {
                    let text = wb.get(f, "text").text();
                    let chars: Vec<char> = text.chars().collect();
                    let at = at.min(chars.len());
                    let a = chars[..at].iter().rposition(|c| *c == '\n').map(|i| i + 1).unwrap_or(0);
                    let b = chars[at..].iter().position(|c| *c == '\n').map(|i| at + i).unwrap_or(chars.len());
                    if a < b { wb.selection = Some((f, a, b)); wb.select_request = Some((f, a, b)); }
                }
            }
        }
        self.send(f, "mouseUp")
    }

    /// The user chose a menu item in the player's menu bar.
    pub fn do_menu(&self, item: &str, menu: &str) { self.exec(&format!("doMenu {}, {};", hypertalk_literal(item), hypertalk_literal(menu))); }

    /// The user painted a stroke with the current paint tool (points in card coordinates).
    pub fn paint(&self, pts: &[(f64, f64)]) {
        if self.is_paused() || pts.is_empty() { return; }
        let mut wb = self.w.borrow_mut();
        if !wb.tool.strokes() || wb.user_level < wb.tool.user_level() { return; }
        wb.begin();
        let card = wb.current_card();
        add_stroke(&mut wb, card, pts);
    }

    /// Whether a script is paused (the stack is modal until it finishes).
    pub fn is_paused(&self) -> bool { self.paused.borrow().is_some() }

    /// What the paused script waits for, if one is paused.
    pub fn waiting(&self) -> Option<Waiting> { if self.is_paused() { self.live.borrow().waiting.clone() } else { None } }

    /// Called by the player every frame: ends a `wait` whose time has come, and checks `wait until`
    /// conditions again. True if a script ran.
    pub fn tick(&self, now_ms: f64) -> bool {
        match self.waiting() {
            Some(Waiting::Until(t)) if now_ms >= t => { self.resume(Some(Dynamic::from(String::new()))); true }
            Some(Waiting::Poll(t)) if now_ms >= t => { self.resume(None); true }
            _ => false,
        }
    }

    /// The player fetched a `load url` request: cache it and tell the object that asked (`urlLoaded <url>`).
    pub fn url_loaded(&self, url: &str, text: &str, to: u32) {
        let (u, t) = (url.to_string(), text.to_string());
        if self.defer(move |rt| rt.url_loaded(&u, &t, to)) { return; }
        self.w.borrow_mut().url_cache.insert(url.to_string(), text.to_string());
        if self.w.borrow().obj(to).is_some() { self.send_with(to, "urlLoaded", vec![Value::Str(url.to_string())]); }
    }

    /// A `load url` failed: `urlLoaded <url>, "error"`, with the reason in `the result`.
    pub fn url_failed(&self, url: &str, why: &str, to: u32) {
        let (u, y) = (url.to_string(), why.to_string());
        if self.defer(move |rt| rt.url_failed(&u, &y, to)) { return; }
        { let mut wb = self.w.borrow_mut(); wb.url_cache.remove(url); wb.result = why.to_string(); }
        if self.w.borrow().obj(to).is_some() {
            self.send_with(to, "urlLoaded", vec![Value::Str(url.to_string()), Value::Str("error".into())]);
        }
    }

    /// Send the `send … in` messages that are due at `now_ms` (ms since 1970). Returns how many were sent.
    /// Nothing is sent while a script is paused.
    pub fn run_timers(&self, now_ms: f64) -> usize {
        if self.is_paused() { return 0; }
        let mut due: std::collections::VecDeque<(f64, u32, String)> = {
            let mut wb = self.w.borrow_mut();
            let (due, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut wb.timers).into_iter().partition(|t| t.0 <= now_ms);
            wb.timers = rest;
            due.into()
        };
        let mut n = 0;
        while let Some((_, obj, msg)) = due.pop_front() {
            if self.is_paused() { self.w.borrow_mut().timers.extend(std::iter::once((0.0, obj, msg)).chain(due.drain(..))); break; }
            if self.w.borrow().obj(obj).is_some() { self.send_with(obj, &msg, vec![]); }
            n += 1;
        }
        n
    }

    /// Start a run that may pause; refused while another is paused (the stack is modal).
    fn run(&self, entry: Entry) -> bool {
        if self.is_paused() { return false; }
        self.start(entry, vec![], None)
    }

    fn start(&self, entry: Entry, tape: Vec<(String, Dynamic)>, resume: Option<Dynamic>) -> bool {
        let snap = self.snapshot();
        { let mut wb = self.w.borrow_mut(); wb.begin(); wb.last_error = None; wb.journal_floor = snap.mark; wb.floor_merges.clear(); wb.visual = None; }
        let tape_rc = self.live.borrow().tape.clone();
        *tape_rc.borrow_mut() = Tape { on: true, replaying: !tape.is_empty(), entries: tape, resume, ..Default::default() };
        let real_fs = self.w.borrow().fs.clone();
        self.w.borrow_mut().fs = Rc::new(RefCell::new(Box::new(TapeFs { inner: real_fs.clone(), tape: tape_rc.clone() })));
        let r = self.exec_entry(&entry);
        { let mut wb = self.w.borrow_mut(); wb.fs = real_fs; wb.journal_floor = 0; wb.locked = false; wb.lock_messages = false; }
        let (entries, diverged) = { let mut t = tape_rc.borrow_mut(); t.on = false; (std::mem::take(&mut t.entries), t.diverged.take()) };
        // A run that went a different way the second time may end before its pause point.
        settle(&self.w, &self.live);
        if let Some(d) = diverged { self.w.borrow_mut().log.push(format!("note: a paused script ran differently when it went on ({d})")); }
        match r {
            Err(e) if is_pause(&e) => {
                self.live.borrow_mut().frames.clear();
                *self.paused.borrow_mut() = Some(Paused { entry, snap, tape: entries });
                true
            }
            r => {
                self.live.borrow_mut().waiting = None;
                let h = match r { Ok(h) => h, Err(e) if is_exit_all(&e) => { self.live.borrow_mut().frames.clear(); true } Err(e) => { let obj = match &entry { Entry::Send { obj, .. } => *obj, Entry::Exec(_) => self.w.borrow().current_card() }; self.fail(obj, e); false } };
                self.run_later();
                h
            }
        }
    }

    fn exec_entry(&self, entry: &Entry) -> R<bool> {
        match entry {
            Entry::Send { obj, msgs, auto } => {
                if *obj != HOME && *obj < LIB0 && self.w.borrow().obj(*obj).is_none() { return Ok(false); }
                if *auto { self.w.borrow_mut().auto_depth += 1; }
                let mut r = Ok(false);
                for (m, a) in msgs {
                    match dispatch(&self.engine, &self.w, &self.live, *obj, m, a.clone()) {
                        Ok(h) => r = Ok(r.unwrap_or(false) || h),
                        Err(e) => { r = Err(e); break; }
                    }
                }
                if *auto { self.w.borrow_mut().auto_depth -= 1; }
                r
            }
            Entry::Exec(text) => {
                let card = self.w.borrow().current_card();
                let ast = match self.engine.compile(&hypertalk_words(&hypertalk_blocks(text))) { Ok(a) => a, Err(e) => { self.w.borrow_mut().last_error = Some(format!("message box: {e}")); return Ok(false); } };
                self.live.borrow_mut().frames.push(Frame { obj: card, target: card, msg: "\u{0}".into(), args: vec![], handled: false, passed: false, globals: vec![], ret: None });
                let res = self.engine.eval_ast_with_scope::<Dynamic>(&mut Scope::new(), &ast);
                self.live.borrow_mut().frames.pop();
                let v = res?;
                if !v.is_unit() { self.w.borrow_mut().log.push(v.to_string()); }
                Ok(true)
            }
        }
    }

    /// Go on with the paused script: roll back what it did, run it again with what it saw on the tape,
    /// and give the pause point `reply`.
    fn resume(&self, reply: Option<Dynamic>) {
        let Some(p) = self.paused.borrow_mut().take() else { return };
        {
            let wb = self.w.borrow();
            self.live.borrow_mut().settle = Some(Settle { sounds: wb.sound_cmds.len(), urls: wb.url_requests.len(), go: wb.go_stack.clone(), edit: wb.edit_request, requests: wb.player_requests.len() });
        }
        self.live.borrow_mut().waiting = None;
        self.restore(p.snap);
        self.start(p.entry, p.tape, reply);
    }

    fn snapshot(&self) -> Snap { take_snap(&self.w.borrow(), &self.live.borrow()) }

    fn restore(&self, s: Snap) {
        let ids = {
            let (mut wb, mut lb) = (self.w.borrow_mut(), self.live.borrow_mut());
            let visited = s.visited.len() as u32;
            let mark = put_snap(&mut wb, &mut lb, s);
            wb.dialog = None; wb.dialog_buttons.clear(); wb.dialog_ask = None; wb.dialog_file = None;
            // Other stacks loaded after the snapshot are gone again; the scripts of those before it stay compiled.
            lb.asts.retain(|k, _| *k < VISIT0 + visited);
            lb.errors.retain(|k, _| *k < VISIT0 + visited);
            drop(lb);
            let mut ids = wb.rollback(mark);
            ids.sort_unstable();
            ids.dedup();
            ids
        };
        for id in ids { self.compile(id); }
    }

    /// Work that waited for a paused script.
    fn run_later(&self) {
        while !self.is_paused() {
            let Some(f) = self.later.borrow_mut().pop_front() else { break };
            f(self);
        }
    }

    /// While a script is paused (the stack is modal), keep `f` for when it finishes; true if kept.
    fn defer(&self, f: impl FnOnce(&Rt) + 'static) -> bool {
        if !self.is_paused() { return false; }
        self.later.borrow_mut().push_back(Box::new(f));
        true
    }

    fn fail(&self, obj: u32, e: Box<EvalAltResult>) {
        self.live.borrow_mut().frames.clear();
        // errorDialog: a handler on the path can take the error text instead of the error display.
        let text = e.to_string();
        let card = self.w.borrow().current_card();
        if !text.contains("errorDialog") {
            if let Ok(true) = dispatch(&self.engine, &self.w, &self.live, card, "errorDialog", vec![Dynamic::from(text)]) {
                self.live.borrow_mut().frames.clear();
                return;
            }
            self.live.borrow_mut().frames.clear();
        }
        let mut wb = self.w.borrow_mut();
        let who = wb.obj(obj).map(|o| format!("{:?} \"{}\"", o.kind, o.name).to_lowercase()).unwrap_or_default();
        wb.last_error = Some(format!("{who}: {e}"));
    }

    pub fn undo(&self) {
        if self.is_paused() { return; }
        let ids = self.w.borrow_mut().undo();
        for id in ids { self.compile(id); }
    }

    pub fn text(&self) -> String { self.w.borrow().file.to_text() }

    /// The stack as its zip container, ready to write to disk.
    pub fn bytes(&self) -> Vec<u8> { self.w.borrow().file.to_zip() }

    /// The player reports that the current sound has ended: clear `the sound`, send `soundDone`.
    pub fn sound_finished(&self) {
        if self.w.borrow_mut().sound.take().is_none() { return; }
        if self.defer(|rt| { let card = rt.w.borrow().current_card(); rt.send_with(card, "soundDone", vec![]); }) { return; }
        let card = self.w.borrow().current_card();
        self.send_with(card, "soundDone", vec![]);
    }

    /// The Home stack as its own file, or None when there is no Home.
    pub fn home_text(&self) -> Option<String> {
        let home = self.w.borrow().home.clone()?;
        let mut f = StackFile::new("Home", 0, 0);
        f.objs.push(home);
        Some(f.to_text())
    }
}

/// An object's script may contain only `on` handlers (and `fn` definitions, which Rhai keeps separately).
/// A loose statement would otherwise run on every message that reaches the object.
fn only_handlers(ast: &AST) -> Result<(), String> {
    use rhai::{Expr, Stmt};
    for st in ast.statements() {
        let ok = match st {
            Stmt::Expr(e) => matches!(e.as_ref(), Expr::Custom(c, _) if matches!(c.tokens.first().map(|t| t.as_str()), Some("on") | Some("function"))),
            Stmt::Noop(_) => true,
            _ => false,
        };
        if !ok {
            let p = st.position();
            return Err(format!("only `on <message>` and `function <name>` handlers may appear at the top of a script (line {})", p.line().unwrap_or(0)));
        }
    }
    Ok(())
}

/// HyperTalk arithmetic and joining. `+ - * / %` work on numbers, and on text that reads as a
/// number (`"41" + 1` is 42); other text is an error. `&` joins any two values as text.
/// Comparisons treat number-like text as numbers.
fn register_coercion(e: &mut Engine) {
    use rhai::FLOAT;
    #[derive(Clone, Copy)]
    enum N { I(i64), F(FLOAT) }
    fn parse(s: &str) -> Option<N> {
        let t = s.trim();
        if t.is_empty() { return None; }
        t.parse::<i64>().map(N::I).ok().or_else(|| t.parse::<FLOAT>().ok().filter(|f| f.is_finite()).map(N::F))
    }
    fn fl(n: N) -> FLOAT { match n { N::I(i) => i as FLOAT, N::F(f) => f } }
    fn arith(op: &str, a: N, b: N) -> R<Dynamic> {
        Ok(match (a, b) {
            (N::I(x), N::I(y)) if op != "/" || (y != 0 && x % y == 0) => match op {
                "+" => x.checked_add(y), "-" => x.checked_sub(y), "*" => x.checked_mul(y), "/" => x.checked_div(y), _ => x.checked_rem(y),
            }.map(Dynamic::from).ok_or_else(|| err("number too large or division by zero"))?,
            _ => { let (x, y) = (fl(a), fl(b)); Dynamic::from(match op { "+" => x + y, "-" => x - y, "*" => x * y, "/" => x / y, _ => x % y }) }
        })
    }
    fn need(s: &str, op: &str) -> R<N> {
        parse(s).ok_or_else(|| err(&if op == "+" { format!("expected a number, not \"{s}\" (join text with &)") } else { format!("expected a number, not \"{s}\"") }))
    }
    fn cmp(op: &str, a: FLOAT, b: FLOAT) -> bool { match op { "==" => a == b, "!=" => a != b, "<" => a < b, ">" => a > b, "<=" => a <= b, _ => a >= b } }
    e.set_fast_operators(false);
    for op in ["+", "-", "*", "/", "%"] {
        e.register_fn(op, move |a: ImmutableString, b: i64| -> R<Dynamic> { arith(op, need(&a, op)?, N::I(b)) });
        e.register_fn(op, move |a: i64, b: ImmutableString| -> R<Dynamic> { arith(op, N::I(a), need(&b, op)?) });
        e.register_fn(op, move |a: ImmutableString, b: FLOAT| -> R<Dynamic> { arith(op, need(&a, op)?, N::F(b)) });
        e.register_fn(op, move |a: FLOAT, b: ImmutableString| -> R<Dynamic> { arith(op, N::F(a), need(&b, op)?) });
        e.register_fn(op, move |a: ImmutableString, b: ImmutableString| -> R<Dynamic> { arith(op, need(&a, op)?, need(&b, op)?) });
    }
    // `&` and `&&` arrive as Cat / CatSp (see hypertalk_words), at HyperTalk's precedence: below + and -,
    // above the comparisons. `&&` joins with a space.
    e.register_custom_operator("Cat", 145).unwrap();
    e.register_custom_operator("CatSp", 145).unwrap();
    e.register_fn("Cat", |a: Dynamic, b: Dynamic| Dynamic::from(format!("{a}{b}")));
    e.register_fn("CatSp", |a: Dynamic, b: Dynamic| Dynamic::from(format!("{a} {b}")));
    // `&` always joins (kept for code that reaches the engine without the rewrite).
    fn join(a: Dynamic, b: Dynamic) -> Dynamic { Dynamic::from(format!("{a}{b}")) }
    e.register_fn("&", join);
    e.register_fn("&", |a: i64, b: i64| join(a.into(), b.into()));
    e.register_fn("&", |a: bool, b: bool| join(a.into(), b.into()));
    e.register_fn("&", |a: ImmutableString, b: ImmutableString| join(a.into(), b.into()));
    e.register_fn("&", |a: ImmutableString, b: i64| join(a.into(), b.into()));
    e.register_fn("&", |a: i64, b: ImmutableString| join(a.into(), b.into()));
    e.register_fn("&", |a: ImmutableString, b: FLOAT| join(a.into(), b.into()));
    e.register_fn("&", |a: FLOAT, b: ImmutableString| join(a.into(), b.into()));
    e.register_fn("&", |a: FLOAT, b: FLOAT| join(a.into(), b.into()));
    e.register_fn("&", |a: i64, b: FLOAT| join(a.into(), b.into()));
    e.register_fn("&", |a: FLOAT, b: i64| join(a.into(), b.into()));
    for op in ["==", "!=", "<", ">", "<=", ">="] {
        e.register_fn(op, move |a: ImmutableString, b: i64| parse(&a).map(|x| cmp(op, fl(x), b as FLOAT)).unwrap_or(op == "!="));
        e.register_fn(op, move |a: i64, b: ImmutableString| parse(&b).map(|y| cmp(op, a as FLOAT, fl(y))).unwrap_or(op == "!="));
        e.register_fn(op, move |a: ImmutableString, b: FLOAT| parse(&a).map(|x| cmp(op, fl(x), b)).unwrap_or(op == "!="));
        e.register_fn(op, move |a: FLOAT, b: ImmutableString| parse(&b).map(|y| cmp(op, a, fl(y))).unwrap_or(op == "!="));
        // Two texts compare as numbers when both are numbers, otherwise as text.
        e.register_fn(op, move |a: ImmutableString, b: ImmutableString| match (parse(&a), parse(&b)) {
            (Some(x), Some(y)) => cmp(op, fl(x), fl(y)),
            _ => { let o = a.as_str().cmp(b.as_str()); match op { "==" => o.is_eq(), "!=" => o.is_ne(), "<" => o.is_lt(), ">" => o.is_gt(), "<=" => o.is_le(), _ => o.is_ge() } }
        });
    }
}

/// For each `on` handler, the variable names it sets with `put … into <name>`.
fn put_targets(ast: &AST) -> HashMap<String, Vec<String>> {
    use rhai::{ASTNode, Expr};
    let mut out: HashMap<String, Vec<String>> = HashMap::new();
    ast.walk(&mut |path: &[ASTNode]| {
        if let Some(ASTNode::Expr(Expr::Custom(c, _))) = path.last() {
            if c.tokens.first().map(|t| t.as_str()) == Some("put") {
                let st = c.state.clone().try_cast::<PutSt>().unwrap_or_default();
                if !st.field {
                    if let Some(Expr::Variable(v, ..)) = c.inputs.last() {
                        let handler = path.iter().rev().find_map(|n| match n {
                            ASTNode::Expr(Expr::Custom(h, _)) if matches!(h.tokens.first().map(|t| t.as_str()), Some("on") | Some("function")) => Some(h.state.to_string()),
                            _ => None,
                        });
                        if let Some(h) = handler {
                            let names = out.entry(h).or_default();
                            let name = v.1.to_string();
                            if !names.contains(&name) { names.push(name); }
                        }
                    }
                }
            }
        }
        true
    });
    out
}

/// Year, month, day for a Unix time (UTC). From Howard Hinnant's civil_from_days.
fn civil(secs: i64) -> (i64, i64, i64) {
    let z = secs.div_euclid(86400) + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (yoe + era * 400 + if m <= 2 { 1 } else { 0 }, m, d)
}

/// HyperTalk constants, verbal operators and built-in functions.
fn register_vocabulary(e: &mut Engine, live: &L) {
    use rhai::{NativeCallContext, FLOAT};
    // Constants, unless a variable of the same name is in scope.
    #[allow(deprecated)]
    e.on_var(|name, _, ctx| {
        let v = match name {
            "empty" if !ctx.scope().contains("empty") => Dynamic::from(""), "space" => Dynamic::from(" "), "tab" => Dynamic::from("\t"), "quote" => Dynamic::from("\""),
            "lineFeed" | "newline" | "Return" => Dynamic::from("\n"),
            "eof" => Dynamic::from("\u{4}"), "formFeed" => Dynamic::from("\u{c}"), "comma" => Dynamic::from(","),
            "pi" => Dynamic::from(std::f64::consts::PI as FLOAT), "up" => Dynamic::from("up"), "down" => Dynamic::from("down"),
            "zero" => Dynamic::from(0_i64), "one" => Dynamic::from(1_i64), "two" => Dynamic::from(2_i64), "three" => Dynamic::from(3_i64),
            "four" => Dynamic::from(4_i64), "five" => Dynamic::from(5_i64), "six" => Dynamic::from(6_i64), "seven" => Dynamic::from(7_i64),
            "eight" => Dynamic::from(8_i64), "nine" => Dynamic::from(9_i64), "ten" => Dynamic::from(10_i64),
            "number" | "integer" | "point" | "rect" | "date" | "logical" | "boolean" => Dynamic::from(format!("\u{a7}type:{name}")),
            _ => return Ok(None),
        };
        if ctx.scope().contains(name) { Ok(None) } else { Ok(Some(v)) }
    });
    fn need(d: &Dynamic) -> R<FLOAT> { num(d).ok_or_else(|| err(&format!("expected a number, not \"{d}\""))) }
    fn int_or_float(x: FLOAT) -> Dynamic { if x.fract() == 0.0 && x.abs() < 9e15 { Dynamic::from(x as i64) } else { Dynamic::from(x) } }
    // Verbal operators. `and`/`or` evaluate both sides (HyperTalk's guide does not say whether it short-circuits).
    e.register_custom_operator("and", 60).unwrap();
    e.register_custom_operator("or", 30).unwrap();
    e.register_custom_operator("is", 90).unwrap();
    e.register_custom_operator("Contains", 110).unwrap();
    e.register_custom_operator("div", 180).unwrap();
    // Two-word operators arrive here as one word, from hypertalk_words: is not, is in, is a/an, is within.
    e.register_custom_operator("isnot", 90).unwrap();
    e.register_custom_operator("isin", 110).unwrap();
    e.register_custom_operator("isa", 90).unwrap();
    e.register_custom_operator("isan", 90).unwrap();
    e.register_custom_operator("iswithin", 110).unwrap();
    e.register_custom_operator("isnota", 90).unwrap();
    e.register_custom_operator("isnotan", 90).unwrap();
    e.register_custom_operator("isnotin", 110).unwrap();
    e.register_custom_operator("isnotwithin", 110).unwrap();
    e.register_custom_operator("mod", 180).unwrap();
    fn truth(d: &Dynamic) -> R<bool> { d.as_bool().or_else(|_| match d.to_string().as_str() { "true" => Ok(true), "false" => Ok(false), _ => Err(()) }).map_err(|_| err(&format!("expected true or false, not \"{d}\""))) }
    e.register_fn("and", |a: Dynamic, b: Dynamic| -> R<bool> { Ok(truth(&a)? && truth(&b)?) });
    e.register_fn("or", |a: Dynamic, b: Dynamic| -> R<bool> { Ok(truth(&a)? || truth(&b)?) });
    // `is`: numbers compare as numbers, text compares ignoring case (as in HyperTalk).
    e.register_fn("is", |a: Dynamic, b: Dynamic| -> bool {
        match (num(&a), num(&b)) { (Some(x), Some(y)) if !a.is_string() || !b.is_string() || (a.to_string().trim().parse::<FLOAT>().is_ok() && b.to_string().trim().parse::<FLOAT>().is_ok()) => x == y, _ => a.to_string().to_lowercase() == b.to_string().to_lowercase() }
    });
    e.register_fn("Contains", |a: Dynamic, b: Dynamic| a.to_string().to_lowercase().contains(&b.to_string().to_lowercase()));
    e.register_fn("isnot", |ctx: NativeCallContext, a: Dynamic, b: Dynamic| -> R<bool> { Ok(!ctx.call_native_fn::<bool>("is", (a, b))?) });
    e.register_fn("isin", |a: Dynamic, b: Dynamic| b.to_string().to_lowercase().contains(&a.to_string().to_lowercase()));
    // `x is a number|integer|point|rect|date|logical|empty`: the type word is a constant "§type:…".
    fn type_test(a: &Dynamic, t: &Dynamic) -> R<bool> {
        let t = t.to_string();
        let Some(t) = t.strip_prefix("\u{a7}type:") else { return Err(err(&format!("is a what? use number, integer, point, rect, date, logical or empty, not \"{t}\""))) };
        let s = a.to_string();
        let nums = |n: usize| { let p: Vec<&str> = s.split(',').collect(); p.len() == n && p.iter().all(|x| x.trim().parse::<f64>().is_ok()) };
        Ok(match t {
            "number" => a.is_int() || a.is_float() || s.trim().parse::<f64>().is_ok(),
            "integer" => a.is_int() || s.trim().parse::<i64>().is_ok(),
            "point" => nums(2),
            "rect" => nums(4),
            "logical" | "boolean" => a.is_bool() || s == "true" || s == "false",
            "date" => { let p: Vec<&str> = s.split('/').collect(); p.len() == 3 && p.iter().all(|x| x.trim().parse::<u32>().is_ok()) }
            "empty" => s.is_empty(),
            _ => false,
        })
    }
    e.register_fn("isa", |a: Dynamic, t: Dynamic| type_test(&a, &t));
    e.register_fn("isnota", |a: Dynamic, t: Dynamic| type_test(&a, &t).map(|b| !b));
    e.register_fn("isnotan", |a: Dynamic, t: Dynamic| type_test(&a, &t).map(|b| !b));
    e.register_fn("isnotin", |a: Dynamic, b: Dynamic| !b.to_string().to_lowercase().contains(&a.to_string().to_lowercase()));
    e.register_fn("isnotwithin", |ctx: NativeCallContext, p: Dynamic, r: Dynamic| -> R<bool> { Ok(!ctx.call_native_fn::<bool>("iswithin", (p, r))?) });
    e.register_fn("isan", |a: Dynamic, t: Dynamic| type_test(&a, &t));
    // `point is within rect`: "h,v" inside "left,top,right,bottom".
    e.register_fn("iswithin", |p: Dynamic, r: Dynamic| -> R<bool> {
        let n = |s: String| -> Vec<f64> { s.split(',').filter_map(|x| x.trim().parse().ok()).collect() };
        let (p, r) = (n(p.to_string()), n(r.to_string()));
        if p.len() != 2 || r.len() != 4 { return Err(err("is within needs a point \"h,v\" and a rect \"left,top,right,bottom\"")); }
        Ok(p[0] >= r[0] && p[0] < r[2] && p[1] >= r[1] && p[1] < r[3])
    });
    e.register_fn("div", |a: Dynamic, b: Dynamic| -> R<Dynamic> { let (x, y) = (need(&a)?, need(&b)?); if y == 0.0 { return Err(err("division by zero")); } Ok(Dynamic::from((x / y).trunc() as i64)) });
    e.register_fn("mod", |a: Dynamic, b: Dynamic| -> R<Dynamic> { let (x, y) = (need(&a)?, need(&b)?); if y == 0.0 { return Err(err("division by zero")); } Ok(int_or_float(x % y)) });
    // `^` is a power, as in HyperTalk (not Rhai's exclusive-or).
    e.register_fn("^", |a: i64, b: i64| -> Dynamic { if (0..63).contains(&b) { a.checked_pow(b as u32).map(Dynamic::from).unwrap_or_else(|| Dynamic::from((a as FLOAT).powf(b as FLOAT))) } else { Dynamic::from((a as FLOAT).powf(b as FLOAT)) } });
    e.register_fn("^", |a: FLOAT, b: FLOAT| a.powf(b));
    e.register_fn("^", |a: i64, b: FLOAT| (a as FLOAT).powf(b));
    e.register_fn("^", |a: FLOAT, b: i64| a.powf(b as FLOAT));
    // Functions.
    e.register_fn("__sortParts", |text: Dynamic, kind: &str| sort_parts(&text.to_string(), kind));
    e.register_fn("__sortBy", sort_by_keys);
    e.register_fn("random", |n: i64| -> i64 { if n < 1 { 0 } else { 1 + (rand_u64() % n as u64) as i64 } });
    e.register_fn("length", |s: Dynamic| s.to_string().chars().count() as i64);
    e.register_fn("offset", |needle: &str, hay: &str| hay.to_lowercase().find(&needle.to_lowercase()).map(|b| hay[..b].chars().count() as i64 + 1).unwrap_or(0));
    e.register_fn("charToNum", |c: &str| c.chars().next().map(|c| c as i64).unwrap_or(0));
    e.register_fn("numToChar", |n: i64| char::from_u32(n as u32).map(|c| c.to_string()).unwrap_or_default());
    for f in ["sqrt", "abs", "round", "trunc", "sin", "cos", "tan", "atan", "exp", "ln", "log2"] {
        e.register_fn(f, move |x: Dynamic| -> R<Dynamic> {
            let v = need(&x)?;
            Ok(match f { "sqrt" => int_or_float(v.sqrt()), "abs" => int_or_float(v.abs()), "round" => Dynamic::from(v.round() as i64), "trunc" => Dynamic::from(v.trunc() as i64),
                "sin" => Dynamic::from(v.sin()), "cos" => Dynamic::from(v.cos()), "tan" => Dynamic::from(v.tan()), "atan" => Dynamic::from(v.atan()),
                "exp" => Dynamic::from(v.exp()), "ln" => Dynamic::from(v.ln()), _ => Dynamic::from(v.log2()) })
        });
    }
    // Whole-number results for floats too (Rhai's own round/trunc return floats).
    e.register_fn("round", |x: FLOAT| x.round() as i64);
    e.register_fn("trunc", |x: FLOAT| x.trunc() as i64);
    // Aggregates over an item list ("1,5,3") or two numbers.
    for f in ["min", "max", "sum", "average"] {
        let agg = move |v: Vec<FLOAT>| -> Dynamic {
            if v.is_empty() { return Dynamic::from(0_i64); }
            int_or_float(match f { "min" => v.iter().cloned().fold(FLOAT::INFINITY, FLOAT::min), "max" => v.iter().cloned().fold(FLOAT::NEG_INFINITY, FLOAT::max), "sum" => v.iter().sum(), _ => v.iter().sum::<FLOAT>() / v.len() as FLOAT })
        };
        e.register_fn(f, move |list: &str| -> R<Dynamic> { let v: Result<Vec<FLOAT>, _> = list.split(delim().as_str()).map(|x| need(&Dynamic::from(x.to_string()))).collect(); Ok(agg(v?)) });
        e.register_fn(f, move |a: Dynamic, b: Dynamic| -> R<Dynamic> { Ok(agg(vec![need(&a)?, need(&b)?])) });
    }
    e.register_fn("annuity", |rate: Dynamic, periods: Dynamic| -> R<FLOAT> { let (r, n) = (need(&rate)?, need(&periods)?); Ok(if r == 0.0 { n } else { (1.0 - (1.0 + r).powf(-n)) / r }) });
    e.register_fn("compound", |rate: Dynamic, periods: Dynamic| -> R<FLOAT> { let (r, n) = (need(&rate)?, need(&periods)?); Ok((1.0 + r).powf(n)) });
    // value("2 + 2"): evaluate text as an expression.
    e.register_fn("value", |ctx: NativeCallContext, src: &str| -> R<Dynamic> { ctx.engine().eval_expression::<Dynamic>(&hypertalk_words(src)).map_err(|e| err(&format!("value: {e}"))) });
    // param(n), paramCount(): the current handler's parameters.
    { let live = live.clone(); e.register_fn("param", move |n: i64| -> Dynamic { live.borrow().frames.last().and_then(|f| f.args.get((n.max(1) - 1) as usize).cloned()).unwrap_or(Dynamic::from("")) }); }
    { let live = live.clone(); e.register_fn("paramCount", move || -> i64 { live.borrow().frames.last().map(|f| f.args.len() as i64).unwrap_or(0) }); }
}

/// HyperTalk word pairs that stock Rhai cannot parse become single words, with the same length so error
/// positions stay right: `is not` → `isnot `, `is in` → `isin `, `is a` → `isa `, `is an` → `isan `,
/// `is within` → `iswithin `, and a `--` comment → `//`. Strings and comments are left alone.
pub fn hypertalk_words(src: &str) -> String {
    let c: Vec<char> = src.chars().collect();
    let mut out: Vec<char> = Vec::with_capacity(c.len());
    let mut i = 0;
    // True at the start of a statement (after `;`, `{`, `}` or at the start); comments do not change it.
    let mut stmt_start = true;
    // Where the current statement began in `out`, to tell `x = 5` (assignment) from `if x = 5` (comparison).
    let mut stmt_begin = 0usize;
    // Only the first `=` of a statement can be an assignment (`let y = a = b` compares the second).
    let mut eq_seen = false;
    let word_at = |i: usize| -> String { c[i..].iter().take_while(|x| x.is_alphanumeric() || **x == '_').collect() };
    let boundary_before = |i: usize| i == 0 || !(c[i - 1].is_alphanumeric() || c[i - 1] == '_');
    while i < c.len() {
        let ch = c[i];
        if !ch.is_whitespace() && !(ch == '/' && c.get(i + 1) == Some(&'/')) && !(ch == '-' && c.get(i + 1) == Some(&'-')) {
            // `return` where a value is expected is HyperTalk's newline constant: `"a" & return & "b"`.
            if ch == 'r' && boundary_before(i) && word_at(i) == "return" && !stmt_start {
                out.push('R'); out.extend("eturn".chars()); i += 6; stmt_start = false; continue;
            }
            stmt_start = matches!(ch, ';' | '{' | '}');
            if stmt_start { out.push(ch); i += 1; stmt_begin = out.len(); eq_seen = false; continue; }
        }
        if ch == '"' || ch == '`' {
            // a string: copy to its end
            out.push(ch); i += 1;
            while i < c.len() && c[i] != ch { if c[i] == '\\' && i + 1 < c.len() { out.push(c[i]); i += 1; } out.push(c[i]); i += 1; }
            if i < c.len() { out.push(c[i]); i += 1; }
            continue;
        }
        if ch == '/' && c.get(i + 1) == Some(&'/') {
            while i < c.len() && c[i] != '\n' { out.push(c[i]); i += 1; }
            continue;
        }
        if ch == '-' && c.get(i + 1) == Some(&'-') {
            // `--` comment, HyperTalk style
            out.push('/'); out.push('/'); i += 2;
            while i < c.len() && c[i] != '\n' { out.push(c[i]); i += 1; }
            continue;
        }
        // HyperTalk commands that are also Rhai method names (sort, push, pop, contains) become
        // capitalized keywords, unless they follow a dot: `sort cards …` vs `list.sort()`.
        if ch.is_ascii_lowercase() && boundary_before(i) {
            let w = word_at(i);
            if COLLIDE.contains(&w.as_str()) && out.iter().rev().find(|x| !x.is_whitespace()) != Some(&'.') {
                let mut cs = w.chars();
                out.extend(cs.next().unwrap().to_uppercase());
                out.extend(cs);
                i += w.chars().count();
                continue;
            }
        }
        // `there is a` keeps its own syntax.
        let after_there = { let before: String = out.iter().collect(); before.trim_end().ends_with("there") };
        if ch == 'i' && boundary_before(i) && word_at(i) == "is" && !after_there {
            let mut j = i + 2;
            let mut ws = 0;
            while j < c.len() && (c[j] == ' ' || c[j] == '\t') { j += 1; ws += 1; }
            if ws > 0 {
                let next = word_at(j);
                if matches!(next.as_str(), "not" | "in" | "a" | "an" | "within") {
                    // `is not a|an|in|within` is one negated operator: isnota, isnotan, isnotin, isnotwithin.
                    let mut k = j + next.chars().count();
                    let mut ws2 = 0;
                    if next == "not" {
                        while k < c.len() && (c[k] == ' ' || c[k] == '\t') { k += 1; ws2 += 1; }
                        let third = word_at(k);
                        if ws2 > 0 && matches!(third.as_str(), "in" | "a" | "an" | "within") {
                            out.extend("isnot".chars());
                            out.extend(third.chars());
                            out.extend(std::iter::repeat(' ').take(ws + ws2));
                            i = k + third.chars().count();
                            continue;
                        }
                    }
                    out.extend("is".chars());
                    out.extend(next.chars());
                    out.extend(std::iter::repeat(' ').take(ws));
                    i = j + next.chars().count();
                    continue;
                }
            }
        }
        // HyperTalk operators: `=` compares (except `x = …` at the start of a statement, `let x = …` and
        // `repeat with i = …`), `<>` is "not equal", `&` and `&&` join (`&&` with a space) at HyperTalk's
        // precedence, `not` is Rhai's `!`.
        let prev = out.iter().rev().find(|x| !x.is_whitespace()).copied();
        if ch == '=' && c.get(i + 1) != Some(&'=') && c.get(i + 1) != Some(&'>') && !matches!(prev, Some('=' | '!' | '<' | '>' | '+' | '-' | '*' | '/' | '%' | '&' | '|' | '^')) {
            let head: String = out[stmt_begin.min(out.len())..].iter().collect();
            let words: Vec<&str> = head.split_whitespace().collect();
            let is_ident = |w: &str| w.chars().next().is_some_and(|x| x.is_alphabetic() || x == '_') && w.chars().all(|x| x.is_alphanumeric() || x == '_');
            // An assignable target: a name followed by `.field` or `[index]` parts (`bricks[i].alive = false`).
            let lvalue = |h: &str| -> bool {
                let h = h.trim();
                let mut cs = h.chars().peekable();
                let mut name = String::new();
                while let Some(&x) = cs.peek() { if x.is_alphanumeric() || x == '_' { name.push(x); cs.next(); } else { break } }
                if !is_ident(&name) { return false; }
                loop {
                    match cs.next() {
                        None => return true,
                        Some('.') => { let mut f = String::new(); while let Some(&x) = cs.peek() { if x.is_alphanumeric() || x == '_' { f.push(x); cs.next(); } else { break } } if !is_ident(&f) { return false; } }
                        Some('[') => { let mut depth = 1; for x in cs.by_ref() { if x == '[' { depth += 1 } if x == ']' { depth -= 1; if depth == 0 { break } } } if depth != 0 { return false; } }
                        Some(x) if x.is_whitespace() => { if cs.all(|y| y.is_whitespace()) { return true; } return false; }
                        _ => return false,
                    }
                }
            };
            let first_eq = !eq_seen;
            eq_seen = true;
            let assignment = first_eq && lvalue(&head)
                || matches!(words.first(), Some(&"let") | Some(&"const"))
                || matches!(words.as_slice(), [r, "with", v] if (*r == "repeat") && is_ident(v));
            if !assignment { out.push('='); out.push('='); i += 1; continue; }
        }
        if ch == '<' && c.get(i + 1) == Some(&'>') { out.push('!'); out.push('='); i += 2; continue; }
        if ch == '&' && c.get(i + 1) == Some(&'&') { out.extend(" CatSp ".chars()); i += 2; continue; }
        if ch == '&' && c.get(i + 1) != Some(&'=') && prev != Some('&') { out.extend(" Cat ".chars()); i += 1; continue; }
        if ch == 'n' && boundary_before(i) && word_at(i) == "not" {
            let before: String = out.iter().collect();
            if !before.trim_end().ends_with("there is") {
                out.push('!'); out.push(' '); out.push(' '); i += 3; continue;
            }
        }
        out.push(ch);
        i += 1;
    }
    out.into_iter().collect()
}

/// Character span [start, end) of chunk n in text.
fn chunk_span(kind: &str, n: i64, text: &str) -> (usize, usize) {
    let chars: Vec<char> = text.chars().collect();
    let i = (n.max(1) - 1) as usize;
    let len = chars.len();
    match kind {
        "char" => (i.min(len), (i + 1).min(len)),
        "word" => {
            let mut spans = vec![];
            let mut start = None;
            for (k, c) in chars.iter().enumerate() {
                match (c.is_whitespace(), start) { (false, None) => start = Some(k), (true, Some(s)) => { spans.push((s, k)); start = None; } _ => {} }
            }
            if let Some(s) = start { spans.push((s, len)); }
            spans.get(i).copied().unwrap_or((len, len))
        }
        _ => {
            let d: Vec<char> = if kind == "line" { vec!['\n'] } else { delim().chars().collect() };
            let (mut start, mut idx, mut k) = (0, 0, 0);
            while k <= len {
                let at_delim = !d.is_empty() && k + d.len() <= len && chars[k..k + d.len()] == d[..];
                if k == len || at_delim {
                    if idx == i { return (start, k); }
                    idx += 1;
                    k += d.len().max(1);
                    start = k;
                    continue;
                }
                k += 1;
            }
            (len, len)
        }
    }
}

/// Seconds from 1 January 1904 to 1 January 1970: HyperCard's `the seconds` counts from 1904.
const EPOCH_1904: i64 = 2_082_844_800;

/// Local seconds since 1970 from HyperCard seconds (since 1904), "m/d/yy[yy]", a long date, or "y,m,d,h,m,s[,dow]" (dateItems).
fn parse_when(s: &str) -> Option<i64> {
    let t = s.trim();
    if let Ok(n) = t.parse::<i64>() { return Some(n - EPOCH_1904); }
    // days_from_civil (Howard Hinnant)
    let days = |y: i64, m: i64, d: i64| -> i64 {
        let y = if m <= 2 { y - 1 } else { y };
        let era = y.div_euclid(400);
        let yoe = y - era * 400;
        let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
        era * 146097 + yoe * 365 + yoe / 4 - yoe / 100 + doy - 719468
    };
    // Long date: "Thursday, November 19, 2026" (weekday optional, month names in English).
    const MONTHS: [&str; 12] = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];
    let lower = t.to_lowercase();
    let words: Vec<&str> = lower.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).collect();
    if let Some(mi) = words.iter().find_map(|w| MONTHS.iter().position(|m| w.len() >= 3 && w.starts_with(m))) {
        let nums: Vec<i64> = words.iter().filter_map(|w| w.parse().ok()).collect();
        if let (Some(d), Some(y)) = (nums.iter().find(|n| (1..=31).contains(*n)), nums.iter().find(|n| **n > 31)) {
            return Some(days(*y, mi as i64 + 1, *d) * 86400);
        }
    }
    if t.contains('/') {
        let p: Vec<i64> = t.split('/').filter_map(|x| x.trim().parse().ok()).collect();
        if p.len() != 3 { return None; }
        let y = if p[2] < 100 { if p[2] < 70 { 2000 + p[2] } else { 1900 + p[2] } } else { p[2] };
        return Some(days(y, p[0], p[1]) * 86400);
    }
    let p: Vec<i64> = t.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    if p.len() >= 6 { return Some(days(p[0], p[1], p[2]) * 86400 + p[3] * 3600 + p[4] * 60 + p[5]); }
    None
}

fn format_when(secs: i64, fmt: &str) -> Option<String> {
    let (y, m, d) = civil(secs);
    let s = secs.rem_euclid(86400);
    let dow = (secs.div_euclid(86400) + 4).rem_euclid(7); // 1970-01-01 was a Thursday; 0 = Sunday
    const DAYS: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    const MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    Some(match fmt {
        "seconds" => (secs + EPOCH_1904).to_string(),
        "abbreviated date" => format!("{}, {} {d}, {y}", &DAYS[dow as usize][..3], &MONTHS[(m - 1) as usize][..3]),
        "long time" => { let h = s / 3600; format!("{}:{:02}:{:02} {}", if h % 12 == 0 { 12 } else { h % 12 }, s / 60 % 60, s % 60, if h < 12 { "AM" } else { "PM" }) }
        "dateitems" => format!("{y},{m},{d},{},{},{},{}", s / 3600, s / 60 % 60, s % 60, dow + 1),
        "date" | "short date" => format!("{m}/{d}/{:02}", y % 100),
        "long date" => format!("{}, {} {d}, {y}", DAYS[dow as usize], MONTHS[(m - 1) as usize]),
        "time" => { let h = s / 3600; format!("{}:{:02} {}", if h % 12 == 0 { 12 } else { h % 12 }, s / 60 % 60, if h < 12 { "AM" } else { "PM" }) }
        _ => return None,
    })
}

/// A stack by name: this one, Home, a library in use, or another stack (loaded once from its file, read-only).
fn other_stack(w: &W, name: &str) -> R<u32> {
    let n = name.trim().trim_end_matches(".stack");
    let same = |o: &Obj| o.name.eq_ignore_ascii_case(n);
    {
        let wb = w.borrow();
        if wb.file.name.eq_ignore_ascii_case(n) || wb.obj(wb.stack_id()).map(same).unwrap_or(false) { return Ok(wb.stack_id()); }
        if wb.home.as_ref().is_some_and(|h| same(h) || n.eq_ignore_ascii_case("home")) { return Ok(HOME); }
        if let Some(i) = wb.libs.iter().position(same) { return Ok(LIB0 + i as u32); }
        if let Some(i) = wb.visited.iter().position(same) { return Ok(VISIT0 + i as u32); }
    }
    let path = w.borrow().find_stack(n).ok_or_else(|| err(&format!("no such stack \"{n}\"")))?;
    let mut obj = load_stack_obj(w, &path)?;
    let mut wb = w.borrow_mut();
    obj.id = VISIT0 + wb.visited.len() as u32;
    obj.parent = None;
    let id = obj.id;
    Rc::make_mut(&mut wb.visited).push(obj);
    Ok(id)
}

/// The stack object of a stack file found by `find_stack` (`demo:<name>` is a built-in demo), named
/// after its file if it has no name of its own.
fn load_stack_obj(w: &W, path: &str) -> R<Obj> {
    let file = match path.strip_prefix("demo:") {
        Some(d) => crate::demo::by_name(d).ok_or_else(|| err("no such demo"))?,
        None => { let bytes = w.borrow().fs.borrow().read_bytes(path).map_err(|e| err(&e))?; StackFile::from_bytes(&bytes).map_err(|e| err(&e))? }
    };
    let mut obj = file.objs.iter().find(|o| o.kind == Kind::Stack).cloned().ok_or_else(|| err("that file has no stack"))?;
    if obj.name.is_empty() { obj.name = file.name.clone(); }
    Ok(obj)
}

/// The topmost visible part of these kinds at a point on the current card.
fn part_at(wb: &World, p: (f64, f64), kinds: &[Kind]) -> Option<u32> {
    wb.parts_on(wb.current_card()).into_iter().rev().find(|id| {
        let Some(o) = wb.obj(*id) else { return false };
        if !kinds.contains(&o.kind) || !wb.get(*id, "visible").truthy() { return false; }
        let r = wb.rect(*id).map(|v| v as f64);
        p.0 >= r[0] && p.0 < r[0] + r[2] && p.1 >= r[1] && p.1 < r[1] + r[3]
    })
}

/// Whether a compiled script has an `on <msg>` handler (`msg` in lowercase).
fn handles(ast: &AST, msg: &str) -> bool {
    use rhai::{Expr, Stmt};
    ast.statements().iter().any(|st| matches!(st, Stmt::Expr(e) if matches!(e.as_ref(), Expr::Custom(c, _) if c.tokens.first().map(|t| t.as_str()) == Some("on") && c.state.to_string() == msg)))
}

/// Paint on a card: one line of its `paint` property per stroke, "<tool> <lineSize> <filled> h,v h,v …".
/// Journaled, so Undo takes a stroke back.
fn add_stroke(wb: &mut World, card: u32, pts: &[(f64, f64)]) {
    if pts.is_empty() { return; }
    let line = Stroke { tool: wb.tool, size: wb.line_size, filled: wb.filled, pts: pts.to_vec() }.to_line();
    wb.append_line(card, "paint", &line);
}

/// Compile `src` as the script of `id`. On error the previous working version stays live.
fn compile_into(engine: &Engine, live: &L, id: u32, src: &str) {
    let mut lb = live.borrow_mut();
    if src.trim().is_empty() { lb.asts.remove(&id); lb.errors.remove(&id); return; }
    match engine.compile(&hypertalk_words(&hypertalk_blocks(src))).map_err(|e| e.to_string()).and_then(|ast| only_handlers(&ast).map(|_| ast)) {
        Ok(ast) => { lb.locals.insert(id, put_targets(&ast)); lb.asts.insert(id, Rc::new(ast)); lb.errors.remove(&id); }
        Err(e) => { lb.errors.insert(id, e); }
    }
}

/// The text of every field as seen on card `c`: (field id, text). Background fields give this card's text.
fn card_texts(wb: &World, c: u32) -> Vec<(u32, String)> {
    let bg = wb.background_of(c);
    wb.file.objs.iter().filter(|o| o.kind == Kind::Field && (o.parent == Some(c) || (bg.is_some() && o.parent == bg))).map(|o| {
        let shared = o.props.get("sharedText").map(|v| v.truthy()).unwrap_or(false);
        let on_bg = o.parent == bg;
        let t = if on_bg && !shared { wb.obj(c).and_then(|cd| cd.props.get(&format!("~{}", o.id))).map(|v| v.text()).unwrap_or_default() } else { o.props.get("text").map(|v| v.text()).unwrap_or_default() };
        (o.id, t)
    }).collect()
}

/// HyperTalk's find forms. Returns the character span of the match in `text`, if any.
fn find_in(form: &str, needle: &str, text: &str) -> Option<(usize, usize)> {
    let lower = text.to_lowercase();
    let n = needle.to_lowercase();
    let chars = |b: usize| lower[..b].chars().count();
    let words: Vec<(usize, &str)> = { let mut v = vec![]; let mut start = None; for (i, c) in lower.char_indices() { match (c.is_alphanumeric(), start) { (true, None) => start = Some(i), (false, Some(s)) => { v.push((s, &lower[s..i])); start = None; } _ => {} } } if let Some(s) = start { v.push((s, &lower[s..])); } v };
    match form {
        "string" => lower.find(&n).map(|b| (chars(b), chars(b) + n.chars().count())),
        "whole" => { let mut from = 0; while let Some(b) = lower[from..].find(&n).map(|x| x + from) { let e = b + n.len(); let ok_l = b == 0 || !lower[..b].chars().last().unwrap().is_alphanumeric(); let ok_r = e >= lower.len() || !lower[e..].chars().next().unwrap().is_alphanumeric(); if ok_l && ok_r { return Some((chars(b), chars(e))); } from = b + 1; } None }
        _ => {
            // normal: every word of the needle starts some word; word: every word matches a whole word; chars: every word appears anywhere.
            let mut first = None;
            for nw in n.split_whitespace() {
                let hit = match form {
                    "word" => words.iter().find(|(_, w)| *w == nw).map(|(b, _)| *b),
                    "chars" => lower.find(nw),
                    _ => words.iter().find(|(_, w)| w.starts_with(nw)).map(|(b, _)| *b),
                }?;
                if first.is_none() { first = Some((chars(hit), chars(hit) + nw.chars().count())); }
            }
            first
        }
    }
}

/// Notes as HyperTalk writes them: letter, optional # or b, optional octave (4 = middle), optional
/// duration (w h q e s t x, with . for dotted), r for a rest; e.g. "c4q e g c5h". Tempo in quarter notes per minute.
fn synth_notes(instrument: &str, tempo: f32, notes: &str) -> Result<Vec<u8>, String> {
    let rate = 22050u32;
    let mut samples: Vec<f32> = vec![];
    let (mut octave, mut dur) = (4i32, 1.0f32);
    for tok in notes.split_whitespace() {
        let mut c = tok.chars().peekable();
        let letter = c.next().unwrap().to_ascii_lowercase();
        let base = match letter { 'c' => 0, 'd' => 2, 'e' => 4, 'f' => 5, 'g' => 7, 'a' => 9, 'b' => 11, 'r' => -100, _ => return Err(format!("\"{tok}\" is not a note")) };
        let mut semis = base;
        // A # after the letter is sharp, a b is flat ("bb" is B flat).
        if let Some(&x) = c.peek() { if x == '#' { semis += 1; c.next(); } else if x == 'b' { semis -= 1; c.next(); } }
        let mut digits = String::new();
        while let Some(&x) = c.peek() { if x.is_ascii_digit() { digits.push(x); c.next(); } else { break } }
        if !digits.is_empty() { octave = digits.parse().unwrap_or(4); }
        if let Some(&x) = c.peek() {
            dur = match x { 'w' => 4.0, 'h' => 2.0, 'q' => 1.0, 'e' => 0.5, 's' => 0.25, 't' => 0.125, 'x' => 0.0625, _ => dur };
            if "whqestx".contains(x) { c.next(); }
            if c.peek() == Some(&'.') { dur *= 1.5; }
        }
        let secs = dur * 60.0 / tempo.max(1.0);
        let n = (secs * rate as f32) as usize;
        if semis < -50 { samples.extend(std::iter::repeat(0.0).take(n)); continue; }
        let midi = 12 * (octave + 1) + semis;
        let f = 440.0 * 2f32.powf((midi - 69) as f32 / 12.0);
        let two_pi = 2.0 * std::f32::consts::PI;
        for i in 0..n {
            let t = i as f32 / rate as f32;
            let v = match instrument.to_lowercase().as_str() {
                "harpsichord" => ((t * two_pi * f).sin() + 0.5 * (t * two_pi * 2.0 * f).sin() + 0.25 * (t * two_pi * 3.0 * f).sin()) * (-t * 6.0).exp() * 0.45,
                "boing" => (t * two_pi * f * (1.0 + 0.5 * (-t * 8.0).exp())).sin() * (-t * 3.0).exp() * 0.7,
                _ => (t * two_pi * f).sin() * (-t * 2.5).exp() * 0.6,
            };
            // A short fade at the end of each note avoids clicks.
            let tail = ((n - i) as f32 / (rate as f32 * 0.01)).min(1.0);
            samples.push(v * tail);
        }
    }
    if samples.is_empty() { return Err("no notes".into()); }
    let data_len = (samples.len() * 2) as u32;
    let mut b = Vec::with_capacity(44 + data_len as usize);
    b.extend_from_slice(b"RIFF"); b.extend_from_slice(&(36 + data_len).to_le_bytes()); b.extend_from_slice(b"WAVEfmt ");
    b.extend_from_slice(&16u32.to_le_bytes()); b.extend_from_slice(&1u16.to_le_bytes()); b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&rate.to_le_bytes()); b.extend_from_slice(&(rate * 2).to_le_bytes()); b.extend_from_slice(&2u16.to_le_bytes()); b.extend_from_slice(&16u16.to_le_bytes());
    b.extend_from_slice(b"data"); b.extend_from_slice(&data_len.to_le_bytes());
    for s in samples { b.extend_from_slice(&((s.clamp(-1.0, 1.0) * 32000.0) as i16).to_le_bytes()); }
    Ok(b)
}

/// HyperTalk's block syntax, translated line by line into the brace form before compiling:
/// `on name a, b` … `end name`, `function f a` … `end f`, `if c then` … `else` … `end if`,
/// `if c then s [else s]` on one line, `repeat …` … `end repeat`, statements without `;`, and `¬` to continue a line.
/// Lines that already end in `{`, `}` or `;` are left alone, so both styles can be mixed. One output line per
/// input line, so error line numbers stay right.
pub fn hypertalk_blocks(src: &str) -> String {
    #[derive(PartialEq, Clone, Copy)]
    enum B { Handler, Func, If, Repeat }
    let lines: Vec<&str> = src.split('\n').collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut stack: Vec<B> = vec![];
    let mut last_single_if = false;
    let mut continued = false;

    // Split a line into code and a trailing comment, outside strings.
    fn split_comment(l: &str) -> (&str, &str) {
        let b = l.as_bytes();
        let mut in_str = false;
        let mut i = 0;
        while i < b.len() {
            match b[i] {
                b'"' => in_str = !in_str,
                b'\\' if in_str => i += 1,
                b'-' | b'/' if !in_str && i + 1 < b.len() && b[i + 1] == b[i] => return (&l[..i], &l[i..]),
                _ => {}
            }
            i += 1;
        }
        (l, "")
    }
    // Position of a whole word, outside strings, case-insensitive.
    fn find_word(code: &str, word: &str) -> Option<usize> {
        let lower = code.to_lowercase();
        let b = lower.as_bytes();
        let mut in_str = false;
        let mut i = 0;
        while i < b.len() {
            if b[i] == b'"' { in_str = !in_str; }
            if !in_str && lower[i..].starts_with(word) {
                let before = i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_');
                let j = i + word.len();
                let after = j >= b.len() || !(b[j].is_ascii_alphanumeric() || b[j] == b'_');
                if before && after { return Some(i); }
            }
            i += 1;
        }
        None
    }
    fn stmt(s: &str) -> String { let s = s.trim(); if s.is_empty() || s.ends_with(';') || s.ends_with('}') || s.ends_with('{') { s.to_string() } else { format!("{s};") } }
    // A whole statement (after `then` / `else`, or on its own line): rewritten like any other (sort … of,
    // menus, user commands). Not for the rest of a continued line, which is only part of a statement.
    fn emit(s: &str) -> String { several_statements(s.trim()).or_else(|| rewrite_statement(s.trim())).unwrap_or_else(|| stmt(s)) }
    fn continues(t: &str) -> bool {
        let lower = t.to_lowercase();
        ["&", "+", "-", "*", "/", "=", "<", ">", ",", "(", "[", "|"].iter().any(|e| t.ends_with(e))
            || [" and", " or", " is", " not", " of", " into"].iter().any(|e| lower.ends_with(e))
    }

    for (idx, raw) in lines.iter().enumerate() {
        let indent: String = raw.chars().take_while(|c| c.is_whitespace()).collect();
        let (code, comment) = split_comment(raw);
        let mut t = code.trim().to_string();
        let was_continued = continued;
        continued = false;
        if t.ends_with('¬') { t.pop(); continued = true; }
        let lower = t.to_lowercase();
        let first = lower.split_whitespace().next().unwrap_or("").to_string();
        let push = |out: &mut Vec<String>, s: String| out.push(format!("{indent}{s}{}{comment}", if comment.is_empty() || s.is_empty() { "" } else { " " }));
        if t.is_empty() || t.ends_with('{') || t.ends_with('}') || t.ends_with(';') || was_continued && !continued && continues(&t) {
            if !t.is_empty() { last_single_if = false; }
            // A brace-style line holding one statement can also be a user command: `check "x", true;`
            let one = t.strip_suffix(';').filter(|b| !b.contains(';') && !b.contains('{') && !b.contains('}'));
            let line = match one.and_then(rewrite_statement) {
                Some(c) if !was_continued => c,
                // Several statements on one line: `sort … of <container>` and menu statements among them are still rewritten.
                _ if !was_continued => several_statements(&t).unwrap_or_else(|| t.clone()),
                _ => t.clone(),
            };
            push(&mut out, line);
            continue;
        }
        if continued || continues(&t) { push(&mut out, t.clone()); continued = true; continue; }
        let next_opens = lines.iter().skip(idx + 1).map(|l| l.trim()).find(|l| !l.is_empty()).map(|l| l.starts_with('{')).unwrap_or(false);
        let single_if_before = last_single_if;
        last_single_if = false;
        let line = match first.as_str() {
            "on" if !next_opens => {
                stack.push(B::Handler);
                let rest = t[2..].trim().replace(',', " ");
                format!("on {rest} {{")
            }
            "function" if !next_opens => {
                stack.push(B::Func);
                let rest = t[8..].trim().replace(',', " ");
                format!("function {rest} {{")
            }
            "end" => { stack.pop(); "}".to_string() }
            "if" => {
                match find_word(&t, "then") {
                    Some(p) => {
                        let cond = t[2..p].trim();
                        let after = t[p + 4..].trim();
                        if after.is_empty() { stack.push(B::If); format!("if {cond} {{") }
                        else {
                            last_single_if = true;
                            match find_word(after, "else") {
                                Some(e) => format!("if {cond} {{ {} }} else {{ {} }}", emit(&after[..e]), emit(&after[e + 4..])),
                                None => format!("if {cond} {{ {} }}", emit(after)),
                            }
                        }
                    }
                    None => stmt(&t),
                }
            }
            "else" => {
                let rest = t[4..].trim();
                let in_block = stack.last() == Some(&B::If);
                let close = if in_block && !single_if_before { "} " } else { "" };
                if rest.to_lowercase().starts_with("if ") {
                    match find_word(rest, "then") {
                        Some(p) => {
                            let cond = rest[2..p].trim();
                            let after = rest[p + 4..].trim();
                            if !in_block || single_if_before { stack.push(B::If); }
                            if after.is_empty() { format!("{close}else if {cond} {{") } else { format!("{close}else if {cond} {{ {}", emit(after)) }
                        }
                        None => stmt(&t),
                    }
                } else if rest.is_empty() {
                    if !in_block || single_if_before { stack.push(B::If); }
                    format!("{close}else {{")
                } else if !in_block || single_if_before {
                    format!("else {{ {} }}", emit(rest))
                } else {
                    format!("{close}else {{ {}", emit(rest))
                }
            }
            "repeat" => {
                stack.push(B::Repeat);
                let rest = t[6..].trim();
                let rest = rest.strip_prefix("for ").unwrap_or(rest);
                if rest.is_empty() { "repeat {".to_string() } else { format!("repeat {rest} {{") }
            }
            // The rest of a continued line (`¬`) is never a command of its own.
            // Several statements on a line are rewritten one by one, so a user command does not swallow the rest.
            _ => if was_continued { stmt(&t) } else { emit(&t) },
        };
        push(&mut out, line);
    }
    out.join("\n")
}

/// Remove chunk n and its delimiter: `delete line 2 of x`, `delete word 3 of field "f"`.
fn delete_chunk(kind: &str, n: i64, text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let (a, b) = chunk_span(kind, n, text);
    if a >= chars.len() && b >= chars.len() { return text.to_string(); }
    let (mut a, mut b) = (a, b);
    match kind {
        "line" | "item" => {
            let d = if kind == "line" { 1 } else { delim().chars().count() };
            if b + d <= chars.len() && b < chars.len() { b += d } else if a >= d { a -= d }
        }
        "word" => { while b < chars.len() && chars[b].is_whitespace() { b += 1; } if b == chars.len() { while a > 0 && chars[a - 1].is_whitespace() { a -= 1; } } }
        _ => {}
    }
    chars[..a].iter().chain(chars[b..].iter()).collect()
}

/// Words that start a statement on their own: commands, keywords, constants. Any other word at the start of a
/// statement, followed by arguments or nothing, is a user command (a message to the message path).
const KNOWN_STATEMENT_WORDS: &[&str] = &[
    // Rhai
    "let", "const", "if", "else", "while", "loop", "for", "do", "return", "break", "continue", "fn", "switch", "throw", "try", "catch",
    "import", "export", "private", "print", "debug", "true", "false", "this",
    // this language
    "on", "function", "end", "pass", "exit", "next", "repeat", "global", "put", "get", "set", "go", "answer", "ask", "wait", "visual", "choose", "drag", "reset", "menu", "find", "beep",
    "send", "new", "hide", "show", "enable", "disable", "mark", "unmark", "play", "lock", "unlock", "with", "add", "subtract",
    "multiply", "divide", "do", "sort", "push", "pop", "edit", "select", "click", "type", "create", "convert", "file", "open", "close",
    "read", "write", "delete", "rename", "load", "url", "start", "stop", "there", "the", "me", "it", "target", "field", "button",
    "card", "canvas", "picture", "word", "line", "item", "char", "character", "first", "second", "third", "fourth", "fifth", "sixth",
    "seventh", "eighth", "ninth", "tenth", "last", "middle", "any", "empty", "space", "tab", "quote", "pi", "zero", "one", "two",
    "three", "four", "five", "six", "seven", "eight", "nine", "ten", "up", "down", "lineFeed", "newline", "comma", "eof",
];

/// Statements rewritten before compiling: sorting a container, menu statements, and user commands.
fn rewrite_statement(line: &str) -> Option<String> { sort_container(line).or_else(|| menu_statement(line)).or_else(|| user_command(line)) }

/// Menu statements, as calls:
/// `doMenu "Next" [, "Go"]`, `reset menuBar`, `delete menu X`, `delete|enable|disable menuItem X of menu Y`,
/// `enable|disable menu X`, `put V into|after|before menu X [with menuMessage[s] M]`.
fn menu_statement(line: &str) -> Option<String> {
    let t = line.trim().trim_end_matches(';').trim_end();
    let words = top_words(t);
    let w = |i: usize| words.get(i).map(|&(a, b)| t[a..b].to_lowercase()).unwrap_or_default();
    let from = |i: usize| words.get(i).map(|&(a, _)| t[a..].trim()).unwrap_or("");
    let span = |i: usize, j: usize| if i < j && j <= words.len() { t[words[i].0..words[j - 1].1].trim() } else { "" };
    let item_of_menu = |op: &str| -> Option<String> {
        let k = (2..words.len()).find(|&k| matches!(w(k).as_str(), "of" | "from") && w(k + 1) == "menu")?;
        Some(format!("__menuItem(\"{op}\", {}, {});", span(2, k), from(k + 2)))
    };
    match (w(0).as_str(), w(1).as_str()) {
        ("domenu", _) if words.len() > 1 => Some(format!("__doMenu({});", from(1))),
        ("reset", "menubar") if words.len() == 2 => Some("__menuReset();".into()),
        ("hide" | "show", "message" | "msg") if words.len() <= 3 && matches!(w(2).as_str(), "" | "box" | "window") => Some(format!("__messageBox({});", w(0) == "show")),
        ("delete", "menu") if words.len() > 2 => Some(format!("__menuDelete({});", from(2))),
        ("delete", "menuitem") => item_of_menu("delete"),
        ("enable" | "disable", "menuitem") => item_of_menu(&w(0)),
        ("enable" | "disable", "menu") if words.len() > 2 => Some(format!("__menuItem(\"{}\", \"\", {});", w(0), from(2))),
        ("put", _) => {
            let k = (2..words.len()).find(|&k| matches!(w(k).as_str(), "into" | "after" | "before") && w(k + 1) == "menu")?;
            let how = w(k);
            let m = (k + 2..words.len()).find(|&m| w(m) == "with" && w(m + 1).starts_with("menumessage"));
            let (name, msgs) = match m { Some(m) => (span(k + 2, m), from(m + 2)), None => (from(k + 2), "\"\"") };
            Some(format!("__menuPut({}, \"{how}\", {name}, {msgs});", span(1, k)))
        }
        _ => None,
    }
}

/// A line holding several statements (`a; b; c`) where some need rewriting (`sort … of`, menus, user
/// commands): each is rewritten on its own. None if there is nothing to rewrite.
fn several_statements(t: &str) -> Option<String> {
    if t.contains('{') || t.contains('}') { return None; }
    let parts = split_statements(t);
    if parts.len() < 2 && !t.trim_end().ends_with(';') { return None; }
    let rewritten: Vec<Option<String>> = parts.iter().map(|st| rewrite_statement(st)).collect();
    if rewritten.iter().all(Option::is_none) { return None; }
    Some(parts.iter().zip(rewritten).map(|(st, r)| r.unwrap_or_else(|| format!("{};", st.trim()))).collect::<Vec<_>>().join(" "))
}

/// Split a line at the `;` outside quotes and brackets (empty pieces dropped).
fn split_statements(t: &str) -> Vec<String> {
    let (mut out, mut cur, mut depth, mut quote) = (vec![], String::new(), 0i32, false);
    for c in t.chars() {
        if quote { cur.push(c); if c == '"' { quote = false; } continue; }
        match c {
            '"' => { quote = true; cur.push(c); }
            '(' | '[' => { depth += 1; cur.push(c); }
            ')' | ']' => { depth -= 1; cur.push(c); }
            ';' if depth == 0 => { if !cur.trim().is_empty() { out.push(std::mem::take(&mut cur)); } else { cur.clear(); } }
            c => cur.push(c),
        }
    }
    if !cur.trim().is_empty() { out.push(cur); }
    out
}

/// Words at the top level of a statement (not inside quotes or brackets), with their byte ranges.
fn top_words(t: &str) -> Vec<(usize, usize)> {
    let (mut out, mut depth, mut quote, mut start) = (vec![], 0i32, false, None::<usize>);
    for (i, c) in t.char_indices() {
        if quote { if c == '"' { quote = false; } continue; }
        match c {
            '"' => { quote = true; start.get_or_insert(i); }
            '(' | '[' | '{' => { depth += 1; start.get_or_insert(i); }
            ')' | ']' | '}' => { depth -= 1; }
            c if c.is_whitespace() && depth == 0 => { if let Some(s) = start.take() { out.push((s, i)); } }
            _ => { start.get_or_insert(i); }
        }
    }
    if let Some(s) = start { out.push((s, t.len())); }
    out
}

const SORT_FLAGS: &[&str] = &["ascending", "descending", "text", "numeric", "international", "datetime"];

/// `sort [lines|items of] <container> [ascending|descending] [text|numeric|dateTime] [by <expr of each>]`
/// → `{ let __p = __sortParts(<container>, "lines"); let __k = []; for each in __p { __k.push(<expr>); }
///      put __sortBy(__p, __k, "lines", "<flags>") into <container>; }`  (all on one line; no closure, since
/// Rhai cannot hash a closure body holding custom syntax).
/// Sorting cards (`sort [cards] … by …`) is left to the `sort` command.
fn sort_container(line: &str) -> Option<String> {
    let t = line.trim().trim_end_matches(';').trim_end();
    let words = top_words(t);
    let w = |i: usize| words.get(i).map(|&(a, b)| t[a..b].to_lowercase()).unwrap_or_default();
    if w(0) != "sort" { return None; }
    let mut i = 1;
    let mut flags = vec![];
    while SORT_FLAGS.contains(&w(i).as_str()) { flags.push(w(i)); i += 1; }
    let first = w(i);
    if first.is_empty() || matches!(first.as_str(), "cards" | "the" | "by" | "marked" | "this") { return None; }
    let kind = if matches!(first.as_str(), "lines" | "items") && w(i + 1) == "of" { i += 2; first.clone() } else { "lines".to_string() };
    let start = words.get(i)?.0;
    let mut j = i;
    while j < words.len() && !SORT_FLAGS.contains(&w(j).as_str()) && w(j) != "by" { j += 1; }
    if j == i { return None; }
    let container = t[start..words[j - 1].1].trim();
    while SORT_FLAGS.contains(&w(j).as_str()) { flags.push(w(j)); j += 1; }
    let key = if w(j) == "by" { Some(t[words.get(j + 1)?.0..].trim()) } else if j < words.len() { return None } else { None };
    let flags = flags.join(" ");
    let key = key.unwrap_or("each");
    Some(format!("{{ let __p = __sortParts({container}, \"{kind}\"); let __k = []; for each in __p {{ __k.push({key}); }} put __sortBy(__p, __k, \"{kind}\", \"{flags}\") into {container}; }}"))
}

/// The lines or items of a text, for sorting.
fn sort_parts(text: &str, kind: &str) -> rhai::Array {
    if text.is_empty() { return vec![]; }
    if kind == "items" { text.split(delim().as_str()).map(|p| Dynamic::from(p.to_string())).collect() } else { text.lines().map(|p| Dynamic::from(p.to_string())).collect() }
}

/// How `sort` compares keys: `sort … numeric`, `sort … dateTime`, or as text (the default; case does not matter).
#[derive(Clone, Copy, PartialEq, Eq)]
enum SortStyle { Text, Numeric, DateTime }

impl SortStyle {
    fn from_flags(flags: &str) -> SortStyle {
        if flags.contains("numeric") { SortStyle::Numeric } else if flags.contains("datetime") { SortStyle::DateTime } else { SortStyle::Text }
    }
    fn cmp(self, a: &str, b: &str) -> std::cmp::Ordering {
        match self {
            // Numbers in order; anything that is not a number goes after them.
            SortStyle::Numeric => { let f = |s: &str| s.trim().parse::<f64>().unwrap_or(f64::INFINITY); f(a).total_cmp(&f(b)) }
            SortStyle::DateTime => { let f = |s: &str| parse_when(s).unwrap_or(i64::MAX); f(a).cmp(&f(b)) }
            SortStyle::Text => a.to_lowercase().cmp(&b.to_lowercase()),
        }
    }
}

/// Sort (key, value) pairs stably by key, as `sort` does: ascending unless `flags` say descending.
fn sort_keyed<T>(keyed: &mut [(String, T)], flags: &str) {
    let style = SortStyle::from_flags(flags);
    let desc = flags.contains("descending");
    keyed.sort_by(|a, b| if desc { style.cmp(&b.0, &a.0) } else { style.cmp(&a.0, &b.0) });
}

/// Sort pieces stably by their keys and join them again.
fn sort_by_keys(parts: rhai::Array, keys: rhai::Array, kind: &str, flags: &str) -> String {
    let mut keyed: Vec<(String, String)> = keys.into_iter().map(|k| k.to_string()).zip(parts.into_iter().map(|p| p.to_string())).collect();
    sort_keyed(&mut keyed, flags);
    let sep = if kind == "items" { delim() } else { "\n".to_string() };
    keyed.into_iter().map(|(_, p)| p).collect::<Vec<_>>().join(&sep)
}

/// `check "x", true` → `__cmd("check", ["x", true])`. None if the line is not a user command.
fn user_command(line: &str) -> Option<String> {
    let t = line.trim().trim_end_matches(';').trim_end();
    let word: String = t.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
    if word.is_empty() || !word.chars().next().unwrap().is_alphabetic() { return None; }
    if KNOWN_STATEMENT_WORDS.iter().any(|k| k.eq_ignore_ascii_case(&word)) { return None; }
    let rest = t[word.len()..].trim();
    // Assignments, calls, methods, indexing, map keys and operators are not commands.
    if rest.starts_with(['(', '.', '[', '=', ':', '+', '-', '*', '/', '%', '<', '>', '!', '&', '|', '^', '?']) { return None; }
    Some(format!("__cmd({word:?}, [{rest}]);"))
}

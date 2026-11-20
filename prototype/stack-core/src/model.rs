use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind { Stack, Background, Card, Button, Field, Canvas, Picture }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value { Bool(bool), Int(i64), Num(f64), Str(String) }

impl Value {
    pub fn text(&self) -> String {
        match self { Value::Bool(b) => b.to_string(), Value::Int(i) => i.to_string(), Value::Num(n) => n.to_string(), Value::Str(s) => s.clone() }
    }
    pub fn int(&self) -> i64 {
        match self { Value::Int(i) => *i, Value::Num(n) => *n as i64, Value::Str(s) => s.trim().parse().unwrap_or(0), Value::Bool(b) => *b as i64 }
    }
    pub fn truthy(&self) -> bool {
        match self { Value::Bool(b) => *b, Value::Str(s) => s == "true", Value::Int(i) => *i != 0, Value::Num(n) => *n != 0.0 }
    }
}
impl From<&str> for Value { fn from(s: &str) -> Self { Value::Str(s.into()) } }
impl From<i64> for Value { fn from(i: i64) -> Self { Value::Int(i) } }
impl From<bool> for Value { fn from(b: bool) -> Self { Value::Bool(b) } }

/// Every object carries its own script as text. `parent` is the next stop on the message path.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Obj {
    pub id: u32,
    pub kind: Kind,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<u32>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub script: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub props: BTreeMap<String, Value>,
}

/// One journal entry. `kind` is "set", "create" or "delete". Entries with the same `txn` undo together.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Change {
    pub txn: u64,
    pub kind: String,
    pub obj: u32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Obj>,
    /// Written by a handler for a system message (openCard, closeCard, frame), not by the user.
    /// Undo skips these: they are consequences of browsing, not edits.
    #[serde(default, skip_serializing_if = "is_false")]
    pub auto: bool,
}

fn is_false(b: &bool) -> bool { !*b }

/// One shape on a canvas part. Not saved: a canvas is redrawn by its scripts.
#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Rect { x: f64, y: f64, w: f64, h: f64, color: String },
    Circle { x: f64, y: f64, r: f64, color: String },
    Text { x: f64, y: f64, text: String, color: String },
}

/// A picture or sound stored in the stack's zip container under `file`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    /// "picture" or "sound".
    pub kind: String,
    pub file: String,
    pub mime: String,
}

/// A file opened with `open file`: its text, where `read` continues, and whether `write` has started.
#[derive(Clone, Debug, Default)]
pub struct OpenFile { pub text: String, pub read_pos: usize, pub written: bool }

/// Commands for the player's audio, queued by scripts.
#[derive(Clone, Debug, PartialEq)]
pub enum SoundCmd { Play(String), Stop, /// A clip made by the runtime (`play "harpsichord" "c e g"`): key and WAV bytes.
    Clip(String, Vec<u8>) }

/// Current file format. 2: zip container, `&` joins text, `+` adds numbers only.
/// 3: `&&` joins with a space and logic is `and` / `or`, as in HyperTalk.
pub const FORMAT: u32 = 3;

/// The whole stack: snapshot plus journal in `stack.toml`, assets beside it, in one zip file.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StackFile {
    pub format: u32,
    pub name: String,
    pub width: i64,
    pub height: i64,
    /// Powers this stack asks for (for example "sound"). Nothing else is reachable from scripts.
    #[serde(default)]
    pub capabilities: Vec<String>,
    pub next_id: u32,
    pub current_card: u32,
    #[serde(default)]
    pub txn: u64,
    pub objs: Vec<Obj>,
    #[serde(default)]
    pub journal: Vec<Change>,
    #[serde(default)]
    pub assets: Vec<Asset>,
    /// Asset contents by name. Kept in the zip, not in the TOML.
    #[serde(skip)]
    pub blobs: BTreeMap<String, Vec<u8>>,
}

impl StackFile {
    pub fn new(name: &str, width: i64, height: i64) -> StackFile {
        StackFile { format: FORMAT, name: name.into(), width, height, capabilities: vec![], next_id: 1, current_card: 0, txn: 0, objs: vec![], journal: vec![], assets: vec![], blobs: Default::default() }
    }

    /// The readable part of the stack: everything except asset contents.
    pub fn to_text(&self) -> String { toml::to_string_pretty(self).expect("stack serializes") }

    pub fn from_text(s: &str) -> Result<Self, String> {
        let mut f: StackFile = toml::from_str(s).map_err(|e| e.to_string())?;
        if f.format < 2 {
            for o in f.objs.iter_mut() { o.script = migrate_joins(&o.script); }
        }
        if f.format < 3 {
            for o in f.objs.iter_mut() { o.script = migrate_logic(&o.script); }
        }
        f.format = FORMAT;
        Ok(f)
    }

    /// Write the zip container: `stack.toml` plus `assets/...`.
    pub fn to_zip(&self) -> Vec<u8> {
        use std::io::Write;
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut z = zip::ZipWriter::new(&mut buf);
            let opts = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
            z.start_file("stack.toml", opts).expect("zip");
            z.write_all(self.to_text().as_bytes()).expect("zip");
            for a in &self.assets {
                if let Some(b) = self.blobs.get(&a.name) {
                    // Pictures and sounds are usually compressed already.
                    let o = if a.mime.contains("wav") || a.mime.contains("svg") { opts } else { opts.compression_method(zip::CompressionMethod::Stored) };
                    z.start_file(a.file.as_str(), o).expect("zip");
                    z.write_all(b).expect("zip");
                }
            }
            z.finish().expect("zip");
        }
        buf.into_inner()
    }

    /// Read a zip container, or a plain TOML stack from format 1.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        use std::io::Read;
        if !bytes.starts_with(b"PK") {
            return StackFile::from_text(std::str::from_utf8(bytes).map_err(|e| e.to_string())?);
        }
        let mut z = zip::ZipArchive::new(std::io::Cursor::new(bytes)).map_err(|e| e.to_string())?;
        let mut text = String::new();
        z.by_name("stack.toml").map_err(|e| format!("no stack.toml in the container: {e}"))?.read_to_string(&mut text).map_err(|e| e.to_string())?;
        let mut f = StackFile::from_text(&text)?;
        for a in f.assets.clone() {
            let mut b = vec![];
            z.by_name(&a.file).map_err(|e| format!("missing {}: {e}", a.file))?.read_to_end(&mut b).map_err(|e| e.to_string())?;
            f.blobs.insert(a.name, b);
        }
        Ok(f)
    }

    /// Add or replace a picture or sound. Returns the stored asset.
    pub fn add_asset(&mut self, name: &str, mime: &str, bytes: Vec<u8>) -> Asset {
        let kind = if mime.starts_with("audio") { "sound" } else { "picture" };
        let ext = mime.rsplit('/').next().unwrap_or("bin").split('+').next().unwrap_or("bin").replace("mpeg", "mp3").replace("x-wav", "wav");
        let safe: String = name.chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' }).collect();
        let a = Asset { name: name.into(), kind: kind.into(), file: format!("assets/{kind}s/{safe}.{ext}"), mime: mime.into() };
        self.assets.retain(|x| x.name != name);
        self.assets.push(a.clone());
        self.blobs.insert(name.into(), bytes);
        a
    }

    pub fn asset(&self, name: &str) -> Option<&Asset> { self.assets.iter().find(|a| a.name.eq_ignore_ascii_case(name)) }
}

/// Format 2 used Rhai's `&&` and `||` for logic; from format 3 `&&` joins with a space, so they become `and` / `or`.
pub fn migrate_logic(script: &str) -> String {
    let c: Vec<char> = script.chars().collect();
    let mut out = String::with_capacity(script.len());
    let (mut i, mut in_str, mut in_comment) = (0, false, false);
    while i < c.len() {
        let ch = c[i];
        if in_comment { if ch == '\n' { in_comment = false; } out.push(ch); i += 1; continue; }
        if ch == '"' && (i == 0 || c[i - 1] != '\\') { in_str = !in_str; }
        if !in_str && ch == '/' && c.get(i + 1) == Some(&'/') { in_comment = true; }
        if !in_str && !in_comment && ((ch == '&' && c.get(i + 1) == Some(&'&')) || (ch == '|' && c.get(i + 1) == Some(&'|'))) {
            out.push_str(if ch == '&' { "and" } else { "or" });
            i += 2;
            continue;
        }
        out.push(ch);
        i += 1;
    }
    out
}

/// Format 1 joined text with `+`. Rewrite each `+` that has a string literal on either side to `&`.
/// Other `+` stay, since they may add numbers.
pub fn migrate_joins(script: &str) -> String {
    let c: Vec<char> = script.chars().collect();
    let mut out = String::with_capacity(script.len());
    let mut in_str = false;
    let mut i = 0;
    while i < c.len() {
        let ch = c[i];
        if ch == '"' && (i == 0 || c[i - 1] != '\\') { in_str = !in_str; }
        if ch == '+' && !in_str && c.get(i + 1) != Some(&'=') {
            let left = c[..i].iter().rev().find(|x| !x.is_whitespace()).copied();
            let right = c[i + 1..].iter().find(|x| !x.is_whitespace()).copied();
            if left == Some('"') || right == Some('"') { out.push('&'); i += 1; continue; }
        }
        out.push(ch);
        i += 1;
    }
    out
}

/// Runtime view of a stack. All mutations go through `set`, `create`, `delete`, so all are journaled.
pub struct World {
    pub file: StackFile,
    pub log: Vec<String>,
    pub dialog: Option<String>,
    pub last_error: Option<String>,
    /// Greater than zero while a system message is being handled.
    pub auto_depth: u32,
    /// Pointer position relative to the card, and keys held down. Set by the player.
    pub mouse: (f64, f64),
    pub keys: std::collections::BTreeSet<String>,
    /// Draw lists, one per canvas part on the current card.
    pub draw: std::collections::HashMap<u32, Vec<Shape>>,
    /// The Home stack's object (id 0): last stop on every message path, shared by all stacks.
    pub home: Option<Obj>,
    /// The sound playing now (`the sound`), and commands waiting for the player.
    pub sound: Option<String>,
    pub sound_cmds: Vec<SoundCmd>,
    /// Visual effect for the next card display, with a counter so the player can tell effects apart.
    pub effect: Option<String>,
    pub effect_seq: u64,
    pub locked: bool,
    /// HyperTalk's global properties and functions that are runtime state, not saved.
    pub item_delim: String,
    pub lock_messages: bool,
    pub result: String,
    pub mouse_down: bool,
    pub click_loc: (f64, f64),
    /// `answer … with "A" or "B"` buttons and `ask` default text (Some = an ask dialog). The script waits
    /// for the reply, which goes into `it`.
    pub dialog_buttons: Vec<String>,
    pub dialog_ask: Option<String>,
    /// `send … in N seconds`: (due time in ms since 1970, object, message).
    pub timers: Vec<(f64, u32, String)>,
    /// `push card` / `pop card`.
    pub push_list: Vec<u32>,
    /// `edit the script of …`: the player opens the script of this object.
    pub edit_request: Option<u32>,
    /// Text selection in a field: (field, start, end) in characters. `selection` is what the player
    /// reports; `select_request` is what `select` asks the player to show.
    pub selection: Option<(u32, usize, usize)>,
    pub select_request: Option<(u32, usize, usize)>,
    /// The field being typed into (for `type`), set when a field gets focus.
    pub focused: Option<u32>,
    /// HyperCard's user level: 1 browsing, 2 typing, 3 painting, 4 authoring, 5 scripting.
    pub user_level: i64,
    /// Files: the file system, the folder relative paths start from (the stack's folder), and open files.
    pub fs: std::rc::Rc<std::cell::RefCell<Box<dyn crate::files::Fs>>>,
    pub fs_base: String,
    pub open_files: std::collections::HashMap<String, OpenFile>,
    /// `answer file` (false) / `ask file` (true): a file dialog over this folder.
    pub dialog_file: Option<(bool, String)>,
    /// `load url`: requests for the player, (url, object to tell), and what has arrived.
    pub url_requests: Vec<(String, u32)>,
    pub url_cache: std::collections::HashMap<String, String>,
    /// Stacks in use as libraries (their stack objects; their scripts sit between this stack and Home).
    pub libs: Vec<Obj>,
    /// `go to stack …`: for the player. The stacks folder to search, and the stacks open in the player.
    pub go_stack: Option<GoStack>,
    pub stacks_dir: String,
    pub open_stacks: Vec<String>,
    /// Minutes east of UTC, reported by the player, for `the date`, `the time` and `convert`.
    pub tz_offset_min: i64,
    /// `the mouseClick`: set by a click, cleared when read.
    pub mouse_click: bool,
    /// Where the last click in a locked field landed: (field, character offset), for clickText/clickLine/clickChunk.
    pub click_text_at: Option<(u32, usize)>,
    /// The part under the pointer and the part being pressed (for mouseWithin, mouseStillDown).
    pub hover: Option<u32>,
    pub pressed: Option<u32>,
    /// Where the running script's journal entries start. A write merged into an older entry records the
    /// value it replaced in `floor_merges`, so `rollback` can take a paused script's changes back.
    pub journal_floor: usize,
    pub floor_merges: Vec<(usize, Option<Value>)>,
    /// Other stacks named by scripts (ids VISIT0, VISIT0 + 1, …).
    pub visited: Vec<Obj>,
    /// `visual effect …`: the effect for the next `go` in this run.
    pub visual: Option<String>,
    /// HyperCard's tool: "browse", "button", "field", or a paint tool ("pencil", "round rect", …).
    pub tool: String,
    /// Paint settings: line width for lines and shapes, and whether shapes are filled.
    pub line_size: i64,
    pub filled: bool,
    /// The menu bar: built-in menus first, then those scripts create. Not saved, as in HyperCard.
    pub menus: Vec<Menu>,
    /// Cards visited, for `go back` (unless lockRecent).
    pub recent: Vec<u32>,
    pub lock_recent: bool,
    /// The player's window: viewport size and screen position (for the screenRect, sizeWindow, moveWindow).
    pub screen: (f64, f64),
    pub window_loc: (f64, f64),
    /// Requests for the player from the built-in menu items: Undo, focus the message box, quit.
    pub undo_request: bool,
    pub message_request: bool,
    pub quit_request: bool,
    /// `hide message box` / `show message box`.
    pub message_visible: bool,
}

/// HyperTalk property names are case-insensitive, and some have HyperTalk spellings for ours.
pub fn canon_prop(p: &str) -> String {
    match p.to_lowercase().as_str() {
        "locktext" | "locked" => "locked",
        "sharedtext" => "sharedText",
        "showname" => "showName",
        "dontwrap" => "dontWrap",
        "itemdelimiter" => "itemDelimiter",
        "lockscreen" => "lockScreen",
        "lockmessages" => "lockMessages",
        "name" => "name", "id" => "id", "visible" => "visible", "style" => "style", "left" => "left", "top" => "top",
        "width" => "width", "height" => "height", "picture" => "picture", "text" => "text", "enabled" => "enabled",
        "hilite" | "highlight" => "hilite", "marked" => "marked", "rect" | "rectangle" => "rect", "loc" | "location" => "loc",
        "owner" => "owner", "number" => "number", "script" => "script", "family" => "family", "dontsearch" => "dontSearch", "searchpaths" => "searchPaths", "autohilite" | "autoHilite" => "autoHilite", "autoselect" => "autoSelect", "linesize" => "lineSize", "filled" => "filled", "lockrecent" => "lockRecent", "icon" => "icon", "showpict" => "showPict", "userlevel" => "userLevel",
        _ => return p.to_string(),
    }.to_string()
}

/// Id of the Home stack object. Stack files number their objects from 1.
pub const HOME: u32 = 0;
/// Ids of library stacks (`start using`): LIB0, LIB0 + 1, … in the order they were started.
pub const LIB0: u32 = 0xF000_0000;
/// Ids of other stacks a script names (`the script of stack "x"`, `send … to stack "x"`): loaded once,
/// read from their files, not saved. Above LIB0, so like libraries they stay out of the journal.
pub const VISIT0: u32 = 0xF800_0000;

/// A menu in the menu bar and its items ("-" is a separator line).
#[derive(Clone, Debug, PartialEq)]
pub struct Menu { pub name: String, pub items: Vec<MenuItem> }

/// A menu item: its text, the message it sends instead of `doMenu` (if any), and its state.
#[derive(Clone, Debug, PartialEq)]
pub struct MenuItem { pub text: String, pub message: String, pub enabled: bool, pub checked: bool }

impl MenuItem { pub fn new(text: &str) -> Self { MenuItem { text: text.into(), message: String::new(), enabled: true, checked: false } } }

/// HyperCard's menus, as far as this player has them. Choosing an item sends `doMenu <item>, <menu>`;
/// when no handler takes it, the item does its built-in action.
pub fn default_menus() -> Vec<Menu> {
    let m = |name: &str, items: &[&str]| Menu { name: name.into(), items: items.iter().map(|t| MenuItem::new(t)).collect() };
    vec![
        m("File", &["New Stack…", "-", "Quit HyperCard"]),
        m("Edit", &["Undo", "-", "New Card", "Delete Card"]),
        m("Go", &["Back", "First", "Prev", "Next", "Last", "-", "Find…", "Message"]),
        m("Tools", &["Browse", "Button", "Field", "-", "Pencil", "Brush", "Eraser", "Spray", "Line", "Curve", "Rectangle", "Round Rect", "Oval", "Regular Polygon", "Polygon"]),
    ]
}

/// The paint tools, in HyperCard's order (tool 6 to 18), and the user level they need.
pub const TOOLS: [&str; 18] = ["browse", "button", "field", "select", "lasso", "pencil", "brush", "eraser", "line", "spray",
    "rectangle", "round rect", "bucket", "oval", "curve", "text", "regular polygon", "polygon"];

/// A request for the player: show another stack (`go to stack …`), in this tab or a new one.
#[derive(Clone, Debug, PartialEq)]
pub struct GoStack { pub path: String, pub card: Option<Value>, pub new_window: bool, pub effect: Option<String> }

impl World {
    pub fn new(file: StackFile) -> Self { World { file, log: vec![], dialog: None, last_error: None, auto_depth: 0, mouse: (0.0, 0.0), keys: Default::default(), draw: Default::default(), home: None, sound: None, sound_cmds: vec![], effect: None, effect_seq: 0, locked: false,
        item_delim: ",".into(), lock_messages: false, result: String::new(), mouse_down: false, click_loc: (0.0, 0.0),
        dialog_buttons: vec![], dialog_ask: None, timers: vec![], push_list: vec![], edit_request: None, hover: None, pressed: None,
        selection: None, select_request: None, focused: None, user_level: 5, tz_offset_min: 0, mouse_click: false, click_text_at: None,
        fs: std::rc::Rc::new(std::cell::RefCell::new(crate::files::default_fs())), fs_base: String::new(), open_files: Default::default(),
        dialog_file: None, url_requests: vec![], url_cache: Default::default(),
        libs: vec![], go_stack: None, stacks_dir: String::new(), open_stacks: vec![], journal_floor: 0, floor_merges: vec![], visited: vec![], visual: None,
        tool: "browse".into(), line_size: 1, filled: false, menus: default_menus(), recent: vec![], lock_recent: false,
        screen: (0.0, 0.0), window_loc: (0.0, 0.0), undo_request: false, message_request: false, quit_request: false, message_visible: true } }

    /// Find a stack file by name or path: the stack's own folder, then the stacks folder, then the search
    /// folders kept in Home (`the searchPaths`, one per line). `Name` also matches `Name.stack`.
    /// The built-in demo stacks are found by name as `demo:<name>`.
    pub fn find_stack(&self, name: &str) -> Option<String> {
        let name = name.trim();
        let mut dirs = vec![self.fs_base.clone(), self.stacks_dir.clone()];
        if let Some(h) = &self.home { if let Some(v) = h.props.get("searchPaths") { dirs.extend(v.text().lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty())); } }
        let fs = self.fs.borrow();
        for d in dirs.iter().filter(|d| !d.is_empty()).chain(std::iter::once(&String::new())) {
            for cand in [name.to_string(), format!("{name}.stack")] {
                let p = crate::files::resolve_path(d, &cand);
                if fs.is_file(&p) { return Some(p); }
            }
        }
        crate::demo::by_name(name).map(|_| format!("demo:{name}"))
    }

    /// A script's path, relative to the stack's folder unless absolute.
    pub fn path(&self, p: &str) -> String { crate::files::resolve_path(&self.fs_base, p) }

    /// Put the cards in a new order (journaled; Undo restores the old order).
    pub fn reorder_cards(&mut self, order: &[u32]) {
        let old: Vec<u32> = self.cards();
        if old.len() != order.len() || old == order { return; }
        let slots: Vec<usize> = self.file.objs.iter().enumerate().filter(|(_, o)| o.kind == Kind::Card).map(|(i, _)| i).collect();
        let by_id: std::collections::HashMap<u32, Obj> = self.file.objs.iter().filter(|o| o.kind == Kind::Card).map(|o| (o.id, o.clone())).collect();
        for (slot, id) in slots.iter().zip(order) { self.file.objs[*slot] = by_id[id].clone(); }
        let key = old.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
        let auto = self.auto_depth > 0;
        self.file.journal.push(Change { txn: self.file.txn, kind: "order".into(), obj: 0, key, old: None, new: None, index: None, snapshot: None, auto });
    }

    pub fn set_effect(&mut self, name: &str) {
        self.effect = Some(name.to_string());
        self.effect_seq += 1;
    }

    /// The next stop on the message path: the parent, or Home after the stack.
    pub fn next_on_path(&self, id: u32) -> Option<u32> {
        let o = self.obj(id)?;
        match o.parent {
            // A background part's messages go to the current card first, as in HyperCard:
            // background button → card → background → stack.
            Some(p) if matches!(o.kind, Kind::Button | Kind::Field | Kind::Canvas | Kind::Picture) && self.obj(p).map(|b| b.kind) == Some(Kind::Background) => Some(self.current_card()),
            Some(p) => Some(p),
            // After the stack: the libraries in use, in order, then Home.
            // Another stack's script: then Home.
            None if id >= VISIT0 => self.home.as_ref().map(|_| HOME),
            None if o.kind == Kind::Stack && id != HOME => {
                let next_lib = if id >= LIB0 { (id - LIB0 + 1) as usize } else { 0 };
                if next_lib < self.libs.len() { Some(LIB0 + next_lib as u32) } else if self.home.is_some() { Some(HOME) } else { None }
            }
            None => None,
        }
    }

    pub fn obj(&self, id: u32) -> Option<&Obj> {
        if id == HOME { self.home.as_ref() } else if id >= VISIT0 { self.visited.get((id - VISIT0) as usize) } else if id >= LIB0 { self.libs.get((id - LIB0) as usize) } else { self.file.objs.iter().find(|o| o.id == id) }
    }
    fn obj_mut(&mut self, id: u32) -> Option<&mut Obj> {
        if id == HOME { self.home.as_mut() } else if id >= VISIT0 { self.visited.get_mut((id - VISIT0) as usize) } else if id >= LIB0 { self.libs.get_mut((id - LIB0) as usize) } else { self.file.objs.iter_mut().find(|o| o.id == id) }
    }
    pub fn stack_id(&self) -> u32 { self.file.objs.iter().find(|o| o.kind == Kind::Stack).map(|o| o.id).unwrap_or(0) }
    pub fn cards(&self) -> Vec<u32> { self.file.objs.iter().filter(|o| o.kind == Kind::Card).map(|o| o.id).collect() }
    pub fn current_card(&self) -> u32 { self.file.current_card }
    pub fn card_number(&self, id: u32) -> usize { self.cards().iter().position(|c| *c == id).map(|i| i + 1).unwrap_or(0) }
    pub fn background_of(&self, card: u32) -> Option<u32> { self.obj(card).and_then(|o| o.parent) }

    /// Background parts first, then the card's own parts: drawing order.
    pub fn parts_on(&self, card: u32) -> Vec<u32> {
        let bg = self.background_of(card);
        let mut v: Vec<u32> = self.file.objs.iter().filter(|o| bg.is_some() && o.parent == bg && is_part(o)).map(|o| o.id).collect();
        v.extend(self.file.objs.iter().filter(|o| o.parent == Some(card) && is_part(o)).map(|o| o.id));
        v
    }

    /// Find a part on the current card by name (case-insensitive) or by number within its kind.
    pub fn find_part(&self, kind: Kind, key: &Value) -> Option<u32> {
        let parts: Vec<&Obj> = self.parts_on(self.current_card()).into_iter().filter_map(|id| self.obj(id)).filter(|o| o.kind == kind).collect();
        match key {
            Value::Int(n) => parts.get((*n as usize).wrapping_sub(1)).map(|o| o.id),
            other => { let name = other.text().to_lowercase(); parts.iter().rev().find(|o| o.name.to_lowercase() == name).map(|o| o.id) }
        }
    }

    /// A background field keeps separate text on every card unless `sharedText` is true.
    fn text_slot(&self, id: u32) -> (u32, String) {
        if let Some(o) = self.obj(id) {
            let on_bg = o.parent.and_then(|p| self.obj(p)).map(|p| p.kind == Kind::Background).unwrap_or(false);
            let shared = o.props.get("sharedText").map(|v| v.truthy()).unwrap_or(false);
            if o.kind == Kind::Field && on_bg && !shared { return (self.current_card(), format!("~{id}")); }
        }
        (id, "text".into())
    }

    pub fn get(&self, id: u32, key: &str) -> Value {
        let Some(o) = self.obj(id) else { return Value::Str(String::new()) };
        match key {
            "name" => Value::Str(o.name.clone()),
            "id" => Value::Int(id as i64),
            "script" => Value::Str(o.script.clone()),
            "number" => Value::Int(match o.kind {
                Kind::Card => self.card_number(id) as i64,
                _ => self.file.objs.iter().filter(|x| x.kind == o.kind && x.parent == o.parent).position(|x| x.id == id).map(|i| i as i64 + 1).unwrap_or(0),
            }),
            "text" => { let (oid, k) = self.text_slot(id); self.obj(oid).and_then(|x| x.props.get(&k)).cloned().unwrap_or(Value::Str(String::new())) }
            "rect" => { let r = ["left", "top", "width", "height"].map(|k| self.get(id, k).int()); Value::Str(format!("{},{},{},{}", r[0], r[1], r[0] + r[2], r[1] + r[3])) }
            "loc" => { let r = ["left", "top", "width", "height"].map(|k| self.get(id, k).int()); Value::Str(format!("{},{}", r[0] + r[2] / 2, r[1] + r[3] / 2)) }
            "owner" => o.parent.and_then(|p| self.obj(p)).map(|p| Value::Str(p.name.clone())).unwrap_or(Value::Str(String::new())),
            "visible" | "enabled" | "showName" | "autoHilite" | "showPict" => o.props.get(key).cloned().unwrap_or(Value::Bool(true)),
            "hilite" | "marked" | "locked" | "sharedText" | "dontWrap" | "dontSearch" => o.props.get(key).cloned().unwrap_or(Value::Bool(false)),
            _ => o.props.get(key).cloned().unwrap_or(Value::Str(String::new())),
        }
    }

    /// Start a new undo group. Call once per user action or top-level message.
    pub fn begin(&mut self) { self.file.txn += 1; }

    /// Journaled property write. `merge` folds the write into the previous entry for the same key (typing, dragging).
    pub fn set(&mut self, id: u32, key: &str, new: Value, merge: bool) {
        let (oid, k) = if key == "text" { self.text_slot(id) } else { (id, key.to_string()) };
        let old = self.raw_get(oid, &k);
        if old.as_ref() == Some(&new) { return; }
        self.raw_set(oid, &k, Some(new.clone()));
        // Home belongs to no stack: its edits stay out of this stack's journal (and out of Undo, for now).
        if oid == HOME || oid >= LIB0 { return; }
        let txn = self.file.txn;
        let auto = self.auto_depth > 0;
        // Typing and dragging fold into one entry; so do repeated automatic writes (a score updated every frame).
        if merge || auto {
            let below_floor = self.file.journal.len() <= self.journal_floor;
            if let Some(last) = self.file.journal.last_mut() {
                if last.kind == "set" && last.obj == oid && last.key == k && last.auto == auto {
                    if below_floor { let i = self.file.journal.len() - 1; let prev = self.file.journal[i].new.clone(); self.floor_merges.push((i, prev)); self.file.journal[i].new = Some(new); return; }
                    last.new = Some(new); return;
                }
            }
        }
        self.file.journal.push(Change { txn, kind: "set".into(), obj: oid, key: k, old, new: Some(new), index: None, snapshot: None, auto });
    }

    fn raw_get(&self, id: u32, key: &str) -> Option<Value> {
        let o = self.obj(id)?;
        match key { "name" => Some(Value::Str(o.name.clone())), "script" => Some(Value::Str(o.script.clone())), _ => o.props.get(key).cloned() }
    }
    fn raw_set(&mut self, id: u32, key: &str, v: Option<Value>) {
        let Some(o) = self.obj_mut(id) else { return };
        match (key, v) {
            ("name", Some(v)) => o.name = v.text(),
            ("script", Some(v)) => o.script = v.text(),
            (_, Some(v)) => { o.props.insert(key.into(), v); }
            (_, None) => { o.props.remove(key); }
        }
    }

    pub fn create(&mut self, kind: Kind, name: &str, parent: Option<u32>, after: Option<u32>) -> u32 {
        let id = self.file.next_id;
        self.file.next_id += 1;
        let obj = Obj { id, kind, name: name.into(), parent, script: String::new(), props: BTreeMap::new() };
        let index = after.and_then(|a| self.file.objs.iter().position(|o| o.id == a)).map(|i| i + 1).unwrap_or(self.file.objs.len());
        self.file.objs.insert(index, obj.clone());
        let auto = self.auto_depth > 0;
        self.file.journal.push(Change { txn: self.file.txn, kind: "create".into(), obj: id, key: String::new(), old: None, new: None, index: Some(index), snapshot: Some(obj), auto });
        id
    }

    /// Delete an object and everything whose message path runs through it.
    pub fn delete(&mut self, id: u32) {
        let children: Vec<u32> = self.file.objs.iter().filter(|o| o.parent == Some(id)).map(|o| o.id).collect();
        for c in children { self.delete(c); }
        if let Some(index) = self.file.objs.iter().position(|o| o.id == id) {
            let obj = self.file.objs.remove(index);
            let auto = self.auto_depth > 0;
            self.file.journal.push(Change { txn: self.file.txn, kind: "delete".into(), obj: id, key: String::new(), old: None, new: None, index: Some(index), snapshot: Some(obj), auto });
        }
        if self.file.current_card == id { self.file.current_card = self.cards().first().copied().unwrap_or(0); }
    }

    /// A new card after the current one, named, on `background` (None: a new background).
    pub fn new_card_named(&mut self, name: &str, background: Option<u32>) -> u32 {
        let bg = match background { Some(b) => b, None => self.new_background() };
        let cur = self.current_card();
        let name = if name.trim().is_empty() { format!("card {}", self.cards().len() + 1) } else { name.trim().to_string() };
        self.create(Kind::Card, &name, Some(bg), Some(cur))
    }

    pub fn backgrounds(&self) -> Vec<u32> { self.file.objs.iter().filter(|o| o.kind == Kind::Background).map(|o| o.id).collect() }

    pub fn new_background(&mut self) -> u32 {
        let n = self.backgrounds().len() + 1;
        let stack = self.stack_id();
        let last = self.backgrounds().last().copied();
        self.create(Kind::Background, &format!("background {n}"), Some(stack), last)
    }

    /// Move a card to another background (journaled; Undo moves it back).
    pub fn set_background(&mut self, card: u32, bg: u32) {
        let Some(old) = self.obj(card).and_then(|o| o.parent) else { return };
        if old == bg || self.obj(bg).map(|o| o.kind) != Some(Kind::Background) { return; }
        if let Some(o) = self.obj_mut(card) { o.parent = Some(bg); }
        let auto = self.auto_depth > 0;
        self.file.journal.push(Change { txn: self.file.txn, kind: "parent".into(), obj: card, key: String::new(), old: Some(Value::Int(old as i64)), new: Some(Value::Int(bg as i64)), index: None, snapshot: None, auto });
    }

    /// Copy a part (offset a little) or a card (with its own parts, placed after it).
    /// A name ending in a number gets the next free number: "pillar 1" → "pillar 2".
    pub fn duplicate(&mut self, id: u32) -> Option<u32> {
        let src = self.obj(id)?.clone();
        let name = self.next_name(&src.name, src.kind, src.parent);
        let copy = match src.kind {
            Kind::Button | Kind::Field | Kind::Canvas | Kind::Picture => {
                let c = self.create(src.kind, &name, src.parent, Some(id));
                for (k, v) in src.props.iter() {
                    let v = match k.as_str() { "left" | "top" => Value::Int(v.int() + 12), _ => v.clone() };
                    self.set(c, k, v, false);
                }
                if !src.script.is_empty() { self.set(c, "script", Value::Str(src.script.clone()), false); }
                c
            }
            Kind::Card => {
                let c = self.create(Kind::Card, &name, src.parent, Some(id));
                for (k, v) in src.props.iter() {
                    // Background field text stored on the card is copied too.
                    if let Some(o) = self.obj_mut(c) { o.props.insert(k.clone(), v.clone()); }
                }
                if !src.script.is_empty() { self.set(c, "script", Value::Str(src.script.clone()), false); }
                let parts: Vec<Obj> = self.file.objs.iter().filter(|o| o.parent == Some(id)).cloned().collect();
                for p in parts {
                    let np = self.create(p.kind, &p.name, Some(c), None);
                    for (k, v) in p.props.iter() { self.set(np, k, v.clone(), false); }
                    if !p.script.is_empty() { self.set(np, "script", Value::Str(p.script.clone()), false); }
                }
                c
            }
            _ => return None,
        };
        Some(copy)
    }


    fn next_name(&self, name: &str, kind: Kind, parent: Option<u32>) -> String {
        let (stem, num) = match name.rsplit_once(' ') {
            Some((a, b)) if b.chars().all(|c| c.is_ascii_digit()) && !b.is_empty() => (a.to_string(), b.parse::<i64>().unwrap_or(0)),
            _ => return name.to_string(),
        };
        let taken = |n: i64| self.file.objs.iter().any(|o| o.kind == kind && o.parent == parent && o.name.eq_ignore_ascii_case(&format!("{stem} {n}")));
        let mut n = num + 1;
        while taken(n) { n += 1; }
        format!("{stem} {n}")
    }

    /// Line up or size several parts. The first id is the reference.
    /// `how`: left, right, top, bottom, width, height, size, across (even gaps left to right), down.
    pub fn arrange(&mut self, ids: &[u32], how: &str) {
        let Some(&first) = ids.first() else { return };
        let r = |w: &World, id: u32| -> [i64; 4] { ["left", "top", "width", "height"].map(|k| w.get(id, k).int()) };
        let rf = r(self, first);
        let mut plan: Vec<(u32, &str, i64)> = vec![];
        match how {
            "left" => for &id in ids { plan.push((id, "left", rf[0])) },
            "top" => for &id in ids { plan.push((id, "top", rf[1])) },
            "right" => for &id in ids { let w = r(self, id)[2]; plan.push((id, "left", rf[0] + rf[2] - w)) },
            "bottom" => for &id in ids { let h = r(self, id)[3]; plan.push((id, "top", rf[1] + rf[3] - h)) },
            "width" => for &id in ids { plan.push((id, "width", rf[2])) },
            "height" => for &id in ids { plan.push((id, "height", rf[3])) },
            "size" => for &id in ids { plan.push((id, "width", rf[2])); plan.push((id, "height", rf[3])) },
            "across" | "down" => {
                let (pos, len) = if how == "across" { (0, 2) } else { (1, 3) };
                let mut v: Vec<(u32, [i64; 4])> = ids.iter().map(|&id| (id, r(self, id))).collect();
                v.sort_by_key(|(_, x)| x[pos]);
                if v.len() < 3 { return; }
                let start = v[0].1[pos];
                let end = v[v.len() - 1].1[pos] + v[v.len() - 1].1[len];
                let total: i64 = v.iter().map(|(_, x)| x[len]).sum();
                let gap = (end - start - total) as f64 / (v.len() - 1) as f64;
                let mut at = start as f64;
                for (id, x) in v { plan.push((id, if how == "across" { "left" } else { "top" }, at.round() as i64)); at += x[len] as f64 + gap; }
            }
            _ => {}
        }
        for (id, k, v) in plan { self.set(id, k, Value::Int(v), false); }
    }

    /// Add a picture or sound to the stack. Journaled: Undo removes it again.
    pub fn add_asset(&mut self, name: &str, mime: &str, bytes: Vec<u8>) {
        let a = self.file.add_asset(name, mime, bytes);
        let auto = self.auto_depth > 0;
        self.file.journal.push(Change { txn: self.file.txn, kind: "asset".into(), obj: 0, key: a.name, old: None, new: None, index: None, snapshot: None, auto });
    }

    pub fn new_card(&mut self) -> u32 {
        let cur = self.current_card();
        let bg = self.background_of(cur);
        let n = self.cards().len() + 1;
        self.create(Kind::Card, &format!("card {n}"), bg, Some(cur))
    }

    /// Undo the most recent user action: the newest group that holds a non-automatic change.
    /// Automatic changes (written by openCard, frame, ...) stay in the journal and are not reverted.
    /// Returns the ids whose scripts changed and must be recompiled.
    pub fn undo(&mut self) -> Vec<u32> {
        let mut recompile = vec![];
        let Some(txn) = self.file.journal.iter().rev().find(|c| !c.auto).map(|c| c.txn) else { return recompile };
        let mut taken = vec![];
        let mut i = self.file.journal.len();
        while i > 0 {
            i -= 1;
            let c = &self.file.journal[i];
            if c.txn < txn { break; }
            if c.txn == txn && !c.auto { taken.push(self.file.journal.remove(i)); }
        }
        for c in taken { self.revert(c, &mut recompile); }
        if self.obj(self.file.current_card).is_none() { self.file.current_card = self.cards().first().copied().unwrap_or(0); }
        recompile
    }

    /// Take back every journal entry from `mark` on, automatic or not, newest first (a paused script's
    /// changes, before it runs again). Returns the ids whose scripts changed.
    pub fn rollback(&mut self, mark: usize) -> Vec<u32> {
        let mut recompile = vec![];
        while self.file.journal.len() > mark {
            let c = self.file.journal.pop().unwrap();
            self.revert(c, &mut recompile);
        }
        for (i, prev) in std::mem::take(&mut self.floor_merges).into_iter().rev() {
            let Some(c) = self.file.journal.get_mut(i) else { continue };
            c.new = prev.clone();
            let (obj, key) = (c.obj, c.key.clone());
            self.raw_set(obj, &key, prev);
            if key == "script" { recompile.push(obj); }
        }
        recompile
    }

    fn revert(&mut self, c: Change, recompile: &mut Vec<u32>) {
        match c.kind.as_str() {
            "set" => { self.raw_set(c.obj, &c.key, c.old); if c.key == "script" { recompile.push(c.obj); } }
            "create" => { self.file.objs.retain(|o| o.id != c.obj); recompile.push(c.obj); }
            "delete" => if let Some(o) = c.snapshot { let i = c.index.unwrap_or(0).min(self.file.objs.len()); self.file.objs.insert(i, o); recompile.push(c.obj); }
            "parent" => { let old = c.old.map(|v| v.int() as u32); if let Some(o) = self.obj_mut(c.obj) { o.parent = old; } }
            "asset" => { self.file.assets.retain(|a| a.name != c.key); self.file.blobs.remove(&c.key); }
            "order" => {
                let old: Vec<u32> = c.key.split(',').filter_map(|x| x.parse().ok()).collect();
                let slots: Vec<usize> = self.file.objs.iter().enumerate().filter(|(_, o)| o.kind == Kind::Card).map(|(i, _)| i).collect();
                let by_id: std::collections::HashMap<u32, Obj> = self.file.objs.iter().filter(|o| o.kind == Kind::Card).map(|o| (o.id, o.clone())).collect();
                if old.len() == slots.len() && old.iter().all(|i| by_id.contains_key(i)) {
                    for (slot, id) in slots.iter().zip(&old) { self.file.objs[*slot] = by_id[id].clone(); }
                }
            }
            _ => {}
        }
    }
}

fn is_part(o: &Obj) -> bool { matches!(o.kind, Kind::Button | Kind::Field | Kind::Canvas | Kind::Picture) }

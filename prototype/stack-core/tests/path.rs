use stack_core::{demo, Kind, Rt, StackFile, Value, Waiting};

fn part(rt: &Rt, kind: Kind, name: &str) -> u32 { rt.w.borrow().find_part(kind, &Value::Str(name.into())).unwrap() }
fn field(rt: &Rt, name: &str) -> String { let id = part(rt, Kind::Field, name); rt.w.borrow().get(id, "text").text() }
#[track_caller]
fn no_error(rt: &Rt) { assert_eq!(rt.w.borrow().last_error, None); }

#[test]
fn file_round_trips() {
    let f = demo::address_book();
    let text = f.to_text();
    let back = StackFile::from_text(&text).expect("parses");
    assert_eq!(back.to_text(), text);
    assert!(text.contains("on mouseUp {"), "scripts are readable in the file");
}

#[test]
fn navigation_and_per_card_text() {
    let rt = Rt::new(demo::address_book());
    assert_eq!(field(&rt, "name"), "Ada Lovelace");
    rt.send(part(&rt, Kind::Button, "Next"), "mouseUp"); no_error(&rt);
    assert_eq!(field(&rt, "name"), "Bill Atkinson");
    assert_eq!(field(&rt, "status"), "Card 2", "stack-level openCard handler ran");
    rt.send(part(&rt, Kind::Button, "Prev"), "mouseUp");
    assert_eq!(field(&rt, "name"), "Ada Lovelace");
}

#[test]
fn answer_find_new_and_undo() {
    let rt = Rt::new(demo::address_book());
    rt.send(part(&rt, Kind::Button, "Hello"), "mouseUp"); no_error(&rt);
    assert_eq!(rt.w.borrow().dialog.as_deref(), Some("Hello, Ada"));
    rt.reply("OK");

    let s = part(&rt, Kind::Field, "search");
    rt.w.borrow_mut().set(s, "text", "nanoseconds".into(), false);
    rt.send(part(&rt, Kind::Button, "Find"), "mouseUp"); no_error(&rt);
    assert_eq!(field(&rt, "name"), "Grace Hopper");

    rt.send(part(&rt, Kind::Button, "New"), "mouseUp"); no_error(&rt);
    assert_eq!(rt.w.borrow().cards().len(), 4);
    assert_eq!(field(&rt, "name"), "");
    rt.undo();
    assert_eq!(rt.w.borrow().cards().len(), 3);
}

#[test]
fn live_edit_intercept_and_capability() {
    let rt = Rt::new(demo::address_book());
    let stack = rt.w.borrow().stack_id();
    // Intercept the built-in `answer` at stack level, without touching the button.
    rt.set_script(stack, "on answer msg { put \"intercepted: \" & msg into field \"notes\"; }");
    assert_eq!(rt.script_error(stack), None);
    rt.send(part(&rt, Kind::Button, "Hello"), "mouseUp"); no_error(&rt);
    assert_eq!(rt.w.borrow().dialog, None);
    assert_eq!(field(&rt, "notes"), "intercepted: Hello, Ada");
    // A broken edit keeps the last working script live.
    rt.set_script(stack, "on answer msg { put ");
    assert!(rt.script_error(stack).is_some());
    let notes = part(&rt, Kind::Field, "notes");
    rt.w.borrow_mut().set(notes, "text", "".into(), false);
    rt.send(part(&rt, Kind::Button, "Hello"), "mouseUp");
    assert_eq!(field(&rt, "notes"), "intercepted: Hello, Ada");
    // Message box, property read/write, capability gate.
    rt.exec("set text of button \"Hello\" to \"Hi\"; put the text of button \"Hello\" into field \"search\";"); no_error(&rt);
    assert_eq!(field(&rt, "search"), "Hi");
    rt.exec("beep;"); no_error(&rt);
    assert_eq!(rt.w.borrow().log.last().map(String::as_str), Some("BEEP"), "sound is always allowed");
    rt.exec("loop { }");
    assert!(rt.w.borrow().last_error.is_some(), "runaway script is stopped");
}

/// H2 evidence: cost of one live script edit (journal + recompile) and of writing the whole file.
#[test]
fn edit_cost() {
    let rt = Rt::new(demo::address_book());
    let b = part(&rt, Kind::Button, "Hello");
    let n = 2000;
    let t = std::time::Instant::now();
    for i in 0..n { rt.set_script(b, &format!("on mouseUp {{\n    answer \"v{i} \" & word 1 of field \"name\";\n}}")); }
    let edit = t.elapsed().as_secs_f64() * 1e6 / n as f64;
    let t = std::time::Instant::now();
    for _ in 0..200 { std::hint::black_box(rt.text()); }
    let ser = t.elapsed().as_secs_f64() * 1e6 / 200.0;
    println!("edit_cost: {edit:.1} µs per script edit, {ser:.1} µs to serialize the stack ({} bytes)", rt.text().len());
    assert!(edit < 100_000.0);
}

#[test]
fn undo_skips_automatic_changes() {
    let rt = Rt::new(demo::address_book());
    // A user edit, then navigation whose openCard handler writes the status field.
    let notes = part(&rt, Kind::Field, "notes");
    { let mut w = rt.w.borrow_mut(); w.begin(); w.set(notes, "text", "edited".into(), false); }
    rt.send(part(&rt, Kind::Button, "Next"), "mouseUp");
    rt.send(part(&rt, Kind::Button, "Prev"), "mouseUp");
    assert_eq!(field(&rt, "status"), "Card 1");
    rt.undo();
    assert_eq!(field(&rt, "notes"), "Wrote the first published program.", "undo reverted the user's edit");
    assert_eq!(field(&rt, "status"), "Card 1", "automatic write left alone");
    // Repeated automatic writes do not grow the journal.
    let before = rt.w.borrow().file.journal.len();
    for _ in 0..20 { rt.send(part(&rt, Kind::Button, "Next"), "mouseUp"); }
    assert!(rt.w.borrow().file.journal.len() <= before + 1);
}

#[test]
fn loose_statements_are_rejected() {
    let rt = Rt::new(demo::address_book());
    let b = part(&rt, Kind::Button, "Hello");
    rt.set_script(b, "answer \"loose\";\non mouseUp { answer \"x\"; }");
    let e = rt.script_error(b).expect("rejected");
    assert!(e.contains("only `on"), "{e}");
    rt.set_script(b, "// a comment\non mouseUp { answer \"x\"; }\n\non mouseDown { }");
    assert_eq!(rt.script_error(b), None);
}

#[test]
fn breakout_runs() {
    let rt = Rt::new(demo::breakout());
    let card = rt.w.borrow().current_card();
    rt.send(card, "openCard");
    assert_eq!(rt.script_error(card), None);
    rt.w.borrow_mut().mouse = (260.0, 0.0);
    let n = 2000;
    let t = std::time::Instant::now();
    for _ in 0..n { assert!(rt.frame()); }
    let us = t.elapsed().as_secs_f64() * 1e6 / n as f64;
    let w = rt.w.borrow();
    assert_eq!(w.last_error, None);
    let shapes: usize = w.draw.values().map(|d| d.len()).sum();
    let journal = w.file.journal.len();
    drop(w);
    println!("breakout: {us:.1} µs per frame, {shapes} shapes, journal {journal} entries, {}", field(&rt, "score"));
    assert!(shapes >= 2);
    assert!(shapes < 53, "bricks that were hit disappear (50 bricks + paddle + ball + text = 53)");
    assert!(journal < 10, "frame writes merge");
}

#[test]
fn text_coerces_to_numbers() {
    let rt = Rt::new(demo::address_book());
    rt.exec(r#"put "41" into field "search"; put field "search" + 1 into field "notes";"#); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "42");
    for (src, want) in [(r#""5" + "3""#, "8"), (r#""Score " & 3"#, "Score 3"), (r#"1 & 2"#, "12"), (r#""a" & true"#, "atrue"), (r#""2.5" * 2"#, "5.0"), (r#""7" / 2"#, "3.5"),
                        (r#""8" / 2"#, "4"), (r#""10" > 9"#, "true"), (r#""10" > "9""#, "true"), (r#""abc" < "abd""#, "true"), (r#""5" == 5"#, "true")] {
        rt.exec(&format!("put {src} into field \"notes\";")); no_error(&rt);
        assert_eq!(field(&rt, "notes"), want, "{src}");
    }
    rt.exec(r#"put "apples" - 1 into field "notes";"#);
    assert!(rt.w.borrow().last_error.as_deref().unwrap_or("").contains("expected a number"));
    rt.exec(r#"put "Score " + 3 into field "notes";"#);
    assert!(rt.w.borrow().last_error.as_deref().unwrap_or("").contains("join text with &"), "+ no longer joins");
}

#[test]
fn home_is_last_on_the_path() {
    let rt = Rt::with_home(demo::address_book(), Some(demo::home()));
    rt.exec(r#"send "help";"#); no_error(&rt);
    assert_eq!(rt.w.borrow().dialog.as_deref(), Some("Messages travel: part, card, background, stack, Home."));
    rt.reply("OK");
    // The stack's own handler wins over Home's.
    let stack = rt.w.borrow().stack_id();
    rt.set_script(stack, "on help { answer \"stack help\"; }");
    rt.exec(r#"send "help";"#);
    assert_eq!(rt.w.borrow().dialog.as_deref(), Some("stack help"));
    rt.reply("OK");
    // A live edit of Home's script applies to every stack, and Home round-trips as its own file.
    rt.set_script(stack, "");
    rt.set_script(stack_core::HOME, "on help { answer \"edited home\"; }");
    rt.exec(r#"send "help";"#);
    assert_eq!(rt.w.borrow().dialog.as_deref(), Some("edited home"));
    let home = StackFile::from_text(&rt.home_text().unwrap()).unwrap();
    assert_eq!(home.objs[0].script, "on help { answer \"edited home\"; }");
    assert!(!rt.text().contains("edited home"), "Home is not written into the stack file");
}

#[test]
fn two_canvases_on_one_card() {
    let rt = Rt::new(demo::breakout());
    let card = rt.w.borrow().current_card();
    let radar = { let mut w = rt.w.borrow_mut(); w.begin(); let id = w.create(Kind::Canvas, "radar", Some(card), None); id };
    rt.set_script(card, &format!("{}\non mouseUp {{ with canvas \"radar\" {{ clear(); circle(10, 10, 4, \"green\"); }} }}", demo::BREAKOUT));
    assert_eq!(rt.script_error(card), None);
    rt.send(card, "openCard");
    rt.frame();
    rt.send(card, "mouseUp"); no_error(&rt);
    let w = rt.w.borrow();
    let play = w.find_part(Kind::Canvas, &Value::Str("playfield".into())).unwrap();
    assert_eq!(w.draw.get(&radar).map(|d| d.len()), Some(1));
    assert!(w.draw.get(&play).map(|d| d.len()).unwrap_or(0) >= 50, "default target is still the first canvas");
}


#[test]
fn chunk_writes() {
    let rt = Rt::new(demo::address_book());
    let run = |src: &str| { rt.exec(src); no_error(&rt); field(&rt, "notes") };
    assert_eq!(run(r#"put "a,b,c" into x; put "B" into item 2 of x; put x into field "notes";"#), "a,B,c");
    assert_eq!(run(r#"put "" into x; put "on" into item 3 of x; put x into field "notes";"#), ",,on", "missing items are added");
    assert_eq!(run(r#"put "one two  three" into field "notes"; put "2" into word 2 of field "notes";"#), "one 2  three", "spacing kept");
    assert_eq!(run(r#"put "x" into word 5 of field "notes";"#), "one 2  three x");
    assert_eq!(run(r#"put "l1\nk,v\nl3" into field "notes"; put "V" into item 2 of line 2 of field "notes";"#), "l1\nk,V\nl3", "nested");
    assert_eq!(run(r#"put "!" after field "notes"; put ">" before line 1 of field "notes";"#), ">l1\nk,V\nl3!");
    assert_eq!(run(r#"put "abc" into x; put "Z" into char 2 of x; put x into field "notes";"#), "aZc");
    // A bare variable ends the chunk expression; a comparison after it applies to the chunk.
    assert_eq!(run(r#"put "pillar 3" into name; if word 1 of name == "pillar" { put "yes" into field "notes"; }"#), "yes");
    assert_eq!(run(r#"put "off,on" into p; let n = 2; if item n of p == "on" { put "off" into item n of p; } put p into field "notes";"#), "off,off");
}

#[test]
fn stacks_from_format_1_are_migrated() {
    let script = "on mouseUp {\n    answer \"Hello, \" + word 1 of field \"name\" + \"!\";\n    put 1 + 2 into x;\n    x += 1;\n}";
    assert_eq!(stack_core::migrate_joins(script), "on mouseUp {\n    answer \"Hello, \" & word 1 of field \"name\" & \"!\";\n    put 1 + 2 into x;\n    x += 1;\n}");
    let mut old = demo::address_book();
    old.format = 1;
    for o in old.objs.iter_mut() { o.script = o.script.replace(" & ", " + "); }
    let f = StackFile::from_text(&old.to_text()).unwrap();
    assert_eq!(f.format, stack_core::FORMAT);
    let rt = Rt::new(f);
    rt.send(part(&rt, Kind::Button, "Hello"), "mouseUp"); no_error(&rt);
    assert_eq!(rt.w.borrow().dialog.as_deref(), Some("Hello, Ada"));
    rt.reply("OK");
}

#[test]
fn zip_container_round_trips_with_assets() {
    let mut f = demo::address_book();
    f.add_asset("click", "audio/wav", vec![1, 2, 3, 4]);
    f.add_asset("Shore view", "image/svg+xml", b"<svg/>".to_vec());
    let bytes = f.to_zip();
    assert!(bytes.starts_with(b"PK"));
    let back = StackFile::from_bytes(&bytes).unwrap();
    assert_eq!(back.blobs.get("click"), Some(&vec![1, 2, 3, 4]));
    assert_eq!(back.asset("shore view").map(|a| a.file.as_str()), Some("assets/pictures/Shore_view.svg"));
    assert_eq!(back.to_text(), f.to_text());
    // The old plain-TOML form still opens.
    assert!(StackFile::from_bytes(f.to_text().as_bytes()).is_ok());
}

#[test]
fn sound_target_visibility_and_effects() {
    let mut f = demo::address_book();
    f.add_asset("click", "audio/wav", vec![0; 8]);
    let rt = Rt::new(f);
    let card = rt.w.borrow().current_card();
    rt.set_script(card, r#"on mouseUp { put the name of target into field "notes"; play "click"; hide target; }
on soundDone { put "done: " & the sound into field "search"; }"#);
    assert_eq!(rt.script_error(card), None);
    let hello = part(&rt, Kind::Button, "Hello");
    rt.set_script(hello, "");
    rt.send(hello, "mouseUp"); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "Hello", "card script sees the button as target");
    assert_eq!(rt.w.borrow().sound.as_deref(), Some("click"));
    assert_eq!(rt.w.borrow().sound_cmds, vec![stack_core::SoundCmd::Play("click".into())]);
    assert_eq!(rt.w.borrow().get(hello, "visible"), Value::Bool(false));
    rt.sound_finished();
    assert_eq!(field(&rt, "search"), "done: done");
    rt.exec(r#"play "nope";"#);
    assert!(rt.w.borrow().last_error.as_deref().unwrap_or("").contains("no sound"));
    rt.exec(r#"show button "Hello"; go next with wipe left;"#); no_error(&rt);
    assert_eq!(rt.w.borrow().effect.as_deref(), Some("wipe-left"));
    rt.exec(r#"lock screen; put "x" into field "notes"; unlock screen with dissolve;"#); no_error(&rt);
    assert_eq!(rt.w.borrow().effect.as_deref(), Some("dissolve"));
    assert_eq!(rt.w.borrow().effect_seq, 2);
}

#[test]
fn pillars_puzzle() {
    let rt = Rt::new(demo::pillars());
    for id in rt.w.borrow().file.objs.iter().map(|o| o.id).collect::<Vec<_>>() { assert_eq!(rt.script_error(id), None, "script of {id}"); }
    let click = |name: &str| { let id = rt.w.borrow().find_part(Kind::Button, &Value::Str(name.into())).unwrap(); rt.send(id, "mouseUp"); no_error(&rt); };
    click("island");
    assert_eq!(rt.w.borrow().effect.as_deref(), Some("dissolve"));
    let lit = |n: u32| { let w = rt.w.borrow(); let id = w.find_part(Kind::Picture, &Value::Str(format!("lit {n}"))).unwrap(); w.get(id, "visible").truthy() };
    click("pillar 3");
    assert!(lit(3));
    click("pillar 3");
    assert!(!lit(3));
    for n in [2, 5, 6] { click(&format!("pillar {n}")); }
    assert_eq!(field(&rt, "status"), "Something heavy moves in the water by the dock.");
    assert_eq!(rt.w.borrow().sound.as_deref(), Some("ship"));
    click("back");
    click("dock");
    assert_eq!(rt.w.borrow().effect.as_deref(), Some("wipe-left"));
    let shown = |name: &str| { let w = rt.w.borrow(); let id = w.find_part(Kind::Picture, &Value::Str(name.into())).unwrap(); w.get(id, "visible").truthy() };
    assert!(shown("ship up") && !shown("ship down"));
    // Saved and reopened, the stack is the same; the puzzle state is in globals, which are not saved.
    let back = StackFile::from_bytes(&rt.bytes()).unwrap();
    assert_eq!(back.blobs.len(), 8);
    println!("pillars.stack: {} bytes", rt.bytes().len());
}

#[test]
fn put_inside_a_block_sets_a_handler_wide_local() {
    let rt = Rt::new(demo::address_book());
    let card = rt.w.borrow().current_card();
    rt.set_script(card, r#"on mouseUp {
    if true { put "inner" into x; }
    put x & "!" into field "notes";
    repeat 2 times { put "a" after y; }
    put y after field "notes";
}"#);
    assert_eq!(rt.script_error(card), None);
    rt.send(card, "mouseUp"); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "inner!aa");
}

#[test]
fn editor_operations() {
    let rt = Rt::new(demo::pillars());
    let pil = rt.w.borrow().cards()[1];
    rt.w.borrow_mut().file.current_card = pil;
    let p1 = part(&rt, Kind::Button, "pillar 1");
    // Duplicate with the next free number; the copy keeps style and is offset.
    let copy = { let mut w = rt.w.borrow_mut(); w.begin(); w.duplicate(p1).unwrap() };
    let (name, style, left) = { let w = rt.w.borrow(); (w.get(copy, "name").text(), w.get(copy, "style").text(), w.get(copy, "left").int()) };
    assert_eq!((name.as_str(), style.as_str()), ("pillar 9", "transparent"));
    assert_eq!(left, rt.w.borrow().get(p1, "left").int() + 12);
    // Align and size several parts to the first one, then spread them evenly.
    let ids: Vec<u32> = (1..=4).map(|n| part(&rt, Kind::Button, &format!("pillar {n}"))).collect();
    { let mut w = rt.w.borrow_mut(); w.begin(); w.set(ids[2], "top", Value::Int(10), false); w.set(ids[3], "width", Value::Int(99), false); }
    { let mut w = rt.w.borrow_mut(); w.begin(); w.arrange(&ids, "top"); w.arrange(&ids, "size"); }
    { let w = rt.w.borrow(); for id in &ids { assert_eq!(w.get(*id, "top").int(), 82); assert_eq!(w.get(*id, "width").int(), 50); } }
    { let mut w = rt.w.borrow_mut(); w.begin(); w.set(ids[1], "left", Value::Int(100), false); w.arrange(&ids, "across"); }
    { let w = rt.w.borrow(); let l: Vec<i64> = ids.iter().map(|id| w.get(*id, "left").int()).collect(); assert_eq!(l, vec![40, 115, 190, 265]); }
    // Duplicate a card: its parts come along. Undo removes card and parts together.
    let before = rt.w.borrow().file.objs.len();
    let card2 = { let mut w = rt.w.borrow_mut(); w.begin(); w.duplicate(pil).unwrap() };
    let n_parts = rt.w.borrow().file.objs.iter().filter(|o| o.parent == Some(card2)).count();
    assert!(n_parts >= 17, "{n_parts}");
    rt.undo();
    assert_eq!(rt.w.borrow().file.objs.len(), before);
    // A named card on a new background, then moved back; Undo reverses the move.
    let (c, bg0) = { let mut w = rt.w.borrow_mut(); w.begin(); let bg0 = w.background_of(pil).unwrap(); (w.new_card_named("cellar", None), bg0) };
    let bg1 = rt.w.borrow().background_of(c).unwrap();
    assert_ne!(bg0, bg1);
    assert_eq!(rt.w.borrow().get(c, "name").text(), "cellar");
    { let mut w = rt.w.borrow_mut(); w.begin(); w.set_background(c, bg0); }
    assert_eq!(rt.w.borrow().background_of(c), Some(bg0));
    rt.undo();
    assert_eq!(rt.w.borrow().background_of(c), Some(bg1));
    // Importing an asset is undoable.
    { let mut w = rt.w.borrow_mut(); w.begin(); w.add_asset("rain", "audio/wav", vec![1, 2]); }
    assert!(rt.w.borrow().file.asset("rain").is_some());
    rt.undo();
    assert!(rt.w.borrow().file.asset("rain").is_none() && !rt.w.borrow().file.blobs.contains_key("rain"));
}

#[test]
fn vocabulary_batch() {
    let rt = Rt::with_home(demo::address_book(), Some(demo::home()));
    let card = rt.w.borrow().current_card();
    let stack = rt.w.borrow().stack_id();
    // Messages the player sends, and messages the runtime sends, reach handlers with their parameters.
    rt.set_script(stack, r#"on startup { put "startup" after field "notes"; }
on openStack { put " openStack" after field "notes"; }
on mouseDown { put " mouseDown" after field "notes"; }
on mouseEnter { put " enter:" & the name of target after field "notes"; }
on keyDown k { put " key:" & k after field "notes"; }
on arrowKey d { put " arrow:" & d & ":" & paramCount() & ":" & param(1) after field "notes"; }
on openField { put " openField" after field "notes"; }
on newCard { put " newCard:" & the number of this card after field "status"; }
on errorDialog e { put "caught: " & word 1 to 3 of e into field "search"; }
on help { put " help" after field "notes"; pass help; }"#);
    assert_eq!(rt.script_error(stack), None);
    { let mut w = rt.w.borrow_mut(); let n = w.find_part(Kind::Field, &Value::Str("notes".into())).unwrap(); w.set(n, "text", "".into(), false); }
    rt.open_stack(true);
    let hello = part(&rt, Kind::Button, "Hello");
    rt.send_with(hello, "mouseDown", vec![]);
    rt.send_with(hello, "mouseEnter", vec![]);
    rt.send_with(card, "keyDown", vec![Value::Str("a".into())]);
    rt.send_with(card, "arrowKey", vec![Value::Str("left".into())]);
    rt.send_with(part(&rt, Kind::Field, "notes"), "openField", vec![]);
    rt.exec(r#"send "help";"#); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "startup openStack mouseDown enter:Hello key:a arrow:left:1:left openField help");
    assert_eq!(rt.w.borrow().dialog.as_deref(), Some("Messages travel: part, card, background, stack, Home."), "pass help reached Home");
    rt.reply("OK");
    // newCard goes to the new card once it exists.
    rt.exec("new card;"); no_error(&rt);
    assert_eq!(field(&rt, "status"), "Card 1 newCard:2", "the new card is inserted after card 1");
    // A script error goes to errorDialog when a handler takes it.
    rt.exec(r#"put "a" - 1 into x;"#);
    assert_eq!(rt.w.borrow().last_error, None);
    assert!(field(&rt, "search").starts_with("caught:"), "{}", field(&rt, "search"));
    // openBackground when the background changes.
    { let mut w = rt.w.borrow_mut(); w.begin(); w.new_card_named("other", None); }
    let bg_card = rt.w.borrow().file.objs.iter().find(|o| o.name == "other").unwrap().id;
    let new_bg = rt.w.borrow().background_of(bg_card).unwrap();
    rt.set_script(new_bg, r#"on openBackground { answer "new background"; }"#);
    rt.exec(r#"go to card "other";"#); no_error(&rt);
    assert_eq!(rt.w.borrow().dialog.as_deref(), Some("new background"));
}

/// The handlers on the reader-test sheet (notes/reader-test.md), braces version, run as written.
pub const READER_PILLARS: &str = r#"on mouseUp {
    global pillars, boat;
    put the name of target into name;
    if word 1 of name != "pillar" { pass mouseUp; }
    put word 2 of name into n;
    if pillars is empty { put "off,off,off,off,off,off,off,off" into pillars; }
    play "click";
    if item n of pillars is "on" {
        put "off" into item n of pillars;
        hide picture ("lit " & n);
    } else {
        put "on" into item n of pillars;
        show picture ("lit " & n);
    }
    if pillars is "off,on,off,off,on,on,off,off" {
        put "up" into boat;
        play "ship";
        put "Something heavy moves in the water by the dock." into field "status";
    }
}"#;

pub const READER_BOUNCE: &str = r#"on frame {
    global ballX, ballY, dx, dy;
    add dx to ballX;
    add dy to ballY;
    if ballX < 6 or ballX > 474 { put -dx into dx; }
    if ballY < 6 { put -dy into dy; }
}"#;

#[test]
fn reader_test_handlers_run() {
    let rt = Rt::new(demo::pillars());
    let pil = rt.w.borrow().cards()[1];
    rt.w.borrow_mut().file.current_card = pil;
    rt.set_script(pil, READER_PILLARS);
    assert_eq!(rt.script_error(pil), None);
    for n in [2, 5, 6] { rt.send(part(&rt, Kind::Button, &format!("pillar {n}")), "mouseUp"); no_error(&rt); }
    assert_eq!(field(&rt, "status"), "Something heavy moves in the water by the dock.");
    let lit = |n: u32| { let w = rt.w.borrow(); let id = w.find_part(Kind::Picture, &Value::Str(format!("lit {n}"))).unwrap(); w.get(id, "visible").truthy() };
    assert!(lit(2) && lit(5) && lit(6) && !lit(1));

    let rt = Rt::new(demo::address_book());
    let card = rt.w.borrow().current_card();
    rt.set_script(card, &format!("{READER_BOUNCE}\non mouseUp {{ global ballX, ballY, dx, dy; put 470 into ballX; put 10 into ballY; put 3 into dx; put -3 into dy; }}"));
    assert_eq!(rt.script_error(card), None);
    rt.send(card, "mouseUp");
    for _ in 0..3 { rt.send(card, "frame"); no_error(&rt); }
    rt.exec("global ballX, ballY, dx, dy; put ballX & \",\" & ballY & \",\" & dx & \",\" & dy into field \"notes\";"); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "473,7,-3,3", "frame 2 reaches (476,4), past the right wall and the top, so both directions flip");
}

#[test]
fn hypertalk_word_pairs_and_comments() {
    use stack_core::hypertalk_words;
    let src = "if x is not \"is a\" -- a comment\n{ y is   in z } // is a";
    let out = hypertalk_words(src);
    assert_eq!(out, "if x isnot  \"is a\" // a comment\n{ y isin    z } // is a");
    assert_eq!(out.chars().count(), src.chars().count(), "same length, so error positions stay right");
    let rt = Rt::new(demo::address_book());
    for (src, want) in [(r#""a" is not "b""#, "true"), (r#""B" is in "abc""#, "true"), (r#""12" is a number"#, "true"), (r#""x" is a number"#, "false"),
                        (r#"5 is an integer"#, "true"), (r#""1,2" is a point"#, "true"), (r#""" is empty"#, "true"), (r#""5,5" is within "0,0,10,10""#, "true"),
                        (r#""11,5" is within "0,0,10,10""#, "false")] {
        rt.exec(&format!("-- comment first\nput {src} into field \"notes\";")); no_error(&rt);
        assert_eq!(field(&rt, "notes"), want, "{src}");
    }
}

fn mem_fs(rt: &Rt) {
    rt.w.borrow_mut().fs = std::rc::Rc::new(std::cell::RefCell::new(Box::new(stack_core::files::MemFs::new())));
    rt.w.borrow_mut().fs_base = "/".into();
}

#[test]
fn answer_and_ask_wait_for_the_reply_in_it() {
    let rt = Rt::new(demo::address_book());
    let hello = part(&rt, Kind::Button, "Hello");
    rt.set_script(hello, r#"on mouseUp
  answer "Save changes?" with "Yes" or "No" or "Cancel"
  put "chose " & it into field "notes"
end mouseUp
on mouseDown
  ask "Your name?" with "Ada"
  if the result is "Cancel" then put "cancelled" into field "search"
  else put "name " & it into field "search"
end mouseDown"#);
    assert_eq!(rt.script_error(hello), None);
    rt.send(hello, "mouseUp"); no_error(&rt);
    assert_eq!(rt.w.borrow().dialog.as_deref(), Some("Save changes?"));
    assert_eq!(rt.w.borrow().dialog_buttons, vec!["Yes", "No", "Cancel"]);
    assert_eq!(rt.waiting(), Some(Waiting::Dialog));
    // Modal: nothing else runs while the script waits.
    assert!(!rt.send(hello, "mouseDown"));
    rt.reply("No"); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "chose No");
    assert_eq!(rt.w.borrow().dialog, None);
    assert!(!rt.is_paused());
    rt.send(hello, "mouseDown"); no_error(&rt);
    assert_eq!(rt.w.borrow().dialog_ask.as_deref(), Some("Ada"));
    rt.reply("Grace");
    assert_eq!(field(&rt, "search"), "name Grace");
    rt.send(hello, "mouseDown");
    rt.cancel(); no_error(&rt);
    assert_eq!(field(&rt, "search"), "cancelled");
}

#[test]
fn a_paused_script_does_nothing_twice() {
    let rt = Rt::new(demo::address_book());
    mem_fs(&rt);
    let hello = part(&rt, Kind::Button, "Hello");
    // Changes before the pause, a global, a file append, random numbers, a loop with two dialogs.
    rt.set_script(hello, r#"on mouseUp
  global gCount
  add 1 to gCount
  put random(1000000) into r
  put r into field "notes"
  put "x" after file "log.txt"
  repeat with i = 1 to 2
    answer "Round " & i with "A" or "B"
    put it after field "search"
  end repeat
  put return & r & "/" & gCount & "/" & file "log.txt" after field "notes"
end mouseUp"#);
    assert_eq!(rt.script_error(hello), None);
    rt.exec(r#"put empty into field "search""#);
    rt.send(hello, "mouseUp"); no_error(&rt);
    let first = field(&rt, "notes");
    assert_eq!(rt.w.borrow().dialog.as_deref(), Some("Round 1"));
    rt.reply("A"); no_error(&rt);
    assert_eq!(rt.w.borrow().dialog.as_deref(), Some("Round 2"));
    assert_eq!(field(&rt, "search"), "A");
    rt.reply("B"); no_error(&rt);
    assert!(!rt.is_paused());
    assert_eq!(field(&rt, "search"), "AB");
    assert_eq!(field(&rt, "notes"), format!("{first}\n{first}/1/x"));
    assert_eq!(rt.w.borrow().fs.borrow().read("/log.txt").unwrap(), "x");
    // One Undo takes back the whole click.
    rt.undo();
    assert_eq!(field(&rt, "search"), "");
}

#[test]
fn wait_for_time_and_for_a_condition() {
    let rt = Rt::new(demo::address_book());
    let hello = part(&rt, Kind::Button, "Hello");
    rt.set_script(hello, r#"on mouseUp
  put "1" into field "notes"
  wait 6 ticks
  put "2" after field "notes"
  wait for 0.5 seconds
  put "3" after field "notes"
  wait until the mouse is down
  put "4" after field "notes"
  wait while the mouse is down
  put "5" after field "notes"
end mouseUp"#);
    assert_eq!(rt.script_error(hello), None);
    let now = || web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).unwrap().as_secs_f64() * 1000.0;
    rt.send(hello, "mouseUp"); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "1");
    let Some(Waiting::Until(t)) = rt.waiting() else { panic!("{:?}", rt.waiting()) };
    assert!(t - now() > 50.0 && t - now() <= 101.0);
    assert!(!rt.tick(now()));
    assert!(rt.tick(t)); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "12");
    let Some(Waiting::Until(t)) = rt.waiting() else { panic!() };
    assert!(rt.tick(t + 1.0)); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "123");
    let Some(Waiting::Poll(t)) = rt.waiting() else { panic!("{:?}", rt.waiting()) };
    assert!(rt.tick(t)); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "123", "still up");
    rt.w.borrow_mut().mouse_down = true;
    let Some(Waiting::Poll(t)) = rt.waiting() else { panic!() };
    assert!(rt.tick(t)); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "1234");
    rt.w.borrow_mut().mouse_down = false;
    let Some(Waiting::Poll(t)) = rt.waiting() else { panic!("{:?}", rt.waiting()) };
    assert!(rt.tick(t)); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "12345");
    assert!(!rt.is_paused());
}

#[test]
fn frame_handlers_cannot_pause() {
    let rt = Rt::new(demo::breakout());
    let card = rt.w.borrow().current_card();
    rt.set_script(card, "on frame\n  wait 1 tick\nend frame");
    rt.frame();
    assert!(rt.w.borrow().last_error.as_deref().unwrap_or("").contains("cannot pause"), "{:?}", rt.w.borrow().last_error);
    assert!(!rt.is_paused());
}

#[test]
fn timers_sort_push_ids_family_edit() {
    let rt = Rt::new(demo::address_book());
    let card = rt.w.borrow().current_card();
    rt.set_script(card, r#"on tick { put "ticked" into field "notes"; }"#);
    rt.exec(r#"send "tick" to this card in 2 seconds;"#); no_error(&rt);
    let due = rt.w.borrow().timers[0].0;
    assert_eq!(rt.run_timers(due - 10.0), 0);
    assert_ne!(field(&rt, "notes"), "ticked");
    assert_eq!(rt.run_timers(due + 1.0), 1);
    assert_eq!(field(&rt, "notes"), "ticked");
    // sort cards by a background field, evaluated on each card; Undo restores the order.
    let names = |rt: &Rt| { let w = rt.w.borrow(); let here = w.current_card(); drop(w); let cards = rt.w.borrow().cards(); let mut v = vec![];
        for c in cards { rt.w.borrow_mut().file.current_card = c; v.push(field(rt, "name")); } rt.w.borrow_mut().file.current_card = here; v };
    rt.exec(r#"sort cards descending by field "name";"#); no_error(&rt);
    assert_eq!(names(&rt), vec!["Grace Hopper", "Bill Atkinson", "Ada Lovelace"]);
    rt.undo();
    assert_eq!(names(&rt), vec!["Ada Lovelace", "Bill Atkinson", "Grace Hopper"]);
    rt.exec(r#"sort by word 2 of field "name";"#); no_error(&rt);
    assert_eq!(names(&rt), vec!["Bill Atkinson", "Grace Hopper", "Ada Lovelace"]);
    // push / pop
    rt.exec(r#"go to card 1; push card; go last; pop card; go to card "card 1";"#); no_error(&rt);
    rt.exec(r#"go to card 2; push this card; go to card 3; pop card; put the number of this card into field "search"; go to card "card 1";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "2");
    rt.exec("let a = [3, 1, 2]; a.sort(); a.push(4); a.pop(); put a into x; put \"abc\".contains(\"b\") into y;"); no_error(&rt);
    // by id
    let id = part(&rt, Kind::Button, "Hello");
    rt.exec(&format!("put the name of button id {id} into field \"notes\";")); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "Hello");
    rt.exec("put the name of button id 9999 into field \"notes\";");
    assert!(rt.w.borrow().last_error.as_deref().unwrap_or("").contains("no button id 9999"));
    // family: radio buttons
    let (a, b) = (part(&rt, Kind::Button, "Prev"), part(&rt, Kind::Button, "Next"));
    rt.exec(r#"set the family of button "Prev" to 1; set the family of button "Next" to 1;"#); no_error(&rt);
    rt.click(a);
    rt.click(b);
    { let w = rt.w.borrow(); assert!(!w.get(a, "hilite").truthy() && w.get(b, "hilite").truthy()); }
    // edit the script of …
    rt.exec(r#"edit the script of button "Hello";"#); no_error(&rt);
    assert_eq!(rt.w.borrow().edit_request, Some(id));
}

#[test]
fn background_parts_send_through_the_card() {
    let rt = Rt::new(demo::address_book());
    let card = rt.w.borrow().current_card();
    rt.set_script(card, r#"on returnKey { put "card took it from " & the name of target into field "notes"; }"#);
    rt.send_with(part(&rt, Kind::Field, "phone"), "returnKey", vec![]); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "card took it from phone");
}

/// Messages the player sends (checked in the browser on 2026-11-19); here, that handlers receive them.
#[test]
fn player_message_names() {
    let rt = Rt::new(demo::address_book());
    let card = rt.w.borrow().current_card();
    rt.set_script(card, r#"on returnKey { put "r" after field "notes"; }
on enterInField { put "e" after field "notes"; }
on commandKeyDown k { put k after field "notes"; }
on mouseStillDown { put "s" after field "notes"; }
on mouseWithin { put "w" after field "notes"; }
on newButton { put "b" after field "notes"; }
on newField { put "f" after field "notes"; }
on suspendStack { put "x" after field "notes"; }
on sizeWindow { put "z" after field "notes"; }
on exitField { put "q" after field "notes"; }
on resumeStack { put "u" after field "notes"; }
on mouseDoubleClick { put "d" after field "notes"; }"#);
    rt.exec(r#"put empty into field "notes";"#);
    for (m, a) in [("returnKey", None), ("enterInField", None), ("commandKeyDown", Some("k")), ("mouseStillDown", None), ("mouseWithin", None), ("newButton", None), ("newField", None), ("suspendStack", None), ("sizeWindow", None), ("exitField", None), ("resumeStack", None), ("mouseDoubleClick", None)] {
        rt.send_with(card, m, a.map(|x| vec![Value::Str(x.into())]).unwrap_or_default()); no_error(&rt);
    }
    assert_eq!(field(&rt, "notes"), "rekswbfxzqud");
}

#[test]
fn selection_click_type_create_convert_part_userlevel() {
    let rt = Rt::new(demo::address_book());
    rt.exec(r#"put "one" & lineFeed & "two,three" & lineFeed & "four" into field "notes";"#); no_error(&rt);
    rt.exec(r#"select line 2 of field "notes"; put the selection into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "two,three");
    rt.exec(r#"select item 2 of line 2 of field "notes"; put the selection into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "three", "nested chunks");
    rt.exec(r#"select word 1 to 2 of field "notes"; put the selectedText into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "one\ntwo,three");
    // type replaces the selection, then continues at the caret.
    rt.exec(r#"select line 3 of field "notes"; type "FOUR"; type "!";"#); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "one\ntwo,three\nFOUR!");
    rt.exec(r#"select after text of field "notes"; put the selectedChunk into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "char 20 to 19 of field \"notes\"");
    // click at a point sends mouseDown and mouseUp to the part there.
    let card = rt.w.borrow().current_card();
    rt.set_script(card, r#"on mouseUp { put "clicked " & the name of target into field "search"; }"#);
    let hello = part(&rt, Kind::Button, "Hello");
    rt.set_script(hello, "");
    rt.exec(r#"click at "370,70";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "clicked Hello");
    rt.exec(r#"click at "600,350";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "clicked card 1");
    // create, then address by part number.
    rt.set_script(card, r#"on newButton { put "new " & the name of target into field "search"; }"#);
    rt.exec(r#"create button "Extra";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "new Extra");
    rt.exec(r#"put the name of part 1 & "/" & the name of part "Extra" into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "name/Extra");
    // convert, UTC.
    rt.exec(r#"put "11/19/26" into d; convert d to seconds; put d into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "3877891200", "HyperCard seconds count from 1 January 1904");
    rt.exec(r#"put "3877891200" into d; convert d to long date; put d into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "Thursday, November 19, 2026");
    rt.exec(r#"put "2026,11,19,13,5,0" into d; convert d to dateItems; put d & " " into field "search"; convert d to time; put d after field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "2026,11,19,13,5,0,5 1:05 PM");
    // userLevel is a global property, 1 to 5.
    rt.exec(r#"set the userLevel to 2; put the userLevel into field "search";"#); no_error(&rt);
    assert_eq!((field(&rt, "search"), rt.w.borrow().user_level), ("2".to_string(), 2));
    rt.exec("set the userLevel to 9;");
    assert!(rt.w.borrow().last_error.is_some());
}

#[test]
fn upgrades_to_full_hypertalk_forms() {
    let rt = Rt::new(demo::address_book());
    let run = |src: &str| -> String { rt.exec(src); no_error(&rt); field(&rt, "search") };
    // repeat, all forms; break and continue inside.
    assert_eq!(run(r#"put empty into field "search"; repeat with i = 1 to 5 { if i == 2 { continue; } if i == 5 { break; } put i after field "search"; }"#), "134");
    assert_eq!(run(r#"put empty into field "search"; repeat with i = 3 down to 1 { put i after field "search"; }"#), "321");
    assert_eq!(run(r#"put 0 into n; repeat while n < 3 { add 1 to n; } repeat until n > 5 { add 1 to n; } put n into field "search";"#), "6");
    assert_eq!(run(r#"put 0 into n; repeat { add 1 to n; if n is 4 { break; } } repeat 2 { add 10 to n; } put n into field "search";"#), "24");
    // put into a range, into the selection; the selectedLine.
    assert_eq!(run(r#"put "abcdef" into x; put "XY" into char 2 to 4 of x; put x into field "search";"#), "aXYef");
    assert_eq!(run(r#"put "a b c d" into field "search"; put "Z" into word 2 to 3 of field "search";"#), "a Z d");
    assert_eq!(run(r#"put "one" & return & "two" into field "search"; select line 2 of field "search"; put "TWO" into the selection; put the selectedLine after field "search";"#), "one\nTWOline 2 of field \"search\"");
    // return as the newline constant, but still a statement where a statement starts.
    let card = rt.w.borrow().current_card();
    rt.set_script(card, "on f {\n    put \"x\" & return & \"y\" into field \"search\";\n    // comment\n    return;\n}");
    assert_eq!(rt.script_error(card), None);
    rt.send(card, "f"); no_error(&rt);
    assert_eq!(field(&rt, "search"), "x\ny");
    // A script writes a script: compiled at once, undoable.
    rt.exec(r#"set the script of button "Hello" to "on mouseUp { put \"written\" into field \"search\"; }";"#); no_error(&rt);
    rt.send(part(&rt, Kind::Button, "Hello"), "mouseUp"); no_error(&rt);
    assert_eq!(field(&rt, "search"), "written");
    // mark cards where / by finding / all.
    rt.exec(r#"mark cards where word 1 of field "name" is "Grace"; put the marked of card 3 & the marked of card 1 into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "truefalse");
    rt.exec(r#"unmark all cards; mark cards by finding "MacPaint"; put the marked of card 2 & the marked of card 3 into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "truefalse");
    // find forms and `in field`; the match becomes the selection.
    rt.exec(r#"go to card 1; find whole "first published"; put the number of this card & ":" & the selectedText into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "1:first published");
    rt.exec(r#"find word "Hop"; put the result into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "not found");
    rt.exec(r#"find "Hop"; put word 1 of field "name" into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "Grace");
    rt.exec(r#"find chars "card" in field "notes"; put the result into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "", "MacPaint, QuickDraw, HyperCard. contains card");
    // Local time from the player's offset; HyperCard seconds; the long date.
    rt.w.borrow_mut().tz_offset_min = 60;
    rt.exec(r#"put the long date & "|" & the abbreviated date & "|" & the long time into field "search";"#); no_error(&rt);
    let s = field(&rt, "search");
    assert!(s.contains(", 20") && s.matches('|').count() == 2 && s.contains(':'), "{s}");
    rt.exec(r#"put the seconds into field "search";"#);
    assert!(field(&rt, "search").parse::<i64>().unwrap() > 3_800_000_000);
    // the mouseClick, clickText / clickLine / clickChunk (set by the player in the browser).
    rt.w.borrow_mut().mouse_click = true;
    rt.exec(r#"put the mouseClick & the mouseClick into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "truefalse");
    let notes = part(&rt, Kind::Field, "notes");
    rt.exec(r#"put "alpha beta" & return & "gamma" into field "notes";"#);
    rt.w.borrow_mut().click_text_at = Some((notes, 13));
    rt.exec(r#"put the clickText & "|" & the clickLine & "|" & the clickChunk into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "gamma|line 2 of field \"notes\"|char 12 to 16 of field \"notes\"");
    // play with notes: a synthesized clip goes to the player.
    rt.w.borrow_mut().sound_cmds.clear();
    rt.exec(r#"play "harpsichord" tempo 200 "c4q e g c5h r";"#); no_error(&rt);
    let clip = match rt.w.borrow().sound_cmds.last() { Some(stack_core::SoundCmd::Clip(_, b)) => b.len(), _ => 0 };
    assert!(clip > 44 + 22050, "about 1.5 s of audio: {clip} bytes");
    rt.exec(r#"play "x" "h";"#);
    assert!(rt.w.borrow().last_error.as_deref().unwrap_or("").contains("not a note"));
}

#[test]
fn hypertalk_operators() {
    let rt = Rt::new(demo::address_book());
    for (src, want) in [
        (r#""a" && "b""#, "a b"), (r#""a" & "b" & 1"#, "ab1"), (r#""a" & "b" = "ab""#, "true"), (r#""a" & "b" is "ab""#, "true"),
        (r#"5 = 5"#, "true"), (r#""5" = 5"#, "true"), (r#"1 <> 2"#, "true"), (r#"not (1 = 2)"#, "true"),
        (r#"true and not false"#, "true"), (r#"2 + 3 & "x""#, "5x"),
    ] {
        rt.exec(&format!("put {src} into field \"notes\";")); no_error(&rt);
        assert_eq!(field(&rt, "notes"), want, "{src}");
    }
    // HyperTalk precedence: `not` binds tightest, so `not 1 = 2` is (not 1) = 2, an error as in HyperTalk.
    rt.exec(r#"put not 1 = 2 into field "notes";"#);
    assert!(rt.w.borrow().last_error.is_some());
    // `x = 5` at the start of a statement still assigns; `let` and `repeat with` too.
    rt.exec(r#"let m = #{ a: [1, 2] }; m.a[1] = 9; put m.a[1] into k; let x = 4; x = 5; let y = 6; put empty into s; repeat with i = 1 to 2 { put i after s; } put x & y & s & (x = 5) & k into field "notes";"#); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "5612true9");
    // Format 2 stacks: && and || were logic; they become and / or.
    assert_eq!(stack_core::migrate_logic(r#"if a && b || c { put "x && y" into z; } // a && b"#), r#"if a and b or c { put "x && y" into z; } // a && b"#);
}

/// The Myst pillar handler, HyperTalk-style: `end` blocks, no semicolons, `=` and `is` comparisons.
pub const PILLARS_HYPERTALK: &str = r#"-- Ported from Myst (1993), the pillar puzzle, in HyperTalk's own block syntax.
on openCard
  global pillars
  repeat with n = 1 to 8
    if item n of pillars is "on" then show picture ("lit " & n)
    else hide picture ("lit " & n)
  end repeat
end openCard

on mouseUp
  global pillars, boat
  put the name of the target into theName
  if word 1 of theName is not "pillar" then pass mouseUp
  put word 2 of theName into n
  if pillars is empty then put "off,off,off,off,off,off,off,off" into pillars
  play "click"
  if item n of pillars = "on" then
    put "off" into item n of pillars
    hide picture ("lit " & n)
  else
    put "on" into item n of pillars
    show picture ("lit " & n)
  end if
  if pillars = "off,on,off,off,on,on,off,off" then
    put "up" into boat
    play "ship"
    put "Something heavy moves in the water by the dock." ¬
      into field "status"
  else if boat is "up" then
    put "down" into boat
    put "Something sinks back into the water." into field "status"
  end if
end mouseUp

function lit n
  return "lit " & n
end lit
"#;

#[test]
fn hypertalk_block_syntax() {
    let rt = Rt::new(demo::pillars());
    let pil = rt.w.borrow().cards()[1];
    rt.w.borrow_mut().file.current_card = pil;
    rt.set_script(pil, PILLARS_HYPERTALK);
    assert_eq!(rt.script_error(pil), None, "translated:\n{}", stack_core::hypertalk_blocks(PILLARS_HYPERTALK));
    assert_eq!(stack_core::hypertalk_blocks(PILLARS_HYPERTALK).lines().count(), PILLARS_HYPERTALK.lines().count(), "one line out per line in");
    for n in [3, 3, 2, 5, 6] { rt.send(part(&rt, Kind::Button, &format!("pillar {n}")), "mouseUp"); no_error(&rt); }
    assert_eq!(field(&rt, "status"), "Something heavy moves in the water by the dock.");
    rt.send(part(&rt, Kind::Button, "pillar 1"), "mouseUp"); no_error(&rt);
    assert_eq!(field(&rt, "status"), "Something sinks back into the water.");
    rt.send(pil, "openCard"); no_error(&rt);
    let lit = |n: u32| { let w = rt.w.borrow(); let id = w.find_part(Kind::Picture, &Value::Str(format!("lit {n}"))).unwrap(); w.get(id, "visible").truthy() };
    assert!(lit(1) && lit(2) && !lit(3));
    // Mixed styles in one script, a function handler, repeat for, and the message box.
    let rt = Rt::new(demo::address_book());
    let card = rt.w.borrow().current_card();
    rt.set_script(card, "on mouseUp\n  put twice(3) into x\n  repeat for 2 times\n    add 1 to x\n  end repeat\n  if x = 8 then put \"eight\" into field \"notes\" else put x into field \"notes\"\n end mouseUp\non keyDown k {\n    put k into field \"search\";\n}\nfunction twice n\n  return n * 2\nend twice");
    assert_eq!(rt.script_error(card), None);
    rt.send(card, "mouseUp"); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "eight");
    rt.exec(r#"if 1 = 1 then put "yes" into field "search" else put "no" into field "search""#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "yes");
    // A HyperTalk error points at the right line.
    rt.set_script(card, "on mouseUp\n  put 1 into x\n  put (2 into y\nend mouseUp");
    let e = rt.script_error(card).unwrap();
    assert!(e.contains("line 3"), "{e}");
}

#[test]
fn file_io() {
    let dir = std::env::temp_dir().join(format!("stack-files-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let rt = Rt::new(demo::address_book());
    rt.w.borrow_mut().fs_base = dir.to_string_lossy().into_owned();
    let run = |src: &str| -> String { rt.exec(src); no_error(&rt); field(&rt, "search") };
    // HyperTalk's open / write / read / close, relative to the stack's folder.
    run(r#"open file "notes.txt"; write "one" & return & "two" & return to file "notes.txt"; write "three" to file "notes.txt"; close file "notes.txt";"#);
    assert_eq!(std::fs::read_to_string(dir.join("notes.txt")).unwrap(), "one\ntwo\nthree");
    assert_eq!(run(r#"open file "notes.txt"; read from file "notes.txt" until return; put it into field "search";"#), "one\n");
    assert_eq!(run(r#"read from file "notes.txt" for 3; put it & "|" & the result into field "search";"#), "two|");
    assert_eq!(run(r#"read from file "notes.txt" until eof; put it & "|" & the result into field "search"; close file "notes.txt";"#), "\nthree|eof");
    run(r#"open file "notes.txt"; write "!" to file "notes.txt" at eof; close file "notes.txt";"#);
    assert_eq!(std::fs::read_to_string(dir.join("notes.txt")).unwrap(), "one\ntwo\nthree!");
    // Whole files, chunks of files, and HyperTalk's `delete` of a chunk.
    assert_eq!(run(r#"put line 2 of file "notes.txt" into field "search";"#), "two");
    run(r#"put "TWO" into line 2 of file "notes.txt"; put return & "four" after file "notes.txt"; delete line 1 of file_text;"#.replace("delete line 1 of file_text;", "").as_str());
    assert_eq!(std::fs::read_to_string(dir.join("notes.txt")).unwrap(), "one\nTWO\nthree!\nfour");
    assert_eq!(run(r#"put file "notes.txt" into t; delete line 1 of t; delete word 2 of t; put t into field "search";"#), "TWO\nfour");
    assert_eq!(run(r#"put "a,b,c" into t; delete item 2 of t; put t into field "search";"#), "a,c");
    // Folders, listing, existence, rename, delete.
    run(r#"create folder "data/sub"; put "x" into file "data/a.txt"; rename file "data/a.txt" to "data/b.txt";"#);
    assert_eq!(run(r#"put the files of folder "data" & "|" & the folders of folder "data" into field "search";"#), "b.txt|sub");
    assert_eq!(run(r#"put (there is a file "data/b.txt") & (there is not a file "data/a.txt") & (there is a folder "data/sub") into field "search";"#), "truetruetrue");
    run(r#"delete file "data/b.txt"; delete folder "data/sub";"#);
    assert_eq!(run(r#"put the files of folder "data" & "|" & (there is a file "nope.txt") into field "search";"#), "|false");
    assert_eq!(run(r#"put file "nope.txt" & "|" & (the result is not empty) into field "search";"#), "|true");
    rt.exec(r#"read from file "closed.txt" until eof;"#);
    assert!(rt.w.borrow().last_error.as_deref().unwrap_or("").contains("not open"));
    let _ = std::fs::remove_dir_all(&dir);

    // The web disk: the same scripts on the in-memory folder, with changes queued for browser storage.
    let rt = Rt::new(demo::address_book());
    rt.w.borrow_mut().fs = std::rc::Rc::new(std::cell::RefCell::new(Box::new(stack_core::files::MemFs::new())));
    rt.w.borrow_mut().fs_base = "/".into();
    rt.exec(r#"put "hello" into file "greeting.txt"; create folder "box"; put file "greeting.txt" & "!" into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "hello!");
    let changes = rt.w.borrow().fs.borrow_mut().take_changes();
    use stack_core::files::FsChange;
    assert_eq!(changes, vec![FsChange::Write { path: "/greeting.txt".into(), text: "hello".into() }, FsChange::Folder { path: "/box".into() }]);
    // answer file puts the path into it; load url / url via the cache.
    let card = rt.w.borrow().current_card();
    rt.set_script(card, r#"on urlLoaded u { put "loaded " & u & ": " & url u into field "notes"; }"#);
    rt.exec(r#"answer file "Pick a text file"; put "picked " & it into field "notes";"#); no_error(&rt);
    assert!(rt.w.borrow().dialog_file.is_some());
    rt.reply("/greeting.txt");
    assert_eq!(field(&rt, "notes"), "picked /greeting.txt");
    rt.exec(r#"load url "https://example.org/x"; put url "https://example.org/x" & (the result) into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "not loaded");
    let (u, to) = rt.w.borrow_mut().url_requests.pop().unwrap();
    rt.url_loaded(&u, "<html>", to);
    assert_eq!(field(&rt, "notes"), "loaded https://example.org/x: <html>");
}

#[test]
fn libraries_and_other_stacks() {
    let dir = std::env::temp_dir().join(format!("stack-libs-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("lib")).unwrap();
    // A library stack on disk, in a search folder.
    let mut lib = StackFile::new("Utilities", 400, 300);
    { let mut w = stack_core::World::new(lib.clone());
      let s = w.create(Kind::Stack, "Utilities", None, None);
      w.set(s, "script", "on shout x\n  answer \"SHOUT: \" & x\nend shout\non libraryStack\n  put \"in use\" into field \"notes\"\nend libraryStack".into(), false);
      w.file.journal.clear(); lib = w.file; }
    std::fs::write(dir.join("lib/Utilities.stack"), lib.to_zip()).unwrap();
    let rt = Rt::with_home(demo::address_book(), Some(demo::home()));
    rt.w.borrow_mut().fs_base = dir.to_string_lossy().into_owned();
    rt.exec(r#"start using stack "Utilities";"#); no_error(&rt);
    assert_eq!(rt.w.borrow().result, "No such stack.");
    rt.exec(&format!(r#"set the searchPaths to "{}"; start using stack "Utilities";"#, dir.join("lib").display())); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "in use", "libraryStack was sent");
    // Its handlers sit between this stack and Home on the message path.
    rt.exec(r#"send "shout" ; put the stacksInUse into field "search";"#); no_error(&rt);
    rt.reply("OK");
    assert_eq!(field(&rt, "search"), "Utilities");
    let card = rt.w.borrow().current_card();
    rt.send_with(card, "shout", vec![Value::Str("hi".into())]); no_error(&rt);
    assert_eq!(rt.w.borrow().dialog.as_deref(), Some("SHOUT: hi"));
    rt.reply("OK");
    rt.exec(r#"send "help";"#); no_error(&rt);
    assert_eq!(rt.w.borrow().dialog.as_deref(), Some("Messages travel: part, card, background, stack, Home."), "Home still last");
    rt.reply("OK");
    rt.exec(r#"stop using stack "Utilities"; put the stacksInUse into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "");
    // go to stack: a request for the player, found by search or as a demo by name.
    rt.exec(r#"go to stack "Utilities" in a new window;"#); no_error(&rt);
    let g = rt.w.borrow_mut().go_stack.take().unwrap();
    assert!(g.path.ends_with("lib/Utilities.stack") && g.new_window, "{g:?}");
    rt.exec(r#"go to card "dock" of stack "Pillars" with dissolve;"#); no_error(&rt);
    let g = rt.w.borrow_mut().go_stack.take().unwrap();
    assert_eq!((g.path.as_str(), g.card, g.new_window, g.effect.as_deref()), ("demo:Pillars", Some(Value::Str("dock".into())), false, Some("dissolve")));
    rt.exec(r#"go stack "Nowhere"; put the result into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "No such stack.");
    rt.exec(r#"go next card; go to card 1;"#); no_error(&rt);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn containers_end_at_their_name() {
    let rt = Rt::new(demo::address_book());
    rt.w.borrow_mut().url_cache.insert("u".into(), "a\nb\nc\nd".into());
    rt.exec(r#"put "x y z" into field "search"; put (the number of lines of url "u" > 3) & (word 2 of field "search" is "y") & (the last word of field "search" = "z") & (first line of url "u" = "a") into field "notes";"#); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "truetruetruetrue");
}

#[test]
fn function_handlers_and_user_commands_travel_the_path() {
    let mut home = demo::home();
    home.script.push_str("\nfunction greeting who\n  return \"Hello, \" & who\nend greeting\n");
    let rt = Rt::with_home(demo::address_book(), Some(home));
    let stack = rt.w.borrow().stack_id();
    rt.set_script(stack, r#"function total a, b
  return a + b
end total

function twice x { return x * 2; }

on check what, ok
  if ok then put "PASS " & what & return after field "notes"
  else put "FAIL " & what & return after field "notes"
end check

function greeting who
  if who is "Grace" then return "Hi Grace"
  pass greeting
end greeting"#);
    assert_eq!(rt.script_error(stack), None);
    let hello = part(&rt, Kind::Button, "Hello");
    rt.set_script(hello, r#"on mouseUp
  put empty into field "notes"
  check "total", total(2, 3) = 5
  check "braces", twice(4) = 8
  check "home", greeting("Ada") = "Hello, Ada"
  check "pass", greeting("Grace") = "Hi Grace"
  check "local fn", local(1) = 2
  check "nested", total(twice(1), total(1, 1)) = 4;
end mouseUp

fn local(x) { return x + 1; }"#);
    assert_eq!(rt.script_error(hello), None);
    rt.send(hello, "mouseUp"); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "PASS total\nPASS braces\nPASS home\nPASS pass\nPASS local fn\nPASS nested\n");
    // A command nobody handles is HyperTalk's "Can't understand"; an unknown function is "not found".
    rt.exec("frobnicate 1, 2");
    assert!(rt.w.borrow().last_error.as_deref().unwrap_or("").contains("Can't understand \"frobnicate\""));
    rt.exec("put nothing(1) into x;");
    assert!(rt.w.borrow().last_error.as_deref().unwrap_or("").contains("nothing"));
    // From the message box, both styles.
    rt.exec(r#"check "box", true"#); no_error(&rt);
    rt.exec(r#"check "box2", total(1, 1) = 2;"#); no_error(&rt);
    assert!(field(&rt, "notes").ends_with("PASS box\nPASS box2\n"));
}

#[test]
fn self_test_stack_passes() {
    let dir = std::env::temp_dir().join(format!("stack-selftest-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let rt = Rt::with_home(demo::self_test(), Some(demo::home()));
    rt.w.borrow_mut().fs_base = dir.to_string_lossy().into_owned();
    for id in rt.w.borrow().file.objs.iter().map(|o| o.id).collect::<Vec<_>>() { assert_eq!(rt.script_error(id), None, "script of {id}"); }
    rt.send(part(&rt, Kind::Button, "Run all tests"), "mouseUp"); no_error(&rt);
    // The run pauses at `wait 3 ticks`, then runs again from the click, replaying what it read.
    assert!(matches!(rt.waiting(), Some(Waiting::Until(_))), "{:?}", rt.waiting());
    while let Some(Waiting::Until(_)) = rt.waiting() {
        std::thread::sleep(std::time::Duration::from_millis(10));
        rt.tick(web_time::SystemTime::now().duration_since(web_time::UNIX_EPOCH).unwrap().as_secs_f64() * 1000.0);
    }
    no_error(&rt);
    let out = field(&rt, "results");
    println!("{out}");
    assert!(!out.contains("FAIL"), "{out}");
    assert!(out.contains(" passed, 0 failed."), "{out}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn sort_lines_and_items_of_containers() {
    let rt = Rt::new(demo::address_book());
    rt.exec("put \"pear\" & return & \"Apple\" & return & \"fig\" into field \"notes\"\nsort lines of field \"notes\""); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "Apple\nfig\npear");
    rt.exec(r#"put "10,9,100,x,2" into v; sort items of v descending numeric; put v into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "x,100,10,9,2", "not-a-number sorts after numbers; descending reverses");
    // by <expr of each>, stable: equal keys keep their order.
    rt.exec("put \"b 2\" & return & \"a 1\" & return & \"c 2\" & return & \"d 1\" into field \"notes\"\nsort field \"notes\" numeric by word 2 of each"); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "a 1\nd 1\nb 2\nc 2");
    rt.exec(r#"put "12/1/26,1/5/25,3/3/26" into d; sort items of d dateTime; put d into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "1/5/25,3/3/26,12/1/26");
    // A handler in HyperTalk style, with a local in the key; sorting cards still works.
    let card = rt.w.borrow().current_card();
    rt.set_script(card, "on sortIt\n  put 2 into n\n  put \"x,b\" & return & \"y,a\" into t\n  sort lines of t by item n of each\n  put t into field \"notes\"\nend sortIt");
    assert_eq!(rt.script_error(card), None);
    rt.send(card, "sortIt"); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "y,a\nx,b");
    rt.exec(r#"sort cards descending by field "name";"#); no_error(&rt);
}

#[test]
fn select_nested_chunks_and_exit_to_hypercard() {
    let rt = Rt::new(demo::address_book());
    rt.exec("put \"a,b,c\" & return & \"dd,ee,ff\" into field \"notes\"\nselect item 2 of line 2 of field \"notes\""); no_error(&rt);
    assert_eq!(rt.w.borrow().selection.map(|s| (s.1, s.2)), Some((9, 11)));
    rt.exec(r#"select char 1 to 2 of item 3 of line 2 of field "notes"; put the selectedText into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "ff");
    rt.exec(r#"select after word 1 of line 1 of field "notes";"#); no_error(&rt);
    assert_eq!(rt.w.borrow().selection.map(|s| (s.1, s.2)), Some((5, 5)));
    // exit to HyperCard ends the calling handlers too, quietly.
    let card = rt.w.borrow().current_card();
    rt.set_script(card, "on outer\n  inner\n  put \"not reached\" into field \"search\"\nend outer\non inner\n  put \"inner\" into field \"search\"\n  exit to HyperCard\nend inner");
    assert_eq!(rt.script_error(card), None);
    rt.send(card, "outer"); no_error(&rt);
    assert_eq!(field(&rt, "search"), "inner");
}

#[test]
fn other_stacks_by_name() {
    let rt = Rt::with_home(demo::address_book(), Some(demo::home()));
    // This stack by name, and another stack, loaded read-only.
    rt.exec(r#"put the name of stack "Address Book" into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "Address Book");
    rt.exec(r#"put the script of stack "Self Test" into field "notes";"#); no_error(&rt);
    assert!(field(&rt, "notes").contains("on runAll"));
    // send … to stack "x": its handlers run, then Home.
    rt.exec(r#"put stackTwice(4) into x; send "help" to stack "Self Test";"#);
    assert!(rt.w.borrow().last_error.is_some(), "a function in another stack is not on this stack's path");
    rt.exec(r#"send "help" to stack "Self Test";"#); no_error(&rt);
    assert_eq!(rt.w.borrow().dialog.as_deref(), Some("Messages travel: part, card, background, stack, Home."));
    rt.reply("OK");
    rt.exec(r#"put there is a stack "Self Test" into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "true");
    rt.exec(r#"put the name of stack "Nowhere" into x;"#);
    assert!(rt.w.borrow().last_error.as_deref().unwrap_or("").contains("no such stack"));
}

#[test]
fn visual_effect_idle_and_autoselect() {
    let rt = Rt::new(demo::address_book());
    rt.exec("visual effect zoom in very slow\ngo next"); no_error(&rt);
    assert_eq!(rt.w.borrow().effect.as_deref(), Some("zoom-in"));
    rt.exec("visual barn door open to black; go next with dissolve;"); no_error(&rt);
    assert_eq!(rt.w.borrow().effect.as_deref(), Some("dissolve"), "go … with wins over visual");
    rt.exec("visual effect wipe left"); no_error(&rt);
    rt.exec("go prev"); no_error(&rt);
    assert_eq!(rt.w.borrow().effect.as_deref(), Some("dissolve"), "a visual effect lasts for its own run only");
    // idle goes to the current card only when a script handles it.
    assert!(!rt.idle());
    let card = rt.w.borrow().current_card();
    rt.set_script(card, "on idle\n  put \"idle\" into field \"search\"\nend idle");
    assert!(rt.idle()); no_error(&rt);
    assert_eq!(field(&rt, "search"), "idle");
    // A list field: autoSelect selects the clicked line.
    let notes = part(&rt, Kind::Field, "notes");
    rt.exec("put \"red\" & return & \"green\" & return & \"blue\" into field \"notes\"; set the locked of field \"notes\" to true; set the autoSelect of field \"notes\" to true;"); no_error(&rt);
    rt.set_script(notes, "on mouseUp\n  put the selectedText & \"/\" & the selectedLine into field \"search\"\nend mouseUp");
    rt.click_field(notes, Some(6)); no_error(&rt);
    assert_eq!(field(&rt, "search"), "green/line 2 of field \"notes\"");
}

#[test]
fn menus_and_domenu() {
    let rt = Rt::new(demo::address_book());
    rt.exec(r#"put the menus into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "File\nEdit\nGo\nTools");
    // Built-in items act when no handler takes doMenu.
    rt.exec(r#"doMenu "Next";"#); no_error(&rt);
    assert_eq!(rt.w.borrow().card_number(rt.w.borrow().current_card()), 2);
    rt.exec(r#"doMenu "Back";"#); no_error(&rt);
    assert_eq!(rt.w.borrow().card_number(rt.w.borrow().current_card()), 1, "Back returns to the card before");
    // A script's own menu, with a menuMessage on one item; doMenu goes to the card first.
    let card = rt.w.borrow().current_card();
    rt.set_script(card, "on doMenu what, which\n  if what is \"Red\" then put \"red from \" & which into field \"search\"\n  else pass doMenu\nend doMenu\non makeBlue\n  put \"blue\" into field \"search\"\nend makeBlue");
    rt.exec("create menu \"Colors\"\nput \"Red,Blue\" into menu \"Colors\" with menuMessages \",makeBlue\"\nput \"Green\" after menu \"Colors\""); no_error(&rt);
    rt.exec(r#"put menu "Colors" & "|" & (there is a menu "Colors") into field "notes";"#); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "Red\nBlue\nGreen|true");
    rt.do_menu("Red", "Colors"); no_error(&rt);
    assert_eq!(field(&rt, "search"), "red from Colors");
    rt.do_menu("Blue", "Colors"); no_error(&rt);
    assert_eq!(field(&rt, "search"), "blue");
    rt.exec("disable menuItem \"Green\" of menu \"Colors\"\ndelete menuItem 1 of menu \"Colors\""); no_error(&rt);
    assert_eq!(rt.w.borrow().menus.last().unwrap().items.iter().map(|i| (i.text.as_str(), i.enabled)).collect::<Vec<_>>(), vec![("Blue", true), ("Green", false)]);
    rt.exec("delete menu \"Colors\""); no_error(&rt);
    rt.exec(r#"put there is a menu "Colors" into field "notes";"#); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "false");
    rt.exec(r#"create menu "Y"; put "A,B" into menu "Y" with menuMessages "a,b""#); no_error(&rt);
    assert_eq!(rt.w.borrow().menus.last().map(|m| m.items.len()), Some(2), "HyperTalk-style line with two statements");
    rt.exec("create menu \"X\"\nreset menuBar\nput the menus into field \"notes\""); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "File\nEdit\nGo\nTools");
    rt.exec(r#"doMenu "Pencil"; put the tool into field "notes";"#); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "pencil tool");
    rt.set_script(card, "on quit\n  put \"bye\" into field \"search\"\nend quit");
    rt.exec(r#"doMenu "Quit HyperCard";"#); no_error(&rt);
    assert_eq!(rt.w.borrow().player_requests.last(), Some(&stack_core::PlayerRequest::Quit));
    assert_eq!(field(&rt, "search"), "bye", "quit is sent before quitting");
    rt.exec(r#"doMenu "Nothing Like This";"#);
    assert!(rt.w.borrow().last_error.as_deref().unwrap_or("").contains("no menu item"));
}

#[test]
fn tools_drag_and_paint() {
    let rt = Rt::new(demo::address_book());
    let hello = part(&rt, Kind::Button, "Hello");
    rt.set_script(hello, "on mouseDown\n  put \"down\" into field \"search\"\nend mouseDown\non mouseStillDown\n  put \".\" after field \"search\"\nend mouseStillDown\non mouseUp\n  put \" up at \" & the mouseLoc after field \"search\"\nend mouseUp");
    let (l, t) = { let w = rt.w.borrow(); (w.get(hello, "left").int(), w.get(hello, "top").int()) };
    rt.exec(&format!("drag from \"{},{}\" to \"{},{}\"", l + 5, t + 5, l + 25, t + 15)); no_error(&rt);
    assert_eq!(field(&rt, "search"), format!("down........ up at {},{}", l + 25, t + 15));
    // The button tool moves the button.
    rt.exec(&format!("choose button tool\ndrag from \"{},{}\" to \"{},{}\" with shiftKey", l + 5, t + 5, l + 45, t + 25)); no_error(&rt);
    assert_eq!((rt.w.borrow().get(hello, "left").int(), rt.w.borrow().get(hello, "top").int()), (l + 40, t + 20));
    rt.exec(r#"choose tool 1; put the tool into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "browse tool");
    // Paint tools need userLevel 3; strokes go into the card's paint, and Undo takes them back.
    rt.exec("set the userLevel to 2\nchoose pencil tool");
    assert!(rt.w.borrow().last_error.as_deref().unwrap_or("").contains("userLevel 3"));
    rt.exec("set the userLevel to 3\nchoose round rect tool\nset the lineSize to 3\nset the filled to true\ndrag from \"10,10\" to \"60,40\""); no_error(&rt);
    rt.exec("choose spray can tool\nreset paint\ndrag from \"0,0\" to \"5,5\"\nchoose tool 8\nclick at \"7,7\""); no_error(&rt);
    let card = rt.w.borrow().current_card();
    assert_eq!(rt.w.borrow().get(card, "paint").text(), "round_rect 3 1 10,10 60,40\nspray 1 0 0,0 5,5\neraser 1 0 7,7");
    rt.paint(&[(1.0, 1.0), (2.0, 3.0)]);
    assert!(rt.w.borrow().get(card, "paint").text().ends_with("\neraser 1 0 1,1 2,3"));
    rt.undo();
    assert!(rt.w.borrow().get(card, "paint").text().ends_with("\neraser 1 0 7,7"));
    rt.exec("choose bucket tool\ndrag from \"1,1\" to \"2,2\"\nput the result into field \"search\""); no_error(&rt);
    assert!(field(&rt, "search").contains("does not paint"));
}

#[test]
fn create_stack_find_marked_delete_card_and_environment() {
    let rt = Rt::new(demo::address_book());
    mem_fs(&rt);
    rt.exec(r#"create stack "Notes"; put there is a stack "Notes" & "/" & the name of stack "Notes" into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "true/Notes");
    rt.exec(r#"create stack "Notes";"#);
    assert!(rt.w.borrow().last_error.as_deref().unwrap_or("").contains("already"));
    // find … of marked cards skips the unmarked ones.
    rt.exec(r#"find "Hopper";"#); no_error(&rt);
    let grace = rt.w.borrow().current_card();
    rt.exec(r#"go first; find "Hopper" of marked cards; put the result into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "not found");
    rt.exec(r#"mark card 2; mark card 3; find "Hopper" of marked cards;"#); no_error(&rt);
    assert_eq!(rt.w.borrow().current_card(), grace);
    rt.exec(r#"find word "Hopper" in field "name" of marked cards; put the result into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "");
    // delete this card, then the next shows; the last card stays.
    let n = rt.w.borrow().cards().len();
    rt.exec("delete this card"); no_error(&rt);
    assert_eq!(rt.w.borrow().cards().len(), n - 1);
    assert!(rt.w.borrow().obj(grace).is_none());
    rt.exec("set lockRecent to true\ngo next\ngo back\nset lockRecent to false");
    rt.w.borrow_mut().screen = (1280.0, 800.0);
    rt.exec(r#"put the screenRect & "|" & the windows & "|" & (the diskSpace > 1000) into field "search";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "0,0,1280,800|card window\nmessage window|true");
}

#[test]
fn hypertalk_precedence_and_put_to_the_message_box() {
    let rt = Rt::new(demo::address_book());
    for (src, want) in [("2 ^ 3", "8"), ("7 div 2 & \",\" & 7 mod 2", "3,1"), ("-2 ^ 2", "4"), ("2 + 3 * 4 = 14", "true"), ("1 < 2 = true", "true"),
                        ("2 ^ 3 ^ 2", "64"), ("\"a\" & \"b\" = \"ab\"", "true"), ("1 + 1 & 2", "22"), ("\"12\" contains \"1\" and 2 > 1", "true"),
                        ("10 - 2 - 3", "5"), ("2 * 3 mod 4", "2"), ("3 is in \"123\" or false", "true"), ("8 / 2 / 2", "2"), ("not true and false", "false")] {
        rt.exec(&format!("put {src}")); no_error(&rt);
        assert_eq!(rt.w.borrow().log.last().map(String::as_str), Some(want), "{src}");
    }
    rt.exec("hide message box"); no_error(&rt);
    assert!(!rt.w.borrow().message_visible);
    rt.exec("show msg"); no_error(&rt);
    assert!(rt.w.borrow().message_visible);
}

#[test]
fn review_regressions() {
    // go back with lockRecent keeps the rest of the recent list; a `go` handler that takes go back keeps it all.
    let rt = Rt::new(demo::address_book());
    rt.exec("go to card 2"); rt.exec("go to card 3");
    let recent: Vec<u32> = rt.w.borrow().recent.iter().copied().collect();
    rt.exec("set the lockRecent to true; go back;"); no_error(&rt);
    assert_eq!(rt.w.borrow().recent.iter().copied().collect::<Vec<_>>(), recent[..1].to_vec());
    let rt = Rt::new(demo::address_book());
    rt.exec("go to card 2"); rt.exec("go to card 3");
    let recent: Vec<u32> = rt.w.borrow().recent.iter().copied().collect();
    let stack = rt.w.borrow().stack_id();
    rt.set_script(stack, "on go x\nend go");
    rt.exec("go back"); no_error(&rt);
    assert_eq!(rt.w.borrow().recent.iter().copied().collect::<Vec<_>>(), recent);
    // Several statements on a line: each user command is its own message.
    let rt = Rt::new(demo::address_book());
    let card = rt.w.borrow().current_card();
    rt.set_script(card, "on mouseUp\n  greet 1; greet 2\nend mouseUp\non greet x\n  put x after field \"search\"\nend greet");
    assert_eq!(rt.script_error(card), None);
    rt.send(card, "mouseUp"); no_error(&rt);
    assert_eq!(field(&rt, "search"), "12");
    // Home by name is the Home stack itself; a pending visual effect goes with `go to stack`.
    let rt = Rt::with_home(demo::address_book(), Some(demo::home()));
    rt.exec(r#"put the name of stack "Home" into field "search"; visual effect dissolve; go to stack "Pillars";"#); no_error(&rt);
    assert_eq!(field(&rt, "search"), "Home");
    assert_eq!(rt.w.borrow().go_stack.as_ref().and_then(|g| g.effect.as_deref()), Some("dissolve"));
    // delete card <variable> and delete card id <n>.
    let rt = Rt::new(demo::address_book());
    let id = rt.w.borrow().cards()[2];
    rt.exec(&format!("put 2 into n; delete card n; delete card id {id}")); no_error(&rt);
    assert_eq!(rt.w.borrow().cards().len(), 1);
    // exit to something other than HyperCard is an error, not a quiet exit.
    rt.exec("exit to nowhere");
    assert!(rt.w.borrow().last_error.as_deref().unwrap_or("").contains("exit to HyperCard"));
    // idle goes only to a script with an idle handler, not one that mentions the word.
    let card = rt.w.borrow().current_card();
    rt.set_script(card, "on mouseUp\n  -- not idle\nend mouseUp");
    assert!(!rt.idle());
    // The in-memory disk: a binary file can be replaced by text, renamed and deleted; a folder with one is not empty.
    use stack_core::files::{Fs, MemFs};
    let mut fs = MemFs::new();
    fs.write_bytes("/d/a.stack", b"PK\x03\x04\xff").unwrap();
    assert!(fs.delete("/d").is_err());
    fs.rename("/d/a.stack", "/d/b.stack").unwrap();
    assert_eq!(fs.read_bytes("/d/b.stack").unwrap(), b"PK\x03\x04\xff");
    fs.write("/d/b.stack", "text").unwrap();
    assert_eq!(fs.read_bytes("/d/b.stack").unwrap(), b"text");
    fs.delete("/d/b.stack").unwrap();
    assert!(!fs.is_file("/d/b.stack"));
}

#[test]
fn one_line_if_statements_are_rewritten_and_strokes_round_trip() {
    let rt = Rt::new(demo::address_book());
    let card = rt.w.borrow().current_card();
    rt.set_script(card, "on check what, ok\n  put what after field \"search\"\nend check\non go2\n  put \"b,a\" into v\n  if true then sort items of v\n  put v into field \"notes\"\n  if false then beep else check \"x\", true\n  if true then doMenu \"Next\"\nend go2");
    assert_eq!(rt.script_error(card), None);
    rt.exec("put empty into field \"search\"");
    rt.send(card, "go2"); no_error(&rt);
    assert_eq!(field(&rt, "search"), "x");
    assert_eq!(rt.w.borrow().card_number(rt.w.borrow().current_card()), 2);
    rt.exec("go prev"); no_error(&rt);
    assert_eq!(field(&rt, "notes"), "a,b", "notes is per card: read it on card 1");
    // A menu item whose name holds a quote mark reaches doMenu intact.
    rt.exec("create menu \"Q\"\nput \"Say \" & quote & \"hi\" & quote into menu \"Q\"");
    let here = rt.w.borrow().current_card();
    rt.set_script(here, "on doMenu what\n  put what into field \"search\"\nend doMenu");
    rt.do_menu("Say \"hi\"", "Q"); no_error(&rt);
    assert_eq!(field(&rt, "search"), "Say \"hi\"");
    // Strokes: one journal entry per stroke, not the whole paint each time.
    let s = stack_core::Stroke { tool: stack_core::Tool::RoundRect, size: 2, filled: true, pts: vec![(1.0, 2.0), (3.0, 4.0)] };
    assert_eq!(s.to_line(), "round_rect 2 1 1,2 3,4");
    assert_eq!(stack_core::Stroke::parse(&s.to_line()), Some(s));
    rt.exec("set the userLevel to 3\nchoose pencil tool");
    for i in 0..3 { rt.paint(&[(i as f64, 0.0)]); }
    let w = rt.w.borrow();
    let card = w.current_card();
    assert_eq!(stack_core::Stroke::parse_all(&w.get(card, "paint").text()).len(), 3);
    assert!(w.file.journal.iter().rev().take(3).all(|c| c.kind == stack_core::ChangeKind::Append && c.new.as_ref().map(|v| v.text().len()) < Some(20)));
    drop(w);
    rt.exec("choose select tool");
    rt.paint(&[(1.0, 1.0)]);
    assert_eq!(stack_core::Stroke::parse_all(&rt.w.borrow().get(card, "paint").text()).len(), 3, "select leaves no stroke");
}

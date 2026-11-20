//! The address-book test stack (the "UI" reference stack of the experiment).
use crate::model::{Kind, StackFile, Value, World};

pub fn address_book() -> StackFile {
    let file = StackFile::new("Address Book", 560, 360);
    let mut w = World::new(file);
    let stack = w.create(Kind::Stack, "Address Book", None, None);
    let bg = w.create(Kind::Background, "people", Some(stack), None);
    let part = |w: &mut World, kind, name: &str, parent, x: i64, y: i64, wd: i64, h: i64, text: &str, script: &str| {
        let id = w.create(kind, name, Some(parent), None);
        for (k, v) in [("left", x), ("top", y), ("width", wd), ("height", h)] { w.set(id, k, Value::Int(v), false); }
        if !text.is_empty() { w.set(id, "text", text.into(), false); }
        if !script.is_empty() { w.set(id, "script", script.into(), false); }
        id
    };
    part(&mut w, Kind::Field, "name", bg, 20, 20, 320, 34, "", "");
    part(&mut w, Kind::Field, "phone", bg, 20, 64, 320, 34, "", "");
    part(&mut w, Kind::Field, "notes", bg, 20, 108, 520, 150, "", "");
    let status = part(&mut w, Kind::Field, "status", bg, 360, 20, 180, 34, "", "");
    w.set(status, "sharedText", true.into(), false);
    w.set(status, "locked", true.into(), false);
    let search = part(&mut w, Kind::Field, "search", bg, 20, 306, 180, 34, "", "");
    w.set(search, "sharedText", true.into(), false);
    part(&mut w, Kind::Button, "Find", bg, 210, 306, 70, 34, "Find", "on mouseUp {\n    find field \"search\";\n}");
    part(&mut w, Kind::Button, "Prev", bg, 300, 306, 70, 34, "◀ Prev", "on mouseUp {\n    go prev;\n}");
    part(&mut w, Kind::Button, "Next", bg, 380, 306, 70, 34, "Next ▶", "on mouseUp {\n    go next;\n}");
    part(&mut w, Kind::Button, "New", bg, 460, 306, 80, 34, "New card", "on mouseUp {\n    new card;\n}");
    w.set(stack, "script", "on openCard {\n    put \"Card \" & the number of card into field \"status\";\n}".into(), false);

    let people = [("Ada Lovelace", "020 7946 0001", "Wrote the first published program."), ("Bill Atkinson", "408 555 0187", "MacPaint, QuickDraw, HyperCard."), ("Grace Hopper", "202 555 0143", "Compilers. Ask about nanoseconds.")];
    let mut prev = None;
    for (i, (name, phone, notes)) in people.iter().enumerate() {
        let c = w.create(Kind::Card, &format!("card {}", i + 1), Some(bg), prev);
        prev = Some(c);
        w.file.current_card = c;
        for (f, t) in [("name", name), ("phone", phone), ("notes", notes)] {
            let id = w.find_part(Kind::Field, &Value::Str(f.into())).unwrap();
            w.set(id, "text", Value::Str(t.to_string()), false);
        }
    }
    let c1 = w.cards()[0];
    w.file.current_card = c1;
    part(&mut w, Kind::Button, "Hello", c1, 360, 64, 180, 34, "Say hello", "on mouseUp {\n    answer \"Hello, \" & word 1 of field \"name\";\n}");
    w.set(status, "text", "Card 1".into(), false);
    w.file.journal.clear();
    w.file.txn = 0;
    w.file
}

/// The game test stack: Breakout on a canvas part, all logic in the card's script.
pub fn breakout() -> StackFile {
    let file = StackFile::new("Breakout", 520, 380);
    let mut w = World::new(file);
    let stack = w.create(Kind::Stack, "Breakout", None, None);
    let bg = w.create(Kind::Background, "game", Some(stack), None);
    let card = w.create(Kind::Card, "play", Some(bg), None);
    w.file.current_card = card;
    let rect = |w: &mut World, id: u32, r: [i64; 4]| { for (k, v) in ["left", "top", "width", "height"].iter().zip(r) { w.set(id, k, Value::Int(v), false); } };
    let canvas = w.create(Kind::Canvas, "playfield", Some(card), None);
    rect(&mut w, canvas, [20, 60, 480, 300]);
    let score = w.create(Kind::Field, "score", Some(card), None);
    rect(&mut w, score, [20, 14, 300, 34]);
    w.set(score, "locked", true.into(), false);
    let restart = w.create(Kind::Button, "Restart", Some(card), None);
    rect(&mut w, restart, [400, 14, 100, 34]);
    w.set(restart, "text", "Restart".into(), false);
    w.set(restart, "script", "on mouseUp {\n    send \"restart\";\n}".into(), false);
    w.set(card, "script", BREAKOUT.into(), false);
    w.file.journal.clear();
    w.file.txn = 0;
    w.file
}

pub const BREAKOUT: &str = r##"on openCard {
    send "restart";
}

on restart {
    global ballX, ballY, dx, dy, paddleX, bricks, score, lives;
    ballX = 240.0; ballY = 220.0; dx = 3.0; dy = -3.0;
    paddleX = 200.0; score = 0; lives = 3;
    bricks = [];
    for row in 0..5 {
        for col in 0..10 {
            bricks.push(#{ x: 6.0 + col * 47.0, y: 30.0 + row * 18.0, alive: true });
        }
    }
}

on frame {
    global ballX, ballY, dx, dy, paddleX, bricks, score, lives;
    if lives > 0 {
        paddleX = the mouseH - the left of canvas "playfield" - 40;
        if isKeyDown("ArrowLeft") { paddleX -= 8; }
        if isKeyDown("ArrowRight") { paddleX += 8; }
        ballX += dx;
        ballY += dy;
        if ballX < 6 or ballX > 474 { dx = -dx; }
        if ballY < 6 { dy = -dy; }
        if ballY > 274 and ballY < 284 and ballX > paddleX and ballX < paddleX + 80 { dy = -abs(dy); }
        if ballY > 300 {
            lives -= 1;
            ballX = 240.0; ballY = 220.0; dy = -3.0;
        }
        for i in 0..bricks.len() {
            let b = bricks[i];
            if b.alive and ballX > b.x and ballX < b.x + 44 and ballY > b.y and ballY < b.y + 14 {
                bricks[i].alive = false;
                dy = -dy;
                score += 10;
            }
        }
    }
    clear();
    for b in bricks {
        if b.alive { rect(b.x, b.y, 44, 14, "#c0392b"); }
    }
    rect(paddleX, 280, 80, 8, "#1b2530");
    circle(ballX, ballY, 6, "#1b5cc0");
    if lives <= 0 { text(150, 170, "Game over. Press Restart.", "#1b2530"); }
    put "Score " & score & "   Lives " & lives into field "score";
}
"##;

/// The Home stack used when none exists yet. Its handlers run for every stack, after the stack's own.
pub fn home() -> crate::model::Obj {
    crate::model::Obj { id: crate::model::HOME, kind: Kind::Stack, name: "Home".into(), parent: None, props: Default::default(),
        script: "// Home: handlers here run for every stack, after the stack's own script.\n\non help {\n    answer \"Messages travel: part, card, background, stack, Home.\";\n}\n".into() }
}

// ---------------------------------------------------------------------------------------------
// Pillars: the multimedia test stack. A port of the pillar puzzle from Myst (1993), with
// pictures drawn as SVG and sounds synthesized here, so the stack carries no third-party media.

const W: i64 = 640;
const PILLAR_X: [i64; 8] = [40, 115, 190, 265, 340, 415, 490, 565];

fn svg(body: &str, w: i64, h: i64) -> Vec<u8> {
    format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}">{body}</svg>"#).into_bytes()
}

fn shore_svg() -> Vec<u8> {
    let pillars: String = (0..8).map(|i| { let x = 205 + i * 17; format!(r##"<rect x="{x}" y="{}" width="9" height="34" fill="#d9d2c0"/>"##, 112 + (i as i64 - 3).abs() * 3) }).collect();
    svg(&format!(r##"<defs><linearGradient id="sky" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#2c3e66"/><stop offset="1" stop-color="#e3a76f"/></linearGradient>
<linearGradient id="sea" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#2d5d73"/><stop offset="1" stop-color="#0f2a38"/></linearGradient></defs>
<rect width="640" height="340" fill="url(#sky)"/><circle cx="520" cy="92" r="26" fill="#f6d9a8"/>
<rect y="190" width="640" height="150" fill="url(#sea)"/>
<path d="M120 200 C170 120 230 96 290 104 C350 112 400 150 430 200 Z" fill="#3b4a35"/>
<path d="M150 200 C200 150 250 140 300 146 C350 152 380 176 400 200 Z" fill="#53633f"/>{pillars}
<rect x="470" y="196" width="150" height="10" fill="#6b4a2f"/><rect x="480" y="206" width="6" height="40" fill="#4a3220"/><rect x="600" y="206" width="6" height="40" fill="#4a3220"/>
<path d="M0 260 Q80 252 160 260 T320 260 T480 260 T640 260" stroke="#7fb0c4" stroke-width="2" fill="none" opacity=".5"/>"##), W, 340)
}

const SYMBOLS: [&str; 8] = [
    r##"<circle cx="25" cy="40" r="11" fill="none" stroke="#3a3326" stroke-width="3"/>"##,
    r##"<path d="M25 27 L37 51 L13 51 Z" fill="none" stroke="#3a3326" stroke-width="3"/>"##,
    r##"<rect x="14" y="29" width="22" height="22" fill="none" stroke="#3a3326" stroke-width="3"/>"##,
    r##"<path d="M25 27 V53 M12 40 H38" stroke="#3a3326" stroke-width="3"/>"##,
    r##"<path d="M10 40 Q17 30 25 40 T40 40" fill="none" stroke="#3a3326" stroke-width="3"/>"##,
    r##"<path d="M25 26 L29 36 L40 37 L31 44 L34 55 L25 49 L16 55 L19 44 L10 37 L21 36 Z" fill="none" stroke="#3a3326" stroke-width="2.5"/>"##,
    r##"<path d="M25 26 L38 40 L25 54 L12 40 Z" fill="none" stroke="#3a3326" stroke-width="3"/>"##,
    r##"<path d="M30 28 A12 12 0 1 0 30 52 A9 9 0 1 1 30 28 Z" fill="none" stroke="#3a3326" stroke-width="2.5"/>"##,
];

fn pillars_svg() -> Vec<u8> {
    let mut body = String::from(r##"<rect width="640" height="340" fill="#5d7f9a"/><rect y="240" width="640" height="100" fill="#8c8466"/><path d="M0 240 H640" stroke="#6d664e" stroke-width="3"/>"##);
    for (i, x) in PILLAR_X.iter().enumerate() {
        body += &format!(r##"<g transform="translate({x} 90)"><rect width="50" height="180" fill="#d9d2c0" stroke="#9c947c" stroke-width="2"/><rect x="-4" y="-8" width="58" height="12" fill="#c8c0aa"/>{}</g>"##, SYMBOLS[i]);
    }
    svg(&body, W, 340)
}

fn glow_svg() -> Vec<u8> {
    svg(r##"<defs><radialGradient id="g"><stop offset="0" stop-color="#ffe9a0" stop-opacity=".95"/><stop offset=".6" stop-color="#ffcf4d" stop-opacity=".45"/><stop offset="1" stop-color="#ffcf4d" stop-opacity="0"/></radialGradient></defs>
<ellipse cx="40" cy="105" rx="40" ry="105" fill="url(#g)"/>"##, 80, 210)
}

fn dock_svg() -> Vec<u8> {
    svg(&r##"<rect width="640" height="340" fill="#7c94a8"/><rect y="170" width="640" height="170" fill="#24495c"/>
<rect x="0" y="230" width="300" height="16" fill="#6b4a2f"/>{}<path d="M0 290 Q100 282 200 290 T400 290 T640 290" stroke="#7fb0c4" stroke-width="2" fill="none" opacity=".5"/>"##
        .replace("{}", &(0..6).map(|i| format!(r##"<rect x="{}" y="246" width="8" height="70" fill="#4a3220"/>"##, 20 + i * 52)).collect::<String>()), W, 340)
}

fn ship_svg(up: bool) -> Vec<u8> {
    let hull = r##"<path d="M10 140 H230 L205 185 H35 Z" fill="#5a3b25"/><rect x="115" y="20" width="8" height="120" fill="#3d2818"/><path d="M123 30 L190 110 H123 Z" fill="#e8dcc0"/>"##;
    let body = if up { hull.to_string() } else { r##"<rect x="115" y="120" width="8" height="40" fill="#3d2818"/><path d="M60 170 Q120 160 180 170" stroke="#9cc3d4" stroke-width="2" fill="none"/>"##.to_string() };
    svg(&body, 240, 200)
}

/// 16-bit mono WAV from samples in -1..1.
fn wav(samples: &[f32], rate: u32) -> Vec<u8> {
    let data_len = (samples.len() * 2) as u32;
    let mut b = Vec::with_capacity(44 + data_len as usize);
    b.extend_from_slice(b"RIFF"); b.extend_from_slice(&(36 + data_len).to_le_bytes()); b.extend_from_slice(b"WAVEfmt ");
    b.extend_from_slice(&16u32.to_le_bytes()); b.extend_from_slice(&1u16.to_le_bytes()); b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&rate.to_le_bytes()); b.extend_from_slice(&(rate * 2).to_le_bytes()); b.extend_from_slice(&2u16.to_le_bytes()); b.extend_from_slice(&16u16.to_le_bytes());
    b.extend_from_slice(b"data"); b.extend_from_slice(&data_len.to_le_bytes());
    for s in samples { b.extend_from_slice(&((s.clamp(-1.0, 1.0) * 32000.0) as i16).to_le_bytes()); }
    b
}

fn click_wav() -> Vec<u8> {
    let rate = 22050;
    let n = (rate as f32 * 0.16) as usize;
    let s: Vec<f32> = (0..n).map(|i| { let t = i as f32 / rate as f32; (t * 2.0 * std::f32::consts::PI * 660.0).sin() * (-t * 30.0).exp() * 0.8 }).collect();
    wav(&s, rate)
}

fn ship_wav() -> Vec<u8> {
    // A low rumble that rises in pitch: deterministic noise plus a sweep, faded in and out.
    let rate = 22050;
    let n = (rate as f32 * 1.8) as usize;
    let mut seed: u32 = 12345;
    let mut lp = 0.0f32;
    let mut phase = 0.0f32;
    let s: Vec<f32> = (0..n).map(|i| {
        let t = i as f32 / n as f32;
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let noise = (seed >> 9) as f32 / (1u32 << 23) as f32 * 2.0 - 1.0;
        lp += (noise - lp) * 0.05;
        phase += 2.0 * std::f32::consts::PI * (55.0 + 110.0 * t) / rate as f32;
        let env = (t * 8.0).min(1.0) * ((1.0 - t) * 4.0).min(1.0);
        (lp * 2.5 + phase.sin() * 0.5) * env * 0.7
    }).collect();
    wav(&s, rate)
}

pub const PILLARS_CARD: &str = r##"// Ported from the pillar puzzle in Myst (1993). Original handler: Cyan Inc.;
// script dump by Andrew Plotkin (erkyrath/mystextract). Cyan's XCMDs are replaced by built-ins.

on openCard {
    global pillars;
    put "Eight pillars, each with a symbol. Someone scratched in the sand: 2, 5, 6." into field "status";
    for n in 1..9 {
        if item n of pillars == "on" { show picture ("lit " & n); } else { hide picture ("lit " & n); }
    }
}

on mouseUp {
    global pillars, boat;
    let name = the name of target;
    if word 1 of name != "pillar" { pass; }
    let n = num(word 2 of name);
    if pillars == "" { pillars = "off,off,off,off,off,off,off,off"; }
    play stop;
    play "click";
    if item n of pillars == "on" {
        put "off" into item n of pillars;
        hide picture ("lit " & n);
    } else {
        put "on" into item n of pillars;
        show picture ("lit " & n);
    }
    if pillars == "off,on,off,off,on,on,off,off" {
        boat = "up";
        play "ship";
        put "Something heavy moves in the water by the dock." into field "status";
    } else if boat == "up" {
        boat = "down";
        play "ship";
        put "Something sinks back into the water." into field "status";
    }
}
"##;

pub const DOCK_CARD: &str = r##"on openCard {
    global boat;
    if boat == "up" {
        show picture "ship up";
        hide picture "ship down";
        put "A ship has risen beside the dock." into field "status";
    } else {
        hide picture "ship up";
        show picture "ship down";
        put "A mast breaks the surface. The rest lies under the water." into field "status";
    }
}
"##;

pub fn pillars() -> StackFile {
    let mut file = StackFile::new("Pillars", W, 400);
    file.add_asset("shore", "image/svg+xml", shore_svg());
    file.add_asset("pillars", "image/svg+xml", pillars_svg());
    file.add_asset("glow", "image/svg+xml", glow_svg());
    file.add_asset("dock", "image/svg+xml", dock_svg());
    file.add_asset("ship up", "image/svg+xml", ship_svg(true));
    file.add_asset("ship down", "image/svg+xml", ship_svg(false));
    file.add_asset("click", "audio/wav", click_wav());
    file.add_asset("ship", "audio/wav", ship_wav());
    let mut w = World::new(file);
    let stack = w.create(Kind::Stack, "Pillars", None, None);
    let bg = w.create(Kind::Background, "scene", Some(stack), None);
    let rect = |w: &mut World, id: u32, r: [i64; 4]| { for (k, v) in ["left", "top", "width", "height"].iter().zip(r) { w.set(id, k, Value::Int(v), false); } };
    let status = w.create(Kind::Field, "status", Some(bg), None);
    rect(&mut w, status, [20, 350, 600, 36]);
    w.set(status, "locked", true.into(), false);
    w.set(status, "sharedText", true.into(), false);
    let picture = |w: &mut World, card: u32, name: &str, asset: &str, r: [i64; 4], visible: bool| {
        let id = w.create(Kind::Picture, name, Some(card), None);
        rect(w, id, r);
        w.set(id, "picture", asset.into(), false);
        if !visible { w.set(id, "visible", false.into(), false); }
        id
    };
    let hotspot = |w: &mut World, card: u32, name: &str, r: [i64; 4], script: &str| {
        let id = w.create(Kind::Button, name, Some(card), None);
        rect(w, id, r);
        w.set(id, "style", "transparent".into(), false);
        if !script.is_empty() { w.set(id, "script", script.into(), false); }
        id
    };

    let shore = w.create(Kind::Card, "shore", Some(bg), None);
    picture(&mut w, shore, "view", "shore", [0, 0, W, 340], true);
    hotspot(&mut w, shore, "island", [120, 96, 310, 104], "on mouseUp {\n    go to card \"pillars\" with dissolve;\n}");
    hotspot(&mut w, shore, "dock", [466, 180, 160, 80], "on mouseUp {\n    go to card \"dock\" with wipe left;\n}");
    w.set(shore, "script", "on openCard {\n    put \"An island at dusk. Pillars stand on the hill; a dock lies to the east.\" into field \"status\";\n}\n".into(), false);

    let pil = w.create(Kind::Card, "pillars", Some(bg), Some(shore));
    picture(&mut w, pil, "view", "pillars", [0, 0, W, 340], true);
    for (i, x) in PILLAR_X.iter().enumerate() { picture(&mut w, pil, &format!("lit {}", i + 1), "glow", [x - 15, 75, 80, 210], false); }
    for (i, x) in PILLAR_X.iter().enumerate() { hotspot(&mut w, pil, &format!("pillar {}", i + 1), [*x, 82, 50, 188], ""); }
    let back = w.create(Kind::Button, "back", Some(pil), None);
    rect(&mut w, back, [20, 290, 130, 34]);
    w.set(back, "text", "◀ To the shore".into(), false);
    w.set(back, "script", "on mouseUp {\n    go to card \"shore\" with dissolve;\n}".into(), false);
    w.set(pil, "script", PILLARS_CARD.into(), false);

    let dock = w.create(Kind::Card, "dock", Some(bg), Some(pil));
    picture(&mut w, dock, "view", "dock", [0, 0, W, 340], true);
    picture(&mut w, dock, "ship down", "ship down", [340, 110, 240, 200], true);
    picture(&mut w, dock, "ship up", "ship up", [340, 110, 240, 200], false);
    let back = w.create(Kind::Button, "back", Some(dock), None);
    rect(&mut w, back, [20, 290, 130, 34]);
    w.set(back, "text", "◀ To the shore".into(), false);
    w.set(back, "script", "on mouseUp {\n    go to card \"shore\" with wipe right;\n}".into(), false);
    w.set(dock, "script", DOCK_CARD.into(), false);

    w.file.current_card = shore;
    w.set(status, "text", "An island at dusk. Pillars stand on the hill; a dock lies to the east.".into(), false);
    w.file.journal.clear();
    w.file.txn = 0;
    w.file
}

/// An empty stack: one background, one card. The starting point for building inside the player.
pub fn blank(name: &str) -> StackFile {
    let mut w = World::new(StackFile::new(name, 640, 400));
    let stack = w.create(Kind::Stack, name, None, None);
    let bg = w.create(Kind::Background, "background 1", Some(stack), None);
    let c = w.create(Kind::Card, "card 1", Some(bg), None);
    w.file.current_card = c;
    w.file.journal.clear();
    w.file
}

/// The built-in demo stacks by name, so `go stack "Breakout"` and `start using` find them anywhere.
pub fn by_name(name: &str) -> Option<StackFile> {
    match name.trim().trim_end_matches(".stack").to_lowercase().as_str() {
        "address book" | "addressbook" => Some(address_book()),
        "breakout" => Some(breakout()),
        "pillars" => Some(pillars()),
        "self test" | "selftest" => Some(self_test()),
        _ => None,
    }
}

// ---------------------------------------------------------------------------------------------
// Self Test: checks itself from buttons. The automatic checks write PASS or FAIL lines; the manual ones
// say what you should see. Written in HyperTalk's own block syntax.

pub const SELF_TEST_STACK: &str = r#"-- Self Test. Each check writes PASS or FAIL into the results field.

on runAll
  put "Self test, " & the long date & ", " & the time & return into field "results"
  testLanguage
  testChunks
  testFiles
  testStacks
  testNavigation
  send "probe" to button "Run all tests"
  put return & countOf("PASS") & " passed, " & countOf("FAIL") & " failed." after field "results"
  put the long date && the time & ": " & countOf("PASS") & " passed, " & countOf("FAIL") & " failed" into file "selftest-last.txt"
end runAll

-- On opening, report the last run, read from a file: on the web this also checks that the browser's
-- files are restored before the stack opens.
on openStack
  if there is a file "selftest-last.txt" then put return & "Last run (from selftest-last.txt): " & file "selftest-last.txt" & return after field "results"
  pass openStack
end openStack

on check what, ok
  if ok then put "PASS " & what & return after field "results"
  else put "FAIL " & what & return after field "results"
end check

function countOf kind
  put 0 into n
  repeat with i = 1 to the number of lines of field "results"
    if word 1 of line i of field "results" is kind then add 1 to n
  end repeat
  return n
end countOf

on testLanguage
  check "= and <>", 2 + 2 = 4 and 3 <> 4
  check "&& joins with a space", "a" && "b" = "a b"
  check "not", not (1 = 2)
  check "is a number", "12" is a number and "x" is not a number
  check "contains", "HyperCard" contains "card"
  check "repeat with", sumTo(4) = 10
  put 0 into n
  repeat while n < 3
    add 1 to n
  end repeat
  check "repeat while", n = 3
  check "value", value("2 * 3") = 6
  check "text as numbers", "41" + 1 = 42
  check "the date", the long date contains ","
end testLanguage

function sumTo m
  put 0 into s
  repeat with i = 1 to m
    add i to s
  end repeat
  return s
end sumTo

on testChunks
  put "one two three" into t
  check "word", word 2 of t = "two"
  put "X" into word 2 of t
  check "put into a word", t = "one X three"
  put "a,b,c" into t
  delete item 2 of t
  check "delete an item", t = "a,c"
  check "ordinal", the last item of "a,b,c" = "c"
  check "range", char 2 to 4 of "abcdef" = "bcd"
  put "alpha" & return & "beta" into field "scratch"
  select line 2 of field "scratch"
  check "select", the selectedText = "beta"
end testChunks

on testFiles
  if there is a file "selftest-tmp.txt" then delete file "selftest-tmp.txt"
  put "one" & return & "two" into file "selftest-tmp.txt"
  check "write a file", there is a file "selftest-tmp.txt"
  check "read a line of a file", line 2 of file "selftest-tmp.txt" = "two"
  open file "selftest-tmp.txt"
  read from file "selftest-tmp.txt" until return
  check "read until return", it = "one" & return
  close file "selftest-tmp.txt"
  put return & "three" after file "selftest-tmp.txt"
  check "append to a file", the number of lines of file "selftest-tmp.txt" = 3
  delete file "selftest-tmp.txt"
  check "delete a file", there is not a file "selftest-tmp.txt"
end testFiles

on testStacks
  start using stack "Breakout"
  check "start using", the stacksInUse contains "Breakout"
  stop using stack "Breakout"
  check "stop using", the stacksInUse is empty
  go to stack "Nowhere"
  check "a missing stack sets the result", the result is "No such stack."
end testStacks

on testNavigation
  push card
  go to card "other"
  check "go to card", the name of this card is "other"
  pop card
  check "pop card", the name of this card is "tests"
  check "number of cards", the number of cards = 3
end testNavigation

function stackTwice x
  return x * 2
end stackTwice
"#;

pub const SELF_TEST_RUN_BUTTON: &str = r#"on mouseUp
  runAll
end mouseUp

-- Called by runAll: a function in the stack script, reached from this button.
on probe
  check "function handler on the message path", stackTwice(21) = 42
end probe
"#;

pub const SELF_TEST_MANUAL: &str = "Manual checks. Click each button and compare with what should happen.\n\n\
1  Pillars in a new tab: a second tab opens on the dock of Pillars. Close it with ×.\n\
2  Pick a file: a file dialog lists this stack's folder; the chosen path appears below.\n\
3  Play notes: you hear four harpsichord notes.\n\
4  Ask a question: type a name and press OK; it appears below.\n\
5  Timer: two seconds later, \"timer fired\" appears below.\n\
6  Fetch a page: the length of this project's README appears below. On the web, only sites that allow cross-origin reads work.\n\
7  Restart the player and open this stack again: the results of the last run are still there (no Save).\n\
8  Edit: switch to Edit, change the label or script of a button here, switch back to Browse and use it.";

pub fn self_test() -> StackFile {
    let mut w = World::new(StackFile::new("Self Test", 640, 520));
    let stack = w.create(Kind::Stack, "Self Test", None, None);
    let bg = w.create(Kind::Background, "main", Some(stack), None);
    let rect = |w: &mut World, id: u32, r: [i64; 4]| { for (k, v) in ["left", "top", "width", "height"].iter().zip(r) { w.set(id, k, Value::Int(v), false); } };
    // The results field is shared by every card, so checks can write to it from anywhere.
    let results = w.create(Kind::Field, "results", Some(bg), None);
    rect(&mut w, results, [20, 70, 600, 220]);
    w.set(results, "locked", true.into(), false);
    w.set(results, "sharedText", true.into(), false);
    w.set(results, "style", "scrolling".into(), false);
    w.set(results, "text", "Press \"Run all tests\". Then try the manual checks on the last card.".into(), false);

    let tests = w.create(Kind::Card, "tests", Some(bg), None);
    let b = w.create(Kind::Button, "Run all tests", Some(tests), None);
    rect(&mut w, b, [20, 20, 160, 34]);
    w.set(b, "text", "Run all tests".into(), false);
    w.set(b, "script", SELF_TEST_RUN_BUTTON.into(), false);
    let scratch = w.create(Kind::Field, "scratch", Some(tests), None);
    rect(&mut w, scratch, [200, 14, 240, 48]);
    let manual = w.create(Kind::Button, "Manual checks", Some(tests), None);
    rect(&mut w, manual, [460, 20, 160, 34]);
    w.set(manual, "text", "Manual checks ▶".into(), false);
    w.set(manual, "script", "on mouseUp\n  go to card \"manual\"\nend mouseUp".into(), false);

    let other = w.create(Kind::Card, "other", Some(bg), Some(tests));
    let back = w.create(Kind::Button, "Back", Some(other), None);
    rect(&mut w, back, [20, 20, 120, 34]);
    w.set(back, "text", "◀ Back".into(), false);
    w.set(back, "script", "on mouseUp\n  go to card \"tests\"\nend mouseUp".into(), false);

    let man = w.create(Kind::Card, "manual", Some(bg), Some(other));
    // On the manual card the buttons sit over the lower part of the shared results field; the card is taller.
    let note = w.create(Kind::Field, "instructions", Some(man), None);
    rect(&mut w, note, [20, 20, 600, 44]);
    w.set(note, "locked", true.into(), false);
    w.set(note, "text", "Manual checks: see the list in the results field after pressing \"List the manual checks\".".into(), false);
    let buttons: [(&str, &str); 7] = [
        ("List the manual checks", "on mouseUp\n  put manualText() into field \"results\"\nend mouseUp"),
        ("1 Pillars in a new tab", "on mouseUp\n  go to card \"dock\" of stack \"Pillars\" in a new window\nend mouseUp"),
        ("2 Pick a file", "on mouseUp\n  answer file \"Pick any file\"\nend mouseUp\non answered p\n  put return & \"picked: \" & p after field \"results\"\nend answered"),
        ("3 Play notes", "on mouseUp\n  play \"harpsichord\" tempo 160 \"c4q e g c5h\"\nend mouseUp"),
        ("4 Ask a question", "on mouseUp\n  ask \"Your name?\" with \"Ada\"\nend mouseUp\non asked n\n  put return & \"name: \" & n after field \"results\"\nend asked"),
        ("5 Timer", "on mouseUp\n  send \"late\" to me in 2 seconds\nend mouseUp\non late\n  put return & \"timer fired\" after field \"results\"\nend late"),
        ("6 Fetch a page", "on mouseUp\n  load url \"https://raw.githubusercontent.com/barafael/uberkarte/main/README.md\"\nend mouseUp\non urlLoaded u, status\n  if status is \"error\" then put return & \"fetch failed: \" & the result after field \"results\"\n  else put return & \"fetched \" & length(url u) & \" characters from \" & u after field \"results\"\nend urlLoaded"),
    ];
    for (i, (name, script)) in buttons.iter().enumerate() {
        let id = w.create(Kind::Button, name, Some(man), None);
        // Three per row, wide enough for their labels; the results field is shortened on this card's side.
        let (col, row) = ((i % 3) as i64, (i / 3) as i64);
        rect(&mut w, id, [20 + col * 202, 300 + row * 34, 196, 30]);
        w.set(id, "text", Value::Str(name.to_string()), false);
        w.set(id, "script", Value::Str(script.to_string()), false);
    }
    // The manual list lives in a function on the card, so the button above finds it on the message path.
    w.set(man, "script", Value::Str(format!("function manualText\n  return {:?}\nend manualText", SELF_TEST_MANUAL)), false);
    // Buttons on the first card sit above the shared results field; the manual card's sit below it.
    w.set(stack, "script", SELF_TEST_STACK.into(), false);
    w.file.current_card = tests;
    w.file.journal.clear();
    w.file.txn = 0;
    w.file
}

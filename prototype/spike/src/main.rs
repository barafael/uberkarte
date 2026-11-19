//! Spike for H1 (message path on stock Rhai) and H3 (English-shaped forms via custom syntax).
//! No UI. Run with `cargo run --release`.
use rhai::{Dynamic, Engine, EvalAltResult, ImmutableString, Position, Scope, AST};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::time::Instant;

type R<T> = Result<T, Box<EvalAltResult>>;

struct Obj {
    name: String,
    parent: Option<usize>,
    ast: Rc<AST>,
    props: BTreeMap<String, Dynamic>,
}
struct Frame {
    obj: usize,
    msg: String,
    args: Vec<Dynamic>,
    handled: bool,
    passed: bool,
}
#[derive(Default)]
struct World {
    objs: Vec<Obj>,
    fields: BTreeMap<String, String>,
    frames: Vec<Frame>,
    log: Vec<String>,
    card: String,
}
type W = Rc<RefCell<World>>;

fn err(s: &str) -> Box<EvalAltResult> {
    Box::new(EvalAltResult::ErrorRuntime(s.into(), Position::NONE))
}

/// Walk the message path from `start`. Returns true if some object handled the message.
fn dispatch(engine: &Engine, w: &W, start: usize, msg: &str, args: Vec<Dynamic>) -> R<bool> {
    let mut cur = Some(start);
    while let Some(id) = cur {
        let (ast, parent) = {
            let wb = w.borrow();
            (wb.objs[id].ast.clone(), wb.objs[id].parent)
        };
        w.borrow_mut().frames.push(Frame { obj: id, msg: msg.to_lowercase(), args: args.clone(), handled: false, passed: false });
        let res = engine.run_ast_with_scope(&mut Scope::new(), &ast);
        let f = w.borrow_mut().frames.pop().unwrap();
        res?;
        if f.handled && !f.passed {
            return Ok(true);
        }
        cur = parent;
    }
    Ok(false)
}

fn me(w: &W) -> usize {
    w.borrow().frames.last().map(|f| f.obj).unwrap_or(0)
}

/// A command is a message first; the built-in runs only if nothing on the path handles it.
fn command(engine: &Engine, w: &W, name: &str, args: Vec<Dynamic>, builtin: impl FnOnce(&mut World, &[Dynamic])) -> R<Dynamic> {
    let start = me(w);
    if !dispatch(engine, w, start, name, args.clone())? {
        builtin(&mut w.borrow_mut(), &args);
    }
    Ok(Dynamic::UNIT)
}

fn chunk(kind: &str, n: i64, text: &str) -> String {
    let i = (n.max(1) - 1) as usize;
    match kind {
        "word" => text.split_whitespace().nth(i).unwrap_or("").to_string(),
        "line" => text.lines().nth(i).unwrap_or("").to_string(),
        "item" => text.split(',').nth(i).unwrap_or("").to_string(),
        _ => text.chars().nth(i).map(|c| c.to_string()).unwrap_or_default(),
    }
}

fn build_engine(w: &W) -> Result<Engine, Box<dyn std::error::Error>> {
    let mut e = Engine::new();
    e.set_max_operations(5_000_000); // sandbox: runaway scripts stop

    // on <message> [params...] { ... }
    let ww = w.clone();
    e.register_custom_syntax_with_state_raw(
        "on",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            let last = syms.last().map(|s| s.as_str()).unwrap_or("");
            Ok(if last == "$block$" { None }
               else if syms.len() == 1 { *state = Dynamic::from(look.to_lowercase()); Some(look.into()) }
               else if look == "{" { Some("$block$".into()) }
               else { Some("$ident$".into()) })
        },
        true,
        move |ctx, inputs, state| {
            let name = state.to_string();
            let args = {
                let mut wb = ww.borrow_mut();
                let Some(f) = wb.frames.last_mut() else { return Ok(Dynamic::UNIT) };
                if f.handled || f.msg != name { return Ok(Dynamic::UNIT) }
                f.handled = true;
                f.args.clone()
            };
            let n = inputs.len();
            for (i, p) in inputs[..n - 1].iter().enumerate() {
                let pname = p.get_string_value().unwrap_or("_").to_string();
                ctx.scope_mut().push(pname, args.get(i).cloned().unwrap_or(Dynamic::from(String::new())));
            }
            match ctx.eval_expression_tree(&inputs[n - 1]) {
                Err(_) if ww.borrow().frames.last().map(|f| f.passed).unwrap_or(false) => Ok(Dynamic::UNIT),
                r => r.map(|_| Dynamic::UNIT),
            }
        },
    );

    // pass  -- leave the handler and let the message continue up the path
    let ww = w.clone();
    e.register_custom_syntax(["pass"], false, move |_, _| {
        if let Some(f) = ww.borrow_mut().frames.last_mut() { f.passed = true; }
        Err(err("__pass__"))
    })?;

    // field <expr>   (read)
    let ww = w.clone();
    e.register_custom_syntax(["field", "$expr$"], false, move |ctx, inputs| {
        let name = ctx.eval_expression_tree(&inputs[0])?.to_string();
        Ok(Dynamic::from(ww.borrow().fields.get(&name).cloned().unwrap_or_default()))
    })?;

    // word|line|item|char <n> of <expr>
    for kind in ["word", "line", "item", "char"] {
        e.register_custom_syntax([kind, "$expr$", "of", "$expr$"], false, move |ctx, inputs| {
            let n = ctx.eval_expression_tree(&inputs[0])?.as_int().map_err(|_| err("chunk index must be a number"))?;
            let text = ctx.eval_expression_tree(&inputs[1])?.to_string();
            Ok(Dynamic::from(chunk(kind, n, &text)))
        })?;
    }

    // put <expr> into <var>   |   put <expr> into field <expr>
    let ww = w.clone();
    e.register_custom_syntax_with_state_raw(
        "put",
        |syms: &[ImmutableString], look: &str, state: &mut Dynamic| {
            Ok(match syms.len() {
                1 => Some("$expr$".into()),
                2 => Some("into".into()),
                3 => { *state = Dynamic::from(look == "field"); Some(if look == "field" { "field".into() } else { "$ident$".into() }) }
                4 if syms[3] == "field" => Some("$expr$".into()),
                _ => None,
            })
        },
        true,
        move |ctx, inputs, state| {
            let v = ctx.eval_expression_tree(&inputs[0])?;
            if !state.as_bool().unwrap_or(false) {
                let var = inputs[1].get_string_value().unwrap_or("it").to_string();
                ctx.scope_mut().set_value(var, v);
            } else {
                let fname = ctx.eval_expression_tree(&inputs[1])?.to_string();
                ww.borrow_mut().fields.insert(fname, v.to_string());
            }
            Ok(Dynamic::UNIT)
        },
    );

    // set <prop> of me to <expr>
    let ww = w.clone();
    e.register_custom_syntax(["set", "$ident$", "of", "me", "to", "$expr$"], false, move |ctx, inputs| {
        let prop = inputs[0].get_string_value().unwrap_or("").to_string();
        let v = ctx.eval_expression_tree(&inputs[1])?;
        let id = me(&ww);
        ww.borrow_mut().objs[id].props.insert(prop, v);
        Ok(Dynamic::UNIT)
    })?;

    // go to card <expr>     (a command: interceptable)
    let ww = w.clone();
    e.register_custom_syntax(["go", "to", "card", "$expr$"], false, move |ctx, inputs| {
        let v = ctx.eval_expression_tree(&inputs[0])?;
        command(ctx.engine(), &ww, "go", vec![v], |w, a| w.card = a[0].to_string())
    })?;

    // answer <expr>         (a command: interceptable)
    let ww = w.clone();
    e.register_custom_syntax(["answer", "$expr$"], false, move |ctx, inputs| {
        let v = ctx.eval_expression_tree(&inputs[0])?;
        command(ctx.engine(), &ww, "answer", vec![v], |w, a| w.log.push(format!("ANSWER: {}", a[0])))
    })?;

    // repeat <n> times { ... }
    e.register_custom_syntax(["repeat", "$expr$", "times", "$block$"], true, |ctx, inputs| {
        let n = ctx.eval_expression_tree(&inputs[0])?.as_int().map_err(|_| err("repeat needs a number"))?;
        for _ in 0..n { let _ = ctx.eval_expression_tree(&inputs[1])?; }
        Ok(Dynamic::UNIT)
    })?;
    Ok(e)
}

fn add(e: &Engine, w: &W, name: &str, parent: Option<usize>, script: &str) -> usize {
    let ast = match e.compile(script) {
        Ok(a) => a,
        Err(x) => { println!("  COMPILE FAIL [{name}]: {x}"); e.compile("").unwrap() }
    };
    let mut wb = w.borrow_mut();
    wb.objs.push(Obj { name: name.into(), parent, ast: Rc::new(ast), props: BTreeMap::new() });
    wb.objs.len() - 1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let w: W = Rc::new(RefCell::new(World::default()));
    let e = build_engine(&w)?;
    w.borrow_mut().fields.insert("order".into(), "two red apples\nurgent, today, cash".into());

    let home = add(&e, &w, "Home", None, r#"
        on ping { }
    "#);
    let stack = add(&e, &w, "stack", Some(home), r#"
        on answer msg { put "stack saw: " + msg into field "trace"; pass; }
        on tick { }
    "#);
    let bg = add(&e, &w, "background", Some(stack), "");
    let card = add(&e, &w, "card", Some(bg), r#"
        on go where { put where into field "wanted"; pass; }
    "#);
    let button = add(&e, &w, "button OK", Some(card), r#"
        on mouseUp {
            put word 2 of field "order" into colour;
            put item 3 of line 2 of field "order" into field "payment";
            set hilite of me to true;
            repeat 2 times { answer "colour is " + colour; }
            go to card "receipt";
        }
    "#);

    println!("== H3: forms ==");
    let forms: &[(&str, &str)] = &[
        ("put into variable", r#"on t { put 1 + 2 into x; }"#),
        ("put into field", r#"on t { put "a" into field "f"; }"#),
        ("chunk read, nested", r#"on t { put char 1 of word 2 of field "order" into c; }"#),
        ("set property", r#"on t { set name of me to "x"; }"#),
        ("command with expr", r#"on t { answer word 1 of "hello world"; }"#),
        ("handler params", r#"on keyDown key mods { answer key; }"#),
        ("rhai if with english cmds", r#"on t { if field "order" == "" { answer "empty"; } else { go to card "next"; } }"#),
        ("no semicolons (expected fail)", "on t { put 1 into x\n answer x }"),
        ("if/then/end if (expected fail)", "on t { if 1 == 1 then answer 1 end if }"),
        ("on/end without braces (expected fail)", "on t\n answer 1;\nend t"),
        ("put into chunk (lvalue; expected fail)", r#"on t { put "z" into word 2 of field "order"; }"#),
    ];
    let (mut ok, mut total) = (0, 0);
    for (label, src) in forms {
        total += 1;
        match e.compile(*src) {
            Ok(_) => { ok += 1; println!("  ok    {label}") }
            Err(x) => println!("  FAIL  {label}: {x}"),
        }
    }
    println!("  {ok}/{total} parse");

    println!("== H1: message path ==");
    let handled = dispatch(&e, &w, button, "mouseUp", vec![])?;
    let unhandled = dispatch(&e, &w, button, "nobodyHandles", vec![])?;
    {
        let wb = w.borrow();
        println!("  mouseUp handled: {handled}");
        println!("  log: {:?}", wb.log);
        println!("  fields: {:?}", wb.fields);
        println!("  card now: {:?}  hilite: {:?}", wb.card, wb.objs[button].props.get("hilite"));
        println!("  unhandled message returns: {unhandled}");
    }
    let _ = bg;

    println!("== H1: dispatch cost ==");
    let n = 200_000;
    for (label, msg) in [("handled 3 levels up (stack)", "tick"), ("handled 4 levels up (Home)", "ping")] {
        let t = Instant::now();
        for _ in 0..n { dispatch(&e, &w, button, msg, vec![])?; }
        println!("  {label}: {:.2} µs/message", t.elapsed().as_secs_f64() * 1e6 / n as f64);
    }
    // sandbox: a runaway handler is stopped by the operation limit
    let bad = add(&e, &w, "runaway", None, "on spin { loop { } }");
    println!("== sandbox ==\n  runaway loop -> {}", match dispatch(&e, &w, bad, "spin", vec![]) { Err(x) => format!("stopped: {x}"), Ok(_) => "NOT stopped".into() });
    Ok(())
}

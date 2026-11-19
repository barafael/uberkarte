//! The HyperCard corpus explorer: one page with a SurrealQL notebook and preset questions. The corpus runs as
//! SurrealDB (WebAssembly) in a Web Worker (crates/hcworker); this crate is the UI.

mod kb;

use hccore::{Example, QueryResult};
use kb::{Kb, Status};
use leptos::prelude::*;
use serde_json::Value;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

/// The query in the page address (`#q=…`), so a result is a link.
fn query_from_hash() -> Option<String> {
    let hash = web_sys::window()?.location().hash().ok()?;
    let q = hash.strip_prefix("#q=")?;
    js_sys::decode_uri_component(q).ok()?.as_string()
}

fn set_hash(q: &str) {
    let enc: String = js_sys::encode_uri_component(q).into();
    if let Some(h) = web_sys::window().and_then(|w| w.history().ok()) {
        let _ = h.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&format!("#q={enc}")));
    }
}

async fn fetch_examples() -> Vec<Example> {
    let Some(w) = web_sys::window() else { return vec![] };
    match hccore::web::response_text(w.fetch_with_str(&format!("data/examples.json?v={}", kb::BUILD)), "examples.json").await {
        Ok(t) => serde_json::from_str(&t).unwrap_or_default(),
        Err(_) => vec![],
    }
}

const START: &str = "-- Ask the corpus anything in SurrealQL, or pick a question on the left.\nSELECT id, title, kind, confidence FROM concept WHERE kind = 'question';";

#[component]
fn App() -> impl IntoView {
    let kb = Kb::start();
    provide_context(kb);
    let examples = LocalResource::new(fetch_examples);
    let text = RwSignal::new(query_from_hash().unwrap_or_else(|| START.to_string()));
    let result = RwSignal::new(None::<QueryResult>);
    let running = RwSignal::new(false);
    let as_json = RwSignal::new(false);
    let picked = RwSignal::new(None::<usize>);

    let run = move || {
        let q = text.get_untracked();
        set_hash(&q);
        running.set(true);
        leptos::task::spawn_local(async move {
            let r = kb.query(q).await;
            result.set(Some(r));
            running.set(false);
        });
    };
    // Run the starting query (or the one in the address) once the corpus is loaded.
    let ready = Memo::new(move |_| matches!(kb.status.get(), Status::Ready { .. }));
    Effect::new(move |done: Option<bool>| {
        if done == Some(true) { return true; }
        if ready.get() { run(); true } else { false }
    });
    let open = Callback::new(move |q: String| { text.set(q); picked.set(None); run(); });

    view! {
        <header>
            <h1>"HyperCard corpus"</h1>
            <span class="sub">"a knowledge graph about HyperCard and HyperTalk, queried in your browser"</span>
            <div class="side">
                <LoadStatus />
                <a href="./player/">"Open the player ▸"</a>
            </div>
        </header>
        <main>
            <aside>
                <h2>"Questions"</h2>
                <ol>
                    {move || examples.get().map(|list| list.iter().enumerate().map(|(i, ex)| {
                        let q = ex.query.clone();
                        let title = ex.title.clone();
                        view! {
                            <li><button class:on=move || picked.get() == Some(i) title=q.clone()
                                on:click=move |_| { text.set(q.clone()); picked.set(Some(i)); run(); }>{title}</button></li>
                        }
                    }).collect_view())}
                </ol>
            </aside>
            <section>
                <h2>"Notebook"</h2>
                <textarea
                    aria-label="SurrealQL query"
                    spellcheck="false"
                    prop:value=move || text.get()
                    on:input=move |ev| text.set(event_target_value(&ev))
                    on:keydown=move |ev: web_sys::KeyboardEvent| {
                        if ev.key() == "Enter" && (ev.ctrl_key() || ev.meta_key()) { ev.prevent_default(); run(); }
                    }
                ></textarea>
                <div class="row">
                    <button class="run" disabled=move || !ready.get() || running.get() on:click=move |_| run()>
                        {move || if running.get() { "Running…" } else { "Run" }}
                    </button>
                    <span class="hint">"Ctrl+Enter runs. Read-only: writes change nothing."</span>
                    <button class="toggle" on:click=move |_| as_json.update(|j| *j = !*j)>
                        {move || if as_json.get() { "Show tables" } else { "Show JSON" }}
                    </button>
                    {move || result.get().map(|r| view! { <span class="hint">{format!("{:.1} ms", r.ms)}</span> })}
                </div>
                {move || result.get().map(|r| view! { <Results result=r as_json=as_json.get() open=open /> })}
            </section>
        </main>
        <footer>
            "The corpus is SurrealDB 4 compiled to WebAssembly, loaded into memory in your browser on each visit and then sealed read-only; "
            "your queries never leave your machine. SurrealDB is under the "
            <a href="https://github.com/surrealdb/surrealdb/blob/main/LICENSE">"Business Source License 1.1"</a>". "
            "After the omdurman and bavovniatko explorers."
        </footer>
    }
}

#[component]
fn LoadStatus() -> impl IntoView {
    let kb = expect_context::<Kb>();
    move || match kb.status.get() {
        Status::Loading(p) => {
            let pct = (p.done_bytes as f64 / p.total_bytes.max(1) as f64 * 100.0).round();
            view! { <span class="status" title=p.file.clone()>{format!("Loading the corpus … {pct}%")}</span> }.into_any()
        }
        Status::Ready { rows, ms } => view! { <span class="status">{format!("{rows} records · ready in {:.1} s", ms / 1000.0)}</span> }.into_any(),
        Status::Failed(e) => view! { <span class="status failed">{format!("The corpus failed to load: {e}")}</span> }.into_any(),
    }
}

#[component]
fn Results(result: QueryResult, as_json: bool, open: Callback<String>) -> impl IntoView {
    if let Some(e) = result.error.clone() {
        return view! { <p class="error">{e}</p> }.into_any();
    }
    let n = result.statements.len();
    result.statements.into_iter().enumerate().map(|(i, s)| {
        let head = if n > 1 { format!("Statement {}", i + 1) } else { String::new() };
        let count = match &s.result { Value::Array(a) => format!("{} row{}", a.len(), if a.len() == 1 { "" } else { "s" }), _ => String::new() };
        view! {
            <div class="stmt">
                <h3>{format!("{head}{}{count}", if head.is_empty() || count.is_empty() { "" } else { " · " })}</h3>
                {match s.error {
                    Some(e) => view! { <p class="error">{e}</p> }.into_any(),
                    None if as_json => view! { <pre>{serde_json::to_string_pretty(&s.result).unwrap_or_default()}</pre> }.into_any(),
                    None => view! { <Table value=s.result open=open /> }.into_any(),
                }}
            </div>
        }
    }).collect_view().into_any()
}

/// Rows of objects as a table (columns in first-seen order); anything else as a single cell.
#[component]
fn Table(value: Value, open: Callback<String>) -> impl IntoView {
    let rows: Vec<Value> = match value { Value::Array(a) => a, v => vec![v] };
    let objects = !rows.is_empty() && rows.iter().all(|r| r.is_object());
    if !objects {
        return view! { <div class="wide"><table><tbody>
            {rows.into_iter().take(500).map(|r| view! { <tr><td><Cell value=r open=open /></td></tr> }).collect_view()}
        </tbody></table></div> }.into_any();
    }
    let mut cols: Vec<String> = vec![];
    for r in &rows {
        for k in r.as_object().unwrap().keys() { if !cols.contains(k) { cols.push(k.clone()); } }
    }
    // `id` first, as it is the handle for every row.
    if let Some(p) = cols.iter().position(|c| c == "id") { let id = cols.remove(p); cols.insert(0, id); }
    let shown = rows.len().min(500);
    view! {
        <div class="wide"><table>
            <thead><tr>{cols.iter().map(|c| view! { <th>{c.clone()}</th> }).collect_view()}</tr></thead>
            <tbody>{rows.into_iter().take(shown).map(|r| {
                let o = r.as_object().cloned().unwrap_or_default();
                view! { <tr>{cols.iter().map(|c| view! { <td><Cell value=o.get(c).cloned().unwrap_or(Value::Null) open=open /></td> }).collect_view()}</tr> }
            }).collect_view()}</tbody>
        </table></div>
    }.into_any()
}

/// One value: record ids are buttons that open the record; lists and objects are shown inline.
#[component]
fn Cell(value: Value, open: Callback<String>) -> impl IntoView {
    let kb = expect_context::<Kb>();
    match value {
        Value::Null => view! { <span class="val-null">"—"</span> }.into_any(),
        Value::String(s) if kb.is_record(&s) => {
            let id = s.clone();
            view! { <button class="rec" title="Open this record" on:click=move |_| open.run(format!(
                "SELECT * FROM {id};\nSELECT meta::tb(id) AS edge, `in` AS `from`, `out` AS `to`, * OMIT id, `in`, `out` FROM {id}<-?, {id}->?;"))>{s}</button> }.into_any()
        }
        Value::String(s) => view! { <span>{s}</span> }.into_any(),
        Value::Array(a) if a.is_empty() => view! { <span class="val-null">"[]"</span> }.into_any(),
        Value::Array(a) => view! { <span class="list">{a.into_iter().enumerate().map(|(i, v)| view! {
            {(i > 0).then_some(", ")}<Cell value=v open=open />
        }).collect_view()}</span> }.into_any(),
        Value::Object(o) => view! { <span class="obj">"{ "{o.into_iter().enumerate().map(|(i, (k, v))| view! {
            {(i > 0).then_some(", ")}{format!("{k}: ")}<Cell value=v open=open />
        }).collect_view()}" }"</span> }.into_any(),
        v => view! { <span>{v.to_string()}</span> }.into_any(),
    }
}


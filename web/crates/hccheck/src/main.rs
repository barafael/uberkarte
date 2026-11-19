//! Load crates/hcapp/public/data exactly as the browser worker does, seal, then:
//! every preset question must run without error as the guest, writes must change nothing,
//! and schema statements must be refused. Exits non-zero otherwise (build.sh and CI run it).

use hccore::{Example, Kb, Source};

struct Dir(std::path::PathBuf);

impl Source for Dir {
    async fn text(&self, path: &str) -> Result<String, String> {
        std::fs::read_to_string(self.0.join(path)).map_err(|e| format!("{path}: {e}"))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let dir = std::path::PathBuf::from(std::env::args().nth(1).unwrap_or_else(|| "crates/hcapp/public/data".into()));
    let mut failures = vec![];
    let t0 = std::time::Instant::now();
    let mut kb = Kb::open().await.expect("open");
    let (manifest, tables) = kb.load(&Dir(dir.clone()), |_| {}).await.unwrap_or_else(|e| panic!("load: {e}"));
    kb.seal().await.expect("seal");
    let rows: u64 = tables.values().sum();
    println!("loaded {} files, {} tables, {rows} records in {:.0} ms", manifest.load.len(), tables.len(), t0.elapsed().as_secs_f64() * 1e3);

    // Every preset question, as the guest.
    let examples: Vec<Example> = serde_json::from_str(&std::fs::read_to_string(dir.join("examples.json")).expect("examples.json")).expect("examples");
    for ex in &examples {
        let r = kb.query(&ex.query).await;
        match r.first_error() {
            Some(e) => failures.push(format!("question \"{}\": {e}", ex.title)),
            None if r.statements.iter().all(|s| s.result.as_array().is_some_and(|a| a.is_empty()) || s.result.is_null()) => {
                failures.push(format!("question \"{}\" returns nothing", ex.title))
            }
            None => {}
        }
    }
    println!("{} preset questions run", examples.len());

    // Read-only: writes are no-ops, schema statements fail.
    let count = |q: &'static str| async { kb.query(q).await.statements.last().map(|s| s.result.clone()) };
    let before = count("RETURN count(SELECT id FROM concept);").await;
    for w in ["CREATE concept:probe SET title = 'x', kind = 'idea', summary = 'x';", "UPDATE concept SET title = 'changed';", "DELETE concept;",
              "RELATE concept:stack->influenced->concept:card;"] {
        let _ = kb.query(w).await;
    }
    let after = count("RETURN count(SELECT id FROM concept);").await;
    if before != after { failures.push(format!("a write got through: {before:?} → {after:?}")); }
    let changed = kb.query("SELECT VALUE title FROM concept:stack;").await;
    if changed.statements.last().map(|s| s.result.to_string()).unwrap_or_default().contains("changed") { failures.push("UPDATE changed a record".into()); }
    for s in ["DEFINE TABLE probe;", "REMOVE TABLE concept;", "INFO FOR DB;"] {
        if kb.query(s).await.first_error().is_none() { failures.push(format!("schema statement allowed: {s}")); }
    }

    if failures.is_empty() {
        println!("ok: read-only, every question answers");
    } else {
        for f in &failures { eprintln!("FAIL {f}"); }
        std::process::exit(1);
    }
}

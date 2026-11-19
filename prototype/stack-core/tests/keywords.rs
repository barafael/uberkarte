//! H3: how much of HyperTalk's vocabulary maps onto this language. Reads tests/keywords.toml,
//! runs every `ours` snippet in a fresh stack, checks every `test` reference, prints the tally.
use serde::Deserialize;
use stack_core::{demo, Rt};
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct Table { kw: Vec<Row> }
#[derive(Deserialize)]
struct Row { id: String, cls: String, #[serde(default)] ours: String, #[serde(default)] test: String }

#[test]
fn keyword_coverage() {
    let table: Table = toml::from_str(include_str!("keywords.toml")).unwrap();
    let tests = include_str!("path.rs");
    let mut failures = vec![];
    let mut by_class: BTreeMap<String, usize> = BTreeMap::new();
    for r in &table.kw {
        *by_class.entry(r.cls.clone()).or_default() += 1;
        let present = matches!(r.cls.as_str(), "clean" | "partial" | "renamed");
        if present && r.ours.is_empty() && r.test.is_empty() && r.id != "op_precedence" { failures.push(format!("{}: classed {} with no evidence", r.id, r.cls)); }
        if !r.test.is_empty() && !tests.contains(&format!("fn {}()", r.test)) { failures.push(format!("{}: no test named {}", r.id, r.test)); }
        if !r.ours.is_empty() {
            let rt = Rt::with_home(demo::address_book(), Some(demo::home()));
            rt.exec(&r.ours);
            let err = rt.w.borrow().last_error.clone();
            if let Some(e) = err { failures.push(format!("{}: `{}` failed: {e}", r.id, r.ours)); }
        }
    }
    let total = table.kw.len();
    let na = by_class.get("na").copied().unwrap_or(0);
    let clean = by_class.get("clean").copied().unwrap_or(0);
    let partial = by_class.get("partial").copied().unwrap_or(0);
    let renamed = by_class.get("renamed").copied().unwrap_or(0);
    println!("H3 coverage over {total} keywords: {by_class:?}");
    println!("  clean: {clean}/{total} = {:.0}%   clean+partial: {:.0}%   any equivalent: {:.0}%   (excluding {na} n/a: clean {:.0}%, any {:.0}%)",
        100.0 * clean as f64 / total as f64, 100.0 * (clean + partial) as f64 / total as f64, 100.0 * (clean + partial + renamed) as f64 / total as f64,
        100.0 * clean as f64 / (total - na) as f64, 100.0 * (clean + partial + renamed) as f64 / (total - na) as f64);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

# The corpus explorer

A static site for asking the corpus questions in SurrealQL. The whole database runs **in the visitor's
browser**: SurrealDB 4.0 compiled to WebAssembly, in memory, in a Web Worker. It loads the corpus on every
visit (about 2 s) and is then sealed read-only. There is no server. The architecture follows the omdurman
rulebook archive and the bavovniatko explorer, cut down to one page: a notebook and the preset questions.

```
../surql/*.surql, ../queries/examples.surql
        └─ export.py ─▶ crates/hcapp/public/data/   the corpus files with select-only permissions,
                                                     examples.json (the preset questions), manifest.json
crates/hccore    open in-memory SurrealDB, load the manifest as root, seal to a guest session, query → JSON
crates/hcworker  Web Worker hosting hccore; JSON messages to and from the app
crates/hcapp     Leptos UI (client-side): the notebook, the preset questions, results as tables or JSON
crates/hccheck   native: load the corpus exactly as the worker does, run every preset question as the
                 guest, probe writes and schema statements
```

## Running it

```sh
python3 web/export.py                            # after any change to surql/ or queries/examples.surql
(cd web && cargo run --release -p hccheck)       # every question must answer; writes must change nothing
(cd web/crates/hcapp && trunk serve --release)   # http://127.0.0.1:8772
web/build.sh                                     # production build → web/crates/hcapp/dist
PUBLIC_URL=/repo/ web/build.sh                   # when served from a sub-path (GitHub Pages)
```

`web/` is its own Cargo workspace. It needs Rust stable with `wasm32-unknown-unknown`, `trunk`, and
optionally `wasm-opt`.

## The page

- **Questions** (left): the numbered examples in `queries/examples.surql`. Clicking one puts it in the
  notebook and runs it.
- **Notebook**: any SurrealQL, any number of statements. Ctrl+Enter runs. Each statement's result shows as
  a table, or as JSON. The query is kept in the page address (`#q=…`), so a result is a link.
- **Record ids** in results are links: clicking one shows the record and every edge in and out of it.

## How it stays read-only

`Kb::open` starts SurrealDB with a random root password that never leaves the worker, and with guest access.
The corpus loads as root; `Kb::seal` then drops to an anonymous guest session for good. `export.py` gives
every table `PERMISSIONS FOR select FULL` and nothing more, so writes are no-ops and `DEFINE`, `REMOVE` and
`INFO` fail. `hccheck` asserts this on every build.

## Published

The GitHub Actions workflow (`.github/workflows/pages.yml`) builds this site and the stack player and
publishes both: the explorer at the site root, the player under `player/`.

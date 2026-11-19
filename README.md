# HyperCard Corpus — a SurrealDB knowledge graph

Design input for building a **2026 successor to HyperCard/HyperTalk**. It holds
HyperCard/HyperTalk concepts, tensions, modern analogs and open questions as
nodes + typed graph edges in SurrealDB 4.0, with a **HyperTalk language
reference** (one `keyword` node per command, function, control structure,
chunk, container, operator, constant, system message, and property) and a
**code corpus** (`code` nodes holding verbatim HyperTalk from Wikipedia,
*Tricks of the HyperTalk Masters*, and the Myst (1993) script dumps).

**In the browser:** `web/` is a static site that runs this corpus as SurrealDB in
WebAssembly, with a SurrealQL notebook and preset questions (see `web/README.md`).
`prototype/` is the successor experiment itself: a Rust stack engine with an
embedded scripting language and a Dioxus player (see `notes/experiment-plan.md`).
GitHub Actions publishes both to GitHub Pages: the explorer at the site root, the
player under `player/` (`.github/workflows/pages.yml`).

Historical research is not the goal. History nodes are kept only where a design
argument leans on them; the rest was pruned on 2026-11-06 (see
`notes/pruning-2026-11-06.md`). The schema is still direction-agnostic: the
graph records modern analogs, tensions and open questions without committing to
a modernization path.

**Current size:** 351 nodes (87 concepts, 149 keywords, 28 code samples,
26 artifacts, 12 persons, 3 orgs, 15 events, 24 sources, 7 threads),
865 edges across 33 edge tables. One weakly connected component, no isolated
nodes — but that holds only because `in_thread` edges are counted; without them
there are 8 components (`python3 queries/analyze.py` prints the breakdown).

**Fact-check status:** every layer was checked against primary sources on
2026-11-06 and corrected; unverifiable claims were softened or removed, not
kept. Read `notes/pruning-2026-11-06.md` before relying on any node. The
earlier ledger, `notes/reconciliation.md`, covers the coverage-critique pass
and is partly superseded. `archive/pre-prune-2026-11-06/` holds the corpus as
it was before pruning (422 nodes / 1023 edges).

## Layout

```
surql/            source of truth, loaded in filename order
  00_schema.surql           tables, fields, indexes (full-text: BM25)
  01_schema_reference.surql keyword + code node tables, reference edge tables
  02_schema_social.surql    worked_with / employed_by / participated_in /
                            spun_off_from / renamed_to / published_by;
                            created_by may point at an org; notes on about +
                            documented_in
  10–13_concepts_*.surql    concept nodes (object model, language,
                            tensions, modern analogs, questions)
  20_people.surql           persons + orgs
  30_artifacts.surql        software: HyperCard family, predecessors,
                            descendants, fan stacks
  40_events.surql           design-relevant events, 1968 demo → 2017 emulation
  41_additions_reconcile.surql  nodes added after the coverage critique that
                            survived pruning (events, preservation + media
                            artifacts, practice nodes, the counter-case, sources)
  50_sources.surql          evidence nodes (full-text indexed bodies)
  60–63_edges_*.surql       all RELATE statements (typed edges)
  65_edges_reconcile.surql  social/institutional edges, event actors, the
                            missing raises/addresses links, thread membership
                            for the nodes in 41
  64_threads.surql          thread nodes + ordered in_thread membership
  70_hypertalk_keywords.surql  154 keyword nodes + defined_in/relates_to edges
  71_hypertalk_code.surql   28 verbatim HyperTalk samples + uses/demonstrates/
                            extracted_from edges
  72_semantics_subgraph.surql  container model, evaluation modes, coercion,
                            precedence + specifies edges from reimplementations
load.sh           wipe ./data, start server, load everything
queries/examples.surql   example traversals (single-line statements)
queries/analyze.py       hubs, leaves, components, paths, coverage checks
notes/graph-traversal-report.md   narrative walk + hub/periphery analysis
                                  (written at 189 nodes / 483 edges)
notes/coverage-critique.md        coverage critique (dated snapshot, 189 nodes)
notes/reconciliation.md           ledger: critique finding → corpus status (partly superseded)
notes/pruning-2026-11-06.md       fact-check + prune ledger: what was cut, what was wrong, what is still unverified
archive/pre-prune-2026-11-06/     the corpus (surql, queries, notes) as it was before pruning
```

## Load

```bash
./load.sh
```

Requires the SurrealDB binary at `~/.surrealdb/surreal` (override with
`SURREAL=/path ./load.sh`). Server: `127.0.0.1:8123`, ns `hypercard`,
db `corpus`, root/root. Store: `surrealkv://./data` (the `file://` scheme was
removed in 4.0).

Gotchas this script defends against — keep them in mind if you edit it:

- The launcher execs a binary named **`surreal-db`**; `pkill -x surreal` does
  not match it. A stale server silently absorbs loads and duplicates every
  `RELATE` edge (node `UPSERT`s dedupe, edges don't). `load.sh` kills both
  names, waits for the port to free, and refuses to load into a store that
  already has tables.
- The `surreal sql` CLI splits multi-statement input at newlines; load via
  HTTP `POST /sql` (what `load.sh` does) for whole files.

## Query

```bash
curl -s -u root:root \
  -H "Surreal-NS: hypercard" -H "Surreal-DB: corpus" \
  -H "Content-Type: application/json" \
  --data-binary "SELECT ->created_by->person.* FROM artifact:hypercard;" \
  http://127.0.0.1:8123/sql
```

Or the CLI for single-line statements:

```bash
~/.surrealdb/surreal sql --endpoint ws://127.0.0.1:8123 \
  --username root --password root --namespace hypercard --database corpus
```

Analysis pass (Python, stdlib only):

```bash
python3 queries/analyze.py
```

## Schema conventions

- Node tables: `concept` (kinds: `object_model`, `language`, `practice`,
  `philosophy`, `idea`, `tension`, `modern_analog`, `domain`, `question`),
  `person`, `org`, `artifact`, `event`, `source`, `thread`, plus the reference
  tables `keyword` (categories: `command`, `function`, `control`, `chunk`,
  `container`, `operator`, `constant`, `message`, `property`, `special`) and
  `code` (kinds: `handler`, `function`, `pattern`, `idiom`, `script`; carries
  the verbatim `script` plus `attribution`). All `SCHEMAFULL`; nodes are
  written with `UPSERT <table>:<slug>` so reloads are idempotent.
- Social/institutional edges: `person --worked_with--> person`,
  `person --employed_by--> org`, `person|org --participated_in--> event`,
  `org --spun_off_from|renamed_to--> org`, `artifact --published_by--> org`.
  `created_by` may point at an org. Writing an event? Give it a
  `participated_in` edge — an event with no actor is a gap.
- Confidence policy for additions: a node with no source link is `medium` at
  most, however sure the author is; `high` is for claims a linked source
  supports. Suspect claims keep their node and are downgraded, not deleted.
- Edge tables: `SCHEMAFULL TYPE RELATION` with explicit `` `in` ``/`` `out` ``
  fields (SCHEMAFULL silently strips undeclared fields, including the graph
  endpoints). Most carry `confidence` (`high`/`medium`/`low`) and optional
  `note`; `part_of` and `in_thread` carry `position` for ordered reading.
- Reference edges: `code --uses--> keyword`, `code --demonstrates--> concept`,
  `code --extracted_from--> source`, `keyword --defined_in-->
  concept:hypertalk` (generated by a `FOR` loop over all keywords),
  `keyword --relates_to--> concept` (curated), `artifact --specifies-->
  concept` (reimplementations that pin down semantics).
- `meta::id()` strips the table prefix — use raw `` `in` ``/`` `out` `` when
  building adjacency maps that compare ids across tables.
- Full-text: `DEFINE ANALYZER OVERWRITE en TOKENIZERS blank, class FILTERS lowercase, snowball(english)`
  + `FULLTEXT ANALYZER en BM25(1.2,0.75) HIGHLIGHTS` on `concept.summary`,
  `concept.body`, `artifact.summary`, `keyword.summary`, `keyword.body`,
  `code.summary`, `code.script`; query with `WHERE body @@ 'hypertext'`.

## SurrealQL 4.0 gotchas (learned the hard way)

- `FOR` needs braces and record values:
  `FOR $kid IN (SELECT VALUE id FROM keyword) { RELATE $kid->defined_in->artifact:hypertalk; };`
  — `$k.id` after `RELATE` is a parse error, and `END;` is not the loop closer.
- `$this` does **not** bind inside `count((SELECT ... WHERE out = $this.id))`
  subqueries — the count silently goes wrong. Use set membership instead:
  `WHERE NOT (id INSIDE (SELECT VALUE out FROM uses))`. Plain `NOTIN` does not
  parse in this build.
- HyperTalk code contains `"` constantly, so `script` bodies are embedded in
  **single-quoted** SurrealQL strings; apostrophes inside become `\'`, and any
  double-quoted SurrealQL string body containing `"` needs `\"`.

## Reference and code-corpus traversals

```bash
# every keyword a code sample uses
curl ... --data-binary "SELECT <-uses<-code.title AS samples FROM keyword:cmd_put;" ...
# which samples demonstrate a concept
curl ... --data-binary "SELECT <-demonstrates<-code.{title, attribution} FROM concept:chunk_expression;" ...
# full-text search over the verbatim scripts
curl ... --data-binary "SELECT title, search::highlight('<em>','</em>',1) AS h FROM code WHERE script @@ 'mouseUp';" ...
# the keyword vocabulary of one category
curl ... --data-binary "SELECT word, summary FROM keyword WHERE category = 'message' ORDER BY word;" ...
```

## Extending

1. Add nodes with `UPSERT` in the appropriate `surql/` file.
2. Add edges as `RELATE a->table->b SET ...` in the matching `6x_edges_*.surql`.
3. `./load.sh` (it fails loudly on schema violations and stale servers).
4. `python3 queries/analyze.py` — check you didn't create an isolated node or
   a dangling endpoint, and that new tensions/questions are wired.

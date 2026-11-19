# Graph Traversal Report — HyperCard Corpus

*Generated against a clean single load: 189 nodes, 483 edges, 21 edge tables, zero duplicates, zero isolated nodes, one weakly connected component. All traversals below were run as SurrealQL against `surrealkv://./data` (ns `hypercard`, db `corpus`).*

---

## 1. A narrative reconstructed purely by following edges

The exercise: start at one concept and refuse to use any knowledge the graph itself doesn't carry. At each step the only question asked of the database is *"what outgoing edges does this node have?"* — and the story is whatever the edge tables and their `note` fields say. Starting node: `concept:memex`.

**Step 1 — `concept:memex`.** Outgoing edges: `created_by → person:vannevar_bush`, `influenced → concept:hypertext`, `maps_to → concept:obsidian_roam`, plus a `part_of`-style thread membership. One `influenced` edge, so the path is forced:

> `concept:memex —influenced→ concept:hypertext`
> edge note: *"Bush's associative trails are Nelson's explicit starting point"* (confidence: high)

The edge itself carries the historiographical claim — Bush to Nelson — with no prose node needed.

**Step 2 — `concept:hypertext`.** Five outgoing `influenced` edges (guide, hypercard, intermedia, note_cards, storyspace) — the 1960s idea fanning out into eighties tools. Taking the artifact that the rest of the corpus keeps pointing at:

> `concept:hypertext —influenced→ artifact:hypercard` (confidence: high)

**Step 3 — `artifact:hypercard`.** The dominant node, 35 outgoing edges. Its `implements` edges say *what HyperCard was*: `stack`, `message_path`, `hypertext`, `hypertalk`, `end_user_programming`, `immediate_persistence`, `script_property`, and — pointedly — `memex` itself. Following the language it implements:

> `artifact:hypercard —implements→ concept:hypertalk`

**Step 4 — `concept:hypertalk`.** Out: `created_by → person:dan_winkler`, `influenced → artifact:javascript`, `influenced → concept:applescript`, `maps_to → concept:javascript_dom`. The JavaScript edge is the one worth walking:

> `concept:hypertalk —influenced→ artifact:javascript`
> edge note: *"Eich cites HyperTalk directly; onmouseup et al."* (confidence: high)

Note what just happened structurally: the lineage from a 1987 artifact to the modern web traveled **artifact → concept → artifact**. Influence in this corpus flows through ideas, not machine-to-machine. (A pure `influenced`-only BFS from `artifact:hypercard` to `artifact:javascript` finds *no path* — the graph insists the detour through the language is the only route.)

**Step 5 — `artifact:javascript`.** Out: `created_by → person:brendan_eich`, `derived_from → concept:hypertalk` (the same claim, pointed the other way, as descent rather than influence), and:

> `artifact:javascript —exemplifies→ concept:q_message_path_modern`

A modern artifact *exemplifying a question* — the message-path problem didn't disappear, it got a second implementation.

**Step 6 — `concept:q_message_path_modern`.** Its incoming edges answer *why this question exists*:

> `concept:no_networking —raises→ concept:q_message_path_modern` (confidence: high)

**Step 7 — `concept:no_networking`.** A tension node. Its outgoing `limits` edge closes the loop back into the original object model:

> `concept:no_networking —limits→ concept:message_path`
> edge note: *"a path that cannot cross a wire"* (confidence: high)

**Step 8 — the revival branch.** Back at `artifact:hypercard`, its `influenced` fan-out includes the revival tools. Taking one:

> `artifact:hypercard —influenced→ artifact:decker` (confidence: high)
> `artifact:decker —derived_from→ artifact:hypercard` (descent *and* inspiration, two parallel edges)
> `artifact:decker —addresses→ concept:platform_lockin` (note: *"cross-platform + html export"*)
> `artifact:decker —addresses→ concept:pseudo_arrays` (note: *"Lil's arrays"*)

**Step 9 — the preservation end.** Following the tension `platform_lockin`:

> `concept:platform_lockin —raises→ concept:q_preservation`
> `concept:q_preservation —documented_in→ source:s_hypercard_org` and `source:s_internet_archive_hc`

The walk ends at *sources* — which is where it should end: evidence.

**The reconstructed narrative, in one breath:** Bush's memex influenced Nelson's hypertext, which influenced HyperCard; HyperCard implemented HyperTalk; HyperTalk influenced JavaScript, which exemplifies the modern message-path question raised by HyperCard's inability to cross a wire — a tension Decker addresses by descending from HyperCard itself, while the platform-lockin tension raises the preservation question that HyperCard.org and the Internet Archive document. Every clause above corresponds to exactly one edge, several carry their justification in `note`, and none required consulting anything outside the graph.

---

## 2. Hub analysis — what the most-connected nodes say

Top nodes by total degree (out/in split in parentheses):

| degree | node | out/in | what it is |
|---|---|---|---|
| 75 | `artifact:hypercard` | 35 / 40 | the subject itself |
| 27 | `thread:t_modernization_tensions` | 0 / 27 | reading list, not claim |
| 20 | `concept:hypertalk` | 9 / 11 | the language |
| 18 | `concept:stack` | 6 / 12 | the data model |
| 18 | `thread:t_stackware_economy` | 0 / 18 | reading list |
| 16 | `thread:t_rise_and_fall`, `thread:t_revival_preservation` | 0 / 16 each | reading lists |
| 15 | `artifact:decker` | 12 / 3 | top revival node |
| 15 | `thread:t_hypertext_lineage` | 0 / 15 | reading list |
| 14 | `concept:message_path`, `concept:hypertext`, `concept:stackware` | — | the load-bearing ideas |
| 13 | `concept:q_preservation`, `source:s_wiki_hypercard` | — | the liveliest question; the best-evidenced node |

What this structure says:

- **One sun, many planets.** Median degree is 4, mean 5.1; `hypercard` at 75 is ~15× the median. The corpus is honestly *about HyperCard*, not about hypertext in general — every other node reaches the story through it. Its in-degree (40) is remarkably heterogeneous: 13 `derived_from` (every descendant descends from it), 7 `located_in` (fan stacks live *inside* it), 6 `contributed_to` (people), 5 `influenced` (predecessors), 3 `about` (events). A flat list flattens exactly this — the fact that four *different kinds* of claim converge on one node is the corpus's central thesis rendered as topology.
- **Threads are pure sinks.** All 7 thread nodes have out-degree 0 and in-degrees 11–27. This is by design and worth stating: threads *curate*, they never *assert*. If a modernization effort later wants "what to read first" to carry weight, the thread nodes are where a `position`-ordered argument already exists — they are the only nodes with a built-in reading order (116 `in_thread` edges all carry `position`).
- **The language is the second hub, not a person or date.** `concept:hypertalk` (20) outranks every person (`bill_atkinson`, 6) and every event (`e_release_1987`, 6). The graph encodes the claim that HyperCard's afterlife is its *language* — the two edges that matter most (`hypertalk → javascript`, `hypertalk ← derived_from ← javascript`) both run through it.
- **Sources can be hubs.** `source:s_wiki_hypercard` (13) is the top evidence node — a single Wikipedia article underwrites 11 claims. That is a *strength signal and a warning*: high-confidence claims resting on one popular source are the first place a real modernization effort should dig.
- **Revival has a capital.** `artifact:decker` (15, out 12) is the best-connected non-Apple artifact, and its profile is all outgoing `addresses`/`implements` edges — Decker is where the corpus parks its "what a successor does" claims.

---

## 3. Periphery analysis — what the quiet nodes say

After the last fixes there are **no isolated nodes** (`concept:idle_time` was the last one; it now sits at `position = 13` of the message path — *"the floor of the path: idle fires when the queue empties"* — and maps to `concept:javascript_dom` for the setTimeout bargain). 51 of 189 nodes have degree ≤ 2; 16 are leaves (degree 1):

- **13 of 27 persons are leaves** — `brendan_eich`, `tim_berners_lee`, `ward_cunningham`, `susan_kare`, `steve_jobs`, `john_sculley`, … Each hangs by exactly one edge (`created_by`, `contributed_to`, `about`, or `in_thread`). This is a deliberate modeling choice with a cost: people are *attribution terminals*. The graph can answer "who made X" but not "who knew whom" — there are no person–person edges (collaboration, mentorship, succession). If the modernization effort cares about the social story (e.g., the Xerox PARC → Apple pipeline), person-to-person edges are the biggest untouched surface.
- **`concept:q_authoring_runtime_unity` is the most eloquent leaf in the corpus.** It sits in `t_modernization_tensions` and *nothing else*. The analysis pass confirms the structural gap: it is the only one of 10 question nodes that **no tension raises** (`raises` covers 9/10). The corpus asks a question it never derives from a documented failure — a genuine hole, visible only because the graph made "unraised question" a queryable state.
- **`concept:target_me_it`** hangs only by `part_of → message_path`. A message-path detail with no `maps_to` edge: the corpus never says what `target` became in the modern stack (arguably `event.target` / `this` in DOM handlers). Another gap the leaf status exposes.
- **`org:livecode`** hangs only by `owned_by ← artifact:livecode`. Orgs are near-terminals by design (apple, 5, is the only org with a real profile) — fine for provenance, thin for institutional history (Claris as employer, the Software Ambassadors non-event, etc.).
- **The tension-coverage matrix has four holes.** 13 tensions; 17 modern analogs `addresses` 9 of them. Unaddressed: `bolted_on_color`, `security_naivete`, `succession_failure`, `web_displacement`. Three of these are honest nulls (no modern analog *does* fix the death-of-the-successor problem — that's the point of recording it), but `security_naivete` is a real gap: the sandboxing story (HyperCard virus → macro virus → today's script permissions) has no analog edge, even though the corpus has both `concept:macro_virus` and a `q_live_editing_web` question that begs for it.

The periphery thus splits cleanly into **designed terminals** (persons, orgs, sources-as-evidence — quiet because they *should* be sinks) and **accidental silence** (`q_authoring_runtime_unity`, `target_me_it`, `security_naivete`'s missing analog — quiet because the corpus hasn't yet said anything through them). Only the graph distinguishes these two kinds of quiet.

---

## 4. What the graph revealed that a flat list wouldn't

1. **A forced detour as a thesis.** The flat list would happily contain "HyperTalk influenced JavaScript" as one bullet. The graph makes you *walk* `hypercard → hypertalk → javascript` and discover that no artifact-to-artifact `influenced` path to JavaScript exists at all. The claim "languages, not machines, carry lineage" isn't written anywhere — it falls out of the route the edges permit.
2. **Direction carries meaning, twice.** `hypercard —influenced→ decker` and `decker —derived_from→ hypercard` are two edges, not one fact restated: inspiration and descent are different claims, and Decker is one of the few nodes where both hold. Similarly `hypertalk → javascript` (influence) vs `javascript → hypertalk` (descent) coexist. A flat list would collapse each pair and lose the distinction.
3. **Sinks and sources are structural facts.** Threads with out-degree 0, sources as terminal evidence, persons as attribution terminals — these are *roles* the nodes play in the argument, visible only as degree profiles. A flat list of "7 reading threads" says nothing about the fact that no claim ever flows *out* of a thread.
4. **Absence became queryable.** "Unraised question", "unaddressed tension", "isolated concept", "dangling edge" are questions you can only ask of a graph. The exercise already caught three: `idle_time` floating free (now the floor of the message path), `q_authoring_runtime_unity` derived from nothing, and a `derived_from` edge pointing at a nonexistent `artifact:hypertalk` (HyperTalk is a concept — the referential-integrity check fixed the ontology).
5. **The container relation nobody thought to list.** Seven fan stacks (`myst`, `the_manhole`, `hard_days_night`, …) connect via `located_in → hypercard`: not "related to", but *shipped inside*. No flat index of "notable HyperCard stacks" expresses that they are parts of the artifact's own history; the edge table does it in one triple each.
6. **Evidence concentration as risk.** The hub analysis surfaced that `s_wiki_hypercard` underwrites 11 claims — a fact about *epistemic fragility* that only a degree count reveals. A bibliography in a flat document would list the same source once and look perfectly healthy.
7. **The corpus's own center of gravity.** Degree distribution (one node at 75, median 4) is an honest confession: this is a HyperCard corpus, not a hypertext survey. A flat collection of 84 concept notes hides that imbalance; the graph wears it on its face — which is exactly what you want before deciding what to modernize, because it tells you where the mass is that any modernization must move.

---

### Reproducing

```bash
cd ~/hypercard-corpus
./load.sh                      # fresh store + full load (refuses stale servers)
python3 queries/analyze.py     # hubs, leaves, components, paths, coverage
```

Every traversal quoted above is a one-liner against `POST /sql`; see `queries/examples.surql` for the patterns (`->created_by->person.*`, `part_of … ORDER BY position`, `body @@ 'hypertext'`, ordered `in_thread`).

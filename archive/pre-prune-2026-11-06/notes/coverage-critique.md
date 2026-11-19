# HyperCard Corpus — Coverage Critique

> **Status (2026-11-06):** this critique is a dated snapshot written against 189 nodes / 483 edges. Its findings have since been triaged — fixed, disputed, or left open — in [`reconciliation.md`](reconciliation.md). Several of its figures and two of its factual corrections (iBooks Author's year; the HyperCard 3.0 cancellation date) do not hold as written.

**Missing, underrepresented, and misrepresented angles**

*Prepared: 2026-11-06 · Against the `surql/` source of truth at commit-state 189 nodes / 483 edges*

---

## Scope and method

This is a critique of **coverage**, not an edit. No corpus files were changed.

The analysis was performed by parsing every `surql/*.surql` file statically (Python, stdlib) rather than querying a running SurrealDB instance. The parse reproduces the README's headline counts exactly:

- **189 nodes:** 81 concepts, 38 artifacts, 27 persons, 21 sources, 12 events, 7 threads, 3 orgs.
- **483 edges** across 21 edge tables.
- **0 dangling endpoints, 0 duplicate triples, 0 self-loops.**

Health checks are clean. The problems below are structural and epistemic, not referential-integrity bugs.

The existing `notes/graph-traversal-report.md` already covers hubs, leaves, thread sinks, and tension holes. This document extends that analysis in four directions the traversal report does not pursue:

1. **Provenance and source quality** — what the graph can actually show.
2. **Social and institutional history** — who is missing, and what relations are missing.
3. **Chronology and geography** — which periods and places are absent.
4. **Content accuracy and framing** — which claims are wrong, overstated, or internally inconsistent.

---

## Executive summary

The corpus is strongest as a **design argument** and weakest as a **historical record**. Its object-model and tension nodes are unusually good: the stack/card/message-path model, the language philosophy, the tensions, and the lineage edges are coherent, annotated, and genuinely explanatory. Its evidence layer, social graph, and post-1997 chronology are thin enough that many of its most confident claims are unsupported.

The recurring pattern is: **the prose knows more than the graph encodes.** Names, systems, dates, and causal claims appear in summaries and thread bodies but have no node or edge, so they cannot be queried, checked, or extended by the graph itself.

Five structural holes dominate:

| # | Hole | One-line evidence |
|---|---|---|
| 1 | **Provenance** | Only 54/189 nodes have a source link; 95 high-confidence nodes are unsourced; `about` (83 edges) has no confidence or notes. |
| 2 | **Social/institutional graph** | Zero person–person edges; 13/27 persons are leaves; 3 orgs, all near-terminal; no employment, funding, standards, or event-participation edges. |
| 3 | **Death and afterlife** | Almost nothing dated 1997–2013; no 2022–2026 nodes despite repeated “in 2026” prose; 1988–89 stackware boom exists only as prose. |
| 4 | **Practice and people** | 19 object-model concepts but 1 `practice` concept; the actual author population (teachers, librarians, students, small businesses) is invisible. |
| 5 | **The counter-case** | `web_displacement` is unaddressed; `q_authoring_runtime_unity` is the only question no tension raises; modern analogs skew to knowledge management and web dev, not media/education/game authoring. |

---

## 1. What the corpus does well

A fair critique should start with the strengths, because they determine what kind of repair is appropriate.

- **The object model is genuinely modeled.** `stack`, `card`, `background`, `part`, `script_property`, `property_model`, `message`, `message_path`, `target_me_it`, `chunk_expression`, `message_box`, `tool_palette`, `region_link`, `find_mark_sort`, `idle_time`, `home_stack`, `player_split`, `fixed_canvas`, `xcmd`, `pseudo_arrays`, `single_file_unity` — 19 nodes that together constitute a real account of how HyperCard worked. This is the corpus's strongest asset.
- **The tensions are specific, not generic.** `no_networking`, `no_inline_text_links`, `no_structured_data`, `bolted_on_color`, `single_user`, `interpreter_performance`, `monolithic_file`, `platform_lockin`, `business_model_clash`, `author_player_politics`, `security_naivete`, `succession_failure`, `web_displacement` are the right tensions, and most are grounded in mechanism rather than vibes.
- **The lineage edges carry argument.** The `hypertalk → javascript` influence, the `hypercard → decker` / `decker → derived_from → hypercard` double edge, and the `influenced` notes are the best-annotated part of the graph (21/37 `influenced` edges have notes; 31/37 have confidence).
- **The revival/preservation cluster is alive.** Decker, Stacksmith/stackimport, Wyldcard, HyperCardPreview, CinsImp, OpenXTalk, LiveCode, hypercard.org, the Internet Archive, and the LoC format work are all present. For a corpus about a dead platform, this is unusually current.
- **The prose is often excellent.** The artifact bodies and concept bodies are written with real historical understanding. The problem is that this prose is frequently ahead of the graph.

The rest of this document is about where that prose and graph diverge.

---

## 2. Missing angles

### 2.1 The empty middle: 1997–2013, and the missing present

The year distribution is heavily front-loaded:

| Year | Nodes | Year | Nodes |
|---:|---:|---:|---:|
| 1945 | 1 | 1996 | 1 |
| 1965 | 1 | 2000 | 1 |
| 1982 | 1 | 2001 | 1 |
| 1983 | 2 | 2004 | 2 |
| 1984 | 1 | 2013 | 2 |
| 1985 | 1 | 2014 | 1 |
| 1986 | 1 | 2016 | 5 |
| **1987** | **37** | 2017 | 2 |
| 1988 | 7 | 2018 | 1 |
| 1989 | 5 | 2019 | 4 |
| 1990 | 10 | 2020 | 2 |
| 1991 | 4 | 2021 | 2 |
| 1992 | 6 | | |
| 1993 | 3 | | |
| 1995 | 5 | | |

There are **no nodes dated 1994, 1997, 1998, 1999, 2002, 2003, 2005–2012, or 2022–2026.**

This produces several specific historical holes:

- **No Jobs return (1997).** The single most consequential event for HyperCard's fate is not an event node.
- **No OS X transition (2001).** HyperCard was never ported to OS X; the Classic environment is the bridge. There is no `e_osx_2001`, no `classic_environment` concept, no `e_classic_removed_2006`.
- **No Intel transition / Classic removal (2006).** The moment the last official authoring environment became PowerPC-only is absent.
- **No iPad (2010) or iBooks Author event (2013).** `artifact:ibooks_author` exists, but the event of its announcement and Atkinson's “partial heir” comment are only in prose.
- **The 2017 30th anniversary** is represented only by the Internet Archive emulation event; there is no dedicated anniversary/retrospective node.
- **No 2022–2026 nodes at all.** The corpus repeatedly says “in 2026” — in `natural_language_syntax`, `no_structured_data`, `business_model_clash`, `artifact:decker`, and `s_github_hypertalk` — but the newest node is 2021. The present is asserted, not modeled.

The 1988–89 stackware boom is also missing as events. The prose says “CompuServe 300+ stacks by 1988” and the source note names the **HyperTalk Standards Committee (1989)**, but there is no `e_compuserve_1988`, no `e_hypertalk_standards_1989`, no `artifact:hyperheaven`, no user-group or magazine nodes. The economy that made HyperCard a mass medium is a thread body, not a record.

### 2.2 Institutions, standards, and the social network

The corpus has **three orgs**, all near-terminal:

| Org | Degree | Edges |
|---|---:|---|
| `org:apple` | 5 | owned_by from five artifacts |
| `org:claris` | 2 | owned_by from Player + a source `about` |
| `org:livecode` | 1 | owned_by from LiveCode |

There are **zero org–org edges**, **zero employment edges**, **zero funding edges**, **zero standards-body edges**, and **zero event-participation edges** for orgs.

Missing institutions that the corpus's own prose invokes:

- **FileMaker / Claris Inc.** — the database lineage where HyperCard's structured-data users went. `no_structured_data` says “serious data users were steered to FileMaker,” but FileMaker is not a node.
- **Brøderbund** (Myst publisher), **Cyan** (Myst developer), **Voyager** (Expanded Books), **Silicon Beach** (SuperCard), **Macromedia** (Director, Authorware), **Apple Education / ACOT**.
- **The HyperTalk Standards Committee** — named in `source:s_wiki_hypertalk` but not modeled.
- **User groups, magazines, conferences** — Macworld, MacWeek, InfoWorld, the HyperCard user groups, the stackware libraries. These are the distribution infrastructure of the medium.

The consequence is that institutions never act. Events are connected to *things*, not *actors*:

- `event:e_hc3_cancelled_2000` — whose prose says “Jobs's Apple kills the successor” — has **no edge to `person:steve_jobs`, `person:kevin_calhoun`, `person:dan_crow`, `org:apple`, or `org:claris`**. It connects only to `artifact:hypercard_3` and `concept:succession_failure`.
- `event:e_release_1987` has no edge to Atkinson, Sculley, or Apple.
- `event:e_hc3_wwdc_1996` has no edge to Dan Crow, who demoed it.

Only **2 event→person `about` edges** exist in the entire graph (Atkinson/Wired and Engelbart/demo). The cancellation, the release, the virus wave, and the discontinuation have no actors; the demo connects only to Engelbart.

### 2.3 The social graph: zero person–person edges

There are **no person–person edges of any kind**. The 27 persons connect only through artifacts (`created_by`, `contributed_to`), sources (`about`), or threads (`in_thread`). **13 of 27 are leaves:**

`steve_jobs`, `john_sculley`, `steve_wozniak`, `susan_kare`, `donald_brown`, `pei_yuan_wei`, `brendan_eich`, `ward_cunningham`, `bill_appleton`, `kevin_cuneo`, `tom_pittman`, `jakob_nielsen`, `tim_berners_lee`.

This makes the corpus unable to answer basic historical questions:

- Who worked with whom on the HyperCard team?
- Who mentored whom (Atkinson → Winkler; Atkinson → Calhoun/Crow)?
- Who left Apple for General Magic, and when?
- Who staffed Claris, and who stayed at Apple?
- Who was on the HyperTalk Standards Committee?
- Who influenced whom *as a person*, not through an artifact?

The existing traversal report already flags this as “the biggest untouched surface.” It remains true. The person set is also overwhelmingly Apple engineers and executives; the actual author population — teachers, librarians, students, small-business users, hobbyists — is statistically invisible. For a corpus whose central thesis is **end-user programming**, the end users are the least represented group.

### 2.4 Place and language

The corpus is essentially US/Apple-centric. Canada appears only through the NFB CinéRobothèque robot and the UQAM/Pantechnicon archive. There are:

- No UK, European, or Japanese stackware scenes.
- No localized HyperCard editions (French, German, Japanese, etc.).
- No non-English sources.
- No non-US institutional deployments beyond the NFB and UQAM.

This matters because HyperCard's international life was real: localized versions, national education programs, and non-English stackware communities are part of the medium's history. A corpus that wants to explain “the largest install base an end-user programming environment ever had” cannot be only Cupertino.

### 2.5 Practice and craft

The concept kinds are heavily skewed toward the object model:

| Kind | Count |
|---|---:|
| `object_model` | 19 |
| `modern_analog` | 17 |
| `tension` | 13 |
| `question` | 10 |
| `domain` | 7 |
| `philosophy` | 5 |
| `idea` | 5 |
| `language` | 4 |
| **`practice`** | **1** |

The single `practice` node is `player_split` — a licensing/distribution practice, not an authoring practice.

Missing practice-level knowledge:

- **Stack idioms and design patterns** — card windows, invisible buttons, `lock screen`, visual effects, background hierarchies, “home stack” conventions, navigation patterns.
- **The authoring workflow** — how a stack was sketched, built, tested, and distributed; how authors learned HyperTalk; what the message box did in practice.
- **Visual culture** — the 1-bit/MacPaint aesthetic, dithering, the look of stackware, the pointing-hand cursor as a design signature.
- **Sound and music practice** — HyperCard's sound resources, QuickTime audio, the BBC Radiophonic Workshop's HyperCard-controlled studio network (mentioned in prose, not modeled).
- **The XCMD/XFCN ecosystem** — only `xcmd` and `compileit` represent a much larger vendor/tooling world.
- **Failure and abandonment** — bit rot, link rot, orphaned stacks, the actual lifecycle of a stackware product.

This is the knowledge a revival project would need most, and it is the least modeled.

### 2.6 Education and e-learning

`concept:education` is one domain node. There is no:

- **Apple Classrooms of Tomorrow (ACOT)** — the 1985–1995 Apple research program in which HyperCard was a central authoring medium. It is a textbook missing institution/event.
- **Classroom-stack artifacts** — the corpus mentions “classroom stacks” in prose but names none.
- **Education research sources** — no ACOT reports, no studies of HyperCard in K–12, no teacher-authored stack collections.
- **Modern e-learning analogs** — Authorware (Wikipedia explicitly says HyperCard authors moved to it), Articulate Storyline, Adobe Captivate, H5P.

This is a significant gap because education was, by the corpus's own account, “the stackware economy's largest informal sector.”

### 2.7 Media, games, and electronic literature

The corpus has Myst, The Manhole, A Hard Day's Night, Marble Springs, and the Electronic Whole Earth Catalog. That is a good start, but the media/game lineage is thin:

- **Flash** is mentioned only in the Director summary; it is not a node. Flash is the closest mass-market successor to HyperCard's “authoring = runtime” model, and its absence is conspicuous.
- **Scratch** is absent (the string appears only as “scratch variable”).
- **Twine** is absent.
- **Cosmic Osmo / Spelunx** (Cyan's pre-Myst HyperCard worlds) are absent.
- **The You Don't Know Jack prototype** (written in HyperCard) is absent.
- **Electronic literature** is under-modeled: `afternoon, a story` — the canonical Storyspace hypertext novel — is absent, as are the Electronic Literature Organization and the critical literature around hypertext fiction.
- **Authorware** and the CBT/e-learning authoring market are absent, despite being where many HyperCard authors went.

### 2.8 Preservation as a format, not just a thread

The preservation thread is about preserving stacks, but the **stack file format has no `file_format` artifact node**. The schema allows `file_format` and `archive`; neither kind is used.

Missing preservation nodes:

- `artifact:stack_format` (`kind = 'file_format'`) — the actual object the format documentation describes.
- `artifact:basilisk_ii`, `artifact:sheepshaver`, `artifact:mini_vmac` (`kind = 'software'`) — the emulators that make the runtime pole real.
- `artifact:internet_archive` (`kind = 'archive'`) — the institution, not just the source.
- `artifact:hypercard_org` exists, but as a `website`, not as a format/documentation artifact.

The result is that the preservation thread names three poles (runtime, format, reimplementation) but only fully models the reimplementation pole.

### 2.9 Law and IP

`artifact:zoomracks` has status `shipped; lawsuit`, but there is:

- No event for the lawsuit.
- No party/org node for the litigants.
- No source for the claim.
- No concept for IP ownership, trademark, or format control.

Similarly, ViolaWWW's patent-litigation history is mentioned in external sources but not in the corpus. Apple's ownership of the stack format, the HyperCard trademark, and the Claris licensing arrangements are all absent as legal history.

### 2.10 The stackware economy as events, not prose

The `t_stackware_economy` thread body is rich: CompuServe 300+ stacks (1988), HyperHeaven on AOL, Voyager's Expanded Books, Renault inventories, the NFB robot arm, classroom stacks, kiosk stacks, CompileIt!, XCMD vendors.

Almost none of that is modeled:

- No `e_compuserve_1988`.
- No `artifact:hyperheaven`.
- No `org:compuserve` or `org:aol`.
- No Renault deployment node.
- No kiosk artifact.
- No user-group or magazine node.

The thread is an essay *about* the graph, not a path *through* it.

---

## 3. Underrepresented: the quantitative picture

### 3.1 Provenance

| Type | Sourced | Total | Unsourced |
|---|---:|---:|---:|
| concept | 22 | 81 | 59 |
| artifact | 21 | 38 | 17 |
| person | 6 | 27 | 21 |
| event | 4 | 12 | 8 |
| org | 1 | 3 | 2 |
| thread | 0 | 7 | 7 |
| **Total** | **54** | **189** | **135** |

**95 high-confidence nodes are unsourced.** The list spans the corpus's most load-bearing claims: `artifact:director`, `artifact:guide`, `artifact:macpaint`, `artifact:viawww`, `artifact:world_wide_web`, `artifact:whole_earth_catalog`, `concept:card`, `concept:message`, `concept:platform_lockin`, `concept:security_naivete`, `concept:single_user`, and most of the modern analogs.

`source:s_wiki_hypercard` is the single most-used source: 13 edges, underwriting ~11 claims. `source:s_github_hypertalk` is a **GitHub search URL**, which is a weak evidentiary object (it is a query, not a document). `s_alsop_time_1987` and `s_wozniak_2001` are explicitly secondary/compilation sources (“reported via secondary compilation,” “as recorded in `s_wiki_hypercard`”).

### 3.2 Sources

| Property | State |
|---|---|
| Total sources | 21 |
| Reliability `high` | 7 |
| Reliability `medium` | 14 |
| No URL | 7 — `s_loc_stack`, `s_mactech_xcmd`, `s_dobbs_xcmd`, `s_stack_format_lorenzi`, `s_alsop_time_1987`, `s_wozniak_2001`, `s_cine_robotech` |
| Kinds | 10 article, 4 archive_site, 3 docs, 1 interview, 1 book, 1 paper, 1 repo |
| Unused kind | `talk` |

There are no primary Apple documents, no patents, no court records, no magazine archives (Macworld, MacWeek, InfoWorld, Byte), no oral histories beyond the Wired interview, and no non-English sources. The evidence base is mostly web secondary.

### 3.3 Persons and orgs

- 27 persons; 13 leaves; 0 person–person edges.
- 3 orgs; degrees 5, 2, 1.
- 0 org–org edges.
- 0 employment, funding, standards, or event-participation edges for orgs.

### 3.4 Events

- 12 events; 4 sourced; 8 unsourced.
- 6 of 12 are product/company milestones; 2 are reception, 2 are security, 1 is preservation, 1 is lineage.
- Only 1 event is not in a thread.
- Only 2 event→person `about` edges exist.
- No event for: the Claris transfer (1990), the HyperTalk Standards Committee (1989), CompuServe (1988), the Jobs return (1997), OS X (2001), Classic removal (2006), the iPad (2010), iBooks Author (2013), or any 2020s revival/AI moment.

### 3.5 Threads

- 7 threads; **0 sourced**.
- All 7 are pure sinks (out-degree 0; in-degree 11–27).
- `in_thread` (116 edges) has **no confidence** on any edge.
- 100/189 nodes are thread members; **82 content/source nodes are in no thread** (31 concepts, 8 artifacts, 20 persons, 1 event, 19 sources, 3 orgs).
- 10 nodes connect **only** via threads: `q_authoring_runtime_unity`, `steve_jobs`, `john_sculley`, and the 7 thread nodes themselves.

### 3.6 Artifacts

- 38 artifacts; **21 lack `created_by`**, **31 lack `owned_by`**.
- Kinds: 32 software, 3 publication, 1 hardware, 1 website, 1 language.
- Unused kinds: `file_format`, `archive`.
- Status values are inconsistent in granularity: `shipped`, `shipped (research)`, `shipped; lawsuit`, `alive`, `alive, MIT`, `alive, open source`, `alive (OpenXTalk fork)`, `historical`, `discontinued 2004`, `discontinued 2020`, `cancelled 2000`, `never shipped; public domain`.
- No `kind = 'stack'`; the corpus's unit of work is only represented by five named examples (Myst, The Manhole, Whole Earth, Hard Day's Night, Marble Springs) under other kinds.

### 3.7 Edge semantics

| Relation | Edges | With confidence | With note |
|---|---:|---:|---:|
| `in_thread` | 116 | 0 | 0 |
| `about` | 83 | 0 | 0 |
| `influenced` | 37 | 31 | 21 |
| `maps_to` | 24 | 24 | 7 |
| `documented_in` | 24 | 0 | 0 |
| `created_by` | 21 | 21 | 3 |
| `derived_from` | 18 | 18 | 7 |
| `part_of` | 18 | 0 | 10 |
| `implements` | 17 | 17 | 6 |
| `exemplifies` | 17 | 17 | 4 |
| `addresses` | 17 | 17 | 10 |
| `raises` | 14 | 14 | 0 |
| `enables` | 13 | 13 | 4 |
| `limits` | 13 | 13 | 2 |
| `applied_to` | 11 | 11 | 0 |
| `located_in` | 8 | 8 | 7 |
| `contrasts_with` | 8 | 8 | 7 |
| `owned_by` | 7 | 0 | 0 |
| `motivates` | 7 | 7 | 0 |
| `contributed_to` | 6 | 6 | 2 |
| `succeeded` | 4 | 3 | 3 |

The largest relation in the corpus (`about`, 17% of all edges) is the **least epistemically annotated**. `documented_in` — the actual provenance relation — is equally unweighted. `in_thread` is pure curation with no confidence. This is the opposite of what a provenance-first corpus should look like.

### 3.8 Tensions and questions

- 13 tensions; 9 addressed; **4 unaddressed:** `bolted_on_color`, `security_naivete`, `succession_failure`, `web_displacement`.
- `security_naivete` is a genuine gap, not an honest null: `macro_virus` and `q_live_editing_web` both exist, but there is no analog edge (e.g., sandboxing, browser permissions, WASM isolation). The `t_modernization_tensions` body *claims* `security_naivete → wasm_plugins`, but the edge table has `wasm_plugins → interpreter_performance` instead.
- 10 questions; 9 raised; **`q_authoring_runtime_unity` is raised by no tension**. It floats in a thread with no derivation from a documented failure.

### 3.9 Connectivity

“One weakly connected component, no isolated nodes” is true only because thread edges are counted:

- **Including** all edges: 1 component.
- **Excluding `in_thread`:** 11 components — 1 giant component plus 10 singletons (`q_authoring_runtime_unity`, `steve_jobs`, `john_sculley`, and the 7 thread nodes).
- **Excluding `in_thread` and provenance (`about`, `documented_in`):** 48 components.

The README and the traversal report state the first fact without the caveat. The graph's connectivity is load-bearing on the two relations that carry no confidence.

---

## 4. Misrepresented

### 4.1 Hard factual errors

| Claim | Location | Problem | Correction / qualification |
|---|---|---|---|
| Myst was “bundled on the **IBM Performa 5300**” | `surql/30_artifacts.surql:145` | The Performa is an **Apple** product line. “IBM” is simply wrong. | “Bundled on various Apple Performa models”; the specific 5300 claim needs a source. |
| Myst's “shipping used a custom engine” | `surql/30_artifacts.surql:146` | The original 1993 Mac release **was HyperCard**, colorized with a proprietary HyperTint. The custom engine came with later ports. | “The original Mac release was a set of HyperCard stacks plus HyperTint; later ports used a custom engine.” |
| Atkinson “stayed anyway” / “stayed through the Jobs years” | `surql/20_people.surql:5`; `surql/12_concepts_tensions.surql:82` | Atkinson left Apple in **1990**; Jobs returned in **1997**. He did not stay through the Jobs years. | Remove the “stayed” claim; distinguish Atkinson's 1990 departure from Jobs's 1997 return. |
| “Sculley's stink” is Atkinson's phrase for the Sculley era | `surql/20_people.surql:25`; `surql/12_concepts_tensions.surql:82` | The quote is **Jobs's** view of the product, reported by Atkinson. The corpus repeatedly turns it into Atkinson's own phrase and, in `business_model_clash`, says Atkinson stayed *because* of the stink — which is incoherent. | “Jobs, according to Atkinson, thought the product had ‘Sculley's stink all over it.’” |
| Claris “became FileMaker Inc. (2001)” | `surql/20_people.surql:102` | Claris was renamed **FileMaker Inc. in 1998** (and reverted to Claris in 2019). | Change to 1998. |
| “The team had been split into Claris during 1987–90” | `surql/40_events.surql:26` | HyperCard was transferred to Claris effective **November 1990**; Claris was founded in 1987. | “HyperCard moved to Claris in late 1990; some developers stayed at Apple, splitting the team.” |

### 4.2 Overstated or unsupported

| Claim | Location | Problem |
|---|---|---|
| ViolaWWW was “the first popular graphical browser” | `surql/20_people.surql:57`; `surql/30_artifacts.surql:96` | It was the first browser with scripting, stylesheets, and tables, and was popular at CERN; **Mosaic** is the browser usually credited with popularizing the graphical web. Qualify as “first with scripting/stylesheets/tables” or “popular in the Unix/CERN niche.” |
| “The largest install base an end-user programming environment has ever had” | `surql/30_artifacts.surql:6`; `surql/40_events.surql:5` | A high-confidence superlative with no source or definition (install base of what? by when? compared to what?). |
| “Best-selling Mac book of its era” | `surql/20_people.surql:89`; `surql/50_sources.surql:36` | Unverified superlative; “era” is undefined. |
| “Billion-dollar shadow” | `surql/30_artifacts.surql:145` | Myst sold millions of copies, but “billion-dollar” is an unsourced revenue claim. |
| “The last environment able to author original stacks is OS X 10.4 Classic on PowerPC” | `surql/30_artifacts.surql:6` | The last **officially supported** environment; the corpus itself documents emulation and reimplementation, which contradicts the absolute phrasing. |
| HyperCard 3.0 “cancelled 2000” | `surql/30_artifacts.surql:19–21`; `surql/40_events.surql:33–34` | Development stalled in 1998 when the QuickTime team pivoted; the team was reassigned in 2000; Dan Crow's own account says the project was canceled in late 1998. The single date hides a two-year sequence. |
| “First document-carried viruses anywhere” | `surql/40_events.surql:15` | HyperCard viruses were among the first macro viruses; “anywhere” is a strong global priority claim that needs a source. |

### 4.3 Structural misrepresentation

**The narrative thread contradicts its own edges.** `surql/64_threads.surql:36` says the modernization matrix maps `security_naivete → wasm_plugins`. The edge table has `wasm_plugins → interpreter_performance`; `security_naivete` is unaddressed. The same paragraph calls `q_collaboration` and `q_who_is_the_author` “tensions” (they are questions) and omits `bolted_on_color`, `succession_failure`, and `web_displacement` from its list of tensions with no addresser. The thread that is supposed to curate the matrix misstates it.

**The graph is narrower than its bibliography.** The source notes name systems that have no nodes:

- `s_github_hypertalk` lists 11 projects; only some become artifacts (Wyldcard, Stacksmith, OpenXTalk, HyperCardPreview, CinsImp). `hc2html`, `openxion`, `hyperjs`, `tilestack`, `novocard`, `baycard` are names in a note.
- `s_creysoft_xtalk` lists SuperCard, ToolBook, Lingo, MetaCard, SenseTalk, Plus/WinPlus, mTropolis, Apple Media Tool, Stagecast. Only SuperCard, ToolBook, Director/Lingo, and LiveCode/MetaCard are nodes.
- `s_wiki_hypertalk` names the HyperTalk Standards Committee; there is no node or event.

This is a specific, fixable form of under-modeling: the corpus's own annotations contain a richer list than the graph.

**“Direction-agnostic” describes the schema, not the selection.** The corpus is explicitly “without committing to a modernization path,” but its selection is a thesis: Apple/Jobs as the institution that failed the environment, stackware as a folk commons, modern analogs as partial heirs. There is no developed position for “the web won for good reasons” or “HyperCard's limits were fundamental.” `web_displacement` is one of the four unaddressed tensions; `q_authoring_runtime_unity` — the question that could challenge the entire premise — is unraised. The corpus asks the question but never derives it from a documented failure.

**Modern analogs skew to knowledge management and web development.** The 17 analogs are dated 1990–2021, mostly 2016–2021:

`www` (1990), `html_hyperlink` (1990), `javascript_dom` (1995), `applescript` (1993), `spa_routing` (2013), `command_palette` (2016), `low_code` (2016), `local_first` (2019), `single_file_html_app` (2020), `notion_airtable` (2016), `wiki` (1995), `obsidian_roam` (2019), `tiddlywiki` (2004), `jupyter` (2014), `wasm_plugins` (2017), `dynamicland` (2016), `ai_codegen` (2021).

Missing branches: media authoring (Flash, Authorware, Storyline), education (Scratch, H5P), games (Twine, Roblox, Dreams), spreadsheets/end-user programming (Excel/Lotus formulas, LabVIEW, Max/MSP), and the 2022–2026 AI moment beyond a single 2021 node.

---

## 5. Schema and edge-level gaps

These are the modeling decisions that make the above hard to fix:

1. **No person–person relation.** Nothing for `worked_with`, `mentored`, `succeeded_by`, `hired`, `co_founded`.
2. **No org–person relation.** Nothing for `employed_by`, `founded`, `led`, `funded`.
3. **No event–person relation.** Events use `about` to concepts/artifacts; there is no `participated_in`, `keynote_by`, `organized_by`.
4. **No org–org relation.** Nothing for `spun_off_from`, `acquired`, `partnered_with`, `competed_with`.
5. **No geography relation.** Nothing for `located_in` a place; the existing `located_in` is artifact-in-artifact (fan stacks inside HyperCard).
6. **No standards/citation relation.** Nothing for `standardized_by`, `cited_in`, `quotes_passage`.
7. **`created_by` never points to an org.** Authorship is always individual; institutional authorship (Apple, Claris, Cyan) is unrepresentable except through `owned_by`.
8. **Provenance is unweighted.** `about` and `documented_in` have no `confidence` and no `note`. The corpus's epistemic layer is the least annotated.
9. **Threads have no sources and no confidence.** `in_thread` is 116 edges of pure curation; there is no way to say a thread's framing is contested.
10. **Artifact taxonomy is underused.** `file_format` and `archive` are allowed but unused; there is no `stack` kind; the Web is typed as `software`.
11. **No stance relation.** `contrasts_with` records contrast, but there is no `criticizes`, `rejects`, or `argues_against`; the corpus cannot model Ted Nelson's critique of HyperCard's thin hypertext as a directed position.
12. **No “anti-successor” concept.** There is no node for the argument that HyperCard should not be revived, or that the web's victory was correct. The graph is structurally biased toward succession.

---

## 6. Prioritized remediation plan

Ordered by leverage, not by ease.

### Tier 1 — epistemic repair (do first)

1. **Provenance pass.** Source every high-confidence node. Prioritize the 95 unsourced high-confidence nodes, starting with the object model (`card`, `message`, `part`, `property_model`), the tensions, and the modern analogs. Add `confidence` and `note` to `about` and `documented_in`.
2. **Replace weak sources.** Turn `s_github_hypertalk` (a search URL) into specific repository sources. Add URLs to the 7 URL-less sources. Add at least one primary archive: an Apple manual, a Macworld/MacWeek archive, a patent, or a court record.
3. **Reconcile thread prose with edges.** Fix `t_modernization_tensions` first: `security_naivete` has no addresser; `q_collaboration` and `q_who_is_the_author` are questions, not tensions; `bolted_on_color`, `succession_failure`, and `web_displacement` are the actual omissions.

### Tier 2 — social and institutional history

4. **Add person–person and employment edges.** Start with the HyperCard team: Atkinson–Winkler, Atkinson–Calhoun, Calhoun–Crow, Atkinson–Hertzfeld (General Magic), Jobs–Atkinson, Sculley–Atkinson. Then the standards body and the Claris split.
5. **Connect events to actors.** `e_hc3_cancelled_2000 → Jobs/Calhoun/Crow/Apple`; `e_release_1987 → Atkinson/Sculley/Apple`; `e_hc3_wwdc_1996 → Crow`; `e_virus_1991 → Apple` (response); `e_discontinued_2004 → Apple`.
6. **Add the missing orgs.** FileMaker/Claris Inc., Brøderbund, Cyan, Voyager, Silicon Beach, Macromedia, Apple Education/ACOT, the HyperTalk Standards Committee.

### Tier 3 — chronology and geography

7. **Fill 1997–2013.** Add events for the Jobs return (1997), OS X (2001), Classic removal/Intel (2006), iPad (2010), iBooks Author (2013), and the 30th anniversary (2017).
8. **Fill 2022–2026.** Add nodes for the current revival/AI moment; the corpus already says “in 2026” in five places. At minimum: an AI-assisted authoring artifact, a 2024–2026 community event, and a post-2021 Decker/Stacksmith update.
9. **Add 1988–89.** CompuServe library, HyperHeaven, the HyperTalk Standards Committee, the first user groups/magazines.
10. **Add non-US coverage.** Localized HyperCard editions, a UK/European/Japanese stackware scene, and at least one non-English source.

### Tier 4 — practice, education, media

11. **Add `practice` nodes.** Stack idioms, navigation patterns, background hierarchies, visual culture, sound/music practice, the XCMD vendor ecosystem, and the authoring workflow.
12. **Add education.** ACOT, classroom-stack artifacts, education research sources, and Authorware/Storyline/H5P as modern analogs.
13. **Add media/game/e-literature.** Flash, Scratch, Twine, Cosmic Osmo/Spelunx, the *You Don't Know Jack* prototype, `afternoon, a story`.
14. **Add `file_format` and `archive` artifacts.** The stack format, Basilisk II/SheepShaver/Mini vMac, the Internet Archive as an artifact.
15. **Add law/IP.** The ZoomRacks lawsuit, Apple's format ownership, the HyperCard trademark, ViolaWWW's patent litigation.

### Tier 5 — framing

16. **Add the counter-case.** A tension or concept for “the web won because HyperCard's limits were real,” and a concept for “the case against revival.” Raise `q_authoring_runtime_unity` from an actual tension so the corpus's central premise is genuinely contested.
17. **Fix the hard errors.** Myst/IBM, Myst/custom engine, Atkinson's departure, the Sculley quote, Claris→FileMaker 1998, HyperCard 3.0's 1998–2000 sequence.

---

## Appendix A — Node inventory

| Table | Count | Notes |
|---|---:|---|
| `concept` | 81 | 19 object_model, 17 modern_analog, 13 tension, 10 question, 7 domain, 5 philosophy, 5 idea, 4 language, 1 practice |
| `artifact` | 38 | 32 software, 3 publication, 1 hardware, 1 website, 1 language |
| `person` | 27 | 13 leaves |
| `source` | 21 | 14 medium, 7 high; 7 no URL |
| `event` | 12 | 8 unsourced |
| `thread` | 7 | all pure sinks; 0 sourced |
| `org` | 3 | apple, claris, livecode |

## Appendix B — Edge inventory

| Edge | Count |
|---|---:|
| `in_thread` | 116 |
| `about` | 83 |
| `influenced` | 37 |
| `maps_to` | 24 |
| `documented_in` | 24 |
| `created_by` | 21 |
| `derived_from` | 18 |
| `part_of` | 18 |
| `implements` | 17 |
| `exemplifies` | 17 |
| `addresses` | 17 |
| `raises` | 14 |
| `enables` | 13 |
| `limits` | 13 |
| `applied_to` | 11 |
| `located_in` | 8 |
| `contrasts_with` | 8 |
| `owned_by` | 7 |
| `motivates` | 7 |
| `contributed_to` | 6 |
| `succeeded` | 4 |

## Appendix C — High-confidence unsourced nodes (95)

**Artifacts (14):** `director`, `guide`, `hard_days_night`, `hyperstudio`, `livecode`, `macpaint`, `marble_springs`, `note_cards`, `rolo`, `storyspace`, `the_manhole`, `viawww`, `whole_earth_catalog`, `world_wide_web`

**Concepts (54):** `applescript`, `art_exhibition`, `author_player_politics`, `background`, `bolted_on_color`, `business_model_clash`, `card`, `chunk_expression`, `command_palette`, `education`, `end_user_programming`, `find_mark_sort`, `fixed_canvas`, `html_hyperlink`, `hypermedia_publishing`, `hypertext`, `idle_time`, `immediate_persistence`, `javascript_dom`, `jupyter`, `local_first`, `low_code`, `memex`, `message`, `message_box`, `monolithic_file`, `natural_language_syntax`, `no_structured_data`, `notion_airtable`, `obsidian_roam`, `part`, `platform_lockin`, `player_split`, `property_model`, `prototyping`, `pseudo_arrays`, `q_authoring_runtime_unity`, `q_collaboration`, `q_constraints`, `q_data_model`, `q_distribution`, `q_live_editing_web`, `q_modern_stack`, `q_who_is_the_author`, `script_property`, `security_naivete`, `single_user`, `spa_routing`, `target_me_it`, `tiddlywiki`, `tool_palette`, `wasm_plugins`, `wiki`, `www`

**Events (8):** `e_claris_player_1992`, `e_demo_1968`, `e_discontinued_2004`, `e_hc2_1990`, `e_hc3_cancelled_2000`, `e_hc3_wwdc_1996`, `e_merryxmas_1993`, `e_virus_1991`

**Persons (19):** `alan_kay`, `bill_appleton`, `brendan_eich`, `dan_crow`, `dan_winkler`, `donald_brown`, `doug_engelbart`, `john_sculley`, `kevin_calhoun`, `kevin_cuneo`, `pei_yuan_wei`, `robert_cailliau`, `steve_jobs`, `ted_nelson`, `tim_berners_lee`, `tom_pittman`, `uli_kusterer`, `vannevar_bush`, `ward_cunningham`

## Appendix D — Person leaves (13)

`steve_jobs`, `john_sculley`, `steve_wozniak`, `susan_kare`, `donald_brown`, `pei_yuan_wei`, `brendan_eich`, `ward_cunningham`, `bill_appleton`, `kevin_cuneo`, `tom_pittman`, `jakob_nielsen`, `tim_berners_lee`

## Appendix E — Tensions and questions

**Tensions (13).** Addressed (9): `no_networking`, `no_inline_text_links`, `no_structured_data`, `single_user`, `interpreter_performance`, `monolithic_file`, `platform_lockin`, `business_model_clash`, `author_player_politics`.
Unaddressed (4): `bolted_on_color`, `security_naivete`, `succession_failure`, `web_displacement`.

**Questions (10).** Raised (9): `q_modern_stack`, `q_message_path_modern`, `q_live_editing_web`, `q_distribution`, `q_data_model`, `q_collaboration`, `q_preservation`, `q_who_is_the_author`, `q_constraints`.
Unraised (1): `q_authoring_runtime_unity`.

## Appendix F — Verification notes

External checks performed for the most consequential factual claims:

- **Claris → FileMaker rename:** 1998, not 2001. (Claris corporate history; multiple sources.)
- **Atkinson's departure from Apple:** 1990, to co-found General Magic. He did not stay through the Jobs years. (Wired, Wikipedia, Pogue interview.)
- **“Sculley's stink”:** Jobs's view, reported by Atkinson; the Wikipedia HyperCard article attributes it to Atkinson's account of Jobs's reasoning, not to Atkinson's own phrase for the Sculley era.
- **HyperCard 3.0:** development stalled in 1998 when the QuickTime team pivoted to QuickTime 4.0 streaming; the team was reassigned in 2000; Dan Crow's account says the project was canceled in late 1998.
- **Myst:** the original 1993 Mac release was built in HyperCard with a proprietary colorized HyperTint; later ports used a custom engine. The Performa is an Apple line, not IBM.
- **ViolaWWW:** first browser with scripting, stylesheets, and tables; popular at CERN; Mosaic is the browser usually credited with popularizing the graphical web.
- **Apple Classrooms of Tomorrow:** 1985–1995; HyperCard was a central authoring medium in ACOT classrooms and assessment work.
- **HyperTalk Standards Committee:** formed 1988–89; named in the corpus's own `s_wiki_hypertalk` note.
- **Authorware:** Wikipedia's HyperCard article explicitly names Macromedia Authorware as one of the systems HyperCard authors moved to.

---

*End of critique. No corpus files were modified.*

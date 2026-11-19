# Reconciling the corpus with the coverage critique

*2026-11-06 · corpus was 422 nodes / 1023 edges when this ledger was written (critique at 189 / 483). **Superseded in part:** a later fact-check and prune (`pruning-2026-11-06.md`) removed many of the nodes added here and corrected others; counts and the "running database" note below describe the earlier state.*

`coverage-critique.md` is kept as written, as a dated snapshot. This note records
what happened to each of its findings. Three outcomes: **fixed** (corpus changed),
**disputed** (critique is wrong, stale, or unverifiable, so the corpus was changed
differently or not at all), **open** (not done, with the reason).

Verification was uneven, so each factual change below says how strongly it is
supported. Pages actually read in full: two TidBITS articles (1990, 1998). Pages
seen only as search results: Wikipedia (HyperStudio, SuperCard) and a news item
on iBooks Author. The Wired 2002 interview could not be fetched at all.

## 1. Hard factual errors (critique §4.1)

| Finding | Outcome |
|---|---|
| Myst "bundled on the IBM Performa 5300" | **Fixed.** Performa is an Apple line. The model number is dropped and the bundling is now "reportedly … some Apple Performa Macs" — the specific claim is still unverified. |
| Myst "shipped on a custom engine" | **Fixed, and the corpus contradicted itself.** `71_hypertalk_code` quotes verbatim HyperTalk from the shipped 1993 disc. The 1993 Mac release was HyperCard; later ports reimplemented it. The `influenced` edge note was corrected too. |
| Atkinson "stayed through the Jobs years" | **Fixed.** Removed from his bio and from `business_model_clash`. The bio already said he left in 1990, so it contradicted itself. Jobs returned in 1996–97. |
| "Sculley's stink" is Atkinson's phrase | **Fixed with a hedge, not the critique's replacement.** The critique says it was Jobs's view, relayed by Atkinson. Wired could not be fetched, so that reading is *also* unverified. The corpus now says the phrase is reported as Jobs's view as relayed by Atkinson, secondhand, medium confidence. The incoherent "stayed *because* of the stink" is gone. |
| Claris became FileMaker Inc. in 2001 | **Fixed → 1998** (Claris name revived 2019). New event `e_claris_renamed_1998`, new org `filemaker_inc`. |
| "Team split into Claris during 1987–90" | **Fixed.** Claris was founded 1987; HyperCard moved to it in 1990. TidBITS (Nov 1990) treats the transfer as already done and gives no date. The critique's "November 1990" is **not** supported — only "by November 1990". |

## 2. Overstated or unsupported claims (§4.2)

All fixed by qualifying, not deleting: ViolaWWW ("first popular graphical
browser" → early browser, first with scripting/stylesheets/tables by common
account; Mosaic popularized the graphical web); "largest install base" (event,
language-philosophy concept, rise-and-fall thread); "best-selling Mac book"
(person, two concept bodies, source note); "billion-dollar shadow"; "last
environment able to author" → "last officially supported"; "first document-carried
viruses anywhere" → "among the earliest on record".

**HyperCard 3.0's date: critique partly wrong.** It says Dan Crow's own account
dates the cancellation to late 1998. That could not be verified. What a
contemporaneous source (TidBITS, 2 Nov 1998) does support: QTi slipped in 1997,
and in late October 1998 the HyperCard team was reported moved to QuickTime with
no further development, announced (per unofficial sources) by Phil Schiller. The
corpus now records a two-stage sequence — new event `e_hc_team_moved_1998`
(medium) — and keeps the 2000 date as the "formal cancellation", flagged as not
re-verified. Where the 2000 date originally came from is unknown.

## 3. Errors the critique missed or introduced

Found while checking, all fixed:

- **iBooks Author was announced January 2012, not 2013.** The corpus said 2013
  and so did the critique's remediation list. A "twenty-one years later" body
  line is now twenty. The Atkinson "heir" remark is traceable only to a 2016
  secondhand forum post, not the 2017 direct statement the corpus asserted; it is
  now hedged everywhere it appears.
- **HyperStudio** is credited to Roger Wagner (Roger Wagner Publishing, 1989, Apple
  IIGS first), not Kevin Cuneo/Brøderbund. `person:roger_wagner` added; Cuneo kept
  at confidence `low` with a `contributed_to` edge saying his role is unverified.
- **SuperCard** is 1989 and published by Silicon Beach, and Bill Appleton did not
  "leave Apple" to make it. Both fixed; "Appleton had worked with Atkinson" removed
  as unsupported.
- **Claris "shipped 2.4.1 (1998)"**: HyperCard had been returned to Apple (TidBITS
  1998), so the org note was wrong.
- **FileMaker "its creator ran the same shop"** was unsupported; now "sold by Claris".
- **HyperCard 2.0 "one year after Atkinson left"**: both are 1990.

## 4. Structural findings (§4.3, §3.8, §3.9)

| Finding | Outcome |
|---|---|
| Thread prose contradicts its edges | **Fixed.** `t_modernization_tensions` rewritten to say exactly what the edges say; it no longer calls questions "tensions" and lists the real null tensions. |
| `security_naivete` has no addresser | **Fixed, partially.** `wasm_plugins → addresses → security_naivete` added at medium confidence, with a note that it is a permission regime, not a fix. |
| `q_authoring_runtime_unity` raised by nothing | **Fixed.** Its own body names three causes; `raises` edges from `author_player_politics`, `security_naivete`, `business_model_clash` added. Questions raised: 10/10. |
| Three tensions unaddressed | **Disputed.** `bolted_on_color`, `succession_failure`, `web_displacement` stay unaddressed — the traversal report already argued these are honest nulls. The thread now says so. |
| No counter-case / anti-successor | **Fixed.** New `concept:anti_successor`, built only from existing tension nodes, raising `q_authoring_runtime_unity` and contrasting `q_modern_stack`. |
| "One component" depends on `in_thread` | **Fixed in the docs.** README states the caveat; `analyze.py` prints component counts with and without curation edges (1 / 8 / 43). |
| Source notes name systems with no node | **Partly fixed.** Authorware, Flash, FileMaker added. `hc2html`, `openxion`, `hyperjs`, `tilestack`, `novocard`, `baycard`, Plus/WinPlus, mTropolis, Stagecast remain names in notes. |
| Modern analogs skew to KM/web | **Partly fixed.** Scratch, Twine, Flash, Authorware added as artifacts. Excel/LabVIEW/Max-MSP/Roblox and H5P/Storyline not added. |

## 5. Missing angles (§2)

| § | Gap | Outcome |
|---|---|---|
| 2.1 | Chronology | **Partly fixed.** Added 1988 (CompuServe), 1990 (Claris transfer), 1997 (Jobs), 1998 ×2, 2001 (OS X), 2006 (Intel), 2010 (iPad), 2012 (iBooks Author), plus `classic_environment`. **Open:** 2017 anniversary (nothing to say beyond the emulation event), and **2022–2026** — the corpus still says "in 2026" in five places with no node under it. Needs real research. |
| 2.2 | Institutions & actors | **Mostly fixed.** 9 orgs added; events with a participant went 0 → 18 of 21 (the critique counted 2 event→person `about` edges). Still without an actor: the two virus events and the 2017 emulation. **Open:** HyperTalk Standards Committee (appears only as a name in a source note), user groups, magazines. |
| 2.3 | Social graph | **Partly fixed.** 5 `worked_with` edges, 9 `employed_by`; person leaves 13 → 10 of 30. Mentorship and Claris staffing are unsourced and not added. |
| 2.4 | Place/language | **Open.** Needs non-English sources; none added. |
| 2.5 | Practice | **Thin.** `stack_idioms` and `visual_culture` added, wired to the verbatim code layer, confidence medium. Authoring workflow, XCMD ecosystem, sound, failure/abandonment still missing. |
| 2.6 | Education | **Partly fixed.** ACOT, Scratch, Authorware. **Open:** classroom stacks, education research sources. ACOT's role for HyperCard is unsourced. |
| 2.7 | Media/games/e-lit | **Mostly fixed.** Flash, Scratch, Twine, Cosmic Osmo, Spelunx, *afternoon*. *You Don't Know Jack* prototype not added (could not verify). |
| 2.8 | Format/runtime artifacts | **Fixed.** `stack_format` (`file_format`), Basilisk II, SheepShaver, Mini vMac, Internet Archive (`archive`). Both previously unused artifact kinds are now used. Emulator first-release years are medium confidence. |
| 2.9 | Law/IP | **Open.** ZoomRacks suit, trademark, format ownership all need research. |
| 2.10 | Stackware economy | **Partly fixed.** CompuServe org + event. HyperHeaven, Renault, kiosks not added (unsourced). |

## 6. Provenance and schema (§3.1–3.7, §5)

- **Unsourced high-confidence nodes:** 95 (critique) → 89 now, *after* adding 43
  nodes. Policy adopted: a node with no source link is at most `medium`. Thirteen
  new nodes the author was confident about were downgraded for that reason.
- **Sources added (5):** two TidBITS pieces, two Wikipedia pages (via search
  results), one news item. Each source note says what was and wasn't read.
- **Still open:** `about`/`documented_in` carry confidence on only 23/110 and
  11/47 edges, almost all of them new. Backfilling them needs a judgement per edge;
  a mechanical default would invent confidence the corpus doesn't have. Also open:
  `s_github_hypertalk` is still a search URL, seven sources still have no URL, no
  primary Apple document or archive, threads still have no sources.
- **Schema, done:** `worked_with`, `employed_by`, `participated_in`,
  `spun_off_from`, `renamed_to`, `published_by`; `created_by` may point at an org;
  `note` on `about` and `documented_in`. **Not done:** geography, standards/citation,
  a stance relation (`criticizes`), a `stack` artifact kind.

## 7. What to check before trusting the additions

Added from general knowledge and not tied to a source in the corpus (all
`medium`): Flash, Scratch, Twine, FileMaker origins, Cosmic Osmo, *afternoon*,
Hertzfeld/General Magic, the Jobs-return, OS X and Intel events,
Basilisk II / SheepShaver / Mini vMac years, Spelunx's year, Authorware's dates,
ACOT's HyperCard role, and the claim that HyperCard authors moved to Authorware
(`low`, taken from the critique's reading of Wikipedia). They are plausible and
mostly well attested, but a provenance pass should replace "plausible" with a link.

**Database state at the time:** the store was reloaded with 422 nodes. It has since
been reloaded again after the prune (353 nodes / 869 edges).

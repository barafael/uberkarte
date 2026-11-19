# Prototype experiment: a HyperTalk-style runtime in Rust

*2026-11-18. Follows the gap analysis (seven underrepresented properties). Code: `prototype/spike/`.*

## Question

Can a Rust host with an embedded dynamic language deliver the four riskiest
properties together: editing the running thing, handlers at any level, readable
scripts, one inspectable file? Target uses: UIs, multimedia, games.

## Stack

| Choice | Decision | Reason |
|---|---|---|
| Script engine | Rhai 1.27, unforked | Sandboxed by default, operation limits, custom syntax, compiles to WASM |
| UI framework | Dioxus (0.8 alpha on crates.io today; pick the latest stable) | Desktop and web player from one codebase; desktop can open a stack file directly |
| File format | One readable text file: snapshot plus append-only change journal | Undo and history for behavior; CRDT sync is a later experiment |
| Graphics | DOM for cards and parts, one canvas part for sprites | DOM covers UIs and multimedia; canvas is the test for real-time games |

The object model and script engine stay in crates with no UI dependency, so a
Leptos renderer over the same model is a two-day comparison, not a rewrite.

## Hypotheses

| # | Hypothesis | Fails if | Status |
|---|---|---|---|
| H1 | The message path works on stock Rhai | Needs a fork, or dispatch > 50 µs per message | **Passed in the spike** (0.3–0.4 µs) |
| H2 | A script or property edit is live with no rebuild | Edit-to-effect > 100 ms, or state lost on edit | **Passed for scripts** (2.9 µs per edit; see player results) |
| H3 | Rhai custom syntax can express English-shaped forms | Under 70% of the 149 keyword nodes map cleanly | **Passed: 79% clean** (117 of 149), 91% with an equivalent |
| H4 | Three stacks can be built entirely inside the tool | Any needs Rust changes after week 3 | **Partly**: one card of Pillars built in the player from an empty stack; see 2026-11-19 |
| H5 | A sprite game holds 60 fps with per-frame handlers | Cannot hold 30 fps on a mid-range laptop | **Passed in the web build** (60 fps; 78 µs native / about 300 µs wasm per frame). Desktop not run |
| H6 | Non-programmers can read a script and predict it | Fewer than 3 of 5 testers predict two handlers | **Dropped** (2026-11-19): the project owner keeps braces without a reader study; materials stay in `reader-test.md` |

Test stacks: an address book with find and sort (UI); a point-and-click scene
with transitions and sound, porting the corpus's 66 Myst lines (multimedia); a
Breakout clone on the canvas part (game).

## Spike results (H1 and H3, moved to the start)

`cargo run --release` in `prototype/spike/`. About 300 lines, no UI.

**What works on stock Rhai**

- Message path: button → card → background → stack → Home. Each object holds
  its script; a message runs the first matching `on <name> { }` going up.
- `pass` leaves the handler and the message continues upward.
- Commands are messages first. `answer` and `go` were intercepted by handlers
  higher on the path and then passed to the built-in.
- English-shaped statements: `put <expr> into <var>`, `put <expr> into field
  <expr>`, `set <prop> of me to <expr>`, `go to card <expr>`, `answer <expr>`,
  `repeat <n> times { }`, handler parameters, nested chunk reads
  (`char 1 of word 2 of field "order"`).
- Dispatch cost: 0.32 µs handled three levels up, 0.40 µs four levels up
  (200,000 messages each, release build, this machine).
- A runaway `loop { }` is stopped by the operation limit.

**What stock Rhai cannot do** (4 of 11 trial forms fail to parse)

- Statements need semicolons.
- Blocks need braces: no `if … then … end if`, no `on mouseUp … end mouseUp`.
- A chunk cannot be written to (`put "z" into word 2 of field "order"`), because
  custom syntax has no assignable expressions. Each writable target needs its own
  hand-written parse branch.

**Reading.** The runtime half of the design is settled: Rhai's evaluator, values
and sandbox carry the message path without a fork. The surface is not HyperTalk;
it is "Rhai with English commands": braces and semicolons stay. Whether that is
readable enough for a novice is H6's question, and it should be tested early
with two versions of the same handler on paper. If braces and semicolons fail
that test, the project needs its own parser that produces the same runtime
calls; the spike shows the runtime would not change.

**Not yet measured:** the 70% criterion against the 149 keyword nodes. The spike
tried 11 forms, not the full list.

## Player results (2026-11-18)

Decision taken: accept braces and semicolons, build on Dioxus 0.7.10.

Code: `prototype/stack-core` (model, journal, file format, script engine; no UI
dependency) and `prototype/player` (Dioxus; desktop by default, web with
`--no-default-features --features web`). Run the desktop player from
`prototype/` with `cargo run -p player`; it creates `stacks/addressbook.stack`
on first run and rewrites it after every change.

**Checked**

- `cargo test -p stack-core`: 5 tests pass. They cover file round-trip,
  navigation, per-card text in background fields, `answer`, `find`, `new card`,
  undo of a new card, intercepting `answer` at stack level, a broken edit
  leaving the last working script live, the capability gate on `beep`, and a
  runaway loop being stopped.
- The web build was driven in headless Chromium: the card buttons navigate, the
  stack's `openCard` handler updates the status field, `answer` shows a dialog,
  the script editor replaces a button's script and the button uses the new
  script with no rebuild, a syntax error is shown while the old script keeps
  running, and the message box runs statements.
- Cost of one live script edit (journal plus recompile): 2.9 µs. Writing the
  whole stack file (2.6 kB): 48.5 µs. Native release build.

**Not checked**

- The desktop window was compiled but never opened, so file writing on every
  change and the measured edit time shown in the editor are untested in use.
- Dragging parts in edit mode, the + Button / + Field / Delete tools, and the
  numeric inspector fields were not exercised.

**Known issues found** (the first two are fixed, see below)

- ~~Undo is polluted by script-driven writes.~~
- ~~Statements outside an `on` block run on every message.~~
- No coercion between text and numbers: `field "count" + 1` concatenates. A
  `num()` function exists as a stopgap.
- The message path has no Home level yet (part, card, background, stack,
  built-in).
- The web build has no persistence; it always opens the demo stack.
- The journal grows without bound.

## Undo rule, canvas part and Breakout (2026-11-18)

**Undo rule.** Every journal entry records whether it was written while a
system message was being handled (`openCard`, `closeCard`, `frame`). Those
entries are marked `auto`. Undo reverts the newest group that contains a
non-automatic change and reverts only its non-automatic changes; automatic ones
stay. Consecutive automatic writes to the same property merge into one entry,
so a score updated every frame adds one entry, not sixty a second. Writes from
`mouseUp` handlers still count as user actions, so Undo after New card removes
the card.

**Loose statements** are now a compile error: a script may contain only
`on <message> { }` handlers. The previous working script stays live, as with
any other compile error.

**Reference names.** A literal name or number ends a part reference, so
`the left of canvas "playfield" - 40` subtracts 40. A computed name still takes
a whole expression and needs parentheses.

**Globals.** `global a, b;` loads shared variables into a handler and writes
them back when it ends, as in HyperTalk. They are runtime state, not saved with
the stack. A script edit does not reset them, so a running game keeps its state
through a live edit.

**Canvas part.** A new part kind. Scripts draw with `clear()`,
`rect(x, y, w, h, colour)`, `circle(x, y, r, colour)` and
`text(x, y, text, colour)`, in canvas coordinates. The player renders the draw
list as SVG. In browse mode, a card with a canvas gets `frame` once per display
frame (requestAnimationFrame, on web and desktop). Scripts read the pointer with
`the mouseH` / `the mouseV` (card coordinates) and keys with `isKeyDown(key)`.

**Breakout** (`demo::breakout`, also in the player's stack menu): fifty bricks,
paddle, ball, score and lives. All game logic is in the card's script, about 50
lines.

**Checked**

- `cargo test -p stack-core`: 8 tests pass, adding undo-after-navigation,
  loose-statement rejection and 2,000 Breakout frames (77.7 µs per frame
  native, release build, journal stays at one entry).
- Web release build in headless Chromium:
  - Undo after edit, Next, Prev restores the edited field and leaves the status
    field alone.
  - Breakout holds 60 fps with about 300 µs of script per frame.
  - The paddle follows real mouse input exactly.
  - Editing the card script during play (changing the ball colour) applied at
    once, and the score and lives carried on.

**Not checked**

- The desktop window has still not been opened. That leaves untested: file
  writing, the frame loop under the desktop webview, and keyboard input. The
  arrow keys were not tested anywhere.
- How fast the SVG redraw is with more shapes than fifty.

**New issues**

- Changes made by `frame` handlers are written to disk only on the next user
  action, because the frame loop does not save.
- The canvas draw list is per world, not per canvas part: only one canvas per
  card works.
- `frame` is sent only in browse mode; edit mode pauses the game, which is
  probably right but was not a decision anyone made.
- In the browser, script timing uses `performance.now()`, whose resolution is
  coarsened, so the 300 µs figure is approximate.

## Schedule (six weeks, one developer)

1. Spike (done). Paper reading test for braces versus `end` blocks. Model,
   journal, file round-trip.
2. Message path in its own crate, capability gating per stack.
3. Dioxus player: browse and edit modes, script editor, message box (H2).
4. Keyword-list coverage (H3 in full). Address book stack.
5. Canvas part and frame loop. Multimedia and game stacks (H4, H5).
6. Reader test with five people (H6). Leptos comparison. Written results.

## Out of scope

Collaboration, network distribution, signing, and a model writing the script.
The last waits until the surface syntax is settled.

## Smaller fixes and editor (2026-11-18)

- **Text and numbers.** Text that reads as a number takes part in arithmetic and
  comparison: `"41" + 1` is 42, `"7" / 2` is 3.5, `"10" > "9"` is true.
  `"Score " + 3` still joins the text, because `+` is also this language's join.
  The two meanings clash: `field "phone" + 0` on "020 7946 0001" joins
  rather than raising an error, since the text has spaces in it. HyperTalk keeps
  `+` for numbers and joins with `&`. **Open decision:** adopt `&` for joining
  and make `+` numbers-only, or keep the double meaning.
- **Home.** The message path is now part, card, background, stack, Home,
  built-in. Home is its own file (`stacks/home.stack`), loaded once and kept
  when another stack opens, with a `help` handler by default. Its script is
  editable from the inspector's Home button. Home edits stay out of the open
  stack's journal, so they never leak into a stack file; the cost is that Undo
  does not cover them yet.
- **Several canvases per card.** Each canvas has its own draw list. Drawing
  goes to the innermost `with canvas "name" { }` block, else to `me` if it is
  a canvas, else to the first canvas on the card.
- **Editor.** The inspector is resizable (drag its left edge, 260–1100 px).
  Scripts are syntax-highlighted: handler names, keywords, commands, object
  words, chunk words, strings, numbers and comments. Tab inserts four spaces.
- **Checked:** 11 core tests and the highlighter test pass. In headless
  Chromium:
  - coercion and Home's `help` work from the message box;
  - dragging the edge 150 px widened the inspector by 150 px;
  - typed text and its highlighting stay identical and aligned.
  - Breakout: about 50 µs per frame natively after the operator change.
- **Not checked:** the desktop window for these changes; long scripts in the
  editor (the highlighter re-runs on every keystroke).

## Next: the bigger points

Order chosen so that each step can change the next.

### 1. Multimedia stack, built inside the tool (H4, about two weeks)

Scope comes from the corpus's Myst handlers. Their commands are mostly
`play` / `play stop` / `the sound is done`, `hide`/`show`, screen locking with
visual effects, chunk writes into variables, and Cyan's own XCMDs (`hc`, `ww`,
`htLock`, `hyperTint`). Needed:

1. **Assets in the stack file.** Pictures and sounds embedded in the one file
   (base64 tables in the TOML for now). Keeps the stack a single file; a
   container format can come later if files get large.
2. **Picture part** (or a card picture), and **transparent buttons** to make
   hotspots over it.
3. **Sound**: `play "name"`, `play stop`, `the sound is done`, behind the
   `sound` capability. Needs a capability prompt in the player, not just an
   error.
4. **Visual effects on `go`**: `go next with dissolve` (and wipe, push) as CSS
   transitions; `lock screen` / `unlock screen`.
5. **`hide` / `show`** for parts.
6. **Chunk writes**: `put "on" into item 3 of x`. This is the gap the spike
   found. It needs hand-written parse branches for each writable target
   (variable, field), and is the hardest item here.
7. **Then build the stack in the player, without Rust changes**: a three-card
   point-and-click scene with the pillar puzzle ported from Myst (XCMD calls
   replaced by built-ins). Every place where Rust was needed is a finding for H4.

### 2. Keyword coverage (H3, about three days, can run beside 1)

Script the 149 keyword nodes from the corpus into test cases: one minimal
HyperTalk use per keyword, its nearest form in this language, and whether it
parses and runs. Report the share that maps cleanly against the 70% threshold,
grouped by failure kind (block syntax, chunk writes, missing command, no
equivalent). It needs no design decisions and settles how far the language is
from HyperTalk.

### 3. Reader test (H6, about one day plus scheduling)

Five people who don't program. Four handlers on paper: two from the address
book, two from Breakout, plus the same two written HyperTalk-style with `end`
blocks for comparison. Ask what each one does; score predictions against the
behaviour. Pass: at least 3 of 5 read two handlers correctly in this language.
If braces score clearly worse than `end` blocks, revisit the decision to keep
them before more stacks are written.

### Open decisions for the project owner

- `+` as join, or HyperTalk's `&` (above).
- Assets embedded in the TOML file, or a container (zip) from the start.
- Whether a stack asking for a capability gets a prompt, or must be granted in
  advance.

## Decisions and multimedia work (2026-11-18)

**Decisions taken by the project owner**

- `&` joins text; `+` adds numbers only (and number-like text). `"Score " + 3`
  is now an error that suggests `&`. `&` replaces Rhai's bitwise `&`; use `&&`
  for "and".
- A stack file is a zip container: `stack.toml` (objects, scripts, journal,
  asset list) plus `assets/pictures/...` and `assets/sounds/...`. Reading the
  stack's source now means unzipping it first. A plain-TOML stack from format 1
  still opens. Its `+` joins are rewritten to `&` wherever a string literal sits
  next to the `+`. Other `+` are left alone, since they may add numbers. The
  player keeps the original as `<name>.stack.v1.bak` before its first save.
  Both existing stacks in `prototype/stacks/` migrate with no script errors
  (checked read-only; five joins rewritten).
- Sound is always allowed. No power is gated by a capability now; the
  `capabilities` list remains for later ones (files, network).

**Built (plan items 1–6 for the multimedia stack)**

- Pictures and sounds stored in the stack; picture parts (`picture` property
  names an asset); transparent buttons (`style` = transparent) as hotspots.
- `play "name"`, `play stop`, `the sound` ("done" when nothing plays), and a
  `soundDone` message to the card when a sound ends. HyperTalk scripts wait
  with `repeat until the sound is done`; here that loop would never end
  (scripts run on the one UI thread), so scripts handle `soundDone` instead.
- `go … with dissolve | wipe left | wipe right | iris open`, and
  `lock screen` / `unlock screen with <effect>`. Effects are CSS animations on
  the new card; a dissolve fades the new card in over the page, not over the
  old card.
- `hide` / `show` for parts, `the target` and `target` as a reference (a card
  script can handle clicks for many buttons).
- Chunk writes: `put x into|before|after item 3 of line 2 of v` for variables
  and fields. Missing items and lines are added, and word spacing is kept.
- A bare variable name now ends a chunk expression, so
  `word 1 of name == "pillar"` compares the word.
- Player: + Picture and + Sound import files (any common image or audio type).
  The inspector gets a picture chooser, a "transparent" switch for buttons and
  the stack's asset list with play buttons. The highlighter knows the new words.

**Pillars** (`demo::pillars`, in the stack menu): three cards: a shore with
hotspots, eight pillars, a dock. The card script for the pillars is the Myst
pillar handler ported. It keeps the original structure (state in a
comma-separated global, a winning pattern compared as text, sound on change),
with Cyan's XCMDs replaced by built-ins. All pictures are generated SVG and
the two sounds are synthesized WAV, so the stack carries no third-party media.
The file is 88 kB.

**Checked**

- `cargo test`: 16 core tests and the highlighter test pass. New ones cover
  chunk writes, migration, the zip round trip, sound / target / hide / effects,
  and solving the puzzle.
- Web release build in headless Chromium (autoplay allowed):
  - Clicking the island hotspot plays the dissolve, the pillars light and go
    out, and 2, 5, 6 solves the puzzle.
  - The browser played `click` and `ship`, sending sound data only on first
    use. `the sound` was "ship" while playing and "done" after it ended.
  - The dock opens with a wipe and shows the risen ship.
  - A PNG imported through + Picture became an asset and a picture part.
- Breakout frame cost: 68–129 µs across runs on this machine (noisy), still
  far inside the 16.7 ms frame.

**Not checked / known gaps**

- The desktop window for any of this, including audio in the desktop webview.
- Asset imports are not in the journal, so Undo does not remove an imported
  picture's asset (it does remove the picture part).
- Undo after a pillar click hides the light but does not reset the puzzle
  state, which lives in a global. Globals and journaled properties can disagree.
- `put … into x` inside an `if` block creates `x` only for that block (Rhai
  scoping); HyperTalk makes it a handler-wide local.
- H4 is still open. The Pillars stack was written in Rust like the others.
  What was checked inside the tool is the import path and the inspector. To
  build Pillars entirely in the player, it still lacks:
  - a way to create a card with a given name, without editing the name after;
  - aligning or resizing several hotspots at once;
  - duplicating a part (eight hotspots by hand);
  - choosing a card's background.

## Editor, H4 and H3 (2026-11-19)

### Editor features (the H4 gaps)

- **Duplicate** (button and Ctrl/Cmd+D): copies the selected parts, or the card
  with its parts. A name ending in a number gets the next free number
  ("pillar 1" → "pillar 2"); copies are offset 12 px.
- **Several parts at once**: shift-click adds or removes parts; dragging moves
  them together. A second toolbar aligns them (left, top, right, bottom), sizes
  them (width, height, size) to the first one selected, and spreads three or more
  evenly (across, down).
- Dragging now writes each part's position once, on release. Before, every mouse
  move wrote two history entries, so one drag filled the history.
- **New card form**: name and background (existing or new) before creating.
  The inspector moves a card to another background; Undo moves it back.
- **Inspector switches**: visible, transparent (buttons), locked text and shared
  text (fields). New **Untitled** stack in the stack menu.
- Picture parts no longer take clicks in browse mode, so hotspots placed under an
  imported picture still work.
- **Undo** now covers asset imports (the asset is removed again) and background
  moves.
- **Handler-wide variables**: a variable first set with `put` inside an `if` or
  `repeat` block now lives for the whole handler, as in HyperTalk. The script
  compiler finds each handler's `put` targets and declares them when the handler
  starts. `it` always exists.

### H4: the pillar card built inside the player

Headless Chromium drove the web build from an empty stack, using only what a
person can do: toolbar buttons, the inspector, file pickers, typing and
shift-click. The media files (pictures, sounds, the card script) were exported
from the demo first, as ordinary files a user would have.

The build was 47 UI steps:
- import the pillar picture;
- add one transparent button, set its name and position, duplicate it seven
  times, move the last one, select all eight, align top and spread across;
- the same for eight hidden glow pictures;
- add a locked status field and import two sounds;
- name the card and type the card script.

Result: the eight hotspots landed at exactly the positions of the Rust-built
demo (40, 115, …, 565), the script compiled live, and in browse mode
clicking 2, 5, 6 solved the puzzle and lit the three pillars. No Rust was
written during the build.

What it took beforehand (each item was a Rust change found by trying):
- duplicate with numbering;
- multi-select with align, size and spread;
- the visible / locked / transparent switches;
- a new empty stack;
- click-through pictures;
- naming a card at creation.

**Status of H4:** one card of one stack was built in the tool. The other two
Pillars cards, Breakout and the address book have not been rebuilt this way.
Transitions between cards are set in scripts, so they need no further UI.

### H3: keyword coverage, measured

`stack-core/tests/keywords.toml` has one row per keyword node (149). Each row
has a class and evidence. A snippet that must run without error, or the name of
a test that exercises the keyword, is required for any keyword counted as
present. `cargo test --test keywords` runs every snippet and prints the tally.

| | clean | partial | renamed | missing | n/a | clean share | any equivalent |
|---|---|---|---|---|---|---|---|
| Before (2026-11-19 morning) | 22 | 20 | 21 | 74 | 12 | 15% | 42% |
| After the vocabulary batch | 72 | 23 | 10 | 32 | 12 | 48% | 70% |

**What the batch added**:
- **Constants:** empty, space, tab, quote, lineFeed, pi, zero–ten, up, down.
- **Verbal operators:** `and`, `or`, `is` (numbers as numbers, text without
  case), `contains` (without case), `div`, `mod`, and `^` as a power.
- **Chunks:**
  - `character`;
  - ranges such as `char 2 to 3 of x` (read only);
  - ordinals (`first word of`, `the last line of`, middle, any);
  - `itemDelimiter`.
- **Commands:**
  - `get` (into `it`) and `add/subtract/multiply/divide`;
  - `do` (Rhai's `do … while` loop is switched off to free the word);
  - `send … to <object>` and `pass <message>`;
  - `enable/disable`, `mark/unmark`;
  - `there is [not] a`.
- **References:** `this card/background/stack`, `card 2`, `card "name"`,
  `background`, `stack`.
- **Functions:**
  - random, length, offset, charToNum/numToChar, value;
  - math functions, which now accept whole numbers;
  - min/max/sum/average, annuity/compound;
  - param/paramCount.
- **`the` reads:**
  - date, time (UTC), seconds, ticks;
  - mouse, mouseLoc, clickLoc;
  - commandKey/shiftKey/optionKey;
  - result (set by `go` and `find`), version;
  - the number of cards / buttons / fields / lines of x.
- **Properties:**
  - rect, loc and owner;
  - enabled, hilite, showName, dontWrap and marked;
  - lockText as a name for locked;
  - lockScreen and lockMessages;
  - property names are now case-insensitive.
- **Messages:**
  - from the runtime: startup, openStack, openBackground/closeBackground,
    newCard (sent to the new card after it exists, as in HyperCard) and
    errorDialog (a handler can take the error text);
  - from the player: mouseDown, mouseEnter/mouseLeave, mouseDoubleClick,
    keyDown, arrowKey (left/right go to the previous/next card unless handled)
    and openField.

Checked: 20 core tests and the keyword test pass. The player-sent messages were
checked in headless Chromium with real mouse and keyboard input. That run found
that keyboard messages need the player to have focus, so the player now takes
focus when it starts. Breakout still costs about 41 µs per frame, so the new
operators and the constants resolver cost nothing measurable.

**What stops the remaining keywords**, in three groups:

1. **Grammar limits of stock Rhai** (about 10 keywords), which no amount of
   custom syntax fixes:
   - block delimiters: `if … then … end if`, `on … end`;
   - `--` comments;
   - two-word operators: `is not`, `is in`, `is a`, `is within`. Rhai's custom
     operators are single words.
   - `&&` (Rhai's "and", HyperTalk's join-with-space) and `return` (the newline
     constant), both taken by Rhai;
   - the precedence of `not`.

   One cost of the batch: the method form `x.contains(y)` no longer parses,
   because `contains` is now an operator.
2. **A runtime limit** (`ask`, `wait`, and `answer` returning the clicked
   button): scripts run to completion on the UI thread and cannot pause until
   the user answers. HyperTalk's dialogs block. This needs either asynchronous
   host functions (Rhai cannot suspend a script) or messages instead of return
   values, the way `soundDone` replaced `repeat until the sound is done`.
3. **Ordinary missing vocabulary** (about 20 keywords), work of the same kind
   as the batch:
   - sort, push/pop, select and the selection, convert, start using;
   - more messages (returnKey, enterInField, mouseStillDown, mouseWithin,
     commandKeyDown, newField);
   - `button id 7`, autoSelect, autoHilite, family, icon, userLevel.

**Verdict on H3.** As stated ("under 70% map cleanly" fails it), H3 fails today:
48% clean. As a question about Rhai's capacity, it passes. If group 3 were built,
the clean share could reach about 124 of 149 (83%); that ceiling is an estimate,
not a measurement. The parts of HyperTalk that stock Rhai cannot take are the
block grammar and a handful of word operators; together they are about 7% of the
vocabulary. That is the size of the question "write our own parser or not". It
also explains why the reader test matters: those few words are on almost every
line.

### H6: prepared

`notes/reader-test.md` has the protocol:
- four handlers (Hello, Status, Pillars, Bounce), each in braces and in `end`
  blocks;
- two counterbalanced groups;
- the questions, the scoring key, the pass rule and a record sheet;
- known confounds, since `is not` has to be `!=` in the braces version.

The braces handlers are the tested code: `reader_test_handlers_run` checks that
the Pillars and Bounce versions behave as the key says. Running the test needs
five people.

## Decision: braces stay, no reader study (2026-11-19)

The project owner decided not to run the reader test and to keep braces and
semicolons. There will be no own parser. The block grammar (`{ }`, `;`) stays
Rhai's. Remaining HyperTalk words are added with custom syntax, operators,
functions, messages, and a same-length rewrite of a few word pairs before
compiling (below).

### Second vocabulary batch (2026-11-19, after the decision)

**Same-length rewrite before compiling** (`hypertalk_words`). It applies to
scripts, the message box, `do` and `value()`:
- `is not`, `is in`, `is a`/`is an` and `is within` become one-word operators;
- a `--` comment becomes `//`;
- `sort`, `push`, `pop` and `contains` are capitalized when used as commands or
  operators, so they no longer take the names of Rhai's own methods.
  `list.sort()`, `a.push(x)` and `"abc".contains("b")` work again (the first
  batch had broken `.contains`).

Every replacement has the same length, so error positions still point at the
right column. Strings, `//` comments and `there is a` are left alone. A variable
named `a` compared with `is a` would be misread as a type test; nothing uses one.

**The runtime limit, handled with messages:**
- `answer "Save?" with "Yes" or "No" or "Cancel"` shows those buttons; the
  click comes back as `answered <button>` to the object that asked.
- `ask "Name?" with "Ada"` comes back as `asked <text>` (empty if cancelled).
- `send "x" [to <object>] in N seconds|ticks` delivers a message later, in
  place of `wait`.

**Other vocabulary:**
- `sort [cards] [ascending|descending] [numeric] by <expr>`: the expression is
  evaluated on each card. Undo restores the order.
- `push card` / `pop card`; `button id 7` and `card id 3` as references.
- `family`: buttons with the same family number act as radio buttons when
  clicked.
- `edit the script of <object>` opens that script in the editor.
- `is a number|integer|point|rect|date|logical|empty`, and `is within`.

**More messages from the player:**
- `returnKey` (Return in a field; a handler stops the newline) and
  `enterInField` (keypad Enter);
- `commandKeyDown`;
- `mouseStillDown` and `mouseWithin`, every frame;
- `newButton` and `newField` when the editor adds a part;
- `suspendStack` when switching stacks.

The frame loop saves only when the history changed, since these messages can
arrive 60 times a second.

**Bug found and fixed:** a background part's messages skipped the card. The
path went part → background → stack, so a card's `returnKey` handler never saw
Return in a background field. It now runs part → card → background → stack, as
in HyperCard.

**Checked:**
- 25 core tests and the keyword test pass.
- In headless Chromium with real input:
  - the three-button `answer` and the `answered` reply;
  - `ask` with its default and the `asked` reply;
  - a 1-second timer, which fired after about a second;
  - `mouseWithin`, about 40 times in half a second;
  - Return in the phone field reaching the card handler, with no newline;
  - Ctrl+K reaching `commandKeyDown`;
  - `edit the script of button "Hello"` opening that script in the editor.
- Breakout: about 56 µs per frame.

**Coverage now:**

| | clean | partial | renamed | missing | n/a | clean | any equivalent |
|---|---|---|---|---|---|---|---|
| First batch | 72 | 23 | 10 | 32 | 12 | 48% | 70% |
| Second batch | 85 | 25 | 10 | 17 | 12 | 57% | 81% |

**Still missing (17):**
- click at / drag / type, convert, create, file I/O, select and the selection,
  start using;
- the stacks, part (as a reference);
- quit, sizeWindow;
- autoSelect, autoHilite, icon, userLevel, showPict.

Some depend on later decisions: file I/O on a permission model, start using on
opening more than one stack. **Renamed for good (10):**
- braces instead of `end` blocks;
- `return`, `break` and `continue` instead of exit / exit repeat / next repeat;
- `go … with <effect>` instead of `visual effect`;
- `frame` instead of `idle`;
- Rhai's operator precedence.

### Third batch: H3 passes (2026-11-19)

**New vocabulary:**
- `select` (one chunk level, `before`/`after`, `text`, `empty`), with the
  selection shown in the field;
- `the selection`, `the selectedText`, `selectedField`, `selectedChunk`,
  `selectedLine`, and `put … into|before|after the selection`;
- `click at`, `type` (typing goes where the selection is);
- `create button|field`;
- `convert` between seconds, dateItems, short date, long date and time;
- `part 3` and `part "name"`;
- `the stacks`.

**Properties:**
- `autoHilite` and `icon` (an icon names a picture in the stack);
- `showPict` (hides the pictures on that card or background);
- `dontSearch`.

**`userLevel`** (1 to 5, kept in Home, as in HyperCard) now shapes the player:
- below 4: no Edit button;
- at 4: the inspector without scripts;
- at 1: fields cannot be typed into.

This is the "ladder of power" from the gap analysis, made settable.

**Partials raised to full HyperTalk behaviour:**
- **`repeat`**, all forms: `repeat`, `repeat N [times]`, `repeat while`,
  `repeat until`, `repeat with i = a [down] to b`.
- **`exit repeat`, `next repeat`, `exit <handler>`.** `break` and `continue`
  are rewritten to commands that raise Rhai's loop signals, because Rhai only
  allows its own `break` inside its own loops.
- **Writing into a range**: `put "XY" into char 2 to 4 of x`.
- **`set the script of …`**: a script can write a script, compiled at once and
  undoable.
- **`mark`**: `mark cards where …`, `mark cards by finding …`, `mark all cards`.
- **`find`**: normal / whole / word / chars / string, and `in field`. The match
  becomes the selection. Shared-text and `dontSearch` fields are skipped.
- **Time**: local, from the browser's time-zone offset. `the seconds` counts
  from 1904, as HyperCard did. Added: `the long date`, `the abbreviated date`,
  `the long time`, `the mouseClick`.
- **Clicks in a locked field**: `the clickText`, `the clickLine` and
  `the clickChunk` work after them.
- **`return`** is the newline constant wherever a statement does not start (the
  rewrite turns it into `Return`).
- **`play "harpsichord" tempo 200 "c4q e g c5h"`**: notes are synthesized into
  a WAV clip.
- **Button styles**: opaque, rectangle, shadow, roundRect, checkBox (toggles),
  radioButton. Field styles: transparent, opaque, rectangle, shadow, scrolling.
- **`exitField`** when a field is left unchanged, `closeField` when changed.
- **Double-click**, as in HyperCard 2.2: the second click of a double-click
  sends only `mouseDoubleClick`.
- **`resumeStack`** when coming back to a stack left with `suspendStack`.

**Bugs found on the way:**
- `word 1 of field "name" is "Grace"` compared the whole field, not the word. A
  `field "…"` container now ends the chunk expression, as a bare variable
  already did.
- `find chars "card" in field "notes"` and `send "x" in 2 seconds` both
  parsed as Rhai's own `in`. A literal now ends at the literal.

**Checked:**
- 27 core tests and the keyword test pass.
- In headless Chromium with real input:
  - the local date and time matched the browser's clock;
  - a checkBox toggled;
  - a double-click gave `mouseUp` then only `mouseDoubleClick`;
  - leaving a field gave `exitField` when it was unchanged and `closeField`
    when it was changed;
  - a click on "gamma" in a locked field gave `the clickText` "gamma" and
    `the clickLine` "line 2";
  - `select word 2` showed "beta" selected;
  - the synthesized notes reached the browser as a WAV clip;
  - the Edit button disappeared at user level 2 and came back at 5.
- Breakout: about 41 µs per frame.

**Coverage, final for now:**

| | clean | partial | renamed | missing | n/a | clean | any equivalent |
|---|---|---|---|---|---|---|---|
| Second batch | 85 | 25 | 10 | 17 | 12 | 57% | 81% |
| Third batch | 107 | 20 | 6 | 4 | 12 | **72%** | 89% |

**H3 passes as stated:** 107 of 149 keywords (72%) work as in HyperTalk,
braces and semicolons aside. Excluding the 12 that only make sense on a classic
Mac, it is 78%.

**What is left:**
- **Renamed (6)**, by Rhai's grammar or by design: braces instead of
  `end if` / `end <handler>`, Rhai's operator precedence, `go … with <effect>`
  instead of `visual effect`, `send … in` instead of `wait`, and `frame`
  instead of `idle`.
- **Missing (4)**: file I/O (needs a permission model), `start using` (needs
  several open stacks), `quit`, `autoSelect`.
- **Partial (20), with the reason for each:**
  - `=` and `<>` are Rhai syntax;
  - `&&` already means "and";
  - `not` has Rhai's precedence;
  - dialogs reply by message, so `it` is not filled;
  - no `drag`;
  - no `create stack`;
  - no `go stack`;
  - `find` lacks `of marked cards`;
  - `select` takes one chunk level;
  - `sort` sorts cards only;
  - `the result` is set by `go` and `find` only;
  - function handlers do not travel the message path;
  - and similar.

## Decisions for the next round (2026-11-19, project owner)

**Several open stacks:**
- tabs in one player window;
- `go to stack "X"` replaces the current tab, as in HyperCard, and
  `… in a new window` opens a new tab;
- stacks are found by searching folders: the current stack's folder, then the
  stacks folder, then search folders kept in Home;
- `start using` opens a stack as a library, with no tab.

**File I/O, "anything goes":**
- the vocabulary:
  - HyperTalk's `open / read / write / close file`;
  - `answer file` / `ask file`;
  - whole-file `put file … into` / `put … into file`;
  - `the files` / `the folders` of a folder;
  - `put url …`;
- text only (UTF-8);
- relative paths are relative to the stack's folder;
- scripts may delete and rename files and create folders;
- on the web, a private browser folder (OPFS) is the disk.

**Closer to HyperTalk:**
- **Both block syntaxes**: HyperTalk's `end` blocks and the current braces.
  The `end` form is translated before compiling; the editor keeps what was
  typed.
- **Comparison**: `=` and `<>` compare. Assignment is `put … into` or `let`.
- **Logic and joining**: `&&` joins with a space; logic is `and`, `or`, `not`
  with HyperTalk's precedence. The demo stacks are rewritten.
- **Dialog replies** stay messages (`answered`, `asked`). File reads fill `it`
  directly.

**Sidebar:** fixed. The inspector is capped at the window width minus a
200 px strip for the card, and the card area scrolls. Checked: dragging it
900 px wider on a 1000 px window stops at 800 px, with no sideways page scroll.

### Round after the owner's decisions (2026-11-19)

**Closer to HyperTalk**

- **Both block syntaxes.** `hypertalk_blocks` translates HyperTalk lines into
  the brace form before compiling, one line out per line in, so error line
  numbers stay right. It covers:
  - `on name a, b` … `end name`;
  - `function f a` … `end f`;
  - `if … then` / `else` / `else if … then` / `end if`, and single-line
    `if … then … [else …]`;
  - `repeat [for] …` … `end repeat`;
  - statements without `;`, and `¬` to continue a line.

  Lines that already end in `{`, `}` or `;` pass through, so both styles mix
  in one script. The Myst pillar handler, written HyperTalk-style, runs
  unchanged in the Pillars stack (test `hypertalk_block_syntax`).
- **`=` and `<>` compare.** `x = …` at the start of a statement still assigns,
  as do `let` and `repeat with i = …`. Assignment targets may be indexed
  (`bricks[i].alive = false`). A first version got this wrong and silently
  turned that Breakout line into a comparison. The Breakout test now checks
  that hit bricks disappear.
- **`&&` joins with a space; logic is `and`, `or`, `not`.** `not` binds tightest,
  as in HyperTalk: `not 1 = 2` is an error, `not (1 = 2)` is true.
- **`&` precedence fixed.** `&` and `&&` now sit between `+`/`-` and the
  comparisons, as in HyperTalk. Before, Rhai's `&` bound looser than `==`, so
  `"a" & "b" is "ab"` parsed wrongly.
- **Migration.** Stack format 3: older stacks have `&&` / `||` rewritten to
  `and` / `or` when opened. The demos are updated.
- **`the target`** is accepted as an object reference.

**File I/O ("anything goes")**

- **On disk:** a `Fs` interface over the real file system (desktop) and an
  in-memory folder (web). The web player restores the folder from the
  browser's private storage (OPFS) at start and writes each change back after
  each action. Relative paths start at the stack's folder.
- **Vocabulary:**
  - `open file`, `read from file … until <char>|eof` / `for N` (into `it`;
    the result is "eof" at the end), `write … to file … [at eof|N]`,
    `close file`;
  - `file "x"` as a whole-file container: `put … into|before|after [line 2 of]
    file "x"`;
  - `delete file|folder`, `rename file … to …`, `create folder`;
  - HyperTalk's `delete line 2 of x` for chunks;
  - `there is a file|folder`, `the files|folders [of folder …]`;
  - `answer file` / `ask file`: a file dialog in the player, with the reply as a
    message;
  - `load url` + `url "…"`: fetched in the background, `urlLoaded` when it
    arrives, then read from the cache. This is LiveCode's form, because a
    script cannot wait for the network.
- **Precedence:** `field "x"`, `file "x"` and `url "x"` now end the container
  after `of` (chunks, ordinals, `the number of`). `the number of lines of url
  "…" > 3` had compared the URL text instead of the count.

**Several stacks**

- **Tabs** in one player window. The stack menu opens a stack in a new tab, or
  switches to it if it is already open.
- **`go to stack "X"`** replaces the showing stack (`closeStack` to the old one).
  `… in a new window` opens a new tab. `go to card "c" of stack "X" [with
  effect]` works. Switching tabs sends `suspendStack` / `resumeStack`.
- **Finding stacks:** the stack's folder, then the stacks folder, then
  `the searchPaths` (kept in Home). The demo stacks are found by name.
- **`start using stack "Lib"` / `stop using`:** the library's stack script sits
  after this stack and before Home on the message path. It gets
  `libraryStack` / `releaseStack`. `the stacksInUse` and `the stacks` list them.
- **Home** is shared across tabs.

**Sidebar:** the inspector cannot extend past the window (see the decisions
section).

**Checked:**
- 32 core tests and the keyword test pass.
- In headless Chromium:
  - `go to stack "Breakout"` replaced the tab;
  - `go to card "dock" of stack "Pillars" in a new window` opened a second tab
    on the dock card;
  - `the stacks` listed both;
  - files written by a script appeared in the browser's storage, and after a
    reload a script read them back;
  - `answer file` listed the folders and files and navigated into a folder;
  - `load url` fetched a page from the same server and the line count compared
    correctly.
- The previous round's browser checks all still pass.
- Breakout: about 50 µs per frame.

**Not checked:**
- The desktop player for any of this: real files, tabs, and `go stack` to a
  file.
- On the web, a stack's `openStack` handler runs before the browser files have
  been restored, so a file read there sees an empty disk.
- `load url` from the web build is subject to the browser's cross-origin rules.

**Coverage:**

| | clean | partial | renamed | missing | n/a | clean | any equivalent |
|---|---|---|---|---|---|---|---|
| Third batch | 107 | 20 | 6 | 4 | 12 | 72% | 89% |
| This round | 116 | 17 | 3 | 2 | 11 | **78%** | 91% |

**Still missing:** `quit` and `autoSelect`.

**Renamed (3):**
- `go … with <effect>` instead of `visual effect`;
- `send … in` instead of `wait`;
- `frame` instead of `idle`.

**Partial (17), mainly:**
- dialogs reply by message instead of filling `it`;
- function handlers are not on the message path;
- no references into other stacks;
- no `drag`, no `create stack`;
- `find` lacks `of marked cards`;
- `select` takes one chunk level;
- `sort` sorts cards only.

### Function handlers, user commands, Self Test (2026-11-19)

**Function handlers on the message path.** `function total a, b` … `end total`
(or with braces) answers `total(…)` calls from anywhere below it: card,
background, stack, libraries, Home. `return` gives the value, and `pass` moves
the call on. This uses Rhai's `on_missing_function` hook: a call that the
script does not define goes along the path. Rhai's own `fn` still works, inside
one script.

**User commands.** A statement that starts with a word that is not a known
command, keyword or assignment is sent as a message with its arguments:
`check "x", true`, Myst's `showPillar theID`, `initGlobals`. This works in both
syntaxes and from the message box. Nothing handling it gives HyperTalk's
error: `Can't understand "frobnicate"`.

**`is not a|an|in|within`** are now negated operators; before, `"x" is not a
number` parsed as `isnot a number`.

**Chains of chunks are one expression.** `word 1 of line i of field "f"` is
parsed as a single chain that ends where its container ends: a variable,
`field|file|url <name>`, or a literal. Literals now end their container
everywhere (`the last item of "a,b,c" = "c"`). `let y = a = b` compares the
second `=`.

**Web start-up.** The first stack's opening messages wait until the browser's
files are restored, so an `openStack` handler can read them. Checked by
reloading: the Self Test's `openStack` read its last-run file back.

**`load url`.** A failure now also sends `urlLoaded <url>, "error"`, with the
reason in `the result`. The screen redraws when a background fetch finishes;
before, it waited for the next user action.

**Web start stack.** The page address can choose the first stack:
`index.html#selftest`.

**Self Test stack** (`demo::self_test`, "Self Test" in the stack menu,
`stacks/selftest.stack` on desktop). Its scripts are written HyperTalk-style.
"Run all tests" runs 28 checks across:
- the language, chunks and files;
- libraries and navigation;
- a function handler reached from a button.

Each check writes PASS or FAIL; the summary goes to `selftest-last.txt`, which
the stack shows when it next opens. The last card has buttons for the manual
checks:
- a new tab;
- the file dialog;
- notes, `ask`, a timer, a page fetch;
- persistence across a restart;
- editing.

**Checked:**
- 34 core tests and the keyword test pass, including the Self Test run natively
  (28 of 28).
- In headless Chromium:
  - 28 of 28 pass;
  - the manual buttons worked: a new tab on Pillars' dock, the file dialog,
    notes, `ask` returned "Grace", the timer fired;
  - the page fetch reported the browser's cross-origin refusal through
    `urlLoaded … "error"`;
  - after a reload, `openStack` read the browser files.

**Not checked: the desktop window.** The Self Test is meant for that:
`cargo run --release -p player -- stacks/selftest.stack` from `prototype/`,
press "Run all tests", then the manual checks.

### The desktop click-through (2026-11-19)

**How it was driven.** The desktop player runs under XWayland in a KDE Wayland
session. Screenshots of the window work with ImageMagick `import`, but
synthetic input does not:
- KWin ignores pointer moves from X11 tools (`xdotool`);
- the sandbox has no access to the virtual input device (`ydotool`).

So the player has a test hook. With `HC_AUTODRIVE=<file.js>` it runs that
script of UI actions inside its own window (DOM clicks and typing, as in the
browser runs), and appends what the script reports to `HC_AUTODRIVE_OUT`. A
shell watcher captured the window at each screenshot point. Without the
variable the hook does nothing. The script used is
`desktop-drive.js` (scratch, reproducible from this description).

**Result in the real desktop window** (WebKitGTK, real files, desktop audio):
- **Automatic checks:** 28 of 28 pass. Your own earlier run at 23:53 also
  recorded 28 of 28.
- **Restart:** results and the last-run file survive it; `openStack` read the
  file.
- **`ask`:** "Grace" came back.
- **Timer:** fired.
- **Notes:** the clip reached the audio element with no error; whether it is
  audible was not checked.
- **File dialog:** listed the real `stacks/` folder; picking a file returned
  `stacks/selftest-last.txt`.
- **Tabs:** Pillars opened in a new tab on the dock card; the tab closed again.
- **Editing:** changing a button's label and script in the inspector applied
  live; the edited handler ran.
- **Page fetch:** example.org failed at first with "TypeError: Load failed",
  because the desktop webview applies the browser's cross-origin rule.

**Fixed after it:**
- **Native fetching.** The desktop player fetches `load url` requests natively
  (`ureq`, on a thread; the frame loop delivers the result), so any site
  works. Rerun: "fetched 577 characters from https://example.org/". The web
  build still fetches through the browser.
- **Highlighting.** HyperTalk block words (`end`, `then`, `function`, `exit`,
  `next`) and `and` / `or` / `not` / `is` are highlighted.
- **Self Test layout.** Wider buttons in rows of three, and a taller scratch
  field. The previous on-disk copy is kept as
  `stacks/selftest.stack.before-layout.bak`.

**Still seen, cosmetic:** at this display scale the edit-mode toolbar wraps,
putting "card 3 of 3 ▶" on a second line.

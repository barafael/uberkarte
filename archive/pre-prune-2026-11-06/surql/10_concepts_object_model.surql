-- Core object model of HyperCard: the stack, its parts, and its machinery.

UPSERT concept:stack SET
  title    = "Stack",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['unit','file','container'],
  summary  = "The stack is HyperCard's single unit of everything: one Mac file that contains the cards, backgrounds, media, and all HyperTalk source — application, database, and document fused into one distributable object.",
  body     = "A HyperCard stack is a single Macintosh file holding cards, backgrounds, buttons, fields, media resources, and every line of HyperTalk source. There is no build step and no separate database: opening the file is running the program, and the source travels with the artifact forever. This unity is why stacks were so easy to distribute — CompuServe libraries carried hundreds of them by 1988 — and why they were so easy to study: any user with a stack and a keystroke could read the program. The Library of Congress identified the stack file format as the durable object worth preserving, precisely because app, data, and UI cannot be separated inside it. The same unity is a liability at scale: one file means no concurrent authors, no incremental distribution, no partial loading — a tension the corpus records as monolithic_file. Stacks range from a single card to thousands; the Electronic Whole Earth Catalog shipped as one 430 MB stack of 9000+ pages.";

UPSERT concept:card SET
  title    = "Card",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['unit','canvas'],
  summary  = "The card is the atomic page of a stack: a fixed 512x342-pixel canvas holding fields and buttons, indexed, findable, and marked — a cross between an index card, a screen, and a database record.",
  body     = "Every stack is a sequence of cards, each a 512x342-pixel canvas — the exact usable screen of a classic Mac — plus its own field contents. Cards behave like index cards in a tray: you flip through them, jump by number, Find across them, mark sets of them, and sort them (Browse, Find, mark, sort are first-class operations, not app features). Because a card is simultaneously a screen and a record, a simple stack is a database for free: one card per person, per book, per idea. The card's fixed geometry is also a cage: no layout reflow, no resize, no windows-within-windows — recorded as the fixed_canvas tension. Deena Larsen's Marble Springs and Voyager's Expanded Books built their art directly on the card-as-page metaphor.";

UPSERT concept:background SET
  title    = "Background",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['layer','template'],
  summary  = "Each card sits on one of a stack's finite set of backgrounds: shared paint, buttons, and fields that repeat behind groups of cards, giving HyperCard its template layer without any notion of a reusable widget.",
  body     = "A stack has a small number of backgrounds; each card belongs to exactly one. Backgrounds hold the paint and parts that should repeat — navigation buttons, titles, form chrome — while the card holds what varies. Foreground parts can be shared across cards on the same background or kept per-card. The background is HyperCard's answer to templates and layout inheritance: powerful enough to build forms and books, but deliberately concrete — you edit the actual thing you will ship, in place, at runtime. There is no component library and no compile-time separation of design from instance; that collapse of design-time and run-time is exactly what modern authoring tools usually restore, and what the authoring_runtime_unity question asks whether we should want back.";

UPSERT concept:part SET
  title    = "Part (button / field)",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['widget','interactive'],
  summary  = "Buttons and fields — the only two interactive parts — are objects with names, IDs, geometry, and their own scripts; they are the leaves of the message path and the atoms of every HyperCard interface.",
  body     = "HyperCard offers exactly two kinds of interactive part: buttons and fields. Each has an ID, a name, a rectangle, a z-order, styles (the button could draw in several classic Mac styles; fields could be plain, locked, or hidden), and a script. That is the whole widget kit — no tables, no scrollbars as objects, no dialogs beyond the OS's. The poverty is the point: with two parts and paint, every stack invents its own controls, and every stack's controls remain inspectable and editable by the next author. Parts are addressed three ways — by ID (stable), by name (readable), by ordinal (positional) — and they receive messages first in the message path, which is what makes `on mouseUp` in a button mean what it means.";

UPSERT concept:script_property SET
  title    = "Script as text property",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['code','inspectable'],
  summary  = "Behavior in HyperCard is a text property of the object it controls: every button, field, card, background, and stack carries its own HyperTalk script, visible and editable in place at any moment.",
  body     = "There is no project pane, no separate code file, no debugger you must attach to. Select any object with the browse tool, choose Script, and its HyperTalk appears as an editable text property — the code for that button lives on that button. Scripts are stored in the stack file, distributed with it, and modifiable by whoever holds the stack, which made HyperCard stacks self-teaching artifacts: Danny Goodman's famous trick was that the HyperTalk Reference stack was itself browsable HyperTalk. The design fuses the program with its data and the programmer with the user; it also means there is no versioning, no diffing, no code review — the script is whatever it is right now, because the stack always saves itself. This single property is probably the most imitated and least reproduced idea in the corpus.";

UPSERT concept:property_model SET
  title    = "Everything has properties",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['reflection','introspection'],
  summary  = "Every object — stack, background, card, button, field, the screen, the mouse — exposes readable and writable properties in HyperTalk, so a running stack can ask and change almost anything about itself.",
  body     = "HyperTalk objects are transparent: `the name of me`, `the loc of card 3`, `the top of this field`, `the selectedText`, `the lockText of field 1`. Nearly every property is writable as well as readable, so UI state is just assignment: `set the lockText of field 1 to true`. There is no private state in the object-system sense; the platform's own state (window, menu, cursor, sound) joins the same namespace. This reflective surface is what lets stacks write stacks — a button can create a field, rename a card, or rewrite its own script at runtime. Modern UI frameworks hide their internals behind opaque components and one-way data flow; HyperCard's bet was that total introspection is what makes an environment learnable and hackable, at the cost of encapsulation and any stable API to build on.";

UPSERT concept:message SET
  title    = "Messages and handlers",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['dispatch','events'],
  summary  = "HyperTalk programs are mostly handlers — blocks like `on mouseUp` — invoked by messages sent to objects; a message with no handler simply vanishes, and unhandled ones travel the message path.",
  body     = "You do not call functions so much as send messages: clicking sends mouseDown then mouseUp to a part; typing sends keyDown; `go` sends a go message; you can `send \"doIt\"` to anything. Handlers (`on ... end on`) declare which messages an object answers; `function` definitions exist for values but the eventful life of a stack runs on messages. Two verbs discipline the flow: `pass <message>` explicitly forwards, `exit <message>` stops propagation. A message nobody handles is silently ignored — no console error, no crash — which is simultaneously HyperCard's forgiveness and its most maddening debugging property. The whole design treats a stack as a society of objects that talk by message, closer to Smalltalk's metaphor than to C's, but flattened into plain English verbs.";

UPSERT concept:message_path SET
  title    = "Message path",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['dispatch','scope','core'],
  summary  = "An unhandled message walks a fixed chain — button, card, background, stack, subscribed stacks, home stack, HyperCard itself, then XCMDs and the OS — so behavior can be defined at any level and overridden below it.",
  body     = "When a part does not handle a message, the message does not die: it walks outward and upward along a fixed path. The order is: the part that received it, its card, its background, the stack, any stacks the stack is `using` (start using), the home stack, HyperCard's built-in handlers, external commands (XCMDs), and finally the operating system. Each level can handle, augment, or pass. This is the mechanism that makes a stack feel alive and layered: a stack-wide `on openStack` catches every card; a background handler overrides the stack for its cards; a button overrides everything for itself. `target` tells a handler which object actually started the message, so one background handler can serve a hundred buttons. JavaScript's DOM event bubbling is the closest mainstream descendant — capture/bubble with a path from node to document — and the message_path_modern question asks what a modern equivalent should be when the 'stack' is a distributed multi-user web app rather than one file on one Mac.";

UPSERT concept:target_me_it SET
  title    = "target, me, it, do, send",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['reflection','dispatch'],
  summary  = "A handful of self-knowledge primitives — `me` (this object), `target` (who really got the message), `it` (a floating default variable), `do` (run a computed string), `send` (dispatch explicitly) — make the message system programmable from inside.",
  body     = "`me` inside a script is the object holding the script, so one handler written on a background can act on whichever button sent the message when combined with `target`. `it` is a global scratch variable that many commands fill implicitly — no declaration, no scope fuss. `do` executes a string as HyperTalk, letting programs write programs at runtime; `send` dispatches any message to any object by name. Together they turn the message path from a fixed platform rule into something stacks themselves can manipulate: generic dispatchers, remote-control stacks, self-modifying buttons. The cost is the classic one — anything can be a string, so nothing can be checked statically. Modern environments reintroduce these powers piecemeal (eval, event.target, command palettes dispatching actions by name) but always guarded; HyperCard shipped them unguarded and called it Tuesday.";

UPSERT concept:chunk_expression SET
  title    = "Chunk expressions",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['strings','data'],
  summary  = "HyperTalk's killer data idea: address text by chunks — word, item, line, char — of any container, read or write, with `itemDelimiter` deciding what an item is; delimited text is the database.",
  body     = "You can say `put word 2 of field \"order\" into temp` or `put \"urgent\" into line 3 of card field \"notes\"`. Chunks — char, word, item, line — slice and assign into any text container, and `itemDelimiter` (default comma) defines items, so any delimited string is a list and any field of lines is a table. Combined with cards-as-records, this is HyperCard's entire data model: text, delimited by convention, navigated by chunk. It is astonishingly usable for end users and astonishingly fragile for engineers — no types, no schema, silent failures on out-of-range chunks, and performance cliffs on large fields. The corpus records this as no_structured_data: the direct ancestor of the question of whether a modern stack should store JSON, SQLite, or CRDTs where HyperCard stored lines and commas.";

UPSERT concept:message_box SET
  title    = "Message box",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['repl','ui'],
  summary  = "A permanent one-line REPL (Cmd-M) attached to the running stack: type `go card 40`, `set the mark of card 2 to true`, or `answer the name of every card` and it executes against the live document.",
  body     = "The message box sits at the bottom of every HyperCard window. It evaluates any HyperTalk against the currently open stack, immediately, with no run button. Users discovered they could talk to their documents: jump, set properties, bulk-edit fields, test a handler before wiring it up. It is the single clearest ancestor of the browser console and of command palettes — an always-available imperative channel into the live thing in front of you — and it is also the security nightmare: any stack can be talked into anything, and any user can rewrite any stack. Every descendant environment inherits that tradeoff. The message box embodies the learnability philosophy: the whole system is addressable in one line of English, so the boundary between using and programming is a matter of taste, not permission.";

UPSERT concept:tool_palette SET
  title    = "Tool palette and paint",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['paint','ui'],
  summary  = "Seven tools — point, finger, paint, erase, pencil, brush, bucket — turn HyperCard into a 1-bit MacPaint that draws directly onto cards and backgrounds; art is not imported, it is painted in place.",
  body     = "HyperCard ships MacPaint's drawing tools inside the authoring environment. With the paint tools you draw bitmap art directly onto a card or background at 512x342 in black and white; with the pencil you place parts; with the finger you navigate. There is no import pipeline to speak of (though resources and later AddColor exist): the visual material of a stack is painted by its author, which keeps the whole artifact handmade and inspectable. The palette is the physical sign of HyperCard's philosophy — the same window is a canvas, a database, and a program, and switching between those modes is a tool change, not a mode switch or a rebuild. Color only arrives in 2.2 (1992) via Color Tools and AddMotion II, bolted onto the 1-bit model — see bolted_on_color.";

UPSERT concept:region_link SET
  title    = "Region links (linkText, linkParts)",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['hypertext','links'],
  summary  = "HyperCard hypertext links are rectangular regions on a card — usually buttons — that `go` to another card or stack; text itself is not linkable unless you fake it with linkText, a gap Jakob Nielsen never let it forget.",
  body     = "Jumping between cards is done by buttons whose scripts say `go card \"chapter 2\"` or `go stack \"glossary\"`. Links are places, not spans: you click a rectangle, not a word inside a sentence. `linkText` and `linkParts` (2.0) let a field's words act as a link region, but the link still covers the whole field or a painted rectangle — there are no first-class inline anchors, no link database, no bidirectional links, and crucially no way to link into a card from outside the stack. Jakob Nielsen's contemporary critique — region-based linking makes dense scholarly hypertext awkward — is well documented and is recorded here as no_inline_text_links. The World Wide Web's inline anchor, addressable from anywhere, is the direct rival idea that won; html_hyperlink maps this contrast precisely.";

UPSERT concept:find_mark_sort SET
  title    = "Find, mark, sort, Browse",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['data','query'],
  summary  = "Whole-stack data operations built into the UI: Find scans fields across cards, marks select sets, sort reorders cards, Browse shows every card as a thumbnail — a database admin layer with no query language.",
  body     = "Because cards are records, HyperCard gives you record operations for free: Find searches marked fields across the stack (case-insensitive, whole-word options), marking builds sets you can operate on, sort physically reorders cards by field, and Browse renders the whole stack as a wall of small card images for visual navigation. HyperTalk mirrors all of it (`find`, `the marks of cards`, `sort cards by ...`). There is no relational model, no joins, no indexes beyond a simple internal one — just scan, mark, act. For thousands of cards this is liberating; for tens of thousands it is slow. The corpus treats Browse as an early visual index — a direct conceptual ancestor of Obsidian's graph view and of every 'wall of cards' UI — and Find/mark as the proto-query whose absence of structure is the no_structured_data tension.";

UPSERT concept:idle_time SET
  title    = "Idle and do in time",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['time','events'],
  summary  = "HyperCard runs an event loop with a bottom: `on idle` fires whenever the queue empties, and `do \"...\" in 3 seconds` schedules future code — timers and animation from the same single-threaded pump.",
  body     = "The message pump has a floor: when nothing is pending, HyperCard sends idle. Stacks use `on idle` for animation, polling, and ambient behavior, and `do <handler> in <seconds>` to schedule work — the whole concurrency model is one thread and a queue. It is simple enough for a first-time author to make something move, and crude enough that long handlers freeze the UI and idle loops spin the fan. The HyperCard 2.0 compiler and the AddMotion era pushed this loop toward real animation. In the corpus this is the ancestor of requestAnimationFrame loops and of every 'just use setTimeout' reality in JavaScript — the same honest single-threaded bargain, recorded under interpreter_performance.";

UPSERT concept:home_stack SET
  title    = "Home stack",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['environment'],
  summary  = "The stack that defines HyperCard's own chrome — menu bar, palette, About box — because the platform itself is just a stack you can replace; HyperCard's UI is its own dogfood.",
  body     = "HyperCard's menus, tool palette, and even its startup behavior come from whichever stack is designated the home stack. Swap the home stack and the platform's face changes. The shipped home stack is itself editable HyperTalk: the Finder-replacement fantasies Alsop wrote about in 1987 lived here, and stacks like HyperDialer and WALT (a phone GUI) literally re-skinned the environment. This is the deepest expression of 'the medium is made of the same stuff as its contents' — the shell is a stack, the reference manual is a stack, and nothing is privileged. It also concentrates the platform-lockin tension: the world you can rebuild is Apple's, on Apple's screen, until Apple stops shipping it.";

UPSERT concept:player_split SET
  title    = "Authoring vs Player",
  kind     = 'practice',
  year     = 1992,
  confidence = 'high',
  tags     = ['distribution','politics'],
  summary  = "HyperCard was one tool for making and using stacks until Claris split it: the free Player runs stacks but cannot edit or script them, and the authoring tools became a paid product — a political wound, not just a licensing one.",
  body     = "Originally anyone with a Mac had HyperCard, so making and using were the same act. After the team moved to Claris, HyperCard 2.2 (1992) shipped a stripped Player — free, view-only — while authoring required the paid product. Users who had built libraries of stacks woke up unable to open them without buying something; the backlash was loud and is well documented. The split encodes a choice every platform since has re-litigated: is the runtime a public utility or a product SKU? It foreshadows the modern free-viewer/paid-editor economy (Acrobat, iBooks Author) and it is recorded as author_player_politics, feeding the who_is_the_author question: when reading and writing are the same tool, everyone is an author; split them and you have customers instead.";

UPSERT concept:fixed_canvas SET
  title    = "Fixed canvas and menu bar",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['layout','constraint'],
  summary  = "Every card is exactly the Mac screen: 512x342, one window, the HyperCard menu bar on top. No resize, no reflow, no multiple windows until 2.0 loosened it slightly — a constraint that shaped every stack ever made.",
  body     = "HyperCard assumes one screen, one size, one stack window. The card is the viewport; the menu bar belongs to HyperCard, not to your app (you can add items to a custom menu, but the bar is the platform's). Authors internalized the canvas the way poets internalize meter — the Whole Earth Catalog's pages, Voyager's books, Myst's scenes are all composed at 512x342. HyperCard 2.0 added multiple windows and window objects, but the card never became resizable or responsive. In the corpus this is both a design gift (total determinism, no layout engine needed) and a web-displacement accelerant: when the world moved to variable viewports and documents-in-browsers, a fixed canvas had nothing to offer. The constraints question asks which of HyperCard's constraints were generative and which were just accidents of 1987 hardware.";

UPSERT concept:xcmd SET
  title    = "XCMDs and XFCNs",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['plugins','extensibility'],
  summary  = "External commands: compiled resources a stack could carry and call as if native HyperTalk words — `sqlconnect`, `playMovie`, `phone` — HyperCard's plugin system, and the last stop on the message path before the OS.",
  body     = "An XCMD is a piece of compiled Mac code (usually C or Pascal) that HyperCard loads as a resource and exposes as a command; an XFCN returns a value. Stacks shipped their XCMDs inside themselves, so a stack could bring its own vocabulary — Oracle shipped an XCMD before it shipped Oracle Card; Hyperdialer drove a modem; the QuickTime team prototyped movie playback in HyperCard with XCMDs before QuickTime existed. Dr. Dobb's and MacTech ran recurring XCMD columns; whole businesses (Heinlein's HyperCom, BeeHive's ADB hardware) were built on them. XCMDs are the corpus's clearest ancestor of the browser extension / native plugin: same power, same trust cliff, same virus surface (the MerryXmas era). They sit at the end of the message path — the platform's admission that it cannot finish itself.";

UPSERT concept:pseudo_arrays SET
  title    = "No arrays (pseudo-arrays)",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['data','gap'],
  summary  = "HyperTalk has no arrays, ever: structured data is faked with delimited text and chunk expressions, or by treating cards as records — a deliberate simplicity that capped every serious application.",
  body     = "There is no map, list, or record type in HyperTalk 2.4.1. Authors simulate: `itemDelimiter` lists, line-per-record fields, one-card-per-row databases, or XCMDs that bring real structures. The choice reads as philosophical — keep the language English-shaped, keep everything text — and the consequence is that every nontrivial stack reinvents its own encoding and every handoff between stacks breaks it. Kevin Calhoun's team added objects (menus, windows) in 2.0 but never arrays; by the mid-90s this was the single most-cited technical reason serious software left HyperCard. In the corpus pseudo_arrays is the purest example of a philosophy (everything is text) hardening into a ceiling (no structured data), and it anchors the data_model question for any modern stack.";

UPSERT concept:single_file_unity SET
  title    = "Single-file unity (app = data = media = source)",
  kind     = 'idea',
  year     = 1987,
  confidence = 'high',
  tags     = ['architecture','core'],
  summary  = "The load-bearing idea: one file that is simultaneously the program, its database, its media, and its readable source — distribution, preservation, inspectability, and the whole stackware economy all fall out of this one decision.",
  body     = "Almost everything distinctive about HyperCard follows from the stack being one self-contained file: you distribute it by copying it (CompuServe, AOL's HyperHeaven, floppy libraries); you preserve it by keeping it (the Library of Congress treated the stack format as the artifact); you learn it by opening it (scripts ride along); you trust or audit it by reading it. And almost everything limiting follows too: no concurrency, no partial updates, no linking in from outside, monolithic growth. The corpus treats single_file_unity as the node with the most descendants — local-first apps, single-file HTML apps, TiddlyWiki, and Jupyter notebooks are all partial returns to it, each keeping the portability while re-opening the source-visibility question HyperCard answered by showing everything.";

UPSERT concept:stackware SET
  title    = "Stackware",
  kind     = 'domain',
  year     = 1988,
  confidence = 'high',
  tags     = ['ecosystem','culture'],
  summary  = "The name the community gave its software: thousands of user-built stacks — catalogs, games, databases, journals, kiosk systems — traded on CompuServe and HyperHeaven, reviewed in magazines, sold in boxes: a programming folk culture.",
  body     = "By 1988 CompuServe's library held 300+ stacks; AOL ran HyperHeaven; magazines reviewed stacks like albums. The output ranged from the Electronic Whole Earth Catalog (430 MB, 9000+ pages) and Voyager's Expanded Books to Myst (its 1993 Mac release built as HyperCard stacks, later ports reimplemented; reportedly bundled with some Apple Performa Macs), Renault's dealer inventory system, the NFB's CinéRobothèque driving a robot arm, the BBC Radiophonic Workshop's 1989 sound work, and the SIDOCI medical-record prototype that became DMR. 'Stackware' names what happens when authoring is free and universal: not an app store, a folk art. The corpus uses stackware as the evidence base for the end_user_programming claim and as the loss recorded in the succession_failure thread — nothing since has had this particular distribution of authors.";

UPSERT concept:macro_virus SET
  title    = "Macro viruses (the first ones)",
  kind     = 'idea',
  year     = 1991,
  confidence = 'high',
  tags     = ['security','history'],
  summary  = "HyperCard stacks produced the first macro viruses: self-replicating HyperTalk that copied itself into every stack you opened — April 1991 in Belgium and the Netherlands, then MerryXmas in 1993 — because a document that can run code is also a vector.",
  body     = "Because scripts are data inside a distributable file, a stack can write a button into your stacks and mail itself onward. The first wild macro viruses appeared in April 1991 (Belgium/Netherlands), and MerryXmas (1993) spread by attaching itself to stacks as Christmas cards arrived. Apple's response was slow and the incident is a founding case study for every document-macro policy since (Word followed in 1995 with Concept). In the corpus macro_virus is the sharpest cost of the script-as-property idea: the same property that makes stacks self-teaching makes them self-propagating. It anchors security_naivete and the modern_stack question — any environment that unifies document and program must eventually answer what HyperCard answered by not answering.";

UPSERT concept:pointing_hand_cursor SET
  title    = "The pointing hand",
  kind     = 'idea',
  year     = 1987,
  confidence = 'medium',
  tags     = ['affordance','ui'],
  summary  = "HyperCard's link affordance: the arrow becomes a pointing hand over anything clickable — an idea usually credited to Susan Kare's design work for HyperCard, and copied almost verbatim by the first web browsers.",
  body     = "Over a button or link, HyperCard's cursor changes to a pointing hand — a tiny semiotic invention that told you 'this will take you somewhere' before any spec said so. The attribution to Susan Kare is widely repeated but only medium-confidence in the sources gathered here. What is well documented is the descendant: ViolaWWW and subsequent browsers adopted the pointing-hand link cursor, and it remains the web's default link affordance 35+ years later. The corpus keeps this node because affordances travel between platforms more faithfully than architectures — the message path is dead; the hand is alive.";

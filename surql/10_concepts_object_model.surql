-- Core object model of HyperCard: the stack, its parts, and its machinery.

UPSERT concept:stack SET
  title    = "Stack",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['unit','file','container'],
  summary  = "The stack is HyperCard's single unit of everything: one Mac file that contains the cards, backgrounds, media, and all HyperTalk source — application, database, and document fused into one distributable object.",
  body     = "A HyperCard stack is a single Macintosh file holding cards, backgrounds, buttons, fields, media resources, and every line of HyperTalk source. There is no build step and no separate database: opening the file is running the program, and the source travels with the artifact forever. This unity is why stacks were so easy to distribute — by disk, and through online libraries — and why they were so easy to study: any user with a stack and a keystroke could read the program. The same unity is a liability at scale: one file means no concurrent authors, no incremental distribution, no partial loading — a tension the corpus records as monolithic_file. Stacks range from a single card to thousands; the Electronic Whole Earth Catalog (1989) shipped as one 430 MB stack of over 9,000 pages.";

UPSERT concept:card SET
  title    = "Card",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['unit','canvas'],
  summary  = "The card is the atomic page of a stack: a canvas (512x342 pixels in HyperCard 1.x and the default thereafter) holding fields and buttons, indexed, findable, and marked — a cross between an index card, a screen, and a database record.",
  body     = "Every stack is a sequence of cards, each a canvas of one size per stack — 512x342 pixels, the screen of the original compact Mac, in HyperCard 1.x and as the default afterwards (2.0 allowed 64 to 1280 pixels per side) — plus its own field contents. Cards behave like index cards in a tray: you flip through them, jump by number, Find across them, mark sets of them, and sort them (Find, mark, sort are first-class operations, not app features). Because a card is simultaneously a screen and a record, a simple stack is a database for free: one card per person, per book, per idea. The card's fixed geometry is also a cage: one size per stack, no layout reflow, no responsive resizing — recorded as the fixed_canvas tension. Deena Larsen's Marble Springs and Voyager's Expanded Books built their art directly on the card-as-page metaphor.";

UPSERT concept:background SET
  title    = "Background",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['layer','template'],
  summary  = "Each card sits on one of a stack's finite set of backgrounds: shared paint, buttons, and fields that repeat behind groups of cards, giving HyperCard its template layer without any notion of a reusable widget.",
  body     = "A stack has a small number of backgrounds; each card belongs to exactly one. Backgrounds hold the paint and parts that should repeat — navigation buttons, titles, form chrome — while the card holds what varies. Parts belong either to a card or to its background; a background part appears on every card of that background, but a background field's text still belongs to each card unless the field's sharedText is set. The background is HyperCard's answer to templates and layout inheritance: powerful enough to build forms and books, but deliberately concrete — you edit the actual thing you will ship, in place, at runtime. There is no component library and no compile-time separation of design from instance; that collapse of design-time and run-time is exactly what modern authoring tools usually restore, and what the authoring_runtime_unity question asks whether we should want back.";

UPSERT concept:part SET
  title    = "Part (button / field)",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['widget','interactive'],
  summary  = "Buttons and fields — the only two interactive parts — are objects with names, IDs, geometry, and their own scripts; they are the leaves of the message path and the atoms of every HyperCard interface.",
  body     = "HyperCard offers exactly two kinds of interactive part: buttons and fields. Each has an ID, a name, a rectangle, a z-order, a style (buttons: transparent, opaque, rectangle, roundRect, shadow, checkBox, radioButton; fields: transparent, opaque, rectangle, shadow, scrolling), properties such as locked or hidden, and a script. That is the whole widget kit — no tables, no scrollbars as objects, no dialogs beyond the OS's. The poverty is the point: with two parts and paint, every stack invents its own controls, and every stack's controls remain inspectable and editable by the next author. Parts are addressed three ways — by ID (stable), by name (readable), by ordinal (positional) — and they receive messages first in the message path, which is what makes `on mouseUp` in a button mean what it means.";

UPSERT concept:script_property SET
  title    = "Script as text property",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['code','inspectable'],
  summary  = "Behavior in HyperCard is a text property of the object it controls: every button, field, card, background, and stack carries its own HyperTalk script, visible and editable in place at any moment.",
  body     = "There is no project pane, no separate code file, no debugger you must attach to. Select any object with the browse tool, choose Script, and its HyperTalk appears as an editable text property — the code for that button lives on that button. Scripts are stored in the stack file, distributed with it, and modifiable by whoever holds the stack, which made HyperCard stacks self-teaching artifacts. The design fuses the program with its data and the programmer with the user; it also means there is no versioning, no diffing, no code review — the script is whatever it is right now, because the stack always saves itself. This single property is probably the most imitated and least reproduced idea in the corpus.";

UPSERT concept:property_model SET
  title    = "Everything has properties",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['reflection','introspection'],
  summary  = "Every object — stack, background, card, button, field — and many parts of the environment (windows, the cursor, the tool, the screen lock) expose readable and writable properties in HyperTalk, so a running stack can ask and change almost anything about itself.",
  body     = "HyperTalk objects are transparent: `the name of me`, `the rect of card 3`, `the top of this field`, `the lockText of field 1`, plus functions such as `the selectedText`. Most properties are writable as well as readable (a few, such as ID and number, are read-only), so UI state is just assignment: `set the lockText of field 1 to true`. There is no private state in the object-system sense; much of the platform's own state (windows, menus, cursor, sound) joins the same namespace. This reflective surface is what lets stacks write stacks — a button can create a field, rename a card, or rewrite its own script at runtime. Modern UI frameworks hide their internals behind opaque components and one-way data flow; HyperCard's bet was that total introspection is what makes an environment learnable and hackable, at the cost of encapsulation and any stable API to build on.";

UPSERT concept:message SET
  title    = "Messages and handlers",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['dispatch','events'],
  summary  = "HyperTalk programs are mostly handlers — blocks like `on mouseUp` — invoked by messages sent to objects; a system message with no handler does nothing, an unhandled command raises 'Can't understand', and unhandled messages travel the message path.",
  body     = "You do not call functions so much as send messages: clicking sends mouseDown then mouseUp to a part; typing sends keyDown; `go` sends a go message; you can `send \"doIt\"` to anything. Handlers (`on mouseUp ... end mouseUp`) declare which messages an object answers; `function` definitions exist for values but the eventful life of a stack runs on messages. Two verbs discipline the flow: `pass <message>` explicitly forwards to the next object (a handler that ends without `pass` consumes the message), and `exit <message>` ends the handler early. System messages such as mouseUp that nothing handles are silently dropped; a command or user-defined message that nothing handles shows a 'Can't understand <name>' error dialog. The silent drop is simultaneously HyperCard's forgiveness and a maddening debugging property. Commands are messages too: a handler named like a HyperTalk command, anywhere in the hierarchy, intercepts it instead of the built-in (and functions can be redefined the same way), per Apple's guide. The whole design treats a stack as a society of objects that talk by message, closer to Smalltalk's metaphor than to C's, but flattened into plain English verbs.";

UPSERT concept:message_path SET
  title    = "Message path",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['dispatch','scope','core'],
  summary  = "An unhandled message walks a fixed chain — part, card, background, stack, any stacks added with `start using`, the Home stack, then HyperCard itself — with each level's external commands consulted before passing on, so behavior can be defined at any level and overridden below it.",
  body     = "When a part does not handle a message, the message does not die: it walks outward and upward along a fixed path. The order is: the part that received it, its card, its background, the stack, any stacks added with `start using` (inserted between the stack and the Home stack), the Home stack, and HyperCard's built-ins. At each stack, Home and HyperCard level the handlers are tried first and then that level's XCMD/XFCN resources; HyperCard checks for externals before its built-in commands; the operating system is not on the path. Each level can handle, augment, or pass. Where a message starts depends on its kind: mouse messages and button/field lifecycle messages start at the part, most other system messages at the current card, and a message sent by a running handler starts at that handler's object. This is the mechanism that makes a stack feel alive and layered: a stack-wide `on openStack` catches every card; a background handler overrides the stack for its cards; a button overrides everything for itself. `target` tells a handler which object actually started the message, so one background handler can serve a hundred buttons. JavaScript's DOM event bubbling is the closest mainstream descendant — capture/bubble with a path from node to document — and the message_path_modern question asks what a modern equivalent should be when the 'stack' is a distributed multi-user web app rather than one file on one Mac.";

UPSERT concept:target_me_it SET
  title    = "target, me, it, do, send",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['reflection','dispatch'],
  summary  = "A handful of self-knowledge primitives — `me` (this object), `target` (who really got the message), `it` (a floating default variable), `do` (run a computed string), `send` (dispatch explicitly) — make the message system programmable from inside.",
  body     = "`me` inside a script is the object holding the script, so one handler written on a background can act on whichever button sent the message when combined with `target`. `it` is a local variable that many commands fill implicitly (get, ask, answer, read, convert) — no declaration needed; variables are local unless declared `global`. `do` executes a string as HyperTalk, letting programs write programs at runtime; `send` dispatches any message to any object by name. Together they turn the message path from a fixed platform rule into something stacks themselves can manipulate: generic dispatchers, remote-control stacks, self-modifying buttons. The cost is the classic one — anything can be a string, so nothing can be checked statically. Modern environments reintroduce these powers piecemeal (eval, event.target, command palettes dispatching actions by name) but always guarded; HyperCard shipped them unguarded and called it Tuesday.";

UPSERT concept:chunk_expression SET
  title    = "Chunk expressions",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['strings','data'],
  summary  = "HyperTalk's killer data idea: address text by chunks — word, item, line, char — of any container, read or write, with `itemDelimiter` deciding what an item is; delimited text is the database.",
  body     = "You can say `put word 2 of field \"order\" into temp` or `put \"urgent\" into line 3 of card field \"notes\"`. Chunks — char, word, item, line — slice and assign into any text container, and `itemDelimiter` (default comma; added in HyperCard 2.1) defines items, so any delimited string is a list and any field of lines is a table. Combined with cards-as-records, this is HyperCard's entire data model: text, delimited by convention, navigated by chunk. It is astonishingly usable for end users and astonishingly fragile for engineers — no types, no schema, silent failures on out-of-range chunks, and performance cliffs on large fields. The corpus records this as no_structured_data: the direct ancestor of the question of whether a modern stack should store JSON, SQLite, or CRDTs where HyperCard stored lines and commas.";

UPSERT concept:message_box SET
  title    = "Message box",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['repl','ui'],
  summary  = "A one-line REPL window (Cmd-M) onto the running stack: type `go card 40` or `set the marked of card 2 to true`, and it executes against the live document.",
  body     = "The message box is its own floating, movable one-line window, toggled with Cmd-M. It evaluates an expression or sends its text as a message to the current card, immediately, with no run button; of the control keywords only do, if and send work in it. It is also the default destination of `put`. Users discovered they could talk to their documents: jump, set properties, bulk-edit fields, test a handler before wiring it up. It is the single clearest ancestor of the browser console and of command palettes — an always-available imperative channel into the live thing in front of you — and it is also the security nightmare: any stack can be talked into anything, and any user can rewrite any stack. Every descendant environment inherits that tradeoff. The message box embodies the learnability philosophy: the whole system is addressable in one line of English, so the boundary between using and programming is a matter of taste, not permission.";

UPSERT concept:tool_palette SET
  title    = "Tool palette and paint",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['paint','ui'],
  summary  = "Eighteen tools — Browse, Button, Field, then Select, Lasso, Pencil, Brush, Eraser, Line, Spray, Rectangle, Round Rect, Bucket, Oval, Curve, Text, Regular Polygon, Polygon — put MacPaint's drawing tools and the part-creation tools in the same window as the running stack; art is painted in place.",
  body     = "HyperCard ships MacPaint's drawing tools inside the authoring environment. With the paint tools you draw bitmap art directly onto a card or background in black and white; with the Button and Field tools you place parts; with the Browse tool (the normal-use hand) you operate the stack. There is no import pipeline to speak of (though PICT resources and, later, color add-ons exist): the visual material of a stack is painted by its author, which keeps the whole artifact handmade and inspectable. The palette is the physical sign of HyperCard's philosophy — the same window is a canvas, a database, and a program, and switching between those modes is a tool change, not a mode switch or a rebuild. Color only arrives in 2.2 (about 1993) via licensed Color Tools and AddMotion II, bolted onto the 1-bit model — see bolted_on_color.";

UPSERT concept:region_link SET
  title    = "Region links (buttons and group text)",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['hypertext','links'],
  summary  = "HyperCard hypertext links are rectangular regions on a card — usually buttons — that `go` to another card or stack; text can be made clickable in HyperCard 2.0+ via 'group' text and script, but there is no first-class inline anchor object.",
  body     = "Jumping between cards is done by buttons whose scripts say `go card \"chapter 2\"` or `go stack \"glossary\"`. Links are places, not spans: you click a rectangle, not a word inside a sentence. HyperCard 2.0 added 'group' text, a text style that makes runs of characters clickable in a locked field; the field's script then uses clickChunk/clickText/clickLine to decide where to go. Anchors therefore exist inline, but their meaning lives in per-field script, not in stored link objects — there is no link database, no bidirectional links, and no way to link into a card from outside HyperCard. Wikipedia records that text hyperlinks 'were awkward to implement and seldom used', and Nielsen's remark that links started from regions, not text objects; both are recorded here as no_inline_text_links. The World Wide Web's inline anchor, addressable from anywhere, is the direct rival idea that won; html_hyperlink maps this contrast precisely.";

UPSERT concept:find_mark_sort SET
  title    = "Find, mark, sort, Browse",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['data','query'],
  summary  = "Whole-stack data operations built into the UI: Find scans every searchable field, marking selects sets of cards, sort reorders cards, Recent shows thumbnails of the last cards visited — a database admin layer with no query language.",
  body     = "Because cards are records, HyperCard gives you record operations for free: Find searches every card and background field except those marked dontSearch (normal, chars, word, whole and string modes), marking sets the boolean `marked` property so a set of cards can be operated on (`mark cards where ...`, `sort marked cards`), sort physically reorders cards by field, and Recent (Go menu) shows miniatures of the last 42 cards visited. HyperTalk mirrors all of it (`find`, `mark`, `sort cards by ...`). There is no relational model, no joins, no indexes beyond a simple internal one — just scan, mark, act. For thousands of cards this is liberating; for tens of thousands it is slow. The corpus treats the Recent card wall as an early visual index — a loose conceptual ancestor of graph views and 'wall of cards' UIs — and Find/mark as the proto-query whose absence of structure is the no_structured_data tension.";

UPSERT concept:idle_time SET
  title    = "Idle and wait",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['time','events'],
  summary  = "HyperCard runs an event loop with a bottom: `on idle` fires whenever the queue empties, and `wait` blocks; there is no timer command in the 2.0/2.1 language guide, so scheduling and animation come from one single-threaded pump.",
  body     = "The message pump has a floor: when nothing is pending, HyperCard sends idle. Stacks use `on idle` for animation, polling, and ambient behavior, and `wait` to pause — the whole concurrency model is one thread and a queue. (HyperTalk's `do` evaluates text immediately; a delayed `send ... in <time>` is documented for LiveCode but does not appear in the HyperCard 2.2 guide.) It is simple enough for a first-time author to make something move, and crude enough that long handlers freeze the UI and idle loops spin the fan. In the corpus this is the ancestor of requestAnimationFrame loops and of every 'just use setTimeout' reality in JavaScript — the same honest single-threaded bargain, recorded under interpreter_performance.";

UPSERT concept:home_stack SET
  title    = "Home stack",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['environment'],
  summary  = "A mandatory stack that holds the Home card (a directory of your stacks), the User Preferences card and search paths — and, because it sits in every stack's message path, a shared library and global override point for scripts and XCMDs.",
  body     = "HyperCard will not run without a Home stack. Its first card, the Home card, is a visual directory of buttons to your most-used stacks; another card holds User Preferences such as the user level. Menus and the tool palette are features of the application (scriptable from 2.0), not of the Home stack. What makes the Home stack powerful is its position: handlers in its script, and XCMDs in its resources, are in every stack's message path, so a script there overrides or extends behavior everywhere — a shared library and a global hook in one. It is an ordinary editable stack, so the environment's own directory is made of the same stuff as its contents. It also concentrates the platform-lockin tension: the world you can rebuild is Apple's, on Apple's screen, until Apple stops shipping it.";

UPSERT concept:player_split SET
  title    = "Authoring vs Player",
  kind     = 'practice',
  year     = 1992,
  confidence = 'high',
  tags     = ['distribution','politics'],
  summary  = "HyperCard was one tool for making and using stacks until Claris split it: the free Player runs stacks but cannot edit or script them, and the authoring tools became a paid product — a political wound, not just a licensing one.",
  body     = "Originally anyone with a Mac had HyperCard, so making and using were the same act. After the team moved to Claris, Apple (whose licence to distribute the full product from Claris ran out on 30 September 1992) began shipping new Macs from 14 September 1992 with only a 'HyperCard 2.1 Player' — free, run-only — while the full authoring product was Claris's to sell. Owners of stacks could still run them but could no longer inspect or edit them without buying something; Wikipedia notes many users were upset. (HyperCard 2.2 came later, around 1993.) The split encodes a choice every platform since has re-litigated: is the runtime a public utility or a product SKU? It foreshadows the modern free-viewer/paid-editor economy (Acrobat, iBooks Author) and it is recorded as author_player_politics, feeding the who_is_the_author question: when reading and writing are the same tool, everyone is an author; split them and you have customers instead.";

UPSERT concept:fixed_canvas SET
  title    = "Fixed canvas and menu bar",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['layout','constraint'],
  summary  = "Cards were 512x342 in HyperCard 1.x and the default after; HyperCard 2.0 allowed one custom size per stack (64 to 1280 pixels) and multiple stack windows, but never reflow or responsive layout — a constraint that shaped every stack ever made.",
  body     = "HyperCard assumes one card size per stack. The card is the viewport; the menu bar belongs to HyperCard, though from 2.0 scripts can create and edit menus. Authors internalized the canvas the way poets internalize meter — the Whole Earth Catalog's pages, Voyager's books, Myst's scenes are largely composed at 512x342. HyperCard 2.0 added variable card sizes (chosen per stack) and up to 18 stack and external windows, but the card never became responsive. In the corpus this is both a design gift (total determinism, no layout engine needed) and a web-displacement accelerant: when the world moved to variable viewports and documents-in-browsers, a fixed canvas had nothing to offer. The constraints question asks which of HyperCard's constraints were generative and which were just accidents of 1987 hardware.";

UPSERT concept:xcmd SET
  title    = "XCMDs and XFCNs",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['plugins','extensibility'],
  summary  = "External commands: compiled resources a stack could carry and call as if native HyperTalk words, HyperCard's plugin system; consulted at each level of the message path after that level's handlers.",
  body     = "An XCMD is a piece of compiled Mac code (usually C or Pascal) that HyperCard loads as a resource and exposes as a command; an XFCN returns a value. Stacks shipped their XCMDs inside themselves, so a stack could bring its own vocabulary — HyperCard 2.2 reached QuickTime through the Movie XCMD, and Wikipedia notes a vendor ecosystem of thousands of externals (compilers, graphing, database access, networking, animation). Dr. Dobb's and MacTech ran XCMD coverage. XCMDs are the corpus's clearest ancestor of the browser extension / native plugin: same power, same trust cliff, same virus surface (the MerryXmas era). They are the platform's admission that it cannot finish itself.";

UPSERT concept:pseudo_arrays SET
  title    = "No arrays (pseudo-arrays)",
  kind     = 'object_model',
  year     = 1987,
  confidence = 'high',
  tags     = ['data','gap'],
  summary  = "HyperTalk has no arrays, ever: structured data is faked with delimited text and chunk expressions, or by treating cards as records — a deliberate simplicity that capped every serious application.",
  body     = "There is no map, list, or record type in HyperTalk 2.4.1. Authors simulate: `itemDelimiter` lists, line-per-record fields, one-card-per-row databases, or XCMDs that bring real structures. The choice reads as philosophical — keep the language English-shaped, keep everything text — and the consequence is that every nontrivial stack reinvents its own encoding and every handoff between stacks breaks it. HyperCard 2.0 added scriptable menus and window properties but never arrays, a gap that serious applications felt. In the corpus pseudo_arrays is the purest example of a philosophy (everything is text) hardening into a ceiling (no structured data), and it anchors the data_model question for any modern stack.";

UPSERT concept:single_file_unity SET
  title    = "Single-file unity (app = data = media = source)",
  kind     = 'idea',
  year     = 1987,
  confidence = 'high',
  tags     = ['architecture','core'],
  summary  = "The load-bearing idea: one file that is simultaneously the program, its database, its media, and its readable source — distribution, preservation, inspectability, and the whole stackware economy all fall out of this one decision.",
  body     = "Almost everything distinctive about HyperCard follows from the stack being one self-contained file: you distribute it by copying it (online libraries, floppy collections); you preserve it by keeping it; you learn it by opening it (scripts ride along); you trust or audit it by reading it. And almost everything limiting follows too: no concurrency, no partial updates, no linking in from outside, monolithic growth. The corpus treats single_file_unity as the node with the most descendants — local-first apps, single-file HTML apps, TiddlyWiki, and Jupyter notebooks are all partial returns to it, each keeping the portability while re-opening the source-visibility question HyperCard answered by showing everything.";

UPSERT concept:stackware SET
  title    = "Stackware",
  kind     = 'domain',
  year     = 1988,
  confidence = 'medium',
  tags     = ['ecosystem','culture'],
  summary  = "The name for the software users built in HyperCard: thousands of user-built stacks — catalogs, games, databases, journals, kiosk systems — passed around through online libraries and disks: a programming folk culture.",
  body     = "Wikipedia notes a whole ecosystem of vendors and a large body of user-made stacks. The output ranged from the Electronic Whole Earth Catalog (1989, 430 MB, 9,000+ pages) and Voyager's Expanded Books to Myst (its 1993 Mac release built as HyperCard stacks, later ports reimplemented; reportedly bundled with some Apple Performa Macs). 'Stackware' names what happens when authoring is free and widespread: not an app store, a folk art. (Other claimed examples — dealer-inventory systems, robot-arm control, medical-record prototypes — appear in the sources but were not independently verified and are omitted here.) The corpus uses stackware as the evidence base for the end_user_programming claim and as the loss recorded in the succession_failure thread — nothing since has had this particular distribution of authors.";

UPSERT concept:macro_virus SET
  title    = "Macro viruses (the first ones)",
  kind     = 'idea',
  year     = 1991,
  confidence = 'high',
  tags     = ['security','history'],
  summary  = "HyperCard stacks produced the first macro viruses: self-replicating HyperTalk that copied itself into every stack you opened — April 1991 in Belgium and the Netherlands, then MerryXmas in 1993 — because a document that can run code is also a vector.",
  body     = "Because scripts are data inside a distributable file, a stack can write a button into your stacks. The first wild macro viruses appeared in April 1991 (Belgium/Netherlands), and MerryXmas (early 1993) followed. Wikipedia notes that few viruses targeted HyperCard and their impact was minimal, but it is an early case study for document-macro policy (Word macro viruses followed in the mid-1990s). In the corpus macro_virus is the sharpest cost of the script-as-property idea: the same property that makes stacks self-teaching makes them self-propagating. It anchors security_naivete and the modern_stack question — any environment that unifies document and program must eventually answer what HyperCard answered by not answering.";



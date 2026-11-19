-- Modern analogs: what carries each HyperCard idea today (direction-agnostic by design).

UPSERT concept:www SET
  title    = "The World Wide Web",
  kind     = 'modern_analog',
  year     = 1990,
  confidence = 'high',
  tags     = ['analog','platform'],
  summary  = "The rival platform that displaced HyperCard and inherited its audience's habits: global addressing, free client, documents-not-apps — plus echoes of HyperCard's surface (browsable cards as pages, a scripting language, a live console).",
  body     = "Berners-Lee and Cailliau's Web (1989–91) solved the two problems HyperCard left open — universal addresses and universal linkability — with a worse model of a document and a better model of a network. It absorbed HyperCard's surface piecemeal: ViolaWWW took HyperCard's concepts (per Wikipedia); the card became the page; the message box returned as the JavaScript console. What it never adopted: the unity of authoring and runtime (you view the web, you publish to it, and those are different software). The corpus treats the Web as both the answer to several HyperCard tensions and the source of a new set — the trade recorded in web_displacement and re-opened by local_first.";

UPSERT concept:html_hyperlink SET
  title    = "The HTML inline hyperlink",
  kind     = 'modern_analog',
  year     = 1990,
  confidence = 'high',
  tags     = ['analog','links'],
  summary  = "The idea that won the link war: an addressable span inside text, one-way, global — resolving no_inline_text_links and region_link's rectangle ontology with a worse UX (no back-of-envelope affordance) and a fatal virtue (every link is a URL).",
  body     = "Where HyperCard's links were places you painted, HTML's are properties of characters. Anchors made dense reference hypertext native (the thing Nielsen said HyperCard couldn't do), and URLs made every unit of content addressable from anywhere — including from stacks' enemies: search engines. The losses are real and documented in the hypertext literature (link rot, no transclusion, one-way amnesia — Nelson's whole complaint), but distribution beat structure. In the corpus html_hyperlink is the map target of region_link and the thing any modern_stack answer must ship on day one.";

UPSERT concept:javascript_dom SET
  title    = "JavaScript + DOM event bubbling",
  kind     = 'modern_analog',
    year     = 1995,
  confidence = 'high',
  tags     = ['analog','language','dispatch'],
  summary  = "The mainstream heir to the message path and HyperTalk itself: Wikipedia reports Brendan Eich was inspired by HyperTalk; DOM events bubble button→card→background→stack-shaped ancestors; the console is the message box reborn in a hostile world.",
  body     = "Brendan Eich has said HyperTalk was an inspiration for JavaScript; the direct lineage of specific features (such as handler names) is not documented here. The DOM's event propagation is a message path with capture/bubble phases and a fixed ancestor chain; querySelector is a chunk expression with CSS grammar; the console is Cmd-M. But the culture inverted HyperCard's: scripts are shipped minified (source hidden by default — the opposite of script_property), editing the page is a developer privilege (DevTools), and the environment's forgiveness was traded for interop. The corpus maps message→message_path→javascript_dom as the strongest surviving lineage in mainstream computing, and uses it in message_path_modern: bubbling proves the idea scales; the web's inability to make bubbling *inspectable and editable by users* proves the rest was culture, not technology.";

UPSERT concept:applescript SET
  title    = "AppleScript",
  kind     = 'modern_analog',
  year     = 1993,
  confidence = 'high',
  tags     = ['analog','language','automation'],
  summary  = "Apple's other natural-language bet, born in HyperCard's shadow (Wikipedia: 'based largely on HyperCard's HyperTalk language'): English-shaped control of *other applications* rather than of your own document — control-from-outside where HyperCard was control-from-inside.",
  body     = "AppleScript (1993) shares HyperTalk's register (`tell application \"Finder\" to open folder \"X\"`) but inverts its architecture: the script lives outside the objects, which expose an OSA dictionary instead of a script property. That inversion is the whole difference — AppleScript automates a walled garden of vendor apps; HyperCard made the garden itself editable. HyperCard 2.2 (1993) added support for OSA languages including AppleScript, and the corpus keeps applescript as the counterfactual branch: natural-language programming accepted by enterprise IT because it respected application boundaries, which is precisely what end-user programming refused to do.";

UPSERT concept:spa_routing SET
  title    = "SPA routing and bundling",
  kind     = 'modern_analog',
  year     = 2013,
  confidence = 'high',
  tags     = ['analog','architecture'],
  summary  = "The modern discipline for what stacks did implicitly: client-side routing gives cards their identity and back-button; bundlers solve monolithic_file by splitting the single file into a graph — then re-delivering it as one app.",
  body     = "A single-page app is a stack-shaped artifact rebuilt with the web's addressability bolted on: routes are card numbers, the component tree is the background/card layer, the router's event system is a message path. The bundler ecosystem is a decade of work on exactly monolithic_file's problems — incremental loading, cacheable parts, shared modules — that HyperCard answered with `start using` and a shrug. The corpus uses spa_routing in the distribution question: HyperCard distributed by copying a file; SPAs distribute by URL but converge on installable shells (PWAs), which is the stack's unity returning as a product decision rather than a file format.";

UPSERT concept:command_palette SET
  title    = "Command palettes and REPLs",
  kind     = 'modern_analog',
  year     = 2016,
  confidence = 'high',
  tags     = ['analog','ui'],
  summary  = "The message box's descendants: Cmd-K palettes, Spotlight, the browser console, raycast — an always-available imperative channel that names actions instead of hunting them, minus the live document it could rewrite.",
  body     = "Every serious tool now ships a one-line channel to the system: editors (Cmd-Shift-P), browsers (the console), OSes (Spotlight, Raycast). They inherit the message box's discoverability trick — you learn the system's verbs by listing them — but almost all of them dispatch to fixed menus of actions, not to a programmable object world. The missing piece is `do`: a palette that could run arbitrary code against the document in front of you would be a security incident, which is the security_naivete tax again. The corpus maps message_box→command_palette and keeps the gap explicit in the live_editing_web question.";

UPSERT concept:low_code SET
  title    = "Low-code / no-code platforms",
  kind     = 'modern_analog',
  year     = 2016,
  confidence = 'medium',
  tags     = ['analog','economics'],
  summary  = "End-user programming re-priced for enterprises: visual builders with gated escape hatches — HyperCard's audience claim (citizen developers) adopted wholesale, its founding clause (free, inspectable, yours) quietly deleted.",
  body     = "OutSystems, Appian, Retool, Glide, and the spreadsheet-adjacent world all sell the HyperCard promise — your people can build their own tools — inside the anti-HyperCard architecture: server-held apps, opaque runtimes, per-seat pricing, export blocked or lossy. The historical pattern the corpus records: every low-code wave rediscovers authoring_runtime_unity locally (builders do edit the live thing) while restoring every boundary HyperCard erased (you cannot see the source, you cannot take it away). business_model_clash explains why: nobody can afford free-on-every-seat empowerment, so empowerment became a SKU.";

UPSERT concept:local_first SET
  title    = "Local-first software",
  kind     = 'modern_analog',
  year     = 2019,
  confidence = 'medium',
  tags     = ['analog','architecture'],
  summary  = "The Ink & Switch essay (Kleppmann, Wiggins, van Hardenberg, McGranaghan, 2019) sets out principles for software where users keep ownership and collaboration still works: local data, instant editing, optional network, syncable replicas. The essay does not mention HyperCard; reading it as a rebuild of HyperCard's bargain is this corpus's interpretation, and it adds exactly the multi-replica merge layer single_user lacked.",
  body     = "Local-first (CRDTs, offline-capable apps, user-owned files with cloud sync) is the closest serious attempt to re-create the stack's properties on the web's terms: the artifact lives with the user, editing is instant and permanent, and the network is optional rather than mandatory. What it still lacks from HyperCard is the environment: local-first gives you your data back but not the language, the message path, or the right to rewrite the app. The corpus maps single_file_unity and immediate_persistence here, and pairs local_first with collaboration as the open half of the project.";

UPSERT concept:single_file_html_app SET
  title    = "The single-file HTML app",
  kind     = 'modern_analog',
  year     = 2020,
  confidence = 'medium',
  tags     = ['analog','format'],
  summary  = "The stack format's quiet return: one .html file with markup, styles, data, and scripts inside, emailable and double-clickable — the same unity, now with source visible by default and no environment to edit it in.",
  body     = "A self-contained HTML file (data inlined, scripts inlined, works offline, opens everywhere a browser exists) is the stack file format re-derived from first principles, and people rediscovered it the way HyperCard users did — as the maximally distributable artifact. Decker's single-file .html export is exactly this move made deliberately by a HyperCard heir. The gap is the environment: HyperCard opened the file *in* its editor; a browser opens HTML in a viewer, and 'view source' is read-only tourism. The corpus uses this node in authoring_runtime_unity and preservation: the format problem is solved; the tooling problem was outsourced to a web culture that treats source as a trade secret.";

UPSERT concept:notion_airtable SET
  title    = "Notion / Airtable",
  kind     = 'modern_analog',
    year     = 2016,
  confidence = 'medium',
  tags     = ['analog','data'],
  summary  = "Card-as-record industrialized: blocks and bases with typed fields, views over the same records, and friendly surfaces — resolving no_structured_data by adding the schema HyperCard refused, at the cost of the read-aloud text substrate.",
  body     = "Notion's page-plus-database and Airtable's base are the honest descendants of the card-as-record idea: one unit that is simultaneously a document and a row, with multiple views (the Browse view's insight generalized). They resolve the data tension properly — typed fields, relations, APIs — and in doing so abandon HyperCard's substrate: nothing is text-with-delimiters you can `put into word 3 of`; the object model is opaque and the scripting (formulas, automations) is a gated DSL. The corpus maps card and find_mark_sort here and files the trade under data_model: structured data and chunk-expression transparency have not yet coexisted in one product.";

UPSERT concept:wiki SET
  title    = "The wiki",
  kind     = 'modern_analog',
  year     = 1995,
  confidence = 'high',
  tags     = ['analog','practice'],
  summary  = "Ward Cunningham's 1995 WikiWikiWeb grew directly out of a HyperCard stack and HyperCard thinking: easy-to-write pages, easy-to-make links, everyone an author — the authoring half of HyperCard surviving as a genre while the runtime half died.",
  body     = "Wikipedia reports Cunningham saying the wiki grew out of HyperCard stacks he wrote in the late 1980s, plus the desire for web pages anyone could edit as easily as a card. The wiki took HyperCard's social technology (authoring is free, editing is expected, links are cheap) and dropped its executable core: a wiki page cannot run you. Wikipedia is the proof that the authoring half scales past anything stackware imagined. The corpus maps end_user_programming and who_is_the_author through wiki, and records the asymmetry: the web adopted HyperCard's *community mechanics* completely and its *environment mechanics* not at all.";

UPSERT concept:obsidian_roam SET
  title    = "Obsidian / Roam (networked notes)",
  kind     = 'modern_analog',
    year = 2020,
  confidence = 'medium',
  tags     = ['analog','personal-knowledge'],
  summary  = "The memex-by-another-name: [[links]] in local markdown, graph views as Browse's heir, backlinks as the two-way links HyperCard never shipped — personal hypertext rebuilt with the link database HyperCard omitted.",
  body     = "Obsidian (local markdown files, a proprietary app with an open community-plugin ecosystem; first beta March 2020) and Roam (outliner blocks, bidirectional links) deliver the memex/HyperCard hybrid people kept asking for: your cards, your machine, links as first-class queryable objects, a graph you can Browse. What they still lack is HyperCard's executable dimension: plugins are JavaScript written by developers, not HyperTalk written by the user on the card. The corpus maps find_mark_sort, region_link, and memex here, and notes the irony: the plugin economy reproduces the XCMD economy — power users buying capability from programmers.";

UPSERT concept:tiddlywiki SET
  title    = "TiddlyWiki",
  kind     = 'modern_analog',
  year     = 2004,
  confidence = 'high',
  tags     = ['analog','single-file'],
  summary  = "The closest living thing to a stack: one HTML file that is the document, the database, the program and the editor — open it, edit in place, and save it back to a single file (how saving works depends on the browser or a helper).",
  body     = "TiddlyWiki is a single self-contained HTML file containing tiddlers (cards), a wiki engine (the platform), a macro/plugin system (parts and messages), and its own source, edited in place in the browser and saved back through a browser-dependent saving method (the single-file saving story has been fragile: a popular Firefox saver add-on stopped working in 2017). It is the strongest existence proof that authoring_runtime_unity survives on the web, kept alive by one determined lineage (Jeremy Ruston) since 2004; version 5 (2013) was a complete rebuild and 5.4.x is current in 2026. Its macros and message-ish dispatch are xTalk-adjacent in spirit if not genealogy. The corpus weights tiddlywiki heavily in modern_stack and live_editing_web: it is the existence proof, and its niche status is the caution — the environment survived, the culture around it did not.";

UPSERT concept:jupyter SET
  title    = "Jupyter notebooks",
  kind     = 'modern_analog',
  year     = 2014,
  confidence = 'high',
  tags     = ['analog','literate'],
  summary  = "The message box scaled to a document: cells of code and prose that evaluate in place against a live kernel — immediate feedback, executable text — but with a kernel outside the document and a file that lies about its state.",
  body     = "A notebook is a stack card that admits it is a program: evaluate a cell, see the result under it, keep the prose beside it. The lineage to HyperCard's message box and literate stacks (Goodman's reference stacks ran code in place) is conceptual, not genetic. Two divergences matter to the corpus: the kernel lives outside the file (state is a server process, not the document), and saved notebooks freeze outputs — violating immediate_persistence and making the file a transcript rather than a live thing. Jupyter is the corpus's evidence that live-evaluation documents thrive in professional niches; the who_is_the_author question sharpens here: notebooks are authored by programmers for programmers, which is the audience HyperCard refused to accept as the whole audience.";

UPSERT concept:wasm_plugins SET
  title    = "WASM and browser extensions",
    kind     = 'modern_analog',
  year     = 2017,
  confidence = 'medium',
  tags     = ['analog','extensibility'],
  summary  = "The performance-and-extension escape hatch XCMDs provided, re-derived for the browser: compiled modules that can only touch what the host imports to them — the same extension bargain, but sandboxed by construction, with permissions and review coming from the surrounding extension system rather than from WebAssembly itself.",
  body     = "WebAssembly gives browsers the XCMD's role: bring your own compiled code and call it like a native. Unlike an XCMD, which had the whole machine, a Wasm module is sandboxed and gets only the capabilities the host passes in; outside the browser, WASI adds capability-based security. Browser extensions add the distribution politics HyperCard never had: review, install prompts, revocation. The corpus maps xcmd here to make the structural point: every sufficiently capable environment grows a native escape hatch, and HyperCard trusted the file; the web sandboxes the code and then trusts the store. security_naivete and macro_virus read forward into this node: the extension malware industry is MerryXmas with a business model.";

UPSERT concept:dynamicland SET
  title    = "Dynamicland",
  kind     = 'modern_analog',
  year = 2017,
  confidence = 'medium',
  tags     = ['analog','research'],
  summary  = "Bret Victor's research lab: live, inspectable, reprogrammable media in physical space. Its Realtalk system lists HyperCard among its influences. The Oakland community space ran from 2017 until Covid and has been closed since; a new space in Berkeley is hoped to open gradually in 2027.",
  body     = "Dynamicland built rooms of live programmable paper: everything shows its source, editing is in-place, and the system is designed to be learned by use — the script-visible-in-place worldview rebuilt after the web. Its own pages name HyperCard (as 'Hypercard in the World') as an inspiration for Realtalk. It is the corpus's proof that the technical ideas remain live research, not nostalgia, and also its honest limit: a lab, not a distribution channel — and currently without a public space. The succession_failure node reads forward here — the ideas survived by leaving industry, again.";

UPSERT concept:ai_codegen SET
  title    = "AI code generation / natural-language programming",
  kind     = 'modern_analog',
  year     = 2021,
  confidence = 'medium',
  tags     = ['analog','frontier'],
  summary  = "The claim HyperTalk made about English, outsourced to a model: describe what you want, get working software — reviving end_user_programming at the exact point (readability vs correctness) where HyperTalk drew its line.",
  body     = "HyperTalk's bet was that a shaped natural language plus a live environment lets non-programmers build working software; LLM codegen bets that unshaped natural language plus a compiler-shaped interpreter does. The corpus places ai_codegen against natural_language_syntax and end_user_programming because the failure modes rhyme: plausible text that is quietly wrong (forgiving semantics' shadow), code you cannot inspect at the granularity you need (script_property's shadow), and capability without permission (who_is_the_author again). By 2025 mainstream tools (for example GitHub Copilot's agent mode, announced February 2025) edit whole projects on request, so the practical question is less whether a model can write the script than what artifact it should write into. The genuinely HyperCard-flavored research direction the corpus flags: AI that edits *live, inspectable, single-file artifacts* — the stack as the substrate agents should write, rather than headless repos.";

-- Questions: the open design questions a modernization must answer (kept as nodes so
-- tensions and analogs can point at them).

UPSERT concept:q_modern_stack SET
  title    = "Q: What is the modern stack?",
  kind     = 'question',
  confidence = 'high',
  tags     = ['question'],
  summary  = "The framing question: what is the unit of distributable, self-contained, self-describing software today — a single-file HTML app, a notebook, a PWA bundle, a CRDT document — and which stack properties survive translation?",
  body     = "Sub-questions the corpus tracks: does the artifact carry its source (script_property) or is source a build secret? Does opening equal running equal editing (authoring_runtime_unity)? Is the canvas fixed (a virtue for agents and humans alike?) or responsive? The candidate answers are the map targets: single_file_html_app, tiddlywiki, local_first documents, jupyter. No answer is endorsed; the graph records which tensions each candidate resolves and which it re-opens.";

UPSERT concept:q_authoring_runtime_unity SET
  title    = "Q: Keep authoring = runtime?",
  kind     = 'question',
  confidence = 'high',
  tags     = ['question'],
  summary  = "HyperCard's core collapse — the viewer is the editor — is now a product decision (Player split, view-only docs, read/render vs write/publish). Should a successor restore the unity, for whom, and with what guardrails?",
  body     = "Evidence on the unity's side: tiddlywiki's survival, Decker's design, DevTools' popularity (developers want what users had). Against: author_player_politics happened for revenue reasons that still apply; macro_virus happened for technical reasons that still apply. The question node is pointed at by player_split, author_player_politics, who_is_the_author, and live_editing_web.";

UPSERT concept:q_message_path_modern SET
  title    = "Q: What is the message path across a network?",
  kind     = 'question',
  confidence = 'high',
  tags     = ['question'],
  summary  = "The message path presumes one addressable world on one machine. With server logic, multiple users, and distributed components, what plays the roles of card, background, home stack — and who may install handlers at each level?",
  body     = "DOM bubbling is the partial answer inside one client; `start using` (shared stacks in the path) is the historical sketch of a dependency-scoped path. The corpus connects this to javascript_dom, spa_routing, and no_networking, and notes the security inversion: a networked message path is an RPC surface, and RPC surfaces need the thing HyperCard refused — interfaces.";

UPSERT concept:q_live_editing_web SET
  title    = "Q: Can the web be made live-editable?",
  kind     = 'question',
  confidence = 'high',
  tags     = ['question'],
  summary  = "Every HyperCard live property — message box, in-place script edit, immediate persistence — exists in the browser as a developer privilege. What would it take to make them user privileges, and what breaks when you do?",
  body     = "The pieces exist: consoles, contenteditable, single-file apps, local-first sync. The blockers are the ones the corpus names: security_naivete (live editing is live execution), business_model_clash (editable means unmonetizable?), and succession of trust (who signs the editable artifact?). TiddlyWiki and Decker are existence proofs at niche scale; dynamicland at lab scale.";

UPSERT concept:q_distribution SET
  title    = "Q: How does stackware distribute now?",
  kind     = 'question',
  confidence = 'high',
  tags     = ['question'],
  summary  = "Online libraries and disks: HyperCard's folk distribution died with its platforms. What is the equivalent commons today — a URL, a registry, an agent marketplace — and does curation kill the folk part?",
  body     = "The corpus records the tension between the web's frictionless distribution (URLs) and the app stores' gated distribution (review, signing), with monolithic_file and single_file_html_app as the format poles and low_code as the economic pole. The unanswered part is cultural: stackware needed a critical mass of free authors with free tools; no modern platform offers both.";

UPSERT concept:q_data_model SET
  title    = "Q: What replaces text-with-delimiters?",
  kind     = 'question',
  confidence = 'high',
  tags     = ['question'],
  summary  = "Chunks and cards-as-records were lovable because they were just text. Structured stores (JSON, SQLite, CRDTs, typed blocks) fix the ceiling but lose the read-aloud substrate. Is there a representation that keeps both?",
  body     = "notion_airtable chose structure; obsidian_roam chose text but weak structure; local_first chooses structure with text-ish sync. The corpus sees this as the hardest technical question in the set because it is really an aesthetics question: HyperTalk's transparency depended on the data being sayable in a sentence.";

UPSERT concept:q_collaboration SET
  title    = "Q: Many authors, one live stack?",
  kind     = 'question',
  confidence = 'high',
  tags     = ['question'],
  summary  = "single_user + immediate_persistence + monolithic_file have no merge story. CRDTs supply the mechanism; what supplies the semantics — concurrent script editing, shared message handlers, authorship of behavior rather than text?",
  body     = "Wikis solved collaborative text; nobody has shipped collaborative *malleable behavior* — two users editing the same live object's script with the object running. The corpus flags this as the least-researched node-pair (collaboration × authoring_runtime_unity) and therefore the most interesting for a modernization bet.";

UPSERT concept:q_preservation SET
  title    = "Q: What is preserved when a stack is preserved?",
  kind     = 'question',
  confidence = 'high',
  tags     = ['question'],
  summary  = "The Library of Congress has said it plans a stack-format description; emulation preserves the runtime; Stacksmith and stackimport preserve structure. But a stack's behavior is its content — what is the fidelity bar, and who sets it?",
  body     = "Nodes in play: platform_lockin (the mortality), single_file_unity (the durable object), script_property (behavior-as-content), and the revival toolchain (stacksmith, wyldcard, emulation). The corpus's position, recorded as edges rather than verdicts: format documentation (hypercard.org) is the load-bearing preservation act, because formats outlive emulators.";

UPSERT concept:q_who_is_the_author SET
  title    = "Q: Who is the author?",
  kind     = 'question',
  confidence = 'high',
  tags     = ['question'],
  summary  = "HyperCard made every user an author by construction; the Player split made them consumers; the web made them publishers-with-permissions; AI codegen makes them clients of a generator. A modernization must pick a side of this fork explicitly.",
  body     = "The graph routes here from player_split, author_player_politics, wiki, ai_codegen, and low_code. The corpus deliberately leaves the question open but records the historical claim with highest confidence: when reading and writing are the same tool, authorship is the default state, and every restriction after that is a policy, not a necessity.";

UPSERT concept:q_constraints SET
  title    = "Q: Which constraints were generative?",
  kind     = 'question',
  confidence = 'high',
  tags     = ['question'],
  summary  = "Fixed canvas, two widgets, no arrays, one file, 1-bit paint: HyperCard's output was shaped by hard limits. A successor must decide which limits were the design (and keep them) and which were 1987's invoice (and pay it).",
  body     = "The corpus's evidence for 'generative': stackware's coherence at 512x342, the folk-art legibility of two parts. Evidence for 'invoice': bolted_on_color, interpreter_performance, no_structured_data exoduses. The question is pointed at by every tension node, because each tension is a constraint whose bill came due — and the modernization's real spec is the line between the two categories.";

-- Domains: what people actually used HyperCard for.

UPSERT concept:hypermedia_publishing SET
  title    = "Hypermedia publishing",
  kind     = 'domain',
  year     = 1989,
  confidence = 'medium',
  tags     = ['domain'],
  summary  = "Books that were stacks: Voyager's Expanded Books (A Hard Day's Night), the Electronic Whole Earth Catalog (430 MB, 9000+ pages), Marble Springs — the brief moment when 'multimedia book' meant 'a HyperCard stack you bought'.",
  body     = "Publishers shipped authoring-capable artifacts: the reader could annotate, rewire, and extend the book, because the book was a stack and the reader had HyperCard. That property vanished when readers lost the authoring tools (the Player split's quiet casualty). The corpus treats this domain as the strongest evidence that authoring_runtime_unity was a market feature, not just a hacker virtue.";

UPSERT concept:education SET
  title    = "Education",
  kind     = 'domain',
  year     = 1988,
  confidence = 'medium',
  tags     = ['domain'],
  summary  = "Classrooms were HyperCard's center of gravity: teachers built drill stacks, labs, timelines, and interactive books; students built the stacks instead of buying them — the end-user-programming thesis at its most literal.",
  body     = "HyperCard shipped free into schools with the same Macs that shipped free into homes; the stackware economy's largest informal sector was educational. The corpus links education to free_distribution (access), learnability_first (feasibility), and wiki (the pedagogical descendant: students writing the textbook).";

UPSERT concept:prototyping SET
  title    = "Prototyping",
  kind     = 'domain',
  year     = 1988,
  confidence = 'medium',
  tags     = ['domain'],
  summary  = "HyperCard as the sketchpad of the industry: Myst shipped as HyperCard stacks, and countless products were 'the HyperCard thing' until they outgrew it.",
  body     = "The domain's pattern: build the real thing in HyperTalk, ship it, then port it when monolithic_file, interpreter_performance, or no_networking bite. The corpus records prototyping as the domain where HyperCard's philosophy and its ceiling meet fastest — and where the industry learned the habit of prototyping in a throwaway medium that later generations spent on Figma and SwiftUI.";

UPSERT concept:lightweight_database SET
  title    = "Lightweight databases",
  kind     = 'domain',
  year     = 1988,
  confidence = 'medium',
  tags     = ['domain'],
  summary  = "Cards-as-records at work: small organizations' inventories, catalogs, address books and records — the 'just a stack' databases.",
  body     = "For a decade, small organizations answered 'we need a database' with a stack, because Find/mark/sort plus chunked fields covered 80% of record-keeping at 0% of SQL. When scale bit, the data had to move to a real database, which is the no_structured_data tension in production. (Specific examples cited in earlier drafts — a dealer-inventory system, an NFB robot-arm catalog, a medical-record prototype — could not be verified and were removed.)";

UPSERT concept:art_exhibition SET
  title    = "Art & exhibition",
  kind     = 'domain',
  year     = 1989,
  confidence = 'medium',
  tags     = ['domain'],
  summary  = "Museums, galleries and artists on the card: kiosk stacks, gallery catalogs, hypertext fiction such as Deena Larsen's HyperCard-based Marble Springs — HyperCard as an artist's medium (Storyspace works such as Afternoon are the contrast case).",
  body     = "Fixed canvas, paint tools, sound, and free distribution made HyperCard a gallery-terminal medium of the early 90s and a genuine fiction platform (Larsen's Marble Springs). The corpus links this domain to fixed_canvas (the frame as artistic constraint) and stackware (the folk economy); Decker's community continues this line directly.";

UPSERT concept:presentation SET
  title    = "Presentation",
  kind     = 'domain',
  year     = 1988,
  confidence = 'medium',
  tags     = ['domain'],
  summary  = "Stacks as slide decks before PowerPoint owned the genre: card-per-slide stacks with hyperlinked branching, sales demos, and self-running kiosks — the domain where 'the presentation is the database' made total sense.",
  body     = "Before 1987's PowerPoint matured, HyperCard was a common presentation tool precisely because branching beat linearity for demos, and the deck could query itself. The corpus marks this domain medium-confidence (well attested in community memory and magazines, thinner in the sources gathered here) and uses it in the constraints question: card-per-slide is the fixed canvas working as a feature.";

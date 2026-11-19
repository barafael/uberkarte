-- Tensions: what limited HyperCard, and why each limitation matters to a modernization.

UPSERT concept:no_networking SET
  title    = "No networking (in the box)",
  kind     = 'tension',
  year     = 1990,
  confidence = 'high',
  tags     = ['network','gap'],
  summary  = "HyperCard shipped with no network model at all: multi-user stacks meant XCMDs like HyperCom or HyperCard 3.0's late internet features — the stack was a single-machine universe while the world moved online.",
  body     = "There is no socket, no URL, no shared stack in stock HyperTalk. Authors bolted on networking through XCMDs — Heinlein's HyperCom, HyperDialer's BBS automation, WALT's phone GUI — and the SIDOCI/EHR prototype had to build its own client-server layer on top. HyperCard 3.0/QTi (shown 1996) finally added internet awareness and stacks-as-QuickTime-movies, five years too late; it stalled and was cancelled in 2000. Whitehead's observation that stacks cannot be linked-to from other stacks is the same gap seen from the link side. In the corpus no_networking is the tension with the clearest modern resolution (the web) and the one that most reframes every other design: a message path that cannot cross a network is a different animal from one that can, which is exactly what the message_path_modern question is about.";

UPSERT concept:no_inline_text_links SET
  title    = "No inline text links",
  kind     = 'tension',
  year     = 1987,
  confidence = 'high',
  tags     = ['hypertext','gap'],
  summary  = "Links are rectangles, not spans: HyperCard's region-based linking (buttons, linkText, linkParts) made dense prose hypertext clumsy — Jakob Nielsen's documented critique — while the web's inline anchor made it native.",
  body     = "To link a word in HyperCard you put a button or a linkText region over it; the link is a place you click, not a property of the text. linkText/linkParts (2.0) narrowed the gap but kept the ontology: regions, not anchors; one-way, stack-local, invisible to any outside addresser. Nielsen's contemporary critiques hammered exactly this for scholarly and reference use. The consequence was that serious long-form hypertext (Storyspace, then the web) grew up elsewhere, and HyperCard's hypertext remained navigational (book-like) rather than annotative (web-like). The corpus pairs this with region_link (the mechanism) and html_hyperlink (the rival idea that resolved it), and flags it in the modern_stack question: inline, addressable, bidirectional links are table stakes now — any HyperCard successor without them is a slideshow.";

UPSERT concept:no_structured_data SET
  title    = "No structured data",
  kind     = 'tension',
  year     = 1987,
  confidence = 'high',
  tags     = ['data','gap'],
  summary  = "Text with delimiters is the whole data model: no types, no schema, no relations, no arrays — Find/mark/sort over chunked text carries a stack until it doesn't, at which point users export to FileMaker and leave.",
  body     = "Cards-as-records plus chunks-as-fields is elegant up to a few thousand cards and a single author. Beyond that: no joins, no constraints, no concurrent writers, no query planner, no migration story. The historical exit ramp was Apple's own product line — serious data users were steered to FileMaker (sold by Claris, the same Apple subsidiary that took over HyperCard in 1990), and stacks that outgrew themselves died of export. In the corpus no_structured_data is the tension Notion/Airtable most directly claim to resolve (typed blocks with a friendly surface), and the data_model question is its open form: what is the chunk expression of 2026 — JSON paths? SQLite with a pretty face? — and does making it structured destroy the read-aloud transparency that made chunks lovable.";

UPSERT concept:bolted_on_color SET
  title    = "Color bolted on",
  kind     = 'tension',
  year     = 1992,
  confidence = 'high',
  tags     = ['media','platform'],
  summary  = "The 1-bit model was the design; color arrived in 2.2 (1992) as Color Tools and AddMotion II add-ons over AddColor resources — proof that HyperCard's core representations (paint on cards) were frozen at 1987 hardware.",
  body     = "Cards were 512x342 black-and-white because the Mac was; the tool palette is MacPaint's; the file format stores 1-bit planes with a PLTE palette hack (documented at hypercard.org). When color hardware arrived, HyperCard 2.2 shipped Color Tools and AddMotion II as bolt-ons over AddColor — palettes layered onto the monochrome planes rather than a re-founded color model. The result worked but read as patchwork, and the era marked the moment the platform's representations stopped tracking its hardware. The corpus files this under platform_lockin's sibling: designing so tightly to one machine that the machine's upgrade breaks the design language. QuickTime (1991) then moved 'rich media' off HyperCard's home turf entirely — the QuickTime team had literally prototyped inside HyperCard via XCMDs.";

UPSERT concept:single_user SET
  title    = "Single-user, single-file",
  kind     = 'tension',
  year     = 1987,
  confidence = 'high',
  tags     = ['collaboration','gap'],
  summary  = "One stack, one file, one author at a time: no merge, no roles, no concurrent editing — collaboration happened by mailing stacks (and infecting each other), not by co-authoring.",
  body     = "The single-file unity that makes stacks portable makes them unshareable in the modern sense: opening a stack is locking into one author's state; there is no branch, no review, no 'who changed this button'. Stack libraries were therefore anthologies of single-author works, not commons-maintained artifacts — the opposite of the wiki model that grew up beside them. The corpus connects single_user to collaboration (the question node), to local_first (the partial modern answer: per-user replicas with sync), and to the uncomfortable pairing with immediate_persistence: always-saved state and no merge machinery are two views of the same missing layer. Any modernization that wants HyperCard's author *population* rather than its artifacts must solve what HyperCard never did: many authors, one live document.";

UPSERT concept:interpreter_performance SET
  title    = "Interpreter performance",
  kind     = 'tension',
  year     = 1989,
  confidence = 'high',
  tags     = ['performance'],
  summary  = "Line-by-line interpretation made loops slow and pushed serious computation into XCMDs; the 2.0 compiler helped but arrived with a debugger, and the debugger was a confession that stacks had become software.",
  body     = "HyperTalk's interpreted purity meant a tight loop could crawl; authors learned to push work into XCMDs or restructure around Find. The 2.0 compiler (1990) bought speed, and AddMotion-era stacks pushed animation on the same pump. But the deeper fact the corpus records is sociological: performance pressure created a two-tier authoring world — friendly HyperTalk for glue, compiled resources for anything demanding — which is exactly the escape hatch that lets a platform's center not hold. Compare JavaScript: its performance escape (WASM, native modules) is structurally identical to XCMDs, and its answer was a decade of JIT arms races Apple never ran for HyperTalk. The interpreter_performance node feeds the modern_stack question: what is the execution story when the same file must be a document on a phone and a program on a server?";

UPSERT concept:monolithic_file SET
  title    = "Monolithic file",
  kind     = 'tension',
  year     = 1987,
  confidence = 'high',
  tags     = ['architecture'],
  summary  = "Everything in one file is a feature until the file is 430 MB: no incremental distribution, no partial load, no module system, no concurrent authors — the whole Earth Catalog problem.",
  body     = "Stacks grew: the Electronic Whole Earth Catalog at 430 MB, Myst-scale media stacks, library collections. Because the unit of distribution is the unit of everything, growth costs full copies, full loads, and full backups; there is no dependency graph, no CDN-friendly chunking, no way to update one button on ten thousand stacks except mailing ten thousand stacks. HyperCard's answer was `start using` — shared utility stacks in the message path — a genuine module idea, but coarse (whole stacks, by name, at the platform level). The corpus reads start using as the most underrated HyperCard mechanism for modernization: a message-path-scoped dependency system. monolithic_file pairs with single_file_unity as its thesis's shadow, and with SPA routing/bundling as the modern discipline that solves distribution by re-splitting what HyperCard unified.";

UPSERT concept:platform_lockin SET
  title    = "Platform lock-in (and death by platform)",
  kind     = 'tension',
  year     = 2004,
  confidence = 'high',
  tags     = ['platform','mortality'],
  summary  = "HyperCard was a Mac-only application dependent on a Mac that no longer exists: Classic emulation died with OS X 10.4 on PowerPC (2004 discontinuation), and authoring requires a 20-year-old machine — the stack outlived the computer.",
  body     = "The stack file is beautifully self-contained; the runtime is not. HyperCard ran only on classic Mac OS; Mac OS X dropped it (final 2.4.1 in 1998, discontinued March 2004), leaving authoring only under Classic on PowerPC — hardware now two decades old. The Internet Archive's 2017 in-browser emulation and tools like Stacksmith/stackimport and HyperCardPreview are archaeology, not continuity. The corpus treats platform_lockin as the mortality fact every HyperCard successor must answer: the web's answer (the browser is the platform, re-implemented forever) is the only one that has actually worked at scale, and it is why the modern_stack question starts with 'runs in a browser' even though Atkinson hated that the browser is a viewer, not a workshop.";

UPSERT concept:business_model_clash SET
  title    = "Business model clash",
  kind     = 'tension',
  year     = 1987,
  confidence = 'high',
  tags     = ['economics'],
  summary  = "A free, bundled empowerment platform has no P&L: HyperCard's founding clause (free on every Mac) made it beloved and made it structurally indefensible when Apple needed products, not gifts — hence Claris, hence the SKU split.",
  body     = "HyperCard generated no direct revenue; it generated goodwill and an ecosystem. Inside Apple that made it a cost center with a cult; when the team moved to Claris, it had to sell units, and the free-Player/paid-authoring split (2.2, 1992) broke the founding promise under commercial logic. Jobs's later dismissal of HyperCard 3.0 (reported secondhand: he disliked a product he saw as carrying 'Sculley's stink' — the patronage of the era that funded it) is the same clash seen from the executive side: a platform whose value is its users' capability is worth nothing on a spreadsheet until someone monetizes the users. The corpus records business_model_clash as the tension most likely to sink any modern revival that copies the philosophy but not the subsidy: who pays for a free-on-every-device authoring environment in 2026, and what do they get?";

UPSERT concept:author_player_politics SET
  title    = "Author/Player politics",
  kind     = 'tension',
  year     = 1992,
  confidence = 'high',
  tags     = ['politics','trust'],
  summary  = "Splitting the environment into free viewer and paid editor was a governance decision that read as a betrayal: users who owned libraries of stacks lost the right to open them — the canonical case of platform trust spent to fund the platform.",
  body     = "The Player split did more than charge money: it reclassified every HyperCard user from author-with-tools to consumer-with-app, and it seeded the argument that running code and reading documents are different privileges (the same argument Acrobat, then the web's read/render vs write/publish split, then Apple's App Store review all re-litigate). The user backlash is documented; the deeper loss was epistemic — a stack you can only view teaches you nothing about itself, and the self-teaching artifact died the day inspection became a paid feature. In the corpus this node is the hinge between business_model_clash (why it happened) and who_is_the_author (what it meant): every modern 'view-only by default, edit if you have the link/plan/permission' product is running the 1992 play, and every wiki's 'anyone can edit' is the HyperCard play.";

UPSERT concept:security_naivete SET
  title    = "Security naivety",
  kind     = 'tension',
  year     = 1991,
  confidence = 'high',
  tags     = ['security'],
  summary  = "Executable documents with unrestricted access, no sandbox, no origin, no permission prompt: HyperCard stacks could rewrite your other stacks, and did — the first macro viruses (1991, MerryXmas 1993) were HyperCard's inheritance tax.",
  body     = "The environment's trust model was the user's face: any stack could do anything any stack could do, because the message path does not check who is calling. Macro viruses exploited exactly the features that made stacks lovely — scripts travel in files, `do` runs strings, idle runs code unprompted. Apple's slow response and the 1993 MerryXmas wave pushed the industry toward the posture that still dominates: documents are inert unless you say otherwise (macro security levels, then Office's disable-by-default, then the browser's same-origin sandbox). The corpus frames security_naivete as the unavoidable counterweight to forgiving_semantics and script_property: total inspectability and total capability are the same property seen from two sides. The live_editing_web and modern_stack questions both bottom out here — an always-editable web is an always-executable web, and the viruses have names now.";

UPSERT concept:succession_failure SET
  title    = "Succession failure",
  kind     = 'tension',
  year     = 2000,
  confidence = 'high',
  tags     = ['institutional'],
  summary  = "No one carried the torch inside Apple: HyperCard 3.0/QTi stalled in 1998 when the team moved to QuickTime and was formally cancelled in 2000; SK8 never shipped (released as public domain); the team dispersed (Calhoun and Crow left 2001); the language's heirs all live off-Apple.",
  body     = "Succession is the quiet tension: HyperCard's ideas were person-shaped (Atkinson, Winkler, then Calhoun's engineering era, then Crow's QTi demo at WWDC 1996) and the institution never converted them into a strategy. Jobs disliked the product line; QTi died on the vine; the final authoring environment required PowerPC Classic; SK8 — the object-oriented HyperTalk successor — shipped nothing and was donated to the public domain. Every serious descendant (SuperCard, LiveCode/OpenXTalk, SenseTalk) is third-party. The corpus treats succession_failure as the institutional lesson for any modernization: environments die of org charts, not of technology, and the preservation thread exists because the succession that failed in 2000 is the one the community completed in emulation instead.";

UPSERT concept:web_displacement SET
  title    = "Web displacement",
  kind     = 'tension',
  year     = 1995,
  confidence = 'high',
  tags     = ['history','competition'],
  summary  = "The web did not beat HyperCard on ideas; it beat it on addressing and distribution: global URLs, inline links, free clients, and a server-side world made the single-file stack a niche by 1995 — a displacement Atkinson publicly regretted in 2002.",
  body     = "By 1993–95 the stack libraries that had been HyperCard's internet (CompuServe, HyperHeaven) were folding into websites. The web copied HyperCard's surface (pointing-hand cursor via ViolaWWW, browsable cards as pages, even the REPL via JavaScript — Eich explicitly citing HyperTalk) while solving its two structural gaps: everything addressable, everything linkable from anywhere. Atkinson's 2002 Wired lament — that HyperCard 'could have been the first Web browser' but was trapped in a 'box-centric culture' — is the corpus's canonical counterfactual. web_displacement is recorded not as a verdict on the design but as the reason every modernization question in this corpus is phrased in web terms: the contest was lost once, and the terms of the loss define the terms of any retry.";

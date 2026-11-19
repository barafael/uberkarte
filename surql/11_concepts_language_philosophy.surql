-- Language, philosophy, and root ideas.

UPSERT concept:hypertalk SET
  title    = "HyperTalk",
  kind     = 'language',
  year     = 1987,
  confidence = 'high',
  tags     = ['language','core'],
  summary  = "HyperCard's scripting language, designed by Dan Winkler under Bill Atkinson: an English-shaped, case-insensitive, weakly typed, array-less message-oriented language built so a non-programmer can read it aloud and be right.",
  body     = "Dan Winkler, a friend of Atkinson's (Wikipedia dates the start of the language to 1987), designed HyperTalk with Pascal in his background and natural language as the target register: `if the mouse is down then put it into the field`, `play harpsichord c e g`, `answer \"Who are you?\"`. It is case-insensitive, whitespace-insensitive, weakly typed with silent coercion, and organized around handlers and messages rather than calls and returns. Statements read as instructions; the message box makes every line immediately testable. HyperTalk is reported to have influenced JavaScript (Wikipedia: Eich was inspired by it), AppleScript (Wikipedia: 'based largely on HyperCard's HyperTalk language'), Lingo, and the whole xTalk family. An on-the-fly compiler and a debugger arrived in HyperCard 2.0 (1990); the antlr grammars-v4 grammar is based on the 2.4.1 grammar. Its deepest bets — English as UI, everything is text, no arrays, forgiving semantics — are each recorded separately as nodes and tensions in this corpus.";

UPSERT concept:natural_language_syntax SET
  title    = "English-shaped syntax",
  kind     = 'language',
  year     = 1987,
  confidence = 'high',
  tags     = ['syntax','learnability'],
  summary  = "HyperTalk's register: commands that parse as sentences, keywords as particles, containers addressed as nouns — `put card field 2 into cd field 3` — trading precision and parsability for read-aloud transparency.",
  body     = "The syntax is engineered for the untrained eye: `put`, `get`, `set ... to`, `go to`, `if ... then`, `repeat for 3 times`, `convert`. Modifiers and prepositions read like prose (`visual effect wipe left` then `go to next card`; `put it into line 3 of card field \"notes\"`), and the same English prepositions chain containers the way paths do. The cost is a famously gnarly grammar (the language is context-sensitive enough that parsers need semantic hints) and a ceiling on expressiveness — no namespaces, no operator overloading, no way to extend the English. The aim was that stacks could be read aloud and understood. In 2026 this node connects straight to ai_codegen: if natural language is the interface, is HyperTalk the historical proof that shaped-natural-language works, or the proof that it needs a compiler-shaped mind behind it (the LLM) to complete the circuit?";

UPSERT concept:hypertalk_compiler SET
  title    = "HyperTalk 2.0 compiler and debugger",
  kind     = 'language',
  year     = 1990,
  confidence = 'high',
  tags     = ['performance','tooling'],
  summary  = "HyperCard 2.0 (1990, Kevin Calhoun leading the upgrade) replaced the interpreter with a compiler that compiles each handler on first execution and caches it in RAM, and added a debugger with checkpoints and stepping — the moment HyperCard admitted its interpreted purity had a performance and maintainability ceiling.",
  body     = "Original HyperTalk was pure interpretation, line by line, which was fine for navigation and painful for anything loop-heavy; stack authors felt it as the 1990s arrived. HyperCard 2.0 shipped a compiler that compiles handlers on their first execution and keeps the compiled code in RAM (it is not saved), plus a debugger with checkpoints and single-stepping that the original philosophy had no room for (you do not debug a document). The team also added scriptable menus, multiple windows and variable card sizes in 2.0; windows have properties but are not objects with scripts. In the corpus this node marks the first time the environment's two goals — learnable toy, serious platform — forced a change in the toy rather than the goal, a fork every 'citizen developer' platform eventually faces.";

UPSERT concept:xtalk_family SET
  title    = "The xTalk family",
  kind     = 'language',
  year     = 1990,
  confidence = 'high',
  tags     = ['language','lineage'],
  summary  = "HyperTalk's dialects: SuperTalk (SuperCard), ToolBook's OpenScript, Lingo/ActionScript (Director), MetaTalk/revTalk/Transcript (MetaCard→Runtime Revolution→LiveCode→OpenXTalk), SenseTalk (and Eggplant QA), PlusTalk/MediaTalk, SK8's language — a real language family with shared grammar and message-passing soul.",
  body     = "Bill Appleton's SuperCard (1989, from Silicon Beach Software) was an early commercial HyperCard-like environment, with SuperTalk; Asymetrix's ToolBook carried OpenScript; Macromedia's Director spoke Lingo (later ActionScript); MetaCard's MetaTalk — later revTalk, then Transcript in LiveCode, now OpenXTalk — is the strongest surviving lineage (LiveCode's open-source edition ended in 2021 and OpenXTalk forks it; LiveCode's HyperCard stack import is lossy); SenseTalk powers Eggplant test automation (unverified here); Apple's own SK8 carried a HyperTalk-descended language. creysoft.com's xtalk archive documents the family. All share the core grammar (put, set, go, chunks, containers) and most share the message path. The corpus treats xtalk as evidence that HyperTalk was a discoverable language design, not just Apple glue — and that its survival required detaching from the Mac: every survivor runs on other platforms, none is a document format, and all became developer tools rather than end-user environments. That trade is the xtalk diaspora's central fact.";

UPSERT concept:end_user_programming SET
  title    = "End-user programming",
  kind     = 'philosophy',
  year     = 1987,
  confidence = 'medium',
  tags     = ['philosophy','core'],
  summary  = "The founding belief: ordinary computer users, given a readable language and a live environment, will write their own software — and the resulting folk stackware is the corpus's evidence that it happened at scale at least once.",
  body     = "HyperCard is the strongest historical case for end-user programming: not a demo, a population. Free on every Mac, learnable via the message box and a reference stack, it produced a large body of user-made stacks (Wikipedia notes a whole ecosystem of vendors and thousands of externals) and a practitioner literature (Goodman, DeVoto). Per Wikipedia, Atkinson would give HyperCard to Apple only if it shipped free on all Macs. The corpus asks what the experiment proves: that demand exists (it does), that capability exists (it does), and that the surrounding economics punish it (they do — see business_model_clash, author_player_politics). Low-code's rise is the same claim re-priced for enterprises; ai_codegen is the same claim with the human author replaced.";

UPSERT concept:forgiving_semantics SET
  title    = "Forgiving semantics",
  kind     = 'philosophy',
  year     = 1987,
  confidence = 'high',
  tags     = ['philosophy','language'],
  summary  = "HyperTalk refuses to punish: case-insensitive, type-coercing, silent on unhandled system messages, permissive about punctuation — `play harpsichord c e g` just works — because an environment that shames learners loses them.",
  body     = "The language bends toward the learner: `PUT` equals `put`, numbers are strings are containers, an unhandled system message such as mouseUp disappears without an error (an unhandled command still raises 'Can't understand'), `the long date` parses like a phrase. The design consequence is a language that is easy to start and hard to trust: errors surface far from their cause, and correctness is behavioral, not declared. The corpus pairs this node with learnability_first (its intent) and security_naivete (its shadow — a forgiving system is also an unguarded one), and with Bret Victor's Learnable Programming, the explorable-code argument against which any such environment can be measured.";

UPSERT concept:immediate_persistence SET
  title    = "Immediate persistence (no Save)",
  kind     = 'philosophy',
  year     = 1987,
  confidence = 'high',
  tags     = ['philosophy','state'],
  summary  = "A stack is always saved: edits to cards, fields, even scripts take effect and persist with no Save command — the document is never a draft, which is liberating for journals and terrifying for shared or versioned work.",
  body     = "There is no dirty flag in HyperCard. Type into a card, move a button, rewrite a handler: it is done, it is on disk, closing the stack keeps it. For the journaling, cataloging, and note-taking uses HyperCard attracted, this is exactly right — the tool never nags and never loses. For software engineering it is exactly wrong: no checkpoints, no undo across sessions, no branch, no review, no 'keep working without committing'. The corpus records immediate_persistence as the purest single-machine zen of the design — a state model that assumes one trusted author on one trusted disk — and reads local-first software as its most thoughtful heir: keep the always-saved feeling, add sync and history underneath. The authoring_runtime_unity question inherits it: if editing is free, what protects the artifact from its users?";

UPSERT concept:learnability_first SET
  title    = "Learnability as design law",
  kind     = 'philosophy',
    year     = 1987,
  confidence = 'medium',
  tags     = ['philosophy','pedagogy'],
  summary  = "A design law this corpus infers from HyperCard's shape: a curious novice should be able to make something useful quickly — hence the message box, browsable reference stacks, a two-part widget kit and a forgiving language.",
  body     = "HyperCard's shape is a sequence of learnability decisions: ship the documentation as browsable stacks (the HyperTalk Reference stack is itself HyperCard); keep one always-open command channel (the message box); keep the widget kit small enough to hold in your head (button, field); keep the language readable. The philosophy has a documented cost curve: the same simplicity that teaches becomes the ceiling that caps (no arrays, one window, no networking) — the corpus models that as each philosophy node motivating a design node and a matching tension node. The constraints question asks whether learnability requires exactly this kind of artificial limitation, which is the unfashionable half of Atkinson's position.";

UPSERT concept:free_distribution SET
  title    = "Free on every Mac",
  kind     = 'philosophy',
  year     = 1987,
  confidence = 'high',
  tags     = ['philosophy','distribution'],
  summary  = "Atkinson's condition for shipping: HyperCard must be free on every Macintosh (existing owners paid $49.95). The install base of millions was manufactured by that clause — and so was the business-model resentment that eventually orphaned the product.",
  body     = "Atkinson reportedly agreed to ship HyperCard only if it came free with every Mac. Apple complied (August 1987; 10,000 copies in the first week, 35,000 in the first month, per Wikipedia). The result: plausibly the largest end-user programming install base of its time (an unsourced superlative), and a product with no revenue line. When the team was moved to Claris, HyperCard had to become a paid SKU to survive commercially, and the free-Player/paid-authoring split followed — the exact reversal of the founding clause. In the corpus free_distribution is the node where philosophy and economics collide: the same decision that created stackware made HyperCard structurally indefensible inside a company. Every modern platform that wants a HyperCard moment faces the identical choice: subsidy or SKU, and SKUs do not create folk cultures.";

UPSERT concept:hypertext SET
  title    = "Hypertext",
  kind     = 'idea',
  year     = 1965,
  confidence = 'high',
  tags     = ['idea','lineage'],
  summary  = "Ted Nelson's 1965 coinage and program: documents with links — non-linear writing, transclusion, two-way structure. HyperCard delivered a cheap, shippable subset (one-way region links between cards) and thereby mainstreamed the word.",
  body     = "Nelson coined 'hypertext' in 1965 and spent decades building its maximal form (Xanadu: transclusion, versioning, two-way links). HyperCard shipped the minimal form: go-messages between cards, one-way, region-sized — and brought the idea to a mass audience. The gap between Nelson's vision and Atkinson's product is the corpus's central critical axis: no inline anchors, no transclusion, no link database, no links from the outside in. The web won by choosing the opposite subset: weak semantics, global addressing, inline links, no transclusion — proving that hypertext's distribution depended on addressing, not on quality of link structure. The corpus keeps hypertext as the root idea node from which both the lineage (memex→guide→hypercard) and the counterfactual (what a HyperCard-web would have been) are drawn.";

UPSERT concept:memex SET
  title    = "Memex",
  kind     = 'idea',
  year     = 1945,
  confidence = 'high',
  tags     = ['idea','lineage'],
  summary  = "Vannevar Bush's 1945 microfilm desk, 'As We May Think': associative trails between documents — the founding image behind every node in this corpus, transmitted to Engelbart, Nelson, and (at one remove) Atkinson.",
  body     = "Bush imagined a researcher's desk that stores all his books and records and lets him build private associative trails between points in them. The memex is not a device that worked; it is the specification everyone kept implementing: Engelbart's NLS (1968 demo), Nelson's hypertext program, Brown's Guide, Apple's HyperCard, and every personal-knowledge tool since. What the memex always included and HyperCard always lacked: linking from anywhere to anywhere, and links as first-class saved objects. What HyperCard added that Bush's desk lacked: executable trails — the association can do something. The corpus anchors the lineage thread here.";

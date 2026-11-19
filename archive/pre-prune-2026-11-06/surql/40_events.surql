-- Events.

UPSERT event:e_release_1987 SET
  title = "HyperCard ships free with every Mac", when = "1987-08-11", confidence = 'high',
  summary = "Macworld Boston, 11 August 1987: Apple announces HyperCard and bundles it free on every Macintosh — plausibly the largest install base any end-user programming environment had reached by then (an unsourced superlative), created by a single distribution clause.",
  body = "10,000 copies in week one, 35,000 in month one (mostly via the free bundle; ~$49–99 for existing owners). Alsop in Time speculates about Finder replacement; the stackware economy (CompuServe 300+ stacks by 1988, HyperHeaven on AOL) follows within a year.";

UPSERT event:e_hc2_1990 SET
  title = "HyperCard 2.0 ships", when = "1990", confidence = 'high',
  summary = "The Calhoun era: compiler, debugger, window/menu objects, multiple windows, linkText — HyperCard re-engineered into a product, the same year Atkinson left Apple.",
  body = "Answers interpreter_performance and part of fixed_canvas; introduces the professional-tooling trajectory that later versions (and the QTi dead end) follow.";

UPSERT event:e_virus_1991 SET
  title = "First macro viruses in the wild", when = "1991-04", confidence = 'high',
  summary = "Self-replicating HyperTalk appears in Belgium and the Netherlands (April 1991) — among the earliest document-borne viruses on record (priority claims of this kind are contested), exploiting scripts-as-data inside stacks.",
  body = "The infection vector is the founding feature: scripts travel in files and run on open. The industry's macro-security posture (Office 1995 onward) is downstream of this event.";

UPSERT event:e_merryxmas_1993 SET
  title = "MerryXmas virus wave", when = "1993-12", confidence = 'high',
  summary = "MerryXmas spreads HyperCard macro viruses as Christmas-card stacks through the stackware libraries — the folk commons briefly becoming a contagion network.",
  body = "Spread through exactly the distribution channels (libraries, compuserve-style exchange) that made stackware; Apple's slow response feeds security_naivete's case.";

UPSERT event:e_claris_player_1992 SET
  title = "Claris ships HyperCard 2.2 with the Player split", when = "1992", confidence = 'high',
  summary = "Free Player, paid authoring, Color Tools and AddMotion II bolted on: the founding clause (free, whole) reversed under Claris's commercial logic, and the user backlash that made it a canonical platform-trust case.",
  body = "HyperCard moved to Claris in 1990 (Claris itself dates from 1987), with some of the team staying at Apple; 2.2 is the first Claris-era flagship and the moment reading a stack and writing one became different products.";

UPSERT event:e_hc3_wwdc_1996 SET
  title = "HyperCard 3.0 / QTi demoed at WWDC", when = "1996-05", confidence = 'high',
  summary = "Dan Crow shows color, internet, and stacks-as-QuickTime-movies in a browser — HyperCard proposing its own web future five years after the web made the proposal unnecessary.",
  body = "The demo is technically the answer to no_networking and web_displacement; the product stalls (1998) and is cancelled (2000).";

UPSERT event:e_hc3_cancelled_2000 SET
  title = "HyperCard 3.0 cancelled", when = "2000 (development ended 1998)", confidence = 'medium',
  summary = "Jobs-era Apple formally ends the successor — development had already stopped in late 1998 — and Calhoun and Crow leave in 2001 — the institution formally declines to succeed the environment.",
  body = "succession_failure's decision node: the ideas leave the company (SK8 already public domain) and survive only in third-party hands.";

UPSERT event:e_discontinued_2004 SET
  title = "HyperCard discontinued", when = "2004-03", confidence = 'high',
  summary = "Apple discontinues HyperCard (final 2.4.1 from 1998); authoring now requires OS X 10.4 Classic on PowerPC — the platform death that turns every stack into an artifact.",
  body = "The stack outlives the computer: from this date the corpus's preservation and revival threads become the main story.";

UPSERT event:e_emulation_2017 SET
  title = "Internet Archive runs HyperCard in the browser", when = "2017", confidence = 'high',
  summary = "The Internet Archive ships in-browser emulation of classic Macs running HyperCard — the first time a stack could be opened by a double-click with no vintage hardware.",
  body = "Runtime-preservation pole of the preservation question; paired with hypercard.org's format pole and Stacksmith's tooling pole.";

UPSERT event:e_atkinson_wired_2002 SET
  title = "Atkinson's Wired interview: the regret", when = "2002", confidence = 'high',
  summary = "Atkinson on why HyperCard missed the internet: 'box-centric culture' — it 'could have been the first Web browser' — the counterfactual the corpus keeps as its central what-if.",
  body = "The interview is the source for the regret, the Sculley-stink line's context, and the free-distribution condition's telling. The corpus's web_displacement node quotes it as the designer's own verdict.";

UPSERT event:e_wozniak_praise_2001 SET
  title = "Wozniak's verdict", when = "2001", confidence = 'high',
  summary = "Steve Wozniak calls HyperCard the best program ever written (for the Mac) — the engineer-culture epitaph, delivered as the product was being cancelled.",
  body = "Paired with the 2000 cancellation in the corpus timeline: maximal praise at the moment of institutional death.";

UPSERT event:e_demo_1968 SET
  title = "Engelbart's 'Mother of All Demos'", when = "1968-12-09", confidence = 'high',
  summary = "NLS demonstrated mouse, hypertext, collaborative editing, and live authoring in one hour — the founding demonstration of the augmentation lineage HyperCard continues.",
  body = "The lineage thread's anchor event: memex (1945) → demo (1968) → Smalltalk/Guide/NoteCards (70s–83) → HyperCard (1987).";

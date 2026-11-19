-- Events.

UPSERT event:e_release_1987 SET
  title = "HyperCard ships free with every Mac", when = "1987-08-11", confidence = 'high',
  summary = "Macworld Boston, 11 August 1987: Apple announces HyperCard and bundles it free with every new Macintosh — plausibly the widest distribution an end-user programming environment had reached by then (an unsourced superlative).",
  body = "10,000 copies in week one, 35,000 in month one (Wikipedia); sold at $49.95 to existing owners and free with every new Mac thereafter. The stackware economy follows within a year.";

UPSERT event:e_hc2_1990 SET
  title = "HyperCard 2.0 ships", when = "1990", confidence = 'high',
  summary = "The Calhoun era: on-the-fly compiler, debugger, scriptable menus, multiple windows, variable card sizes and group text — HyperCard re-engineered into a product, in the same year Atkinson left Apple.",
  body = "Answers interpreter_performance and part of fixed_canvas; introduces the professional-tooling trajectory that later versions (and the QTi dead end) follow.";

UPSERT event:e_virus_1991 SET
  title = "First macro viruses in the wild", when = "1991-04", confidence = 'high',
  summary = "Self-replicating HyperTalk appears in Belgium and the Netherlands (April 1991) — among the earliest document-borne viruses on record (priority claims of this kind are contested), exploiting scripts-as-data inside stacks.",
  body = "The infection vector is the founding feature: scripts travel in files and run on open. The industry's macro-security posture (Office 1995 onward) is downstream of this event.";

UPSERT event:e_merryxmas_1993 SET
  title = "MerryXmas virus wave", when = "1993", confidence = 'high',
  summary = "MerryXmas, discovered in early 1993 (Wikipedia), another HyperCard macro virus; Wikipedia notes few viruses targeted HyperCard and their impact was minimal.",
  body = "Spread through the same file-sharing channels that made stackware; feeds security_naivete as a case where documents that run code were a vector.";

UPSERT event:e_claris_player_1992 SET
  title = "Apple begins shipping the view-only HyperCard Player", when = "1992-09", confidence = 'high',
  summary = "From 14 September 1992 Apple ships new Macs with only a run-only 'HyperCard 2.1 Player' as its licence to ship the full product from Claris expires: the free, whole founding clause is reversed, and the backlash becomes a canonical platform-trust case.",
  body = "TidBITS (21 Sep 1992) reports Performas were the first Macs to include the Player; Claris sold the full product. HyperCard 2.2 followed later (about 1993).";

UPSERT event:e_hc3_wwdc_1996 SET
  title = "HyperCard 3.0 / QTi demoed at WWDC", when = "1996-05", confidence = 'high',
  summary = "Kevin Calhoun shows an alpha of HyperCard 3.0 / QTi (QuickTime Interactive) at WWDC: a HyperCard that authors interactive QuickTime movies playable in browsers — proposed years after the web made the proposal less necessary.",
  body = "The alpha is the answer to no_networking and web_displacement; the product stalls (1998) and the team is reassigned (2000).";

UPSERT event:e_hc3_cancelled_2000 SET
  title = "HyperCard 3.0 cancelled", when = "2000 (development ended 1998)", confidence = 'medium',
  summary = "Per Wikipedia, the HyperCard engineering team is reassigned in 2000 after Jobs decides to abandon the product (development had effectively stopped in late 1998); Calhoun and Crow leave in 2001.",
  body = "succession_failure's decision node: the ideas leave the company and survive only in third-party hands.";

UPSERT event:e_discontinued_2004 SET
  title = "HyperCard discontinued", when = "2004-03", confidence = 'high',
  summary = "Apple discontinues HyperCard (final 2.4.1 from 1998); authoring now requires OS X 10.4 Classic on PowerPC — the platform death that turns every stack into an artifact.",
  body = "The stack outlives the computer: from this date the corpus's preservation and revival threads become the main story.";

UPSERT event:e_emulation_2017 SET
  title = "Internet Archive runs HyperCard in the browser", when = "2017", confidence = 'high',
  summary = "In August 2017 the Internet Archive lets HyperCard stacks run in the browser through its in-browser classic-Mac emulation — stacks usable with no vintage hardware.",
  body = "Runtime-preservation pole of the preservation question; paired with hypercard.org's format pole and Stacksmith's tooling pole.";

UPSERT event:e_atkinson_wired_2002 SET
  title = "Atkinson's regret, Wired, 2002", when = "2002", confidence = 'high',
  summary = "Leander Kahney's Wired article (14 Aug 2002) quotes Atkinson: 'I grew up in a box-centric culture at Apple. If I'd grown up in a network-centric culture, like Sun, HyperCard might have been the first Web browser.'",
  body = "The corpus's web_displacement node uses this as the designer's own counterfactual. The original wired.com page could not be fetched; the quotation was confirmed from a MacTech item of the same day.";



UPSERT event:e_demo_1968 SET
  title = "Engelbart's 'Mother of All Demos'", when = "1968-12-09", confidence = 'high',
  summary = "NLS demonstrated mouse, hypertext, collaborative editing, and live authoring in one hour — the founding demonstration of the augmentation lineage HyperCard continues.",
  body = "The lineage thread's anchor event: memex (1945) → demo (1968) → Smalltalk and other hypertext systems (70s–83) → HyperCard (1987).";

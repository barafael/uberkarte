-- People and organizations.

UPSERT person:bill_atkinson SET
  name = "Bill Atkinson", role = "creator of HyperCard, Macintosh engineer", birth = 1951, confidence = 'high',
  bio  = "Apple engineer (MacPaint, QuickDraw) who created HyperCard (1987) under creator code WILD — originally WildCard, renamed for trademark risk — with development beginning March 1985. Shipped it on the condition it be free on every Mac. Left Apple in 1990, co-founding General Magic (Hawkeye, then Genesis; later Wavesave). In a 2002 Wired interview he regretted that HyperCard 'could have been the first Web browser' but was trapped in 'box-centric culture'; he is reported to have named iBooks Author as its closest modern heir (secondhand). The phrase 'Sculley's stink' is reported as Jobs's view of the product as relayed by Atkinson, not Atkinson's own label for the era; that attribution rests on secondary accounts here (medium confidence).";

UPSERT person:dan_winkler SET
  name = "Dan Winkler", role = "designer of HyperTalk", confidence = 'high',
  bio  = "Friend of Atkinson's, brought in 1986 to design HyperTalk: an English-shaped, forgiving scripting language with Pascal in its background and read-aloud transparency as its goal. The language's register — `put`, `go to`, `the loc of` — is his; he is the least-celebrated author of the most imitated part of the system.";

UPSERT person:kevin_calhoun SET
  name = "Kevin Calhoun", role = "HyperCard engineering lead (2.0 era)", confidence = 'high',
  bio  = "Led HyperCard 2.0 (1990): the compiler, the debugger, menu and window objects, multiple windows — the turn from Atkinson's artifact to engineered platform. Stayed through the Claris years; left in 2001 with Dan Crow. His account of the era is the corpus's main source for the engineering-ization of HyperCard.";

UPSERT person:dan_crow SET
  name = "Dan Crow", role = "HyperCard 3.0 / QTi engineer", confidence = 'high',
  bio  = "Public face and engineer of HyperCard 3.0 and QuickTime-Internet (QTi): the WWDC 1996 demo showed color, internet connectivity, and stacks rendered as QuickTime-vr movies playable in a browser — HyperCard's own bridge to the web, shown five years late. The project stalled (1998) and was cancelled (2000); Crow left Apple in 2001.";

UPSERT person:steve_jobs SET
  name = "Steve Jobs", role = "Apple co-founder; HyperCard's skeptical executive", confidence = 'high',
  bio  = "Returned to Apple in 1996–97 (interim CEO in 1997) and is reported to have disliked HyperCard 3.0; development stopped in 1998, the line was formally cancelled in 2000 per this corpus, and the product discontinued in March 2004. In the corpus he stands at succession_failure and business_model_clash: the moment the company's center of gravity decided an environment was not a product.";

UPSERT person:john_sculley SET
  name = "John Sculley", role = "Apple CEO at HyperCard's launch", confidence = 'high',
  bio  = "CEO who shipped HyperCard free with every Mac and praised it publicly; the patronage that created HyperCard's freedom and — in Jobs's view, as reported secondhand ('Sculley's stink') — tainted it afterward.";

UPSERT person:steve_wozniak SET
  name = "Steve Wozniak", role = "Apple co-founder; early admirer", confidence = 'high',
  bio  = "Called HyperCard (in 2001, looking back) arguably the best program ever written for the Mac — the corpus's shorthand for the engineer's verdict matching the public's.";

UPSERT person:susan_kare SET
  name = "Susan Kare", role = "designer (Macintosh icons; HyperCard cursor work)", confidence = 'medium',
  bio  = "Designer of the Macintosh's founding visual vocabulary; widely credited with HyperCard's pointing-hand link cursor. Attribution is repeated but not nailed down in the sources gathered here — hence confidence medium on that specific claim.";

UPSERT person:vannevar_bush SET
  name = "Vannevar Bush", role = "memex (1945)", birth = 1890, confidence = 'high',
  bio  = "Atlantic editor and science administrator whose 1945 'As We May Think' described the memex — the associative-trail desk that every node in the lineage thread is answering.";

UPSERT person:ted_nelson SET
  name = "Ted Nelson", role = "coined hypertext (1965); Project Xanadu", birth = 1937, confidence = 'high',
  bio  = "Coined 'hypertext' in 1965 and spent sixty years building its maximal form (transclusion, two-way links, versioning). His critique of HyperCard's thin subset is the corpus's standing reminder of what the product chose not to be.";

UPSERT person:doug_engelbart SET
  name = "Douglas Engelbart", role = "NLS; the 1968 demo", birth = 1925, confidence = 'high',
  bio  = "Demonstrated NLS in December 1968 — mouse, hypertext, collaborative editing, live authoring — the Big Bang the whole augmentation lineage, HyperCard included, descends from. Atkinson's team knew the demo's descendants, not the demo itself.";

UPSERT person:alan_kay SET
  name = "Alan Kay", role = "Smalltalk, Dynabook; Xerox PARC / Apple fellow", birth = 1940, confidence = 'high',
  bio  = "Smalltalk (1972–80) is the direct environment ancestor of HyperCard's live, inspectable, message-passing world; the Dynabook is its mission statement. Kay consulted at Apple in the HyperCard era and remains the corpus's theorist of why environments beat tools.";

UPSERT person:donald_brown SET
  name = "Donald A. Brown", role = "author of Guide (1982)", confidence = 'high',
  bio  = "Wrote Guide at Apple (1982, shipped by OWL/Guide Inc.): the first commercial hypertext application, with conditional links and computed links — HyperCard's closest direct ancestor and a documented influence on Atkinson.";

UPSERT person:pei_yuan_wei SET
  name = "Pei-Yuan Wei", role = "author of ViolaWWW", confidence = 'high',
  bio  = "UC Berkeley student who built ViolaWWW (1992), an early graphical browser (by common account the first with scripting, stylesheets and tables; Mosaic is usually credited with popularizing the graphical web), explicitly taking its concepts from HyperCard — inline media, scripting, and the pointing-hand link cursor crossed into the web through his work.";

UPSERT person:brendan_eich SET
  name = "Brendan Eich", role = "creator of JavaScript", confidence = 'high',
  bio  = "Designed JavaScript at Netscape (1995) citing HyperTalk as a direct influence — Scheme in the bed, HyperTalk on the surface; event handler names like onmouseup are HyperTalk's. The message box's grandchild is the browser console.";

UPSERT person:ward_cunningham SET
  name = "Ward Cunningham", role = "inventor of the wiki (1995)", confidence = 'high',
  bio  = "Invented the WikiWikiWeb in 1995 out of pattern-language browsing habits formed on HyperCard stacks; the wiki is HyperCard's authoring half, detached from its runtime half and set loose on the web.";

UPSERT person:john_earnest SET
  name = "John Earnest", role = "author of Decker", confidence = 'high',
  bio  = "Built Decker, the closest modern re-imagining of HyperCard: single-file stacks, a HyperTalk-descended language (Lil, with Q's array bent), 1-bit-plus aesthetic, MIT-licensed, exporting whole stacks to single .html files. beyondloom.com/decker is the corpus's live-systems anchor.";

UPSERT person:uli_kusterer SET
  name = "Uli Kusterer", role = "Stacksmith; stackimport", confidence = 'high',
  bio  = "Maintains the serious toolchain for original stack files: stackimport parses HyperCard 2.x stack files into modern structures, Stacksmith rebuilds and re-authors them natively. The corpus's load-bearing preservation engineer.";

UPSERT person:bill_appleton SET
  name = "Bill Appleton", role = "SuperCard (first HyperCard clone)", confidence = 'high',
  bio  = "Silicon Beach Software developer (World Builder, 1986) who created SuperCard (1989): the first commercial HyperCard clone, with SuperTalk — founding member of the xTalk diaspora and proof the design was separable from Apple.";

UPSERT person:kevin_cuneo SET
  name = "Kevin Cuneo", role = "HyperStudio (role unverified)", confidence = 'low',
  bio  = "Named in earlier versions of this corpus as HyperStudio's creator. HyperStudio is otherwise credited to Roger Wagner (Roger Wagner Publishing, 1989); a search found no link between Cuneo and the product, so this node is kept at low confidence pending a source.";

UPSERT person:tom_pittman SET
  name = "Tom Pittman", role = "CompileIt!", confidence = 'high',
  bio  = "Wrote CompileIt! (1989): a 68k assembler/compiler that bootstrapped itself inside HyperCard's XCMD mechanism, letting stack authors compile their own code — the community's answer to interpreter_performance, and a small miracle in its own right.";

UPSERT person:danny_goodman SET
  name = "Danny Goodman", role = "author, HyperTalk handbook", confidence = 'high',
  bio  = "His Handbook of Operations / Complete HyperTalk Handbook was widely cited as a best-seller of its era (unsourced superlative); he wrote the HyperTalk Reference stack — a book about HyperCard that was itself a HyperCard stack, the purest artifact of the self-teaching medium.";

UPSERT person:jakob_nielsen SET
  name = "Jakob Nielsen", role = "usability critic of region-linking", birth = 1957, confidence = 'high',
  bio  = "Usability researcher whose late-80s/early-90s critiques of HyperCard's region-based linking (vs. inline text links) for dense hypertext are the corpus's documented contemporary counter-argument on hypertext design.";

UPSERT person:robert_cailliau SET
  name = "Robert Cailliau", role = "CERN; co-founder of the Web", confidence = 'high',
  bio  = "Berners-Lee's colleague and propagator at CERN (the 1990 proposal, HyperTape ambitions before it). Included because the Web's origin story overlaps the HyperCard story year-for-year, and the corpus's web_displacement node needs both sides of the trade.";

-- Organizations
UPSERT org:apple SET
  name = "Apple Computer",
  note = "Creator of HyperCard and its home 1987–90 and again after Claris returned it; owner of the stack format; stopped HyperCard 3.0 development (1998), formally cancelled it (2000 per this corpus) and discontinued the line (March 2004).";

UPSERT org:claris SET
  name = "Claris",
  note = "Apple software subsidiary (founded 1987) that took over HyperCard in 1990 and shipped 2.2 with the Player split (1992); HyperCard later returned to Apple (TidBITS, 1998: it 'languished and was returned to Apple'), so 2.4.1 (1998) is not a Claris release. Renamed FileMaker Inc. in 1998; the Claris name was revived in 2019.";

UPSERT org:livecode SET
  name = "LiveCode (formerly MetaCard / Runtime Revolution / RunRev)",
  note = "Strongest surviving xTalk lineage: MetaCard → Runtime Revolution → LiveCode → OpenXTalk; imports original HyperCard stacks; the diaspora's main line.";

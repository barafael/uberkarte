-- People and organizations.

UPSERT person:bill_atkinson SET
  name = "Bill Atkinson", role = "creator of HyperCard, Macintosh engineer", birth = 1951, confidence = 'high',
  bio  = "Apple engineer (MacPaint, QuickDraw) who created HyperCard (1951–2025; he died on 5 June 2025). HyperCard shipped in 1987 under creator code WILD — originally WildCard — with development beginning March 1985, and Wikipedia reports he would give it to Apple only if it shipped free on all Macs. He left Apple in 1990 and co-founded General Magic. In Leander Kahney's Wired article of 14 Aug 2002 he is quoted: 'I grew up in a box-centric culture at Apple. If I'd grown up in a network-centric culture, like Sun, HyperCard might have been the first Web browser.' In 2017 he said of iBooks Author, 'I think it's the closest thing we have today to what HyperCard was' (as cited by Wikipedia). Per Wikipedia, Jobs disliked HyperCard because Atkinson had stayed at Apple to finish it instead of joining him at NeXT and, in Atkinson's words, 'it had Sculley's stink all over it'.";

UPSERT person:dan_winkler SET
  name = "Dan Winkler", role = "designer of HyperTalk", confidence = 'high',
  bio  = "Designer of HyperTalk, an English-shaped, forgiving scripting language with read-aloud transparency as its goal (Wikipedia dates the language's creation to 1987). The language's register — `put`, `go to`, `the loc of` — is his.";

UPSERT person:kevin_calhoun SET
  name = "Kevin Calhoun", role = "HyperCard engineering lead (2.0 era)", confidence = 'high',
  bio  = "HyperCard engineer at Apple who in late 1989 led the effort that became HyperCard 2.0 (1990, with an on-the-fly compiler), and later led the QuickTime-Interactive effort that produced the HyperCard 3.0 alpha shown at WWDC in May 1996. Left Apple in 2001 with Dan Crow (Wikipedia).";

UPSERT person:dan_crow SET
  name = "Dan Crow", role = "HyperCard 3.0 / QTi engineer", confidence = 'high',
  bio  = "Led HyperCard development through the late 1990s (Wikipedia), when public demos showed color, internet connectivity and QuickTime playback. The 3.0/QTi alpha shown at WWDC 1996 was demoed by Kevin Calhoun. Development stalled in 1998; the engineering team was reassigned in 2000; Crow left Apple in 2001.";









UPSERT person:vannevar_bush SET
  name = "Vannevar Bush", role = "memex (1945)", birth = 1890, confidence = 'high',
  bio  = "Atlantic editor and science administrator whose 1945 'As We May Think' described the memex — the associative-trail desk that every node in the lineage thread is answering.";

UPSERT person:ted_nelson SET
  name = "Ted Nelson", role = "coined hypertext (1965); Project Xanadu", birth = 1937, confidence = 'high',
  bio  = "Coined 'hypertext' in 1965 and has long pursued its maximal form (transclusion, two-way links, versioning, as in Project Xanadu); the contrast with HyperCard's one-way region links is a standing reference point for this corpus.";

UPSERT person:doug_engelbart SET
  name = "Douglas Engelbart", role = "NLS; the 1968 demo", birth = 1925, confidence = 'high',
  bio  = "Demonstrated NLS in December 1968 — mouse, hypertext, collaborative editing, live authoring — the founding demonstration of the augmentation lineage this corpus places HyperCard in.";

UPSERT person:alan_kay SET
  name = "Alan Kay", role = "Smalltalk, Dynabook; Xerox PARC / Apple fellow", birth = 1940, confidence = 'high',
  bio  = "Designer of Smalltalk and the Dynabook concept at Xerox PARC, later an Apple Fellow. Smalltalk is a conceptual ancestor of the live, message-passing environment idea that HyperCard also embodies; this corpus draws that line as a design reading, not as a documented influence on HyperCard.";









UPSERT person:john_earnest SET
  name = "John Earnest", role = "author of Decker", confidence = 'high',
  bio  = "Built Decker, a HyperCard-inspired environment: single-file stacks, the Lil language (influenced by Q and array-oriented), a 1-bit-plus aesthetic, MIT-licensed, with a Web-Decker build that is a single standalone HTML file. beyondloom.com/decker is the corpus's live-systems anchor.";

UPSERT person:uli_kusterer SET
  name = "Uli Kusterer", role = "Stacksmith; stackimport", confidence = 'high',
  bio  = "Author of stackimport, a command-line tool that parses HyperCard 2.x stack files, and of Stacksmith, a work-in-progress macOS environment that uses it. Apparently also the maintainer of hypercard.org (an inference; the page does not say so).";









UPSERT person:jakob_nielsen SET
  name = "Jakob Nielsen", role = "usability critic of region-linking", birth = 1957, confidence = 'high',
  bio  = "Usability researcher. Wikipedia's HyperCard article quotes his remark that HyperCard was 'really only a hypermedia program since its links started from regions on a card, not text objects' — the corpus's documented contemporary counter-argument on hypertext design.";



-- Organizations
UPSERT org:apple SET
  name = "Apple Computer",
  note = "Creator of HyperCard and its home 1987–90 and again after Claris returned it; owner of the stack format; stopped HyperCard 3.0 development (1998), formally cancelled it (2000 per this corpus) and discontinued the line (March 2004).";

UPSERT org:claris SET
  name = "Claris",
  note = "Apple software subsidiary (founded 1987) that took over HyperCard around 1990 and sold the full product while Apple distributed a view-only Player (from September 1992); HyperCard later returned to Apple (TidBITS, 1998), so 2.4.1 (1998) is not a Claris release. Renamed FileMaker Inc. in 1998 and the Claris name revived in 2019 (from general knowledge, not re-verified here).";

UPSERT org:livecode SET
  name = "LiveCode (formerly MetaCard / Runtime Revolution / RunRev)",
  note = "Strongest surviving xTalk lineage: MetaCard → Runtime Revolution → LiveCode → OpenXTalk; converts original HyperCard stacks on open, lossily (resources and report templates are not carried); the open-source Community edition ended in 2021 and OpenXTalk forks it.";

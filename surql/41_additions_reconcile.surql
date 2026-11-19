-- Nodes added to reconcile the corpus with notes/coverage-critique.md.
-- Convention: anything not yet tied to a source says so in its text, and carries
-- confidence 'medium' or lower. See notes/reconciliation.md for the ledger.

-- People ----------------------------------------------------------------------






-- Organizations ---------------------------------------------------------------


















-- Events ----------------------------------------------------------------------






UPSERT event:e_hc_team_moved_1998 SET
  title = "HyperCard team moved to QuickTime; development ends", when = "1998-10", confidence = 'medium',
  summary = "Late October 1998: Apple reportedly tells employees HyperCard will no longer be developed and moves the team to QuickTime — the real end of the HyperCard 3.0 / QTi line, two years before the formal cancellation this corpus dates 2000.",
  body = "Reported contemporaneously by TidBITS (2 Nov 1998), attributing the announcement to Phil Schiller at an employee meeting and noting Apple made no official statement. QTi had already slipped in 1997 when Apple shipped QuickTime 3 without it. Medium confidence: single contemporaneous report, unofficial.";





UPSERT event:e_intel_2006 SET
  title = "Intel transition ends practical Classic authoring", when = "2006", confidence = 'medium',
  summary = "Apple's first Intel Macs (January 2006) cannot run Classic, and Classic is dropped from OS X with 10.5; the last officially supported HyperCard authoring path becomes PowerPC-only.",
  body = "The 2004 discontinuation ended sales; the 2006 transition ended running the tool on new hardware. From here the corpus's preservation and revival threads are the only way to author.";

UPSERT event:e_ipad_2010 SET
  title = "iPad ships without an authoring environment", when = "2010-01", confidence = 'medium',
  summary = "Apple's iPad (announced January 2010) makes the touch-first card as large a device class as the original Mac — and, by design, ships with no end-user authoring environment.",
  body = "Medium confidence on relevance: included because the HyperCard-shaped hole (a device whose unit of software is a screen) opened here and was answered by apps and later by iBooks Author, not by a stack medium.";

UPSERT event:e_ibooks_author_2012 SET
  title = "iBooks Author announced", when = "2012-01-19", confidence = 'high',
  summary = "Apple announces iBooks Author at its education event: free authoring, gated publishing — the 1992 author/Player politics replayed twenty years later by the company that owned the original.",
  body = "An earlier version of this corpus dated the tool 2013. Atkinson is reported (secondhand) to have named it HyperCard's closest modern heir.";

-- Concepts --------------------------------------------------------------------


UPSERT concept:stack_idioms SET
  title    = "Stack idioms and craft",
  kind     = 'practice',
  year     = 1988,
  confidence = 'medium',
  tags     = ['practice','craft'],
  summary  = "The working repertoire of stack authors: lock screen to hide redraw, visual effects between cards, invisible buttons over painted art, animation on the idle pump, navigation by convention — craft knowledge that lived in books and other people's stacks, not in the manual.",
  body     = "This is the one place the corpus names authoring as practice rather than mechanism. Its evidence is the verbatim code layer (lock-all patterns, visual-effect sending, idle-driven animation, Myst's black-frame handling) and the DeVoto/Goodman practitioner literature; no survey of how typical stacks were built exists in the sources gathered. Medium confidence: the idioms are demonstrated, their prevalence is not.";

UPSERT concept:visual_culture SET
  title    = "Visual culture of stackware",
  kind     = 'practice',
  year     = 1987,
  confidence = 'medium',
  tags     = ['practice','aesthetics'],
  summary  = "The MacPaint look as a shared aesthetic: 1-bit bitmaps, dithering, the 512x342 frame, pointing-hand cursors — a visual vocabulary that came from the tool's constraints and became the medium's identity.",
  body     = "fixed_canvas and macpaint explain the constraint; this node records that the constraint became style (and, in Decker, a deliberate revival style). Medium confidence: supported by the artifacts the corpus holds rather than by any dedicated source on stackware design.";

UPSERT concept:anti_successor SET
  title    = "The case against succession",
  kind     = 'philosophy',
  year     = 1995,
  confidence = 'medium',
  tags     = ['counter-case','framing'],
  summary  = "The argument that HyperCard's limits were real and the web won for good reasons: no addressing, no sandbox, a single-file unit that does not scale, a model that rewarded one author on one machine — so a faithful successor would inherit failure modes, not a lost future.",
  body     = "The corpus is direction-agnostic in schema but, in selection, tells a succession story (an institution failed an environment). This node states the opposing reading in its strongest form, built only from the corpus's own tension nodes: web_displacement (the web copied the surface and solved the gaps), security_naivete (the trust model cannot survive a networked world), monolithic_file and no_structured_data (scale), and business_model_clash (no one pays for a free-on-every-device tool). It does not claim the question is settled; it makes q_authoring_runtime_unity a question the corpus can actually lose.";

-- Artifacts: preservation as format and runtime -------------------------------
UPSERT artifact:stack_format SET
  title = "HyperCard stack file format", kind = 'file_format', year = 1987, status = 'community-documented', confidence = 'high',
  summary = "The binary container for stacks, cards, backgrounds, resources and scripts: the object that survives when the program does not, documented by hypercard.org and Lorenzi's notes; Apple never published a specification.",
  body = "The preservation thread names three poles — runtime, format, reimplementation — and this is the format pole as an artifact. Everything downstream (stackimport, Stacksmith, HyperCardPreview, Wyldcard's loaders) rests on reverse-engineered documentation; the Library of Congress has said it plans a format description (2021) but none was found.";







UPSERT artifact:internet_archive SET
  title = "Internet Archive", kind = 'archive', year = 1996, status = 'alive', confidence = 'high',
  summary = "Brewster Kahle's non-profit digital library: the institution behind the 2017 in-browser Macintosh emulation that let a stack be opened with a double-click — an archive as artifact, not just as a source citation.",
  body = "Included as an artifact so the corpus's archive kind is no longer empty and the runtime-preservation pole has an institutional actor.";

-- Artifacts: families the critique found missing -----------------------------




UPSERT artifact:flash SET
  title = "Macromedia Flash", kind = 'software', year = 1996, status = 'end-of-life 2020', confidence = 'medium',
  summary = "Animation and interactive-media authoring tool (FutureSplash, then Macromedia Flash, 1996) whose scripting this corpus routes down the Director/Lingo line: the web's mass-market authoring medium — and one that split authoring tool from Player exactly as HyperCard 2.2 did.",
  body = "The obvious media-authoring successor, and an instructive one: Flash reproduced the free-player/paid-authoring split (author_player_politics) and then died of platform politics and security, not of the idea.";

UPSERT artifact:scratch SET
  title = "Scratch", kind = 'software', year = 2007, status = 'alive', confidence = 'medium',
  summary = "A block-based programming environment for learners that originated at the MIT Media Lab (2007) and is now stewarded by the Scratch Foundation: live-editable sprites and stages in a shared online community — the closest living answer to HyperCard's classroom role.",
  body = "Evidence for q_authoring_runtime_unity (the project is edited while it runs) and for the education domain's survival without HyperCard.";

UPSERT artifact:twine SET
  title = "Twine", kind = 'software', year = 2009, status = 'alive', confidence = 'medium',
  summary = "Chris Klimas's tool for authoring interactive fiction as linked passages, publishing each story as a single HTML file — the hypertext-fiction and games branch of the card idea, on the web.",
  body = "Single-file publishing (single_file_unity) plus link-as-structure, in an authoring tool that non-programmers actually use.";







-- Sources used for the corrections above -------------------------------------


UPSERT source:s_tidbits_1998 SET
  title = "TidBITS: Alas, HyperCard! (2 Nov 1998)", url = "https://tidbits.com/1998/11/02/alas-hypercard/", kind = 'article', reliability = 'medium',
  note = "Contemporaneous report (Geoff Duncan) of the end of HyperCard development: QTi slipping in 1997, the team moved to QuickTime in October 1998, Schiller reported (via 'reliable sources') announcing it internally, no official Apple statement; says HyperCard 'languished' at Claris and 'was returned to Apple'. Read in full 2026-11-06. Does not mention Crow or Calhoun.";

UPSERT source:s_tnw_ibooks_2012 SET
  title = "The Next Web: Apple announces iBooks Author (19 Jan 2012)", url = "https://thenextweb.com/news/apple-announces-ibook-author-a-mac-app-for-authoring-books", kind = 'article', reliability = 'medium',
  note = "News report dating the announcement to 19 January 2012. Located by search 2026-11-06; headline and date checked, not read in full.";



UPSERT source:s_supercard_wiki SET
  title = "Wikipedia: SuperCard", url = "https://en.wikipedia.org/wiki/SuperCard", kind = 'article', reliability = 'medium',
  note = "States SuperCard was created by Bill Appleton and published by Silicon Beach Software in 1989, combining ideas from World Builder, HyperCard, SuperPaint and the Mac UI. Located by search 2026-11-06.";

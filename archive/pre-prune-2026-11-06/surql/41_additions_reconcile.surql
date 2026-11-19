-- Nodes added to reconcile the corpus with notes/coverage-critique.md.
-- Convention: anything not yet tied to a source says so in its text, and carries
-- confidence 'medium' or lower. See notes/reconciliation.md for the ledger.

-- People ----------------------------------------------------------------------
UPSERT person:andy_hertzfeld SET
  name = "Andy Hertzfeld", role = "original Macintosh engineer; General Magic co-founder", confidence = 'medium',
  bio  = "Member of the original Macintosh team and, with Bill Atkinson and Marc Porat, a co-founder of General Magic (1990) — the destination of Atkinson's post-Apple career and the node that connects the HyperCard author to the Mac team's later diaspora.";

UPSERT person:roger_wagner SET
  name = "Roger Wagner", role = "author of HyperStudio", confidence = 'high',
  bio  = "Apple II educator-developer whose Roger Wagner Publishing released HyperStudio (1989, Apple IIGS first, later Mac and Windows) — the education-market HyperCard-like. Corrects an earlier attribution of HyperStudio to Kevin Cuneo.";

UPSERT person:michael_joyce SET
  name = "Michael Joyce", role = "author of afternoon, a story", confidence = 'medium',
  bio  = "Hypertext novelist whose afternoon, a story (written 1987, published by Eastgate in 1990 on Storyspace) is the canonical literary hypertext of the era — the electronic-literature branch that grew beside HyperCard rather than inside it.";

-- Organizations ---------------------------------------------------------------
UPSERT org:filemaker_inc SET
  name = "FileMaker Inc.",
  note = "Claris renamed itself FileMaker Inc. in 1998; the company took the Claris name back in 2019. Home of FileMaker, the database that data-heavy HyperCard users were steered to.";

UPSERT org:general_magic SET
  name = "General Magic",
  note = "Apple spin-off (1990) co-founded by Bill Atkinson, Andy Hertzfeld and Marc Porat; the company Atkinson left Apple to build.";

UPSERT org:cyan SET
  name = "Cyan",
  note = "Rand and Robyn Miller's studio: HyperCard worlds (Cosmic Osmo, Spelunx) before Myst — the clearest case of a stackware shop becoming a mass-market studio.";

UPSERT org:broderbund SET
  name = "Brøderbund",
  note = "Software publisher; published Myst (1993).";

UPSERT org:voyager SET
  name = "The Voyager Company",
  note = "Publisher of the Expanded Books line (A Hard Day's Night, 1990) and Criterion laserdiscs — author-capable hypermedia books on HyperCard-era assumptions.";

UPSERT org:silicon_beach SET
  name = "Silicon Beach Software",
  note = "Mac developer (Dark Castle, World Builder); published SuperCard (1989), the first commercial HyperCard clone. Later acquired by Aldus.";

UPSERT org:macromedia SET
  name = "Macromedia",
  note = "Publisher of Director/Lingo (via Macromind), Authorware (acquired 1992) and Flash — the commercial multimedia-authoring market where many HyperCard authors went. Acquired by Adobe in 2005.";

UPSERT org:compuserve SET
  name = "CompuServe",
  note = "Online service whose forum libraries carried the earliest stackware exchange (300+ stacks by 1988, per this corpus's own prose; sourcing pending).";

UPSERT org:acot SET
  name = "Apple Classrooms of Tomorrow (ACOT)",
  note = "Apple's classroom-technology research program (begun 1985, ran into the 1990s). HyperCard was one of the authoring media in its classrooms; the extent of that role is attested in secondary accounts and not yet sourced in this corpus.";

-- Events ----------------------------------------------------------------------
UPSERT event:e_claris_transfer_1990 SET
  title = "HyperCard moves to Claris", when = "1990", confidence = 'medium',
  summary = "Apple transfers HyperCard to its software subsidiary Claris (a TidBITS piece of November 1990 already treats it as done): free-and-whole becomes sold-and-supported, with a stripped-down copy still bundled on every Mac and the full product at $49.",
  body = "The transfer is the hinge between business_model_clash's two halves — the founding clause and the commercial logic — and precedes the 1992 Player split. HyperCard later returned to Apple, which is why 2.4.1 (1998) is not a Claris release. Exact transfer date unconfirmed; month not asserted.";

UPSERT event:e_compuserve_library_1988 SET
  title = "CompuServe's HyperCard library passes 300 stacks", when = "1988", confidence = 'medium',
  summary = "By 1988 CompuServe's forum library held 300+ stacks — the first mass distribution channel for user-built stackware, a year after release.",
  body = "Carried forward from this corpus's stackware prose rather than a located primary source; treat the count as indicative. The channel is the one the 1991–93 virus events later turned into a contagion path.";

UPSERT event:e_claris_renamed_1998 SET
  title = "Claris becomes FileMaker Inc.", when = "1998", confidence = 'medium',
  summary = "Claris renames itself FileMaker Inc. (1998) after refocusing on its database; the Claris name returns in 2019. The HyperCard-era subsidiary effectively ends the year HyperCard development does.",
  body = "An earlier version of this corpus dated the rename 2001. Coincides with HyperCard's return to Apple's side of the ledger and the end of any commercial home for the product.";

UPSERT event:e_hc_team_moved_1998 SET
  title = "HyperCard team moved to QuickTime; development ends", when = "1998-10", confidence = 'medium',
  summary = "Late October 1998: Apple reportedly tells employees HyperCard will no longer be developed and moves the team to QuickTime — the real end of the HyperCard 3.0 / QTi line, two years before the formal cancellation this corpus dates 2000.",
  body = "Reported contemporaneously by TidBITS (2 Nov 1998), attributing the announcement to Phil Schiller at an employee meeting and noting Apple made no official statement. QTi had already slipped in 1997 when Apple shipped QuickTime 3 without it. Medium confidence: single contemporaneous report, unofficial.";

UPSERT event:e_jobs_return_1997 SET
  title = "Jobs returns to Apple", when = "1996-12 to 1997", confidence = 'medium',
  summary = "Apple buys NeXT (announced December 1996), Steve Jobs returns and becomes interim CEO in 1997 — the change of institutional center of gravity under which HyperCard 3.0 ends.",
  body = "The single most consequential event for HyperCard's fate is a change of owner's priorities, not a technical one: this is succession_failure's backdrop. Note the order: Atkinson had left in 1990, seven years earlier, so he is not a Jobs-era participant.";

UPSERT event:e_osx_2001 SET
  title = "Mac OS X ships; HyperCard stays in Classic", when = "2001-03", confidence = 'medium',
  summary = "Mac OS X 10.0 ships; HyperCard is never ported and runs only inside the Classic environment — the platform bridge that determines every later preservation constraint.",
  body = "From here HyperCard authoring depends on Classic, which runs on PowerPC Macs only. See classic_environment.";

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
UPSERT concept:classic_environment SET
  title    = "Classic environment",
  kind     = 'idea',
  year     = 2001,
  confidence = 'medium',
  tags     = ['platform','mortality'],
  summary  = "Mac OS X's compatibility layer for Mac OS 9 applications: the only route by which HyperCard ever ran on OS X, available on PowerPC Macs only and dropped from OS X 10.5 — HyperCard's authoring life support, and its plug.",
  body     = "HyperCard was never ported to OS X; Classic let the 2.4.1 binary run there until the Intel transition made Classic unavailable on new Macs. 'The last officially supported authoring environment' means exactly this — Classic on PowerPC under OS X 10.4 — which is why platform_lockin is a mortality fact and why emulation (Basilisk II, SheepShaver, Mini vMac) and reimplementation (Wyldcard, Stacksmith) became the preservation story.";

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
  summary = "The binary container for stacks, cards, backgrounds, resources and scripts: the object that survives when the program does not, and the thing hypercard.org, the Library of Congress and Lorenzi's notes all document.",
  body = "The preservation thread names three poles — runtime, format, reimplementation — and this is the format pole as an artifact rather than only as sources about it. Everything downstream (stackimport, Stacksmith, HyperCardPreview, Wyldcard's loaders) is built on documentation reverse-engineered from the format.";

UPSERT artifact:basilisk_ii SET
  title = "Basilisk II", kind = 'software', year = 1997, status = 'alive, open source', confidence = 'medium',
  summary = "Christian Bauer's 68k Macintosh emulator: runs System 7 era Mac OS and with it HyperCard 2.x authoring on modern hardware — the runtime pole of preservation, in practice.",
  body = "Year is the first-release date as best recalled; confirm before relying on it. Pairs with SheepShaver (PowerPC) and Mini vMac (compact Macs).";

UPSERT artifact:sheepshaver SET
  title = "SheepShaver", kind = 'software', year = 1998, status = 'alive, open source', confidence = 'medium',
  summary = "PowerPC Mac OS emulator in the Basilisk II family: runs Mac OS 9 and therefore the last Classic-era HyperCard 2.4.1 workflow — the emulated stand-in for the PowerPC Macs that authoring now otherwise requires.",
  body = "Medium confidence on the year (originally a BeOS-era project, open-sourced later).";

UPSERT artifact:mini_vmac SET
  title = "Mini vMac", kind = 'software', year = 2004, status = 'alive, open source', confidence = 'medium',
  summary = "Compact-Mac emulator (Paul C. Pratt): boots original System 6/7 environments in which HyperCard 1.x and 2.x ran — a minimalist runtime pole.",
  body = "Medium confidence on the year.";

UPSERT artifact:internet_archive SET
  title = "Internet Archive", kind = 'archive', year = 1996, status = 'alive', confidence = 'high',
  summary = "Brewster Kahle's non-profit digital library: the institution behind the 2017 in-browser Macintosh emulation that let a stack be opened with a double-click — an archive as artifact, not just as a source citation.",
  body = "Included as an artifact so the corpus's archive kind is no longer empty and the runtime-preservation pole has an institutional actor.";

-- Artifacts: families the critique found missing -----------------------------
UPSERT artifact:filemaker SET
  title = "FileMaker", kind = 'software', year = 1985, status = 'alive', confidence = 'medium',
  summary = "The Mac flat-file database that became Claris's FileMaker II (1988): the structured-data exit ramp the corpus's own prose says serious HyperCard data users were steered to.",
  body = "Origins predate Claris (an independent Mac database from 1985); under Claris it was the commercial sibling of the HyperCard line, and under FileMaker Inc. it outlived it. Evidence node for no_structured_data's exit-ramp claim.";

UPSERT artifact:authorware SET
  title = "Authorware", kind = 'software', year = 1987, status = 'discontinued', confidence = 'medium',
  summary = "Flowchart-style multimedia and e-learning authoring tool (Authorware Inc., acquired by Macromedia in 1992): where much of the computer-based-training market HyperCard authors served ended up.",
  body = "Medium confidence on dates and on the migration claim, which rests on the Wikipedia HyperCard article naming Authorware among the systems HyperCard authors moved to; not independently re-checked.";

UPSERT artifact:flash SET
  title = "Macromedia Flash", kind = 'software', year = 1996, status = 'end-of-life 2020', confidence = 'medium',
  summary = "Animation and interactive-media authoring tool (FutureSplash, then Macromedia Flash, 1996) whose scripting this corpus routes down the Director/Lingo line: the web's mass-market authoring medium — and one that split authoring tool from Player exactly as HyperCard 2.2 did.",
  body = "The obvious media-authoring successor, and an instructive one: Flash reproduced the free-player/paid-authoring split (author_player_politics) and then died of platform politics and security, not of the idea.";

UPSERT artifact:scratch SET
  title = "Scratch", kind = 'software', year = 2007, status = 'alive', confidence = 'medium',
  summary = "MIT Media Lab's block-based programming environment for learners (2007): live-editable sprites and stages in a shared online community — the closest living answer to HyperCard's classroom role.",
  body = "Evidence for q_authoring_runtime_unity (the project is edited while it runs) and for the education domain's survival without HyperCard.";

UPSERT artifact:twine SET
  title = "Twine", kind = 'software', year = 2009, status = 'alive', confidence = 'medium',
  summary = "Chris Klimas's tool for authoring interactive fiction as linked passages, publishing each story as a single HTML file — the hypertext-fiction and games branch of the card idea, on the web.",
  body = "Single-file publishing (single_file_unity) plus link-as-structure, in an authoring tool that non-programmers actually use.";

UPSERT artifact:cosmic_osmo SET
  title = "Cosmic Osmo and the Worlds Beyond the Mackerel", kind = 'software', year = 1989, status = 'historical', confidence = 'medium',
  summary = "Cyan's first HyperCard world (1989): an explorable point-and-click universe, the direct pre-Myst prototype of the clickable-world genre.",
  body = "Shows Cyan's path — HyperCard stackware, then the commercial leap. Spelunx followed in 1991.";

UPSERT artifact:spelunx SET
  title = "The Spelunx and the Caves of Mr. Seudo", kind = 'software', year = 1991, status = 'historical', confidence = 'medium',
  summary = "Cyan's second HyperCard world (1991), aimed at children: more evidence that Myst was the third step of a HyperCard-hosted line of work, not a leap from nowhere.",
  body = "Medium confidence on the year.";

UPSERT artifact:afternoon SET
  title = "afternoon, a story", kind = 'publication', year = 1990, status = 'shipped', confidence = 'medium',
  summary = "Michael Joyce's hypertext novel (Eastgate, 1990; written 1987) on Storyspace: the canonical electronic-literature work of the card era and the clearest case of serious hypertext growing beside HyperCard, not inside it.",
  body = "Companion to Marble Springs on the Storyspace branch; the corpus's evidence for the literary-hypertext community that no_inline_text_links failed.";

-- Sources used for the corrections above -------------------------------------
UPSERT source:s_tidbits_1990 SET
  title = "TidBITS: HyperCard Bits & Pieces (5 Nov 1990)", url = "https://tidbits.com/1990/11/05/hypercard-bits-pieces/", kind = 'article', reliability = 'high',
  note = "Contemporaneous trade coverage (Adam Engst): treats the Apple-to-Claris transfer as done, describes the stripped-down bundled copy vs the $49 full Claris distribution, quotes Kevin Calhoun's version/System chart. Read in full 2026-11-06; gives no transfer date.";

UPSERT source:s_tidbits_1998 SET
  title = "TidBITS: Alas, HyperCard! (2 Nov 1998)", url = "https://tidbits.com/1998/11/02/alas-hypercard/", kind = 'article', reliability = 'medium',
  note = "Contemporaneous report (Geoff Duncan) of the end of HyperCard development: QTi slipping in 1997, the team moved to QuickTime in October 1998, Schiller reported (via 'reliable sources') announcing it internally, no official Apple statement; says HyperCard 'languished' at Claris and 'was returned to Apple'. Read in full 2026-11-06. Does not mention Crow or Calhoun.";

UPSERT source:s_tnw_ibooks_2012 SET
  title = "The Next Web: Apple announces iBooks Author (19 Jan 2012)", url = "https://thenextweb.com/news/apple-announces-ibook-author-a-mac-app-for-authoring-books", kind = 'article', reliability = 'medium',
  note = "News report dating the announcement to 19 January 2012. Located by search 2026-11-06; headline and date checked, not read in full.";

UPSERT source:s_hyperstudio_wiki SET
  title = "Wikipedia: HyperStudio", url = "https://en.wikipedia.org/wiki/HyperStudio", kind = 'article', reliability = 'medium',
  note = "Credits Roger Wagner with creating HyperStudio (1989, Apple IIGS); a Tech & Learning profile dates the release to May 1989 and names Roger Wagner Publishing. Located by search 2026-11-06; no source found linking Kevin Cuneo to the product.";

UPSERT source:s_supercard_wiki SET
  title = "Wikipedia: SuperCard", url = "https://en.wikipedia.org/wiki/SuperCard", kind = 'article', reliability = 'medium',
  note = "States SuperCard was created by Bill Appleton and published by Silicon Beach Software in 1989, combining ideas from World Builder, HyperCard, SuperPaint and the Mac UI. Located by search 2026-11-06.";

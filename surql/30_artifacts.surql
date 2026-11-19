-- Artifacts: software, formats, publications, archives.

UPSERT artifact:hypercard SET
  title = "HyperCard", kind = 'software', year = 1987, status = 'discontinued 2004', confidence = 'high',
  summary = "Apple's hypermedia card-stack environment (released 11 Aug 1987, Macworld Boston): stacks, cards, HyperTalk, paint, and a message path — free on every Macintosh, written in Apple Pascal by Bill Atkinson's team under creator code WILD.",
  body = "Announced at Macworld Boston on 11 August 1987 and bundled free with every Mac (existing owners could buy it for roughly $49–99), HyperCard sold 10,000 copies in its first week and 35,000 in its first month. It combined a hypermedia browser, a flat database, a paint program, and the HyperTalk language into one application whose unit of work — the stack — was a single self-contained file. Development began March 1985; the product was Atkinson's, renamed from WildCard for trademark reasons, written in Apple Pascal. Versions: 1.x (1987–89), 2.0 (1990, on-the-fly compiler and debugger), 2.1 (1991), 2.2 (Wikipedia says 1992; contemporary press reviews it in early 1994 — color via licensed Color Tools and AddMotion II), 2.3 (1996), 2.4 (April 1998), final 2.4.1 (1998); discontinued March 2004. A separate HyperCard IIGS (1991) came from a different group. HyperCard moved to Claris around 1990 (a TidBITS piece of November 1990 already treats the transfer as done) and later came back to Apple. The last officially supported environment for authoring original stacks is OS X 10.4 Classic on PowerPC; emulation and reimplementation (see the preservation thread) now carry authoring further.";

UPSERT artifact:hypercard_2 SET
  title = "HyperCard 2.0", kind = 'software', year = 1990, status = 'shipped', confidence = 'high',
  summary = "The Calhoun-era release (1990): an on-the-fly compiler, a debugger, scriptable menus, multiple stack windows and variable card sizes — HyperCard's coming-of-age as a software product.",
  body = "HyperCard 2.0 added the compiler that answered interpreter_performance (handlers compile on first execution and are cached in RAM), a debugger with checkpoints and stepping, user-definable menus, up to 18 stack and external windows, variable card sizes (64 to 1280 pixels, one size per stack) and 'group' text for clickable runs of text — loosening fixed_canvas as far as it ever moved. Wyldcard, the Java reimplementation, targets the later 2.4.1 rather than 2.0.";



UPSERT artifact:hypercard_3 SET
  title = "HyperCard 3.0 / QTi", kind = 'software', year = 1996, status = 'development ended 1998; cancelled 2000', confidence = 'high',
  summary = "The unreleased successor, an editor for interactive QuickTime movies (QTi, QuickTime Interactive). An alpha was shown at WWDC in May 1996 (Kevin Calhoun); development stalled in 1998 and the team was reassigned in 2000.",
  body = "QTi (QuickTime Interactive) was to extend QuickTime with interactivity and a low-level programming language, with HyperCard 3.0 as an editor for interactive QuickTime movies — stacks would become QuickTime movies playable in browsers via QuickTime (QuickTime VR was embeddable content, not the output format). The sequence has stages that a single date hides: the alpha was shown in 1996 (Calhoun); QTi slipped in 1997; per a contemporaneous TidBITS report (2 Nov 1998, attributing the announcement to Phil Schiller), the HyperCard team was moved to QuickTime work in late October 1998 with no further HyperCard development planned; Wikipedia says the engineering team was reassigned in 2000 after Jobs decided to abandon the product, and that Calhoun and Crow left in 2001. This corpus dates the formal cancellation 2000. succession_failure's central exhibit.";

UPSERT artifact:hypercard_player SET
  title = "HyperCard Player", kind = 'software', year = 1992, status = 'shipped', confidence = 'high',
  summary = "The free, view-only HyperCard 2.1 Player that Apple began shipping with new Macs on 14 September 1992, when its licence from Claris to ship the full product was expiring: runs stacks, cannot edit or script them.",
  body = "The Player answered a real problem (distributing stacks to Macs without the full product) by dropping the authoring half of the founding premise. Wikipedia notes many users were upset; it is the template for every free-viewer/paid-editor split since.";

UPSERT artifact:sk8 SET
  title = "SK8", kind = 'software', year = 1995, status = 'never shipped; source released 1997', confidence = 'high',
  summary = "An Apple Advanced Technology Group authoring environment (Ruben Kleiman; developed about 1988–97), HyperTalk-influenced, written in Macintosh Common Lisp; never shipped as a product.",
  body = "SK8 was an object-oriented multimedia authoring environment from Apple's ATG. It was not released as a product; its Macintosh Common Lisp source was released to the public in 1997 (Wikipedia's infobox says open source; HyperCard's article says public domain). Earlier versions of this corpus attributed it to Atkinson's group; no source supports that.";





UPSERT artifact:compileit SET
  title = "CompileIt!", kind = 'software', year = 1989, status = 'shipped', confidence = 'medium',
  summary = "Tom Pittman's tool for compiling HyperTalk into native 68k XCMDs/XFCNs, eventually self-hosted (Pittman said he compiled it in itself) — the community's proof that the platform's escape hatch could grow a new platform.",
  body = "A compiler delivered as a HyperCard tool: authors wrote HyperTalk and produced native XCMDs/XFCNs. The release year (the corpus said 1989) was not verified. The interpreter_performance tension's most elegant workaround.";











UPSERT artifact:supercard SET
  title = "SuperCard", kind = 'software', year = 1989, status = 'discontinued', confidence = 'high',
  summary = "Bill Appleton's HyperCard-like environment with SuperTalk, from Silicon Beach Software (1989), later owned by Solutions Etcetera: color and richer windowing. Discontinued; the site was taken down in 2024.",
  body = "SuperCard was an early commercial reinterpretation of the HyperCard model, with its own language, SuperTalk. It is cited here as evidence that the environment's ideas were a design, not an Apple artifact. Last release 4.8.1 (2020); product discontinued after its co-owner died in 2024. (Earlier claims that it had real arrays are unverified.)";



UPSERT artifact:toolbook SET
  title = "ToolBook", kind = 'software', year = 1990, status = 'discontinued 2021', confidence = 'high',
  summary = "Asymetrix's Windows book/pages/OpenScript environment (first released 1990): HyperCard's metaphor set for Windows and corporate training, later SumTotal, which ended sales and support on 31 December 2021.",
  body = "Books, pages and OpenScript (an xTalk dialect) made corporate training content in the image of HyperCard. The diaspora's Windows branch; discontinued.";

UPSERT artifact:director SET
  title = "Macromedia Director / Lingo", kind = 'software', year = 1987, status = 'discontinued 2017', confidence = 'high',
  summary = "Director (1987, from Macromind's VideoWorks) with its Lingo scripting language (1988 per Wikipedia): a cast/score media environment whose scripting language Wikipedia lists among HyperTalk's influences. Adobe discontinued it in 2017.",
  body = "Lingo's syntax and event model are xTalk; Director became the CD-ROM era's authoring engine. The corpus routes hypertalk → Lingo → ActionScript as the language's most commercially consequential descendant line.";





UPSERT artifact:javascript SET
  title = "JavaScript", kind = 'language', year = 1995, status = 'alive', confidence = 'high',
  summary = "Brendan Eich's Netscape language (1995), which Wikipedia says was inspired by HyperTalk, among other influences: it runs the web's DOM event path for the audience HyperCard lost.",
  body = "JavaScript's lineage is mixed (Scheme-like semantics, Java-like syntax); the HyperTalk influence is reported but the extent of it (for example handler names) is not documented here. The corpus's evidence that HyperTalk's ideas outlived its platform, with the audience going to the web.";

UPSERT artifact:livecode SET
  title = "LiveCode", kind = 'software', year = 2001, status = 'commercial product continues; open-source edition ended 2021', confidence = 'high',
  summary = "MetaCard → Runtime Revolution → LiveCode: the main commercial xTalk lineage, whose language is a HyperTalk dialect and which can convert original HyperCard stacks on open (lossily). Its open-source Community edition was discontinued on 31 August 2021; OpenXTalk is a community fork of that edition.",
  body = "LiveCode converts HyperCard stacks when they are opened with go (the resource fork is dropped, report templates are not carried) and its language is a HyperTalk dialect with real arrays. The commercial product continues; the open branch lives on as OpenXTalk. The corpus's answer to 'could the language survive the platform?' — yes, as a developer tool; not as an end-user environment.";

UPSERT artifact:openxtalk SET
  title = "OpenXTalk", kind = 'software', year = 2021, status = 'beta', confidence = 'medium',
  summary = "A community fork of the legacy LiveCode Community Edition, begun after LiveCode Ltd dropped open-source builds in 2021; keeps the xTalk stack model and Transcript language in public hands. In beta as of 2026.",
  body = "OpenXTalk carries the stack/card/background/message-path model natively. It forks the last open-source LiveCode edition; whether it imports HyperCard stacks was not confirmed here. Beta releases (0.2.7 release candidates) as of 2026.";

UPSERT artifact:decker SET
  title = "Decker", kind = 'software', year = 2021, status = 'alive, MIT', confidence = 'high',
  summary = "John Earnest's HyperCard-inspired environment (beyondloom.com/decker; MIT): single-file .deck stacks, the Lil language (influenced by Q, array-oriented), a 1-bit aesthetic, and a Web-Decker build that is a single standalone HTML file.",
  body = "Decker is the corpus's proof of live continuity: cards, buttons, fields, sound and paint with an authoring-first philosophy, plus the two things HyperCard never shipped — arrays (via Lil) and a web build. Its community remains active in 2026 (a December 2026 'Deck-Month' jam is announced on its site). The first-release year (corpus says 2021) was not verified.";

UPSERT artifact:wyldcard SET
  title = "Wyldcard", kind = 'software', year = 2018, status = 'open source; activity unverified', confidence = 'medium',
  summary = "A Java/Swing clone of HyperCard and HyperTalk (MIT) that implements most of HyperTalk 2.4.1 and imports HyperCard 2.x stacks (not XCMDs); the first-release year and recent activity were not verified.",
  body = "Behavior-faithful reimplementation rather than reinterpretation — the corpus's fidelity-preservation pole, paired with Stacksmith's format-preservation pole and emulation's runtime pole. No tagged releases found.";

UPSERT artifact:stacksmith SET
  title = "Stacksmith + stackimport", kind = 'software', year = 2017, status = 'work in progress; activity unverified', confidence = 'medium',
  summary = "Uli Kusterer's work-in-progress macOS environment inspired by HyperCard, with its own scripting language; it uses the stackimport tool to import original HyperCard stacks.",
  body = "stackimport is a command-line tool that Stacksmith uses to import HyperCard stacks. The repository is marked work-in-progress; its release year and recent activity were not verified.";

UPSERT artifact:hypercardpreview SET
  title = "HyperCardPreview", kind = 'software', year = 2017, status = 'alive', confidence = 'medium',
  summary = "Pierre Lorenzi's macOS viewer for HyperCard stacks (Swift, MIT; first reported 2017, v1.4 of 2019 the latest listed): view-only, no scripting or editing, with a QuickLook plugin.",
  body = "Read-only viewing of original stacks on macOS; Lorenzi's format notes are a variant documentation source alongside hypercard.org. There is no iOS version.";



UPSERT artifact:myst SET
  title = "Myst", kind = 'software', year = 1993, status = 'shipped', confidence = 'high',
  summary = "Cyan's interactive island (1993): among the best-selling games of its decade; the original Mac release was built as HyperCard stacks, and it was reportedly bundled with some Apple Performa Macs — stackware's commercial shadow.",
  body = "The shipped 1993 Mac release was HyperCard: its scripts survive on the disc and are quoted verbatim in this corpus's code layer (71_hypertalk_code; every line was checked against the mystextract repository). Later ports, including Windows (1994), used other engines. Myst shows what the card model could carry commercially — and the displacement: the commercial reach came through ports that left HyperCard behind.";

UPSERT artifact:whole_earth_catalog SET
  title = "Electronic Whole Earth Catalog", kind = 'publication', year = 1989, status = 'shipped', confidence = 'high',
  summary = "The Electronic Whole Earth Catalog (1989, per the Internet Archive): 430 MB and over 9,000 pages as a single HyperCard stack on CD-ROM — the counterculture's index rebuilt on the card model.",
  body = "A hypermedia_publishing landmark and the monolithic_file tension's first famous case: the whole product was one 430 MB stack. (Claims that readers could extend it, or that it shipped with a HyperCard runtime, were not verified.)";









UPSERT artifact:oracle_card SET
  title = "Oracle Card / Oracle Media Objects", kind = 'software', year = 1992, status = 'shipped', confidence = 'medium',
  summary = "Oracle Card (c.1991–92), Oracle's redistribution of the Plus HyperCard-clone engine with SQL database connectivity, later renamed Oracle Media Objects: the database giant briefly building on a HyperCard-like surface.",
  body = "Wikipedia describes Oracle Media Objects as essentially a redistribution of the Plus runtime engine and as the first cross-platform HyperCard clone; it is not an XCMD (an Oracle XCMD for HyperCard existed separately, per Wikipedia's HyperCard article). Evidence that the lightweight_database domain was visible to the enterprise.";



UPSERT artifact:ibooks_author SET
  title = "iBooks Author", kind = 'software', year = 2012, status = 'discontinued 2020', confidence = 'high',
  summary = "Apple's book-authoring tool (announced 19 January 2012; discontinued 2020): drag-and-drop multi-touch books. Atkinson said in 2017 (as cited by Wikipedia) that it is 'the closest thing we have today to what HyperCard was'; its publishing terms replayed the free-authoring, gated-distribution politics.",
  body = "Authoring free, reading free, publishing gated to Apple's store — the author/Player politics again, inside Apple, twenty years later; last release 2.6.1 (2018), discontinued 2020. The corpus's evidence that the succession question was never resolved inside the company that owned it.";

UPSERT artifact:hypercard_org SET
  title = "hypercard.org (format documentation)", kind = 'website', status = 'alive', confidence = 'high',
  summary = "A HyperCard resource page that links a description of the stack file format on disk (plus a variant extended by Pierre Lorenzi) — the load-bearing documentation for stack preservation.",
  body = "hypercard.org's file-format page describes the general structure of a stack on disk (for example: card pictures have two 1-bit layers). Without such documentation there is no stackimport, no Stacksmith, no faithful reimplementation. The corpus's answer to q_preservation's fidelity question: document the format, and the format outlives every emulator. (The corpus's earlier site year of 2016 and its AddColor/PLTE claims could not be confirmed.)";

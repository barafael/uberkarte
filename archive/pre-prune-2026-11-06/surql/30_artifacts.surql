-- Artifacts: software, formats, publications, archives.

UPSERT artifact:hypercard SET
  title = "HyperCard", kind = 'software', year = 1987, status = 'discontinued 2004', confidence = 'high',
  summary = "Apple's hypermedia card-stack environment (released 11 Aug 1987, Macworld Boston): stacks, cards, HyperTalk, paint, and a message path — free on every Macintosh, written in Apple Pascal by Bill Atkinson's team under creator code WILD.",
  body = "Announced at Macworld Boston on 11 August 1987 and bundled free with every Mac (existing owners could buy it for roughly $49–99), HyperCard sold 10,000 copies in its first week and 35,000 in its first month. It combined a hypermedia browser, a flat database, a paint program, and the HyperTalk language into one application whose unit of work — the stack — was a single self-contained file. Development began March 1985; the product was Atkinson's, renamed from WildCard for trademark reasons, written in Apple Pascal. Time's Alsop speculated in 1987 that it might replace the Finder; Sculley called it Apple's most creative product since the Mac; Wozniak later ranked it the best program ever written for the machine. Versions: 1.x (1987–89), 2.0 (1990, compiler/debugger), 2.1 (1991, +IIGS-era work), 2.2 (1992, Claris, color bolt-ons, Player), 2.4 (1996), final 2.4.1 (1998); discontinued March 2004. HyperCard moved to Claris in 1990 (a TidBITS piece of November 1990 already treats the transfer as done) and later came back to Apple. The last officially supported environment for authoring original stacks is OS X 10.4 Classic on PowerPC; emulation and reimplementation (see the preservation thread) now carry authoring further.";

UPSERT artifact:hypercard_2 SET
  title = "HyperCard 2.0", kind = 'software', year = 1990, status = 'shipped', confidence = 'high',
  summary = "The Calhoun-era rewrite release (1990): JIT-style compiler, real debugger, menu and window objects, multiple windows — HyperCard's coming-of-age as a software product, and the last version made mostly at Apple.",
  body = "HyperCard 2.0 added the compiler that answered interpreter_performance, the debugger that admitted stacks were software, linkText/linkParts for text-region links, and window/menu objects with multiple windows — loosening fixed_canvas as far as it ever moved. It remains the reference implementation most revival projects (Wyldcard, Stacksmith) target.";

UPSERT artifact:hypercard_iigs SET
  title = "HyperCard IIGS", kind = 'software', year = 1991, status = 'shipped', confidence = 'high',
  summary = "The Apple IIGS port (1991): 4096-color palettes and IIGS sound, plus a translator stack to port Mac stacks — HyperCard's only official color platform and a quiet argument that the 1-bit model was a choice, not a law.",
  body = "Shipped for the GS line with color and sound the Mac version lacked until AddColor-era bolt-ons, and a translator stack to move Mac stacks across. Its existence is the corpus's cleanest evidence for the bolted_on_color tension being a platform-politics failure, not a technical one.";

UPSERT artifact:hypercard_3 SET
  title = "HyperCard 3.0 / QTi", kind = 'software', year = 1996, status = 'development ended 1998; cancelled 2000', confidence = 'high',
  summary = "The cancelled successor shown at WWDC 1996 (Dan Crow): color, internet connectivity, and stacks exported as QuickTime-Internet vr movies playable in a browser — HyperCard's own web bridge, built five years too late and killed by the company that needed it earlier.",
  body = "QTi (QuickTime-Internet) would have rendered stacks as QuickTime-vr movies embeddable in browsers, with HyperTalk running over the web — the Atkinson counterfactual ('could have been the first Web browser') made real in 1996. The sequence has two stages, which a single date hides: QTi slipped in 1997 (Apple shipped QuickTime 3 without it) and, per a contemporaneous TidBITS report (2 Nov 1998, attributing the announcement to Phil Schiller), the HyperCard team was moved to QuickTime work in late October 1998 with no further HyperCard development planned. This corpus dates the formal cancellation 2000 (not re-verified here). Jobs, then interim CEO, is reported to have disliked the product line; Calhoun and Crow left in 2001. succession_failure's central exhibit.";

UPSERT artifact:hypercard_player SET
  title = "HyperCard Player", kind = 'software', year = 1992, status = 'shipped', confidence = 'high',
  summary = "Claris's free, view-only runtime (with 2.2): runs stacks, cannot edit or script them — the moment reading and writing a stack became different products, and the user backlash that made author_player_politics a permanent platform lesson.",
  body = "Bundled free to run stacks while authoring moved behind the paid product, the Player answered a real problem (distributing stacks to Macs without HyperCard) by amputating the founding premise. Documented user backlash; the template for every free-viewer/paid-editor split since.";

UPSERT artifact:sk8 SET
  title = "SK8", kind = 'software', year = 1995, status = 'never shipped; public domain', confidence = 'high',
  summary = "Apple's object-oriented HyperTalk successor (mid-90s, Atkinson's group): never released, and donated to the public domain when the group closed — the strongest 'what if' artifact in the lineage.",
  body = "SK8 carried HyperTalk's descendant language into a modern object system with QuickTime-grade media. It shipped nothing; Apple released the code as public domain. Its people and ideas scattered into the very web tools (and later iBooks Author, which Atkinson is reported to have named as an heir) that replaced the environment it continued.";

UPSERT artifact:rolo SET
  title = "Rolo", kind = 'software', year = 1985, status = 'shipped', confidence = 'high',
  summary = "Atkinson's card-index contact manager (1985) for the Mac — the direct ancestor whose card metaphor, stack navigation, and find/sort machinery grew up into HyperCard.",
  body = "Rolo's cards, stacks of them, and search were the seed object model; HyperCard generalized Rolo's cards into a universal medium. Atkinson's earlier BBS-automation work (the phone-modem stacks that became part of the Hyperdialer lineage) fed the same design instinct.";

UPSERT artifact:macpaint SET
  title = "MacPaint", kind = 'software', year = 1984, status = 'shipped', confidence = 'high',
  summary = "Atkinson's 1-bit paint program (1984) whose tools, canvas size, and bitmapped aesthetic HyperCard inherited wholesale — the tool palette is MacPaint's, and the card is a Paint canvas that learned to navigate.",
  body = "The 512x342 canvas, pencil/brush/bucket, and the whole handmade-bitmap look of stackware are MacPaint's inheritance. The corpus uses macpaint to date fixed_canvas to hardware, not ideology: the card was the screen because the screen was Paint's page.";

UPSERT artifact:compileit SET
  title = "CompileIt!", kind = 'software', year = 1989, status = 'shipped', confidence = 'high',
  summary = "Tom Pittman's 68k compiler that ran inside HyperCard as an XCMD and compiled itself — the community's proof that the platform's escape hatch could grow a new platform.",
  body = "A bootstrapping 680x0 assembler/compiler delivered as a HyperCard extension: authors wrote and compiled native code from inside stacks. The corpus's favorite single artifact of stackware ingenuity, and the interpreter_performance tension's most elegant workaround.";

UPSERT artifact:guide SET
  title = "Guide", kind = 'software', year = 1982, status = 'shipped', confidence = 'high',
  summary = "Donald Brown's Guide (written at Apple 1982, shipped by OWL): the first commercial hypertext application — conditional, computed links in a portable runtime — and the documented direct ancestor of HyperCard.",
  body = "Guide's link-typing and its cross-platform VM influenced Atkinson directly; HyperCard's simplicity is partly a reaction to Guide's link-typing complexity. The lineage thread runs through this node with high confidence.";

UPSERT artifact:storyspace SET
  title = "Storyspace", kind = 'software', year = 1987, status = 'shipped', confidence = 'high',
  summary = "Bolter & Joyce's writing environment (1987): maps of protected, densely linked prose — hypertext fiction's authoring tool, HyperCard's serious-scholar sibling and Nielsen's preferred answer to the link problem.",
  body = "Contemporaneous with HyperCard and aimed at authorship of non-linear texts with visible structure (maps, guards, links as objects). The corpus contrasts it with HyperCard in the no_inline_text_links and who_is_the_author discussions: same year, opposite bets on who the author is.";

UPSERT artifact:note_cards SET
  title = "NoteCards", kind = 'software', year = 1983, status = 'shipped (research)', confidence = 'high',
  summary = "Xerox PARC's NoteCards (1983): card-based hypertext with typed links and a browser — the research-system ancestor that put 'card' in HyperCard's vocabulary.",
  body = "PARC's study of how researchers take notes produced the card metaphor, thumbnail browsing, and link types. HyperCard took the card and dropped the typing — the corpus's cleanest example of simplification-as-distribution-strategy.";

UPSERT artifact:intermedia SET
  title = "Intermedia", kind = 'software', year = 1986, status = 'shipped', confidence = 'medium',
  summary = "Brown's post-Guide system (Brown University / Knowledge Index): bidirectional, typed links over compound documents — the scholarly-hypertext branch the web's addressing model eventually buried.",
  body = "Intermedia carried Guide's lineage toward two-way linking and multi-user authoring — the features HyperCard skipped. Medium confidence on some design details from the sources gathered here; the lineage claim itself is well attested.";

UPSERT artifact:zoomracks SET
  title = "ZoomRacks", kind = 'software', year = 1983, status = 'shipped; lawsuit', confidence = 'medium',
  summary = "Meridian's card-stack outliner (1983), sued by Apple over stack/card look-and-feel — the legal boundary Apple drew around the card metaphor before HyperCard even shipped, and settled.",
  body = "The lawsuit (settled) is the corpus's evidence that Apple understood the card-stack UI as proprietary capital early. Medium confidence on specifics; the case is documented in hypertext histories.";

UPSERT artifact:supercard SET
  title = "SuperCard", kind = 'software', year = 1989, status = 'shipped', confidence = 'high',
  summary = "Bill Appleton's HyperCard clone with SuperTalk (1989, Silicon Beach Software): color, resizable windows, real arrays — the first proof that HyperCard's ideas were a design, not an Apple artifact, and the xTalk diaspora's founding product.",
  body = "Appleton, a Silicon Beach developer (World Builder, 1986), built SuperCard from HyperCard, SuperPaint and the Mac interface; it shipped the features users demanded (color, arrays, windows) as a paid developer tool — establishing the pattern (clone the environment, professionalize the audience) that LiveCode later completed.";

UPSERT artifact:hyperstudio SET
  title = "HyperStudio", kind = 'software', year = 1989, status = 'shipped', confidence = 'high',
  summary = "Roger Wagner's education-market HyperCard-like (1989, Roger Wagner Publishing; Apple IIGS first, later Mac and Windows): stacks, buttons, HyperCard-style authoring for classrooms — end-user programming as a school product.",
  body = "HyperStudio sold the card-stack authoring model into schools throughout the 90s, including as a HyperCard stack translator on the IIGS. The corpus's evidence that the education domain outlived the platform by a decade.";

UPSERT artifact:toolbook SET
  title = "ToolBook", kind = 'software', year = 1988, status = 'shipped', confidence = 'high',
  summary = "Asymetrix's Windows book/pages/OpenScript environment (1988): HyperCard's metaphor set ported to the enterprise — the corporate twin, later Novell/SumTotal.",
  body = "Books, pages, and OpenScript (an xTalk dialect) made corporate CBT in the image of HyperCard. The diaspora's Windows branch; medium-to-high confidence on dates.";

UPSERT artifact:director SET
  title = "Macromedia Director / Lingo", kind = 'software', year = 1987, status = 'shipped', confidence = 'high',
  summary = "Director (1987, from Macromind's VideoWorks) with Lingo (1990): the cast/score media environment whose scripting language is HyperTalk's child — and via ActionScript, Flash's, and the web's.",
  body = "Lingo's syntax and event model are xTalk; Director became the CD-ROM era's authoring engine. The corpus routes hypertalk → Lingo → ActionScript as the language's most commercially consequential descendant line.";

UPSERT artifact:viawww SET
  title = "ViolaWWW", kind = 'software', year = 1992, status = 'historical', confidence = 'high',
  summary = "Pei-Yuan Wei's browser (1992), explicitly modeled on HyperCard: inline media, scripting, and the pointing-hand link cursor — the physical carrier of HyperCard's affordances into the web.",
  body = "An early graphical browser (1992), written at Berkeley in a HyperCard-flavored scripting language and, by common account, the first with scripting, stylesheets and tables; popular in Unix and CERN circles, while Mosaic (1993) is usually credited with popularizing the graphical web; Wei has said he took the concepts from HyperCard. When Netscape arrived, the affordances stayed and the tool died — displacement in miniature.";

UPSERT artifact:world_wide_web SET
  title = "The World Wide Web", kind = 'software', year = 1990, status = 'alive', confidence = 'high',
  summary = "Berners-Lee and Cailliau's CERN system (1989–91): URI + HTTP + HTML — the addressing-and-linking platform that resolved HyperCard's two structural gaps and displaced its environment.",
  body = "Designed in 1989, running by Christmas 1990, public 1991, browser-warm by 1993. The corpus treats the Web as the counterfactual twin: same decade, same card/page intuition, opposite decisions about where the author lives (server vs. card).";

UPSERT artifact:javascript SET
  title = "JavaScript", kind = 'language', year = 1995, status = 'alive', confidence = 'high',
  summary = "Eich's Netscape language (1995), designed under explicit HyperTalk influence: the message box's descendant syntax running the web's message path (DOM events) for the audience HyperCard lost.",
  body = "Ten days of design, Scheme semantics, HyperTalk surface (onmouseup and friends), Java costume. The corpus's strongest living evidence that HyperTalk's language design was right — and that its platform's death transferred its audience, not its ideas, to a successor.";

UPSERT artifact:livecode SET
  title = "LiveCode", kind = 'software', year = 2001, status = 'alive (OpenXTalk fork)', confidence = 'high',
  summary = "MetaCard → Runtime Revolution → LiveCode → OpenXTalk: the runnable xTalk lineage that imports original HyperCard stacks and still speaks Transcript (revTalk/MetaTalk) — HyperTalk's strongest surviving descendant.",
  body = "LiveCode's IDE imports HyperCard stacks (cards, backgrounds, scripts translated) and its language is a HyperTalk dialect with real arrays. OpenXTalk continues the open branch. The corpus's answer to 'could the language survive the platform?' — yes, as a developer tool; not as an end-user environment.";

UPSERT artifact:openxtalk SET
  title = "OpenXTalk", kind = 'software', year = 2019, status = 'alive', confidence = 'medium',
  summary = "The open-source community fork of LiveCode's Runtime Revolution lineage: keeping the xTalk stack model and Transcript language in public hands.",
  body = "Forked when LiveCode's stewardship wobbled; carries the stack/card/background/message-path model natively. Medium confidence on organizational details; the lineage itself is documented.";

UPSERT artifact:decker SET
  title = "Decker", kind = 'software', year = 2021, status = 'alive, MIT', confidence = 'high',
  summary = "John Earnest's HyperCard re-imagining (beyondloom.com/decker): single-file .deck stacks, the Lil language (HyperTalk's shape with Q-flavored arrays), 1-bit-plus aesthetic, whole stacks exporting to single .html files.",
  body = "Decker is the corpus's proof of live continuity: cards, buttons, fields, sound, paint, a message-path-ish event system, and an authoring-first philosophy, with the two things HyperCard never shipped — arrays and a web export. Its community makes new stackware in 2026. The Lil language's array bent is a direct reply to pseudo_arrays.";

UPSERT artifact:wyldcard SET
  title = "Wyldcard", kind = 'software', year = 2018, status = 'alive, open source', confidence = 'medium',
  summary = "A Java re-implementation of HyperCard 2.4.1 (name from WildCard): running original stacks on modern machines with the original behavior as the spec.",
  body = "Behavior-faithful reimplementation rather than reinterpretation — the corpus's fidelity-preservation pole, paired with Stacksmith's format-preservation pole and emulation's runtime pole. Medium confidence on start date.";

UPSERT artifact:stacksmith SET
  title = "Stacksmith + stackimport", kind = 'software', year = 2017, status = 'alive', confidence = 'high',
  summary = "Uli Kusterer's toolchain: stackimport parses original HyperCard 2.x stack files; Stacksmith edits, runs, and re-authors them natively on macOS — preservation with a forward door.",
  body = "The serious path from archived .stack files to living, editable artifacts, built on the hypercard.org format documentation (with Pierre Lorenzi's variant notes). The corpus's primary tool for the preservation question's 'and then what?'.";

UPSERT artifact:hypercardpreview SET
  title = "HyperCardPreview", kind = 'software', year = 2020, status = 'alive', confidence = 'medium',
  summary = "Pierre Lorenzi's iOS/macOS stack viewer: original stacks readable on phones, with format research feeding the wider toolchain.",
  body = "Read-only viewing of original stacks on modern Apple hardware; Lorenzi's format documentation is a variant source alongside hypercard.org. Medium confidence on dates.";

UPSERT artifact:cinsimp SET
  title = "CinsImp", kind = 'software', year = 2019, status = 'alive', confidence = 'medium',
  summary = "Joshua Hawcroft's HyperCard-compatible environment for Apple platforms: original stacks run, and new ones can be authored in HyperTalk again.",
  body = "Aims at authoring-capable compatibility (not just playback) on modern macOS/iOS. Medium confidence on maturity; included as evidence the authoring half is being rebuilt, not just the viewer half.";

UPSERT artifact:myst SET
  title = "Myst", kind = 'software', year = 1993, status = 'shipped', confidence = 'high',
  summary = "Cyan's interactive island (1993): among the best-selling games of its decade; the original Mac release was built as HyperCard stacks (QuickTime movies, HyperCard navigation, XCMDs such as Hypertint for color), and it was reportedly bundled with some Apple Performa Macs — stackware's commercial shadow.",
  body = "The Robyn and Rand Miller prototype grew in the HyperCard idiom, and the shipped 1993 Mac release still was HyperCard: its scripts survive on the disc and are quoted verbatim in this corpus's code layer (71_hypertalk_code). Later ports, including Windows, reimplemented the card-and-link structure on other engines. Myst is the corpus's proof of what the card model could carry commercially — and of the displacement: the commercial reach came through ports that left HyperCard behind.";

UPSERT artifact:whole_earth_catalog SET
  title = "Electronic Whole Earth Catalog", kind = 'publication', year = 1990, status = 'shipped', confidence = 'high',
  summary = "The 1990 CD-ROM (430 MB, 9000+ pages) of the Whole Earth catalog/Review as a HyperCard stack: the counterculture's index rebuilt on the card model — the era's flagship stack.",
  body = "Shipped on CD with a HyperCard runtime; readers could browse, search, and extend the catalog — a hypermedia_publishing landmark and the monolithic_file tension's first famous victim (430 MB was the whole product).";

UPSERT artifact:hard_days_night SET
  title = "A Hard Day's Night (Voyager Expanded Book)", kind = 'publication', year = 1990, status = 'shipped', confidence = 'high',
  summary = "Voyager's Expanded Book of the Beatles film (1990): the CD-ROM book you could annotate, rewire, and extend — the 'book as stack' thesis at its most beloved.",
  body = "Voyager's Expanded Books line (with Beethoven's 9th among them) shipped author-capable hypermedia books on HyperCard-era assumptions; the Player split quietly ended the author-capable part.";

UPSERT artifact:marble_springs SET
  title = "Marble Springs", kind = 'publication', year = 1993, status = 'shipped', confidence = 'high',
  summary = "Deena Larsen's hypertext fiction (1993): a ghost town of linked cards — Storyspace-era craft, the literary branch of hypertext that grew beside HyperCard rather than inside it.",
  body = "Larsen's genealogies-of-ghosts novel was built in Storyspace (Eastgate) and published 1993; it is the corpus's canonical literary hypertext of the card era and a deliberate contrast case: same card intuition, different platform, protected authorship instead of open authoring.";

UPSERT artifact:the_manhole SET
  title = "The Manhole", kind = 'software', year = 1987, status = 'shipped', confidence = 'high',
  summary = "Broderbund's interactive world (1987, from MindCage): a HyperCard-adjacent click-world that won early Macworld awards — the toy/ambient branch of the card medium.",
  body = "Cyan-adjacent lineage (Rand and Robyn Miller worked in this space before Myst); the corpus uses it to date the 'clickable world' genre to the stack era.";

UPSERT artifact:focal_point SET
  title = "Focal Point / Reports for HyperCard", kind = 'software', year = 1988, status = 'shipped', confidence = 'medium',
  summary = "Focal Point (Activision→Mediagenic): a HyperCard-like development environment and the Reports add-on — the professional-tools ecosystem that grew around the stack.",
  body = "Shipped by a game company that believed the card model was a development platform; Reports For HyperCard added report generation. Medium confidence on corporate history details.";

UPSERT artifact:oracle_card SET
  title = "Oracle Card / Oracle Media Objects", kind = 'software', year = 1992, status = 'shipped', confidence = 'medium',
  summary = "Oracle's XCMD (Oracle Card, 1992) that grew into Oracle Media Objects: the database giant briefly building on HyperCard's surface — the enterprise noticing the folk medium.",
  body = "Started as an SQL-connecting XCMD, became a full xTalk-ish multimedia development tool. The corpus's evidence that the lightweight_database domain's exit ramp was visible to everyone, including Oracle.";

UPSERT artifact:beehive SET
  title = "BeeHive", kind = 'hardware', year = 1991, status = 'historical', confidence = 'medium',
  summary = "BeeHive Technologies' ADB-input hardware + HyperCard control layer: stacks driving the whole Mac (keyboard/mouse level) — home automation and kiosk control from a card.",
  body = "Let stacks act as the Mac's front panel for appliances and lab gear via ADB-level control. Medium confidence on product details; the NFB CinéRobothèque (a stack driving a robot arm cataloguing film) is the high-confidence sibling exhibit for stacks-as-control-planes.";

UPSERT artifact:ibooks_author SET
  title = "iBooks Author", kind = 'software', year = 2012, status = 'discontinued 2020', confidence = 'high',
  summary = "Apple's book-authoring tool (announced January 2012): drag-drop multi-touch books on a card-like canvas — Atkinson is reportedly on record naming it HyperCard's closest modern heir (a secondhand report; date and wording unverified here); its Player/publisher split replayed 1992 exactly.",
  body = "Authoring free, reading free, publishing gated — the author/Player politics again, inside Apple, twenty years later, and discontinued in 2020. The corpus's proof that the succession question was never resolved inside the company that owned it.";

UPSERT artifact:hypercard_org SET
  title = "hypercard.org (format documentation)", kind = 'website', year = 2016, status = 'alive', confidence = 'high',
  summary = "The community's reverse-engineered documentation of the stack file format — cards, resources, AddColor/PLTE encodings, with Lorenzi's variant notes — the load-bearing artifact of stack preservation.",
  body = "Without this documentation there is no stackimport, no Stacksmith, no faithful Wyldcard. The corpus's answer to q_preservation's fidelity question: document the format, and the formats outlive every emulator.";

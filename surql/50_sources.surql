-- Sources: what this corpus is built on, with reliability grades.

UPSERT source:s_wiki_hypercard SET
  title = "Wikipedia: HyperCard", url = "https://en.wikipedia.org/wiki/HyperCard", kind = 'article', reliability = 'medium',
  note = "Backbone for dates, versions, Claris history, QTi, discontinuation, similar-systems list, Nielsen critique, macro viruses, Myst/Performa, IIGS, SK8 public domain. Cross-checked against other sources for load-bearing claims.";

UPSERT source:s_wiki_hypertalk SET
  title = "Wikipedia: HyperTalk", url = "https://en.wikipedia.org/wiki/Hypertalk", kind = 'article', reliability = 'medium',
  note = "Language design, Winkler, influences (JavaScript/AppleScript/Lingo), xTalk family, HyperTalk Standards Committee (1989), 2.0 compiler, antlr grammar basis.";

UPSERT source:s_wired_2002 SET
  title = "Leander Kahney, Wired (14 Aug 2002): article on Atkinson's HyperCard regret (headline not verified)", url = "https://www.mactech.com/2002/08/14/hypercard-as-a-web-browser", kind = 'article', reliability = 'medium',
  note = "A news article, not an interview. Wired's own page could not be fetched; the URL here is a MacTech item of the same day that quotes it. Confirmed quotations: 'I grew up in a box-centric culture at Apple. If I'd grown up in a network-centric culture, like Sun, HyperCard might have been the first Web browser.' The earlier note's claims about Sculley and the WILD/WildCard naming were not found in the mirrored text; Wikipedia is the source used for the free-on-every-Mac condition.";

UPSERT source:s_beyondloom SET
  title = "John Earnest — Decker (beyondloom.com/decker)", url = "https://beyondloom.com/decker/", kind = 'article', reliability = 'high',
  note = "Design essays behind Decker and a celebrated HyperCard appreciation/analysis; the corpus's live-systems anchor and a strong secondary reading of the message path and stack unity.";

UPSERT source:s_hypercard_org SET
  title = "hypercard.org — stack file format documentation", url = "https://hypercard.org/", kind = "docs", reliability = 'medium',
  note = "A HyperCard resource page linking a description of the stack format on disk (and a variant extended by Pierre Lorenzi). Reverse-engineered documentation underlies stackimport and similar tools. The earlier note's AddColor/PLTE claim was not confirmed.";

UPSERT source:s_loc_stack SET
  title = "Library of Congress, The Signal: 'All Hyped Up for HyperCard: Further Adventures with an Apple Legacy Format' (July 2021)", kind = 'docs', reliability = 'medium',
  note = "States that Apple never made the stack specification available and that the authors plan a format description for the Library's Sustainability of Digital Formats site; no completed LoC format page was found.", url = "https://blogs.loc.gov/thesignal/2021/07/all-hyped-up-for-hypercard-further-adventures-with-an-apple-legacy-format/";



UPSERT source:s_dobbs_xcmd SET
  title = "Dr. Dobb's Journal — HyperCard/XCMD coverage", kind = 'article', reliability = 'medium',
  note = "Technical coverage of external commands and the compiled-code escape hatch; corroborates interpreter_performance workarounds.";

UPSERT source:s_goodman_handbook SET
  title = "Danny Goodman — The Complete HyperTalk Handbook", kind = 'book', reliability = 'high',
  note = "Widely cited as a best-seller of its era (superlative unverified); also the HyperTalk Reference stack itself — evidence for learnability_first and the self-documenting medium.";



UPSERT source:s_victor_learnable SET
  title = "Bret Victor — Learnable Programming", url = "https://learnableprogramming.com", kind = 'article', reliability = 'medium',
  note = "Essay on explorable, feedback-rich programming environments. The earlier note said it cites HyperTalk; that was not verified (the page was not fetched).";

UPSERT source:s_nielsen_critique SET
  title = "Jakob Nielsen — HyperCard usability critiques (late 1980s–early 1990s)", kind = 'article', reliability = 'low',
  note = "Nielsen's remark that HyperCard's links 'started from regions on a card, not text objects' is quoted in Wikipedia's HyperCard article; the original text was not retrieved.";



UPSERT source:s_hypercard_center SET
  title = "hypercard.center — web edition of the HyperCard HyperTalk Reference", kind = 'archive_site', reliability = 'medium',
  note = "A web port of the HyperTalk Reference and Help Extras stacks with search; begun March 2022, active in 2026. Not a stackware community hub.";





UPSERT source:s_internet_archive_hc SET
  title = "Internet Archive blog: HyperCard on the Archive — celebrating 30 years of HyperCard (11 Aug 2017)", url = "https://blog.archive.org/2017/08/11/hypercard-on-the-archive-celebrating-30-years-of-hypercard/", kind = 'archive_site', reliability = 'high',
  note = "2017 in-browser emulation of HyperCard; runtime-preservation pole for q_preservation.";

UPSERT source:s_stack_format_lorenzi SET
  title = "Pierre Lorenzi — HyperCard file format notes (variant)", kind = 'docs', reliability = 'medium',
  note = "Independent format documentation used by HyperCardPreview; cross-checks hypercard.org.";







UPSERT source:s_hc_script_guide SET
  title = "Apple — HyperCard Script Language Guide: The HyperTalk Language (HyperCard 2.2 edition)", url = "https://cancel.fm/stuff/share/HyperCard_Script_Language_Guide_1.pdf", kind = 'docs', reliability = 'high',
  note = "Apple's reference for HyperCard 2.2: containers and values, system-message tables, commands, functions, properties, operator precedence (table 7-2), constants, and a categorized index of every command, function, property and system message. The PDF at this URL (a copy on a third-party host, not Apple's server) has a usable text layer. On 2026-11-06 every keyword node was compared against the guide's index for word, category and one-line description; nodes that agreed carry confidence high. Not checked: every parameter form of every command.";

UPSERT source:s_jaed_tricks SET
  title = "Jeanne A. E. DeVoto — Tricks of the HyperTalk Masters: Techniques for Stack Development (1992)", url = "http://www.jaedworks.com/hypercard/HT-Masters/scripting.html", kind = 'book', reliability = 'high',
  note = "Practitioner reference: message-path edge cases, the rules for the, it/result/target/me, chunk-based arrays, doMenu traps, style. Code excerpts were compared line by line with the cited page on 2026-11-06; a few lines carry corpus-added comments.";

UPSERT source:s_mystextract SET
  title = "Andrew Plotkin — mystextract: Myst (1993) HyperCard script dumps", url = "https://github.com/erkyrath/mystextract", kind = 'repo', reliability = 'high',
  note = "HyperTalk script dumps from the 1993 Mac release of Myst, extracted from its HyperCard stacks. All 66 lines of the four Myst samples in the code layer were found verbatim in the repository's scripts (checked 2026-11-06). Copyright is Cyan's; the repository page did not state a licence.";

UPSERT source:s_antlr_hypertalk SET
  title = "grammars-v4 HyperTalk grammar (Matt DeFano)", url = "https://github.com/antlr/grammars-v4/tree/master/hypertalk", kind = 'repo', reliability = 'medium',
  note = "ANTLR4 grammar based on HyperTalk 2.4.1, part of the WyldCard project; a machine-readable copy of the keyword inventory. Licence not confirmed here.";
UPSERT source:s_hc_guide_archive SET title = "HyperCard Script Language Guide (Claris/Apple, HyperCard 2.0/2.1 era, 1989–90) — archive.org full text", url = "https://archive.org/download/manualsplus_12453/manualsplus_12453_djvu.txt", kind = 'docs', reliability = 'high', note = "Primary reference read for the 2026-11-06 fact-check of the object model: message path, unhandled-message behaviour, containers, `it`, tools, marking, card sizes, group text. Covers 2.0/2.1; features added in 2.1/2.2 may be absent. OCR text, partly garbled in tables.";
UPSERT source:s_hc_users_guide_1987 SET title = "HyperCard User's Guide (Apple, 1987) — archive.org full text", url = "https://archive.org/download/apple-hypercard-users-guide-1987/HyperCard_Users_Guide_1987_djvu.txt", kind = 'docs', reliability = 'high', note = "Primary reference: Home stack and Home card, Recent (last 42 cards), user levels. Read for the 2026-11-06 fact-check.";
UPSERT source:s_tidbits_1992 SET title = "TidBITS: HyperCard Player Bundled with Macs (21 Sep 1992)", url = "https://tidbits.com/1992/09/21/hypercard-player-bundled-with-macs/", kind = 'article', reliability = 'medium', note = "Contemporaneous report: from 14 Sep 1992 Apple ships new Macs (Performas first) with a run-only 'HyperCard 2.1 Player'; Apple's licence from Claris to distribute HyperCard expires 30 Sep 1992. Read 2026-11-06.";
UPSERT source:s_tidbits_1994 SET title = "TidBITS: HyperCard 2.2: The Great Becomes Greater (14 Feb 1994)", url = "https://tidbits.com/1994/02/14/hypercard-2-2-the-great-becomes-greater/", kind = 'article', reliability = 'medium', note = "Contemporaneous review of HyperCard 2.2 (so 2.2 dates to about late 1993/early 1994, not 1992 as Wikipedia says): color via PICTs and AddMotion II, the Movie XCMD, AppleScript support, 'HyperCard went to Claris and nearly died, then came back to Apple'. Read 2026-11-06.";

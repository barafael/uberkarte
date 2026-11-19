-- Sources: what this corpus is built on, with reliability grades.

UPSERT source:s_wiki_hypercard SET
  title = "Wikipedia: HyperCard", url = "https://en.wikipedia.org/wiki/HyperCard", kind = 'article', reliability = 'medium',
  note = "Backbone for dates, versions, Claris history, QTi, discontinuation, similar-systems list, Nielsen critique, macro viruses, Myst/Performa, IIGS, SK8 public domain. Cross-checked against other sources for load-bearing claims.";

UPSERT source:s_wiki_hypertalk SET
  title = "Wikipedia: HyperTalk", url = "https://en.wikipedia.org/wiki/Hypertalk", kind = 'article', reliability = 'medium',
  note = "Language design, Winkler, influences (JavaScript/AppleScript/Lingo), xTalk family, HyperTalk Standards Committee (1989), 2.0 compiler, antlr grammar basis.";

UPSERT source:s_wired_2002 SET
  title = "Wired 10.08: The God & Machine of HyperCard (Atkinson interview)", url = "https://www.wired.com/2002/08/god-machine/", kind = 'interview', reliability = 'high',
  note = "Primary testimony: free-on-every-Mac condition, box-centric regret, 'could have been the first Web browser', Sculley-era context, WILD/WildCard naming.";

UPSERT source:s_beyondloom SET
  title = "John Earnest — Decker (beyondloom.com/decker)", url = "https://beyondloom.com/decker/", kind = 'article', reliability = 'high',
  note = "Design essays behind Decker and a celebrated HyperCard appreciation/analysis; the corpus's live-systems anchor and a strong secondary reading of the message path and stack unity.";

UPSERT source:s_hypercard_org SET
  title = "hypercard.org — stack file format documentation", url = "https://hypercard.org/", kind = "docs", reliability = 'high',
  note = "Community reverse-engineering of the stack format (cards, resources, AddColor/PLTE); basis for stackimport/Stacksmith; Lorenzi's variant notes hosted alongside.";

UPSERT source:s_loc_stack SET
  title = "Library of Congress — HyperCard stack format (Sustainability/Digital Preservation)", kind = 'docs', reliability = 'high',
  note = "LoC sustainability assessment treating the stack file as the preservable object; format documentation status; grounds the q_preservation thread.";

UPSERT source:s_mactech_xcmd SET
  title = "MacTech / Video & Computer Photography — XCMD columns", kind = 'article', reliability = 'medium',
  note = "Era coverage of the XCMD ecosystem (Oracle XCMD, HyperCom, CompileIt!, developer tooling); corroborates the plugin economy.";

UPSERT source:s_dobbs_xcmd SET
  title = "Dr. Dobb's Journal — HyperCard/XCMD coverage", kind = 'article', reliability = 'medium',
  note = "Technical coverage of external commands and the compiled-code escape hatch; corroborates interpreter_performance workarounds.";

UPSERT source:s_goodman_handbook SET
  title = "Danny Goodman — The Complete HyperTalk Handbook", kind = 'book', reliability = 'high',
  note = "Widely cited as a best-seller of its era (superlative unverified); also the HyperTalk Reference stack itself — evidence for learnability_first and the self-documenting medium.";

UPSERT source:s_whitehead SET
  title = "Whitehead — HyperCard and the Web (comparative paper)", kind = 'paper', reliability = 'medium',
  note = "Source for the structural gap: HyperCard stacks cannot be linked-to from other stacks; HyperCard-vs-web link models.";

UPSERT source:s_victor_learnable SET
  title = "Bret Victor — Learnable Programming", url = "https://learnableprogramming.com", kind = 'article', reliability = 'high',
  note = "Cites HyperTalk as a lost ancestor of explorable code environments; the modern research lineage for learnability_first.";

UPSERT source:s_nielsen_critique SET
  title = "Jakob Nielsen — HyperCard usability critiques (late 1980s–early 1990s)", kind = 'article', reliability = 'medium',
  note = "Contemporary critique of region-based linking vs inline text links; grounds no_inline_text_links.";

UPSERT source:s_github_hypertalk SET
  title = "GitHub search: hypertalk revival repositories", url = "https://github.com/search?q=hypertalk", kind = 'repo', reliability = 'medium',
  note = "Verified 2026-11: wyldcard, stacksmith/stackimport, openxtalk, hypercardpreview, cinsimp, hc2html, openxion, hyperjs, tilestack, novocard, dataorganizer/baycard. No 'HyperTalk 2.0' project found (claim dropped).";

UPSERT source:s_hypercard_center SET
  title = "hypercard.center — community revival hub", kind = 'archive_site', reliability = 'medium',
  note = "Current revival community: stacks, tools, discussion; corroborates active stackware in 2024–2026.";

UPSERT source:s_creysoft_xtalk SET
  title = "creysoft.com — xTalk family archive", kind = 'archive_site', reliability = 'medium',
  note = "Long-running catalog of xTalk languages and HyperCard-like systems; corroborates the diaspora list (SuperCard, ToolBook, Lingo, MetaCard, SenseTalk, Plus/WinPlus, mTropolis, Apple Media Tool, Stagecast).";

UPSERT source:s_pantechnicon SET
  title = "Pantechnicon (UQAM) — HyperCard collections", url = "https://pantechnicon.org", kind = 'archive_site', reliability = 'medium',
  note = "Academic archive of stacks and HyperCard-era software (UQAM); with jaedworks and folkstream, the corpus's evidence for institutional stack preservation.";

UPSERT source:s_internet_archive_hc SET
  title = "Internet Archive — Macintosh emulation in browser", url = "https://archive.org/details/softwarelibrary_macs", kind = 'archive_site', reliability = 'high',
  note = "2017 in-browser emulation of HyperCard; runtime-preservation pole for q_preservation.";

UPSERT source:s_stack_format_lorenzi SET
  title = "Pierre Lorenzi — HyperCard file format notes (variant)", kind = 'docs', reliability = 'medium',
  note = "Independent format documentation used by HyperCardPreview; cross-checks hypercard.org.";

UPSERT source:s_alsop_time_1987 SET
  title = "Richard Alsop in Time (1987) on HyperCard", kind = 'article', reliability = 'medium',
  note = "Contemporary press: the Finder-replacement speculation; Sculley-era enthusiasm. Reported via secondary compilation in s_wiki_hypercard.";

UPSERT source:s_wozniak_2001 SET
  title = "Wozniak on HyperCard (2001 quote)", kind = 'article', reliability = 'medium',
  note = "'Best program ever written' attribution as recorded in s_wiki_hypercard's reception section.";

UPSERT source:s_cine_robotech SET
  title = "NFB — CinéRobothèque (HyperCard-driven robot arm)", kind = 'article', reliability = 'medium',
  note = "National Film Board of Canada's stack-driven catalogue robot; documented in HyperCard community histories; grounds lightweight_database/art_exhibition domains.";

UPSERT source:s_hc_script_guide SET
  title = "Apple — HyperCard Script Language Guide: The HyperTalk Language (1996)", url = "https://cancel.fm/stuff/share/HyperCard_Script_Language_Guide_1.pdf", kind = 'docs', reliability = 'high',
  note = "Official Apple reference for HyperCard 2.x: container/value model, system-message tables 8-1..8-4, commands, properties, operator precedence appendix E. The corpus's operational-semantics anchor.";

UPSERT source:s_jaed_tricks SET
  title = "Jeanne A. E. DeVoto — Tricks of the HyperTalk Masters: Techniques for Stack Development (1992)", url = "http://www.jaedworks.com/hypercard/HT-Masters/scripting.html", kind = 'book', reliability = 'high',
  note = "Practitioner reference: message-path edge cases, the rules for the, it/result/target/me zoo, chunk-based arrays, recursion limits, doMenu traps, style. Code excerpts quoted with attribution.";

UPSERT source:s_mystextract SET
  title = "Andrew Plotkin — mystextract: Myst (1993) HyperCard script dumps", url = "https://github.com/erkyrath/mystextract", kind = 'repo', reliability = 'high',
  note = "Verbatim HyperTalk from a shipped 1993 bestseller, extracted from the ISO with a stack parser. Scripts (c) Cyan Inc.; dump used for study with attribution. Shows XCMD-heavy practice (Hypertint, MooV, DeCurse).";

UPSERT source:s_antlr_hypertalk SET
  title = "grammars-v4 HyperTalk grammar (Matt DeFano, MIT)", url = "https://github.com/antlr/grammars-v4/tree/master/hypertalk", kind = 'repo', reliability = 'medium',
  note = "Machine-readable ANTLR4 grammar: enumerates commands, functions, chunks, constants and operator precedence as parser rules. A second, executable copy of the keyword inventory; stricter than real HyperCard outside handlers.";

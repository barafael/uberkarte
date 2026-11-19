-- Narrative threads: ordered reading paths through the graph.

UPSERT thread:t_hypertext_lineage SET
  title = "The hypertext lineage",
  question = "How did the idea of linked documents reach HyperCard, and where did HyperCard pass it on?",
  body = "Bush's memex (1945) → Nelson's hypertext (1965) → Engelbart's demo (1968) → the research/commercial branch (Guide, NoteCards, Intermedia, Storyspace, ZoomRacks) → HyperCard's cheap mainstream subset (1987) → the affordance transfer through ViolaWWW to the browser, and the structure transfer through Cunningham to the wiki and through Obsidian/Roam back to the memex ideal. Reading the thread as edges: follow influenced out from memex; the path forks at hypercard into web (affordances) and xTalk (language) and preservation (format).";

UPSERT thread:t_web_before_the_web SET
  title = "The web that almost was",
  question = "In what precise sense was HyperCard a pre-web web, and what does Atkinson's regret actually claim?",
  body = "HyperCard had the audience, the links, the media, and the live console — but not addresses. The thread runs stack↔www (contrasts_with), region_link↔html_hyperlink (the link war), pointing_hand→viawww→world_wide_web (the affordance migration), message_box→javascript_dom (the console returns), and closes on e_atkinson_wired_2002 and web_displacement: the counterfactual stated by the designer himself.";

UPSERT thread:t_xtalk_diaspora SET
  title = "The xTalk diaspora",
  question = "When the platform died, what exactly survived of the language, and in whose hands?",
  body = "HyperTalk (Winkler, 1987) forks immediately: SuperTalk (Appleton's SuperCard), OpenScript (ToolBook), Lingo→ActionScript (Director), JavaScript (Eich, explicitly), AppleScript (inverted architecture), MetaTalk→revTalk→Transcript (MetaCard→LiveCode→OpenXTalk, the strongest line, still importing stacks), SenseTalk, SK8's unreleased heir. Decker's Lil is the return of the dialect to the environment. The diaspora's shared fact: every survivor became a developer tool; none remained an end-user environment.";

UPSERT thread:t_rise_and_fall SET
  title = "Rise, split, and end",
  question = "Did HyperCard die of its limits or of its ownership?",
  body = "Free on every Mac (1987) creates what was plausibly the largest end-user install base of its time — and no revenue line. 2.0 engineers it into a product (1990); the Claris transfer and Player split (1992) break the founding clause; color arrives bolted on (2.2); QTi proposes the web bridge five years late (1996); cancellation (2000) and discontinuation (2004) close it. The graph's verdict, as edges: every fall node is entered from business_model_clash or succession_failure, not from a technical tension.";

UPSERT thread:t_stackware_economy SET
  title = "The stackware economy",
  question = "What did a million free authors actually build, and what ended it?",
  body = "CompuServe's 300+ stacks (1988) and HyperHeaven; the Electronic Whole Earth Catalog and Voyager's Expanded Books; Myst's commercial shadow; Renault inventories and the NFB robot arm; classroom stacks and kiosk stacks; CompileIt! and the XCMD vendors. Ended by three edges at once: the Player split (authors demoted), the macro viruses (trust spent, 1991–93), and web displacement (audience moved). Decker's community is the folk line's living remnant.";

UPSERT thread:t_revival_preservation SET
  title = "Revival and preservation",
  question = "What is worth keeping of a dead environment, and through which layer — runtime, format, or behavior?",
  body = "Three poles: emulation (Internet Archive, 2017 — runtime), format documentation (hypercard.org + Lorenzi — the durable layer), and re-implementation (Wyldcard for behavior, Stacksmith for authoring-forward, HyperCardPreview for reach, CinsImp for live authoring, Decker for continuation). The LoC's stance and this corpus's: formats outlive emulators; behavior specs outlive both only if someone keeps running them.";

UPSERT thread:t_modernization_tensions SET
  title = "Tensions a modernization must answer",
  question = "Which HyperCard decisions were the design, which were 1987's invoice, and who pays it now?",
  body = "The corpus's tension/analog matrix, stated as the edge table has it. Tensions with at least one modern analog: no_networking→www, no_inline_text_links→html_hyperlink, no_structured_data→notion_airtable (and FileMaker, the historical exit ramp), single_user→local_first, monolithic_file→spa_routing, interpreter_performance→javascript_dom/wasm_plugins, platform_lockin→local_first/tiddlywiki/decker, author_player_politics→wiki/tiddlywiki, business_model_clash→low_code, and — partially — security_naivete→wasm_plugins (a permission-and-review regime around extension code, not a fix for executable documents). Tensions with no addresser, recorded as honest nulls rather than gaps: bolted_on_color, succession_failure, web_displacement (no analog repairs a lost race or a dead institution; the web is the thing that won). Separately, natural_language_syntax maps to ai_codegen, which addresses it. Questions are not tensions and carry no addressers by design: q_collaboration is raised by single_user and q_who_is_the_author by author_player_politics; q_authoring_runtime_unity is raised by author_player_politics, security_naivete and business_model_clash. The thread also holds the counter-case: anti_successor argues that the web won for good reasons and that a faithful successor would inherit HyperCard's failure modes — the position that lets the corpus's central premise lose.";

-- Membership (position = reading order in the thread) -----------------------
RELATE concept:memex->in_thread->thread:t_hypertext_lineage SET `position` = 1;
RELATE concept:hypertext->in_thread->thread:t_hypertext_lineage SET `position` = 2;
RELATE event:e_demo_1968->in_thread->thread:t_hypertext_lineage SET `position` = 3;
RELATE artifact:guide->in_thread->thread:t_hypertext_lineage SET `position` = 4;
RELATE artifact:note_cards->in_thread->thread:t_hypertext_lineage SET `position` = 5;
RELATE artifact:intermedia->in_thread->thread:t_hypertext_lineage SET `position` = 6;
RELATE artifact:storyspace->in_thread->thread:t_hypertext_lineage SET `position` = 7;
RELATE artifact:zoomracks->in_thread->thread:t_hypertext_lineage SET `position` = 8;
RELATE artifact:hypercard->in_thread->thread:t_hypertext_lineage SET `position` = 9;
RELATE concept:region_link->in_thread->thread:t_hypertext_lineage SET `position` = 10;
RELATE concept:pointing_hand_cursor->in_thread->thread:t_hypertext_lineage SET `position` = 11;
RELATE artifact:viawww->in_thread->thread:t_hypertext_lineage SET `position` = 12;
RELATE concept:wiki->in_thread->thread:t_hypertext_lineage SET `position` = 13;
RELATE concept:obsidian_roam->in_thread->thread:t_hypertext_lineage SET `position` = 14;
RELATE artifact:decker->in_thread->thread:t_hypertext_lineage SET `position` = 15;

RELATE artifact:hypercard->in_thread->thread:t_web_before_the_web SET `position` = 1;
RELATE concept:stackware->in_thread->thread:t_web_before_the_web SET `position` = 2;
RELATE concept:home_stack->in_thread->thread:t_web_before_the_web SET `position` = 3;
RELATE concept:message_box->in_thread->thread:t_web_before_the_web SET `position` = 4;
RELATE concept:region_link->in_thread->thread:t_web_before_the_web SET `position` = 5;
RELATE concept:html_hyperlink->in_thread->thread:t_web_before_the_web SET `position` = 6;
RELATE artifact:world_wide_web->in_thread->thread:t_web_before_the_web SET `position` = 7;
RELATE person:robert_cailliau->in_thread->thread:t_web_before_the_web SET `position` = 8;
RELATE artifact:viawww->in_thread->thread:t_web_before_the_web SET `position` = 9;
RELATE artifact:javascript->in_thread->thread:t_web_before_the_web SET `position` = 10;
RELATE concept:javascript_dom->in_thread->thread:t_web_before_the_web SET `position` = 11;
RELATE concept:web_displacement->in_thread->thread:t_web_before_the_web SET `position` = 12;
RELATE event:e_atkinson_wired_2002->in_thread->thread:t_web_before_the_web SET `position` = 13;

RELATE concept:hypertalk->in_thread->thread:t_xtalk_diaspora SET `position` = 1;
RELATE artifact:supercard->in_thread->thread:t_xtalk_diaspora SET `position` = 2;
RELATE artifact:toolbook->in_thread->thread:t_xtalk_diaspora SET `position` = 3;
RELATE artifact:director->in_thread->thread:t_xtalk_diaspora SET `position` = 4;
RELATE artifact:javascript->in_thread->thread:t_xtalk_diaspora SET `position` = 5;
RELATE concept:applescript->in_thread->thread:t_xtalk_diaspora SET `position` = 6;
RELATE artifact:livecode->in_thread->thread:t_xtalk_diaspora SET `position` = 7;
RELATE artifact:openxtalk->in_thread->thread:t_xtalk_diaspora SET `position` = 8;
RELATE artifact:sk8->in_thread->thread:t_xtalk_diaspora SET `position` = 9;
RELATE concept:xtalk_family->in_thread->thread:t_xtalk_diaspora SET `position` = 10;
RELATE artifact:decker->in_thread->thread:t_xtalk_diaspora SET `position` = 11;

RELATE concept:free_distribution->in_thread->thread:t_rise_and_fall SET `position` = 1;
RELATE event:e_release_1987->in_thread->thread:t_rise_and_fall SET `position` = 2;
RELATE artifact:hypercard->in_thread->thread:t_rise_and_fall SET `position` = 3;
RELATE event:e_hc2_1990->in_thread->thread:t_rise_and_fall SET `position` = 4;
RELATE artifact:hypercard_2->in_thread->thread:t_rise_and_fall SET `position` = 5;
RELATE event:e_claris_player_1992->in_thread->thread:t_rise_and_fall SET `position` = 6;
RELATE concept:player_split->in_thread->thread:t_rise_and_fall SET `position` = 7;
RELATE concept:bolted_on_color->in_thread->thread:t_rise_and_fall SET `position` = 8;
RELATE artifact:hypercard_3->in_thread->thread:t_rise_and_fall SET `position` = 9;
RELATE event:e_hc3_wwdc_1996->in_thread->thread:t_rise_and_fall SET `position` = 10;
RELATE person:steve_jobs->in_thread->thread:t_rise_and_fall SET `position` = 11;
RELATE event:e_hc3_cancelled_2000->in_thread->thread:t_rise_and_fall SET `position` = 12;
RELATE concept:succession_failure->in_thread->thread:t_rise_and_fall SET `position` = 13;
RELATE event:e_discontinued_2004->in_thread->thread:t_rise_and_fall SET `position` = 14;
RELATE concept:business_model_clash->in_thread->thread:t_rise_and_fall SET `position` = 15;
RELATE person:john_sculley->in_thread->thread:t_rise_and_fall SET `position` = 16;

RELATE concept:stackware->in_thread->thread:t_stackware_economy SET `position` = 1;
RELATE event:e_release_1987->in_thread->thread:t_stackware_economy SET `position` = 2;
RELATE artifact:whole_earth_catalog->in_thread->thread:t_stackware_economy SET `position` = 3;
RELATE artifact:hard_days_night->in_thread->thread:t_stackware_economy SET `position` = 4;
RELATE artifact:myst->in_thread->thread:t_stackware_economy SET `position` = 5;
RELATE artifact:the_manhole->in_thread->thread:t_stackware_economy SET `position` = 6;
RELATE artifact:compileit->in_thread->thread:t_stackware_economy SET `position` = 7;
RELATE artifact:oracle_card->in_thread->thread:t_stackware_economy SET `position` = 8;
RELATE artifact:beehive->in_thread->thread:t_stackware_economy SET `position` = 9;
RELATE concept:education->in_thread->thread:t_stackware_economy SET `position` = 10;
RELATE concept:lightweight_database->in_thread->thread:t_stackware_economy SET `position` = 11;
RELATE concept:art_exhibition->in_thread->thread:t_stackware_economy SET `position` = 12;
RELATE concept:hypermedia_publishing->in_thread->thread:t_stackware_economy SET `position` = 13;
RELATE concept:presentation->in_thread->thread:t_stackware_economy SET `position` = 14;
RELATE concept:macro_virus->in_thread->thread:t_stackware_economy SET `position` = 15;
RELATE event:e_virus_1991->in_thread->thread:t_stackware_economy SET `position` = 16;
RELATE event:e_merryxmas_1993->in_thread->thread:t_stackware_economy SET `position` = 17;
RELATE concept:author_player_politics->in_thread->thread:t_stackware_economy SET `position` = 18;

RELATE concept:platform_lockin->in_thread->thread:t_revival_preservation SET `position` = 1;
RELATE event:e_discontinued_2004->in_thread->thread:t_revival_preservation SET `position` = 2;
RELATE artifact:hypercard_org->in_thread->thread:t_revival_preservation SET `position` = 3;
RELATE artifact:stacksmith->in_thread->thread:t_revival_preservation SET `position` = 4;
RELATE person:uli_kusterer->in_thread->thread:t_revival_preservation SET `position` = 5;
RELATE artifact:wyldcard->in_thread->thread:t_revival_preservation SET `position` = 6;
RELATE artifact:hypercardpreview->in_thread->thread:t_revival_preservation SET `position` = 7;
RELATE person:pierre_lorenzi->in_thread->thread:t_revival_preservation SET `position` = 8;
RELATE artifact:cinsimp->in_thread->thread:t_revival_preservation SET `position` = 9;
RELATE person:joshua_hawcroft->in_thread->thread:t_revival_preservation SET `position` = 10;
RELATE event:e_emulation_2017->in_thread->thread:t_revival_preservation SET `position` = 11;
RELATE artifact:decker->in_thread->thread:t_revival_preservation SET `position` = 12;
RELATE person:john_earnest->in_thread->thread:t_revival_preservation SET `position` = 13;
RELATE concept:q_preservation->in_thread->thread:t_revival_preservation SET `position` = 14;
RELATE source:s_pantechnicon->in_thread->thread:t_revival_preservation SET `position` = 15;
RELATE source:s_internet_archive_hc->in_thread->thread:t_revival_preservation SET `position` = 16;

RELATE concept:q_modern_stack->in_thread->thread:t_modernization_tensions SET `position` = 1;
RELATE concept:q_authoring_runtime_unity->in_thread->thread:t_modernization_tensions SET `position` = 2;
RELATE concept:q_message_path_modern->in_thread->thread:t_modernization_tensions SET `position` = 3;
RELATE concept:q_live_editing_web->in_thread->thread:t_modernization_tensions SET `position` = 4;
RELATE concept:q_distribution->in_thread->thread:t_modernization_tensions SET `position` = 5;
RELATE concept:q_data_model->in_thread->thread:t_modernization_tensions SET `position` = 6;
RELATE concept:q_collaboration->in_thread->thread:t_modernization_tensions SET `position` = 7;
RELATE concept:q_preservation->in_thread->thread:t_modernization_tensions SET `position` = 8;
RELATE concept:q_who_is_the_author->in_thread->thread:t_modernization_tensions SET `position` = 9;
RELATE concept:q_constraints->in_thread->thread:t_modernization_tensions SET `position` = 10;
RELATE concept:www->in_thread->thread:t_modernization_tensions SET `position` = 11;
RELATE concept:html_hyperlink->in_thread->thread:t_modernization_tensions SET `position` = 12;
RELATE concept:javascript_dom->in_thread->thread:t_modernization_tensions SET `position` = 13;
RELATE concept:applescript->in_thread->thread:t_modernization_tensions SET `position` = 14;
RELATE concept:spa_routing->in_thread->thread:t_modernization_tensions SET `position` = 15;
RELATE concept:command_palette->in_thread->thread:t_modernization_tensions SET `position` = 16;
RELATE concept:low_code->in_thread->thread:t_modernization_tensions SET `position` = 17;
RELATE concept:local_first->in_thread->thread:t_modernization_tensions SET `position` = 18;
RELATE concept:single_file_html_app->in_thread->thread:t_modernization_tensions SET `position` = 19;
RELATE concept:notion_airtable->in_thread->thread:t_modernization_tensions SET `position` = 20;
RELATE concept:wiki->in_thread->thread:t_modernization_tensions SET `position` = 21;
RELATE concept:obsidian_roam->in_thread->thread:t_modernization_tensions SET `position` = 22;
RELATE concept:tiddlywiki->in_thread->thread:t_modernization_tensions SET `position` = 23;
RELATE concept:jupyter->in_thread->thread:t_modernization_tensions SET `position` = 24;
RELATE concept:wasm_plugins->in_thread->thread:t_modernization_tensions SET `position` = 25;
RELATE concept:dynamicland->in_thread->thread:t_modernization_tensions SET `position` = 26;
RELATE concept:ai_codegen->in_thread->thread:t_modernization_tensions SET `position` = 27;

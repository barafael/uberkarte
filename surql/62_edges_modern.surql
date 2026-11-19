-- Modern-analog mappings, tensions, and the questions they raise.

-- maps_to ------------------------------------------------------------------
RELATE concept:region_link->maps_to->concept:html_hyperlink SET note = "the rival solution to linking", confidence = 'high';

RELATE concept:message_path->maps_to->concept:javascript_dom SET confidence = 'high';
RELATE concept:message->maps_to->concept:javascript_dom SET confidence = 'high';
RELATE concept:message_box->maps_to->concept:command_palette SET confidence = 'high';
RELATE concept:stack->maps_to->concept:single_file_html_app SET confidence = 'high';
RELATE concept:stack->maps_to->concept:tiddlywiki SET confidence = 'high';
RELATE concept:stack->maps_to->concept:jupyter SET note = "the executable document line", confidence = 'medium';
RELATE concept:card->maps_to->concept:notion_airtable SET confidence = 'high';
RELATE concept:card->maps_to->concept:wiki SET note = "page-as-card", confidence = 'medium';
RELATE concept:find_mark_sort->maps_to->concept:obsidian_roam SET confidence = 'medium';
RELATE concept:memex->maps_to->concept:obsidian_roam SET confidence = 'high';
RELATE concept:hypertext->maps_to->concept:www SET confidence = 'high';
RELATE concept:hypertalk->maps_to->concept:javascript_dom SET confidence = 'high';
RELATE concept:hypertalk->maps_to->concept:applescript SET confidence = 'medium';
RELATE concept:natural_language_syntax->maps_to->concept:ai_codegen SET confidence = 'medium';
RELATE concept:xcmd->maps_to->concept:wasm_plugins SET confidence = 'high';
RELATE concept:single_file_unity->maps_to->concept:local_first SET confidence = 'high';
RELATE concept:immediate_persistence->maps_to->concept:local_first SET confidence = 'high';
RELATE concept:end_user_programming->maps_to->concept:low_code SET confidence = 'high';
RELATE concept:stackware->maps_to->concept:wiki SET confidence = 'medium';
RELATE concept:chunk_expression->maps_to->concept:notion_airtable SET note = "the data-substrate trade", confidence = 'medium';
RELATE concept:player_split->maps_to->concept:low_code SET note = "view/edit split as pricing", confidence = 'medium';

-- limits (tension -> what it constrains) ------------------------------------
RELATE concept:idle_time->part_of->concept:message_path SET position = 13, note = "the floor of the path: idle fires when the queue empties";
RELATE concept:idle_time->maps_to->concept:javascript_dom SET note = "the single-threaded pump bargain: on idle -> requestAnimationFrame, do-in-seconds -> setTimeout", confidence = 'medium';
RELATE concept:no_structured_data->limits->concept:card SET confidence = 'high';
RELATE concept:no_structured_data->limits->concept:chunk_expression SET confidence = 'high';
RELATE concept:monolithic_file->limits->concept:stack SET confidence = 'high';
RELATE concept:single_user->limits->concept:stack SET confidence = 'high';
RELATE concept:interpreter_performance->limits->concept:hypertalk SET confidence = 'high';
RELATE concept:platform_lockin->limits->concept:stack SET note = "mortality: the file outlives the machine", confidence = 'high';
RELATE concept:security_naivete->limits->concept:script_property SET confidence = 'high';
RELATE concept:no_inline_text_links->limits->concept:region_link SET confidence = 'high';
RELATE concept:no_networking->limits->concept:message_path SET note = "a path that cannot cross a wire", confidence = 'high';
RELATE concept:bolted_on_color->limits->concept:tool_palette SET confidence = 'medium';
RELATE concept:web_displacement->limits->concept:stackware SET confidence = 'high';
RELATE concept:fixed_canvas->limits->concept:card SET confidence = 'medium';
RELATE concept:pseudo_arrays->limits->concept:hypertalk SET confidence = 'high';

-- raises (tension -> question) ----------------------------------------------
RELATE concept:no_networking->raises->concept:q_message_path_modern SET confidence = 'high';
RELATE concept:no_inline_text_links->raises->concept:q_modern_stack SET confidence = 'high';
RELATE concept:no_structured_data->raises->concept:q_data_model SET confidence = 'high';
RELATE concept:pseudo_arrays->raises->concept:q_data_model SET confidence = 'high';
RELATE concept:bolted_on_color->raises->concept:q_constraints SET confidence = 'medium';
RELATE concept:single_user->raises->concept:q_collaboration SET confidence = 'high';
RELATE concept:interpreter_performance->raises->concept:q_modern_stack SET confidence = 'medium';
RELATE concept:monolithic_file->raises->concept:q_distribution SET confidence = 'high';
RELATE concept:platform_lockin->raises->concept:q_preservation SET confidence = 'high';
RELATE concept:business_model_clash->raises->concept:q_distribution SET confidence = 'high';
RELATE concept:author_player_politics->raises->concept:q_who_is_the_author SET confidence = 'high';
RELATE concept:security_naivete->raises->concept:q_live_editing_web SET confidence = 'high';
RELATE concept:succession_failure->raises->concept:q_preservation SET confidence = 'medium';
RELATE concept:web_displacement->raises->concept:q_modern_stack SET confidence = 'high';

-- addresses (analog -> tension it resolves) ---------------------------------
RELATE concept:www->addresses->concept:no_networking SET confidence = 'high';
RELATE concept:html_hyperlink->addresses->concept:no_inline_text_links SET confidence = 'high';
RELATE concept:notion_airtable->addresses->concept:no_structured_data SET confidence = 'high';
RELATE concept:local_first->addresses->concept:platform_lockin SET note = "partially: runs anywhere, yours", confidence = 'medium';
RELATE concept:local_first->addresses->concept:single_user SET note = "CRDT merge layer", confidence = 'medium';
RELATE concept:javascript_dom->addresses->concept:interpreter_performance SET note = "JIT era", confidence = 'medium';
RELATE concept:wasm_plugins->addresses->concept:interpreter_performance SET confidence = 'medium';
RELATE concept:spa_routing->addresses->concept:monolithic_file SET note = "bundling splits the file", confidence = 'high';
RELATE concept:wiki->addresses->concept:author_player_politics SET note = "anyone can edit", confidence = 'high';
RELATE concept:tiddlywiki->addresses->concept:author_player_politics SET confidence = 'high';
RELATE concept:tiddlywiki->addresses->concept:platform_lockin SET confidence = 'medium';
RELATE artifact:decker->addresses->concept:pseudo_arrays SET note = "Lil's arrays", confidence = 'high';
RELATE artifact:decker->addresses->concept:platform_lockin SET note = "cross-platform + html export", confidence = 'medium';
RELATE concept:low_code->addresses->concept:business_model_clash SET note = "monetized empowerment", confidence = 'medium';
RELATE concept:ai_codegen->addresses->concept:natural_language_syntax SET note = "completes the English bet with a model", confidence = 'medium';
RELATE concept:obsidian_roam->addresses->concept:no_inline_text_links SET note = "backlinks as link database", confidence = 'medium';
RELATE concept:dynamicland->addresses->concept:q_live_editing_web SET confidence = 'medium';

-- contrasts_with ------------------------------------------------------------
RELATE concept:applescript->contrasts_with->concept:hypertalk SET note = "control from outside vs from inside", confidence = 'high';
RELATE concept:html_hyperlink->contrasts_with->concept:region_link SET confidence = 'high';

RELATE concept:low_code->contrasts_with->concept:end_user_programming SET note = "same audience, opposite boundaries", confidence = 'high';
RELATE concept:wiki->contrasts_with->concept:stackware SET note = "authoring without runtime", confidence = 'high';
RELATE concept:www->contrasts_with->concept:stack SET note = "addressed fragments vs unified artifact", confidence = 'high';
RELATE concept:jupyter->contrasts_with->concept:immediate_persistence SET note = "frozen outputs, kernel outside the file", confidence = 'medium';


-- event edges ---------------------------------------------------------------
RELATE event:e_release_1987->about->artifact:hypercard;
RELATE event:e_release_1987->about->concept:free_distribution;
RELATE event:e_hc2_1990->about->artifact:hypercard_2;
RELATE event:e_hc2_1990->about->concept:hypertalk_compiler;
RELATE event:e_virus_1991->about->concept:macro_virus;
RELATE event:e_merryxmas_1993->about->concept:macro_virus;
RELATE event:e_claris_player_1992->about->artifact:hypercard_player;
RELATE event:e_claris_player_1992->about->concept:player_split;
RELATE event:e_hc3_wwdc_1996->about->artifact:hypercard_3;
RELATE event:e_hc3_cancelled_2000->about->artifact:hypercard_3;
RELATE event:e_hc3_cancelled_2000->about->concept:succession_failure;
RELATE event:e_discontinued_2004->about->artifact:hypercard;
RELATE event:e_discontinued_2004->about->concept:platform_lockin;
RELATE event:e_emulation_2017->about->concept:q_preservation;
RELATE event:e_atkinson_wired_2002->about->concept:web_displacement;
RELATE event:e_atkinson_wired_2002->about->person:bill_atkinson;

RELATE event:e_demo_1968->about->person:doug_engelbart;
RELATE event:e_demo_1968->about->concept:end_user_programming;

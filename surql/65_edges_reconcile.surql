-- Edges added to reconcile the corpus with notes/coverage-critique.md.
-- Facts here are limited to what the corpus's own prose, the sources added in
-- 41_additions_reconcile, or well-attested public history support; confidence
-- says which.

-- Person <-> person ----------------------------------------------------------
RELATE person:bill_atkinson->worked_with->person:dan_winkler SET note = "Atkinson brought his friend Winkler in (1986) to design HyperTalk", confidence = 'high';
RELATE person:kevin_calhoun->worked_with->person:dan_crow SET note = "Calhoun led HyperCard 2.0 and the QTi effort; Crow led development later in the 1990s; both left Apple in 2001 (Wikipedia)", confidence = 'high';




-- Person -> org --------------------------------------------------------------
RELATE person:bill_atkinson->employed_by->org:apple SET note = "Mac engineer; left 1990", confidence = 'high';





RELATE person:kevin_calhoun->employed_by->org:apple SET note = "left 2001", confidence = 'medium';
RELATE person:dan_crow->employed_by->org:apple SET note = "left 2001", confidence = 'high';


-- Actors in events -------------------------------------------------------------
RELATE person:bill_atkinson->participated_in->event:e_release_1987 SET note = "creator of the product announced", confidence = 'medium';

RELATE org:apple->participated_in->event:e_release_1987 SET note = "announced and bundled it", confidence = 'high';
RELATE person:kevin_calhoun->participated_in->event:e_hc2_1990 SET note = "2.0 engineering lead", confidence = 'high';


RELATE org:claris->participated_in->event:e_claris_player_1992 SET note = "shipped 2.2 and the Player split", confidence = 'high';

RELATE org:apple->participated_in->event:e_hc3_wwdc_1996 SET confidence = 'high';
RELATE org:apple->participated_in->event:e_hc_team_moved_1998 SET note = "moved the team to QuickTime", confidence = 'medium';

RELATE person:kevin_calhoun->participated_in->event:e_hc3_cancelled_2000 SET note = "left Apple the following year", confidence = 'medium';
RELATE person:dan_crow->participated_in->event:e_hc3_cancelled_2000 SET note = "led late-1990s development; left Apple in 2001", confidence = 'medium';
RELATE org:apple->participated_in->event:e_hc3_cancelled_2000 SET confidence = 'high';
RELATE org:apple->participated_in->event:e_discontinued_2004 SET confidence = 'high';



RELATE org:apple->participated_in->event:e_intel_2006 SET confidence = 'high';
RELATE org:apple->participated_in->event:e_ipad_2010 SET confidence = 'high';
RELATE org:apple->participated_in->event:e_ibooks_author_2012 SET confidence = 'high';



RELATE person:bill_atkinson->participated_in->event:e_atkinson_wired_2002 SET note = "the interviewee", confidence = 'high';

RELATE person:doug_engelbart->participated_in->event:e_demo_1968 SET note = "presenter", confidence = 'high';

-- Org <-> org ------------------------------------------------------------------
RELATE org:claris->spun_off_from->org:apple SET note = "software subsidiary, 1987", confidence = 'high';



-- Institutional authorship / publishing / ownership ---------------------------












RELATE artifact:stack_format->owned_by->org:apple;




-- Lineage and exemplars for the media / education / games branches -------------



RELATE artifact:director->influenced->artifact:flash SET note = "scripting line routed hypertalk -> Lingo -> ActionScript elsewhere in this corpus", confidence = 'medium';

RELATE artifact:flash->exemplifies->concept:author_player_politics SET note = "free Player, paid authoring tool: the 1992 split reappears on the web", confidence = 'high';
RELATE artifact:scratch->exemplifies->concept:q_authoring_runtime_unity SET note = "projects are edited while they run", confidence = 'medium';
RELATE artifact:scratch->applied_to->concept:education SET confidence = 'high';

RELATE artifact:twine->exemplifies->concept:single_file_unity SET note = "a story publishes as one HTML file", confidence = 'high';




-- Preservation: format and runtime as artifacts --------------------------------
RELATE artifact:stack_format->implements->concept:stack SET note = "the file form of the unit", confidence = 'high';
RELATE artifact:stack_format->enables->artifact:stacksmith SET note = "documented by hypercard.org; basis for stackimport", confidence = 'high';
RELATE artifact:stack_format->enables->artifact:hypercardpreview SET confidence = 'high';
RELATE artifact:stack_format->enables->artifact:wyldcard SET confidence = 'medium';
RELATE artifact:stack_format->documented_in->source:s_hypercard_org SET confidence = 'high';
RELATE artifact:stack_format->documented_in->source:s_loc_stack SET confidence = 'high';
RELATE artifact:stack_format->documented_in->source:s_stack_format_lorenzi SET confidence = 'medium';



RELATE artifact:internet_archive->exemplifies->concept:q_preservation SET note = "institutional runtime preservation", confidence = 'high';
RELATE artifact:internet_archive->documented_in->source:s_internet_archive_hc SET confidence = 'high';
RELATE event:e_emulation_2017->about->artifact:internet_archive SET confidence = 'high';



-- Event about-edges (events attached to the ideas they bear on) -----------------




RELATE event:e_hc_team_moved_1998->about->artifact:hypercard_3 SET confidence = 'medium';
RELATE event:e_hc_team_moved_1998->about->concept:succession_failure SET confidence = 'medium';




RELATE event:e_intel_2006->about->concept:platform_lockin SET confidence = 'high';
RELATE event:e_ipad_2010->about->concept:q_authoring_runtime_unity SET confidence = 'low';
RELATE event:e_ibooks_author_2012->about->artifact:ibooks_author SET confidence = 'high';
RELATE event:e_ibooks_author_2012->about->concept:author_player_politics SET confidence = 'high';

-- Source about / documented_in for the new corrections --------------------------



RELATE source:s_tidbits_1998->about->event:e_hc_team_moved_1998 SET confidence = 'medium', note = "single contemporaneous report from unofficial sources";
RELATE event:e_hc_team_moved_1998->documented_in->source:s_tidbits_1998 SET confidence = 'medium';
RELATE artifact:hypercard_3->documented_in->source:s_tidbits_1998 SET confidence = 'medium', note = "supports the 1997 QTi slip and the Oct 1998 end of development";
RELATE org:claris->documented_in->source:s_tidbits_1998 SET confidence = 'medium', note = "HyperCard returned to Apple from Claris";
RELATE source:s_tnw_ibooks_2012->about->event:e_ibooks_author_2012 SET confidence = 'high';
RELATE event:e_ibooks_author_2012->documented_in->source:s_tnw_ibooks_2012 SET confidence = 'high';



RELATE source:s_supercard_wiki->about->artifact:supercard SET confidence = 'medium';

RELATE artifact:supercard->documented_in->source:s_supercard_wiki SET confidence = 'medium';

-- Close the matrix honestly: edges the critique found missing -------------------
-- security_naivete had no addresser although the thread claimed one.
RELATE concept:wasm_plugins->addresses->concept:security_naivete SET note = "partial: a permission-and-review regime around extension code, not a fix for executable documents", confidence = 'medium';
-- q_authoring_runtime_unity was raised by no tension; its own body names three.
RELATE concept:author_player_politics->raises->concept:q_authoring_runtime_unity SET note = "the split that made unity a product decision", confidence = 'high';
RELATE concept:security_naivete->raises->concept:q_authoring_runtime_unity SET note = "technical reasons the unity was dropped still apply", confidence = 'high';
RELATE concept:business_model_clash->raises->concept:q_authoring_runtime_unity SET note = "revenue reasons the unity was dropped still apply", confidence = 'medium';

-- The counter-case
RELATE concept:web_displacement->motivates->concept:anti_successor SET note = "the web copied the surface and solved the structural gaps", confidence = 'medium';
RELATE concept:security_naivete->motivates->concept:anti_successor SET confidence = 'medium';
RELATE concept:monolithic_file->motivates->concept:anti_successor SET confidence = 'medium';
RELATE concept:anti_successor->raises->concept:q_authoring_runtime_unity SET note = "makes the corpus's central premise contestable", confidence = 'medium';
RELATE concept:anti_successor->contrasts_with->concept:q_modern_stack SET note = "argues the question may have the wrong shape", confidence = 'medium';
RELATE concept:anti_successor->contrasts_with->concept:succession_failure SET note = "succession_failure assumes succession was owed", confidence = 'medium';

-- Practice nodes tied to the verbatim code layer
RELATE code:jaed_lockall->demonstrates->concept:stack_idioms SET note = "lock-screen craft";
RELATE code:jaed_visual_send->demonstrates->concept:stack_idioms SET note = "visual effects between cards";
RELATE code:jaed_anim->demonstrates->concept:stack_idioms SET note = "animation on the idle pump";
RELATE code:myst_blackframe->demonstrates->concept:visual_culture SET note = "frame handling in a shipped stack";
RELATE concept:fixed_canvas->motivates->concept:visual_culture SET note = "constraint became style", confidence = 'medium';
RELATE concept:stack_idioms->part_of->concept:stackware SET note = "craft inside the folk economy";
RELATE concept:visual_culture->part_of->concept:stackware SET note = "the look of the folk economy";

-- Thread membership for the new nodes -------------------------------------------


RELATE event:e_hc_team_moved_1998->in_thread->thread:t_rise_and_fall SET `position` = 19;


RELATE event:e_intel_2006->in_thread->thread:t_rise_and_fall SET `position` = 22;
RELATE event:e_ibooks_author_2012->in_thread->thread:t_rise_and_fall SET `position` = 23;
RELATE event:e_ipad_2010->in_thread->thread:t_rise_and_fall SET `position` = 24;

RELATE artifact:stack_format->in_thread->thread:t_revival_preservation SET `position` = 18;



RELATE artifact:internet_archive->in_thread->thread:t_revival_preservation SET `position` = 22;




RELATE concept:stack_idioms->in_thread->thread:t_stackware_economy SET `position` = 23;
RELATE concept:visual_culture->in_thread->thread:t_stackware_economy SET `position` = 24;


RELATE artifact:flash->in_thread->thread:t_xtalk_diaspora SET `position` = 13;
RELATE artifact:scratch->in_thread->thread:t_xtalk_diaspora SET `position` = 14;
RELATE artifact:twine->in_thread->thread:t_xtalk_diaspora SET `position` = 15;
RELATE concept:anti_successor->in_thread->thread:t_modernization_tensions SET `position` = 28;
RELATE person:kevin_calhoun->participated_in->event:e_hc3_wwdc_1996 SET note = "showed the HyperCard 3.0 / QTi alpha (TidBITS; folkstream)", confidence = 'medium';
RELATE artifact:hypercard_3->created_by->person:kevin_calhoun SET note = "QTi effort 'once again under the direction of Kevin Calhoun' (Wikipedia); Dan Crow led development later in the 1990s", confidence = 'medium';
RELATE concept:message_path->documented_in->source:s_hc_guide_archive SET note = "handler-then-XCMD order at each level; no OS on the path; where messages start", confidence = 'high';
RELATE concept:message->documented_in->source:s_hc_guide_archive SET note = "unhandled system message vs \"Can't understand\"; pass/exit semantics", confidence = 'high';
RELATE concept:idle_time->documented_in->source:s_hc_guide_archive SET note = "no timer command in the guide; wait and idle", confidence = 'high';
RELATE concept:target_me_it->documented_in->source:s_hc_guide_archive SET note = "`it` is local", confidence = 'high';
RELATE concept:tool_palette->documented_in->source:s_hc_guide_archive SET note = "tools numbered 1-18", confidence = 'high';
RELATE concept:find_mark_sort->documented_in->source:s_hc_guide_archive SET note = "find scope; marked property; mark commands", confidence = 'high';
RELATE concept:message_box->documented_in->source:s_hc_guide_archive SET note = "floating one-line window; only do/if/send keywords", confidence = 'high';
RELATE concept:region_link->documented_in->source:s_hc_guide_archive SET note = "group text and clickChunk; no linkText", confidence = 'high';
RELATE concept:fixed_canvas->documented_in->source:s_hc_guide_archive SET note = "card sizes 64-1280 from 2.0; 18 windows", confidence = 'high';
RELATE concept:home_stack->documented_in->source:s_hc_guide_archive SET note = "scripts and externals in every stack path", confidence = 'high';
RELATE concept:container_model->documented_in->source:s_hc_guide_archive SET note = "containers: fields, variables, selection, message box", confidence = 'high';
RELATE concept:evaluation_modes->documented_in->source:s_hc_guide_archive SET note = "the sqrt of 4+12; unquoted-literal fallback", confidence = 'high';
RELATE concept:hypertalk_compiler->documented_in->source:s_hc_guide_archive SET note = "compile on first execution, cached in RAM; windows are not objects", confidence = 'high';
RELATE concept:part->documented_in->source:s_hc_guide_archive SET note = "button and field styles", confidence = 'high';
RELATE concept:background->documented_in->source:s_hc_guide_archive SET note = "shared vs per-card field text", confidence = 'high';
RELATE concept:property_model->documented_in->source:s_hc_guide_archive SET note = "card properties; read-only ids", confidence = 'high';
RELATE concept:pseudo_arrays->documented_in->source:s_hc_guide_archive SET note = "no arrays", confidence = 'high';
RELATE concept:home_stack->documented_in->source:s_hc_users_guide_1987 SET note = "Home card, User Preferences, Home stack required", confidence = 'high';
RELATE concept:find_mark_sort->documented_in->source:s_hc_users_guide_1987 SET note = "Recent shows up to the last 42 cards", confidence = 'high';
RELATE concept:player_split->documented_in->source:s_tidbits_1992 SET note = "Player bundled from 14 Sep 1992; Claris licence expiring 30 Sep 1992", confidence = 'high';
RELATE event:e_claris_player_1992->documented_in->source:s_tidbits_1992 SET note = "dated Sep 1992", confidence = 'high';
RELATE artifact:hypercard_player->documented_in->source:s_tidbits_1992 SET note = "2.1 Player shipped with new Macs", confidence = 'high';
RELATE concept:bolted_on_color->documented_in->source:s_tidbits_1994 SET note = "color through PICT-based Color Tools and AddMotion II in 2.2", confidence = 'high';
RELATE artifact:hypercard->documented_in->source:s_tidbits_1994 SET note = "2.2 reviewed Feb 1994; Claris history", confidence = 'medium';
RELATE concept:applescript->documented_in->source:s_tidbits_1994 SET note = "HyperCard 2.2 incorporates AppleScript", confidence = 'medium';
RELATE event:e_atkinson_wired_2002->documented_in->source:s_wired_2002 SET note = "Atkinson quotation (via MacTech item of the same day)", confidence = 'high';

-- Edges added to reconcile the corpus with notes/coverage-critique.md.
-- Facts here are limited to what the corpus's own prose, the sources added in
-- 41_additions_reconcile, or well-attested public history support; confidence
-- says which.

-- Person <-> person ----------------------------------------------------------
RELATE person:bill_atkinson->worked_with->person:dan_winkler SET note = "Atkinson brought his friend Winkler in (1986) to design HyperTalk", confidence = 'high';
RELATE person:kevin_calhoun->worked_with->person:dan_crow SET note = "2.0 engineering lead and the QTi/3.0 engineer; both left Apple in 2001", confidence = 'high';
RELATE person:bill_atkinson->worked_with->person:andy_hertzfeld SET note = "Macintosh team colleagues; General Magic co-founders (1990)", confidence = 'high';
RELATE person:bill_atkinson->worked_with->person:steve_jobs SET note = "Macintosh era; Jobs later returned to an Apple Atkinson had already left", confidence = 'high';
RELATE person:bill_atkinson->worked_with->person:john_sculley SET note = "executive patron under whom HyperCard shipped free", confidence = 'medium';

-- Person -> org --------------------------------------------------------------
RELATE person:bill_atkinson->employed_by->org:apple SET note = "Mac engineer; left 1990", confidence = 'high';
RELATE person:bill_atkinson->employed_by->org:general_magic SET note = "co-founder, 1990", confidence = 'high';
RELATE person:andy_hertzfeld->employed_by->org:general_magic SET note = "co-founder, 1990", confidence = 'high';
RELATE person:andy_hertzfeld->employed_by->org:apple SET note = "original Macintosh team", confidence = 'high';
RELATE person:steve_jobs->employed_by->org:apple SET note = "co-founder; returned 1996-97", confidence = 'high';
RELATE person:john_sculley->employed_by->org:apple SET note = "CEO at HyperCard's launch", confidence = 'high';
RELATE person:kevin_calhoun->employed_by->org:apple SET note = "left 2001", confidence = 'medium';
RELATE person:dan_crow->employed_by->org:apple SET note = "left 2001", confidence = 'high';
RELATE person:bill_appleton->employed_by->org:silicon_beach SET note = "author of World Builder and SuperCard", confidence = 'high';

-- Actors in events -------------------------------------------------------------
RELATE person:bill_atkinson->participated_in->event:e_release_1987 SET note = "creator of the product announced", confidence = 'medium';
RELATE person:john_sculley->participated_in->event:e_release_1987 SET note = "CEO who praised it publicly", confidence = 'medium';
RELATE org:apple->participated_in->event:e_release_1987 SET note = "announced and bundled it", confidence = 'high';
RELATE person:kevin_calhoun->participated_in->event:e_hc2_1990 SET note = "2.0 engineering lead", confidence = 'high';
RELATE org:claris->participated_in->event:e_claris_transfer_1990 SET note = "received HyperCard", confidence = 'high';
RELATE org:apple->participated_in->event:e_claris_transfer_1990 SET note = "transferred it", confidence = 'high';
RELATE org:claris->participated_in->event:e_claris_player_1992 SET note = "shipped 2.2 and the Player split", confidence = 'high';
RELATE person:dan_crow->participated_in->event:e_hc3_wwdc_1996 SET note = "gave the demo", confidence = 'high';
RELATE org:apple->participated_in->event:e_hc3_wwdc_1996 SET confidence = 'high';
RELATE org:apple->participated_in->event:e_hc_team_moved_1998 SET note = "moved the team to QuickTime", confidence = 'medium';
RELATE person:steve_jobs->participated_in->event:e_hc3_cancelled_2000 SET note = "reported to have disliked the product line; interim CEO from 1997", confidence = 'medium';
RELATE person:kevin_calhoun->participated_in->event:e_hc3_cancelled_2000 SET note = "left Apple the following year", confidence = 'medium';
RELATE person:dan_crow->participated_in->event:e_hc3_cancelled_2000 SET note = "left Apple the following year", confidence = 'medium';
RELATE org:apple->participated_in->event:e_hc3_cancelled_2000 SET confidence = 'high';
RELATE org:apple->participated_in->event:e_discontinued_2004 SET confidence = 'high';
RELATE org:apple->participated_in->event:e_jobs_return_1997 SET note = "acquired NeXT", confidence = 'high';
RELATE person:steve_jobs->participated_in->event:e_jobs_return_1997 SET confidence = 'high';
RELATE org:apple->participated_in->event:e_osx_2001 SET confidence = 'high';
RELATE org:apple->participated_in->event:e_intel_2006 SET confidence = 'high';
RELATE org:apple->participated_in->event:e_ipad_2010 SET confidence = 'high';
RELATE org:apple->participated_in->event:e_ibooks_author_2012 SET confidence = 'high';
RELATE org:filemaker_inc->participated_in->event:e_claris_renamed_1998 SET note = "the renamed company", confidence = 'high';
RELATE org:claris->participated_in->event:e_claris_renamed_1998 SET note = "the company renamed", confidence = 'high';
RELATE org:compuserve->participated_in->event:e_compuserve_library_1988 SET confidence = 'medium';
RELATE person:bill_atkinson->participated_in->event:e_atkinson_wired_2002 SET note = "the interviewee", confidence = 'high';
RELATE person:steve_wozniak->participated_in->event:e_wozniak_praise_2001 SET confidence = 'high';
RELATE person:doug_engelbart->participated_in->event:e_demo_1968 SET note = "presenter", confidence = 'high';

-- Org <-> org ------------------------------------------------------------------
RELATE org:claris->spun_off_from->org:apple SET note = "software subsidiary, 1987", confidence = 'high';
RELATE org:general_magic->spun_off_from->org:apple SET note = "1990", confidence = 'high';
RELATE org:claris->renamed_to->org:filemaker_inc SET note = "1998; the Claris name was revived in 2019", confidence = 'high';

-- Institutional authorship / publishing / ownership ---------------------------
RELATE artifact:myst->created_by->org:cyan SET note = "institutional authorship; Rand and Robyn Miller at Cyan", confidence = 'high';
RELATE artifact:cosmic_osmo->created_by->org:cyan SET confidence = 'high';
RELATE artifact:spelunx->created_by->org:cyan SET confidence = 'medium';
RELATE artifact:afternoon->created_by->person:michael_joyce SET confidence = 'high';
RELATE person:kevin_cuneo->contributed_to->artifact:hyperstudio SET note = "earlier versions of this corpus credited him as creator; role unverified, creator otherwise credited to Roger Wagner", confidence = 'low';
RELATE artifact:myst->published_by->org:broderbund SET confidence = 'high';
RELATE artifact:hard_days_night->published_by->org:voyager SET confidence = 'high';
RELATE artifact:supercard->published_by->org:silicon_beach SET note = "1989", confidence = 'high';
RELATE artifact:director->published_by->org:macromedia SET note = "via Macromind, which became Macromedia", confidence = 'medium';
RELATE artifact:authorware->published_by->org:macromedia SET note = "acquired 1992", confidence = 'medium';
RELATE artifact:flash->published_by->org:macromedia SET confidence = 'high';
RELATE artifact:filemaker->owned_by->org:filemaker_inc;
RELATE artifact:stack_format->owned_by->org:apple;
RELATE artifact:cosmic_osmo->located_in->artifact:hypercard SET note = "a HyperCard stack", confidence = 'high';
RELATE artifact:spelunx->located_in->artifact:hypercard SET note = "a HyperCard stack", confidence = 'medium';
RELATE artifact:afternoon->located_in->artifact:storyspace SET note = "Storyspace, not HyperCard", confidence = 'high';

-- Lineage and exemplars for the media / education / games branches -------------
RELATE artifact:hypercard->influenced->artifact:cosmic_osmo SET note = "the medium the world was built in", confidence = 'high';
RELATE artifact:cosmic_osmo->influenced->artifact:myst SET note = "Cyan's HyperCard worlds precede Myst", confidence = 'high';
RELATE artifact:spelunx->influenced->artifact:myst SET confidence = 'medium';
RELATE artifact:director->influenced->artifact:flash SET note = "scripting line routed hypertalk -> Lingo -> ActionScript elsewhere in this corpus", confidence = 'medium';
RELATE artifact:hypercard->influenced->artifact:authorware SET note = "HyperCard authors reportedly moved to Authorware (per the Wikipedia HyperCard article as read by the critique; not re-checked)", confidence = 'low';
RELATE artifact:flash->exemplifies->concept:author_player_politics SET note = "free Player, paid authoring tool: the 1992 split reappears on the web", confidence = 'high';
RELATE artifact:scratch->exemplifies->concept:q_authoring_runtime_unity SET note = "projects are edited while they run", confidence = 'medium';
RELATE artifact:scratch->applied_to->concept:education SET confidence = 'high';
RELATE artifact:authorware->applied_to->concept:education SET note = "computer-based training", confidence = 'high';
RELATE artifact:twine->exemplifies->concept:single_file_unity SET note = "a story publishes as one HTML file", confidence = 'high';
RELATE artifact:afternoon->exemplifies->concept:hypertext SET confidence = 'high';
RELATE artifact:filemaker->addresses->concept:no_structured_data SET note = "the historical exit ramp, per this corpus's own prose", confidence = 'medium';
RELATE org:acot->applied_to->concept:education SET note = "classroom research program in which HyperCard was an authoring medium (extent unsourced)", confidence = 'medium';

-- Preservation: format and runtime as artifacts --------------------------------
RELATE artifact:stack_format->implements->concept:stack SET note = "the file form of the unit", confidence = 'high';
RELATE artifact:stack_format->enables->artifact:stacksmith SET note = "documented by hypercard.org; basis for stackimport", confidence = 'high';
RELATE artifact:stack_format->enables->artifact:hypercardpreview SET confidence = 'high';
RELATE artifact:stack_format->enables->artifact:wyldcard SET confidence = 'medium';
RELATE artifact:stack_format->documented_in->source:s_hypercard_org SET confidence = 'high';
RELATE artifact:stack_format->documented_in->source:s_loc_stack SET confidence = 'high';
RELATE artifact:stack_format->documented_in->source:s_stack_format_lorenzi SET confidence = 'medium';
RELATE artifact:basilisk_ii->exemplifies->concept:q_preservation SET note = "runtime pole", confidence = 'medium';
RELATE artifact:sheepshaver->exemplifies->concept:q_preservation SET note = "runtime pole (PowerPC Classic)", confidence = 'medium';
RELATE artifact:mini_vmac->exemplifies->concept:q_preservation SET note = "runtime pole (compact Macs)", confidence = 'medium';
RELATE artifact:internet_archive->exemplifies->concept:q_preservation SET note = "institutional runtime preservation", confidence = 'high';
RELATE artifact:internet_archive->documented_in->source:s_internet_archive_hc SET confidence = 'high';
RELATE event:e_emulation_2017->about->artifact:internet_archive SET confidence = 'high';
RELATE concept:classic_environment->enables->artifact:hypercard SET note = "the only OS X route to running it, and PowerPC only", confidence = 'high';
RELATE concept:classic_environment->raises->concept:q_preservation SET confidence = 'high';

-- Event about-edges (events attached to the ideas they bear on) -----------------
RELATE event:e_claris_transfer_1990->about->concept:business_model_clash SET confidence = 'high';
RELATE event:e_claris_transfer_1990->about->org:claris SET confidence = 'high';
RELATE event:e_claris_renamed_1998->about->org:claris SET confidence = 'high';
RELATE event:e_compuserve_library_1988->about->concept:stackware SET confidence = 'medium';
RELATE event:e_hc_team_moved_1998->about->artifact:hypercard_3 SET confidence = 'medium';
RELATE event:e_hc_team_moved_1998->about->concept:succession_failure SET confidence = 'medium';
RELATE event:e_jobs_return_1997->about->concept:succession_failure SET confidence = 'medium';
RELATE event:e_osx_2001->about->concept:classic_environment SET confidence = 'high';
RELATE event:e_osx_2001->about->concept:platform_lockin SET confidence = 'high';
RELATE event:e_intel_2006->about->concept:classic_environment SET confidence = 'high';
RELATE event:e_intel_2006->about->concept:platform_lockin SET confidence = 'high';
RELATE event:e_ipad_2010->about->concept:q_authoring_runtime_unity SET confidence = 'low';
RELATE event:e_ibooks_author_2012->about->artifact:ibooks_author SET confidence = 'high';
RELATE event:e_ibooks_author_2012->about->concept:author_player_politics SET confidence = 'high';

-- Source about / documented_in for the new corrections --------------------------
RELATE source:s_tidbits_1990->about->event:e_claris_transfer_1990 SET confidence = 'high', note = "treats the transfer as done by Nov 1990; no date given";
RELATE source:s_tidbits_1990->about->org:claris SET confidence = 'high';
RELATE event:e_claris_transfer_1990->documented_in->source:s_tidbits_1990 SET confidence = 'medium', note = "supports 'by November 1990'; the month of transfer is not asserted";
RELATE source:s_tidbits_1998->about->event:e_hc_team_moved_1998 SET confidence = 'medium', note = "single contemporaneous report from unofficial sources";
RELATE event:e_hc_team_moved_1998->documented_in->source:s_tidbits_1998 SET confidence = 'medium';
RELATE artifact:hypercard_3->documented_in->source:s_tidbits_1998 SET confidence = 'medium', note = "supports the 1997 QTi slip and the Oct 1998 end of development";
RELATE org:claris->documented_in->source:s_tidbits_1998 SET confidence = 'medium', note = "HyperCard returned to Apple from Claris";
RELATE source:s_tnw_ibooks_2012->about->event:e_ibooks_author_2012 SET confidence = 'high';
RELATE event:e_ibooks_author_2012->documented_in->source:s_tnw_ibooks_2012 SET confidence = 'high';
RELATE source:s_hyperstudio_wiki->about->artifact:hyperstudio SET confidence = 'medium';
RELATE source:s_hyperstudio_wiki->about->person:roger_wagner SET confidence = 'medium';
RELATE artifact:hyperstudio->documented_in->source:s_hyperstudio_wiki SET confidence = 'medium';
RELATE source:s_supercard_wiki->about->artifact:supercard SET confidence = 'medium';
RELATE source:s_supercard_wiki->about->person:bill_appleton SET confidence = 'medium';
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
RELATE event:e_claris_transfer_1990->in_thread->thread:t_rise_and_fall SET `position` = 17;
RELATE event:e_claris_renamed_1998->in_thread->thread:t_rise_and_fall SET `position` = 18;
RELATE event:e_hc_team_moved_1998->in_thread->thread:t_rise_and_fall SET `position` = 19;
RELATE event:e_jobs_return_1997->in_thread->thread:t_rise_and_fall SET `position` = 20;
RELATE event:e_osx_2001->in_thread->thread:t_rise_and_fall SET `position` = 21;
RELATE event:e_intel_2006->in_thread->thread:t_rise_and_fall SET `position` = 22;
RELATE event:e_ibooks_author_2012->in_thread->thread:t_rise_and_fall SET `position` = 23;
RELATE event:e_ipad_2010->in_thread->thread:t_rise_and_fall SET `position` = 24;
RELATE concept:classic_environment->in_thread->thread:t_revival_preservation SET `position` = 17;
RELATE artifact:stack_format->in_thread->thread:t_revival_preservation SET `position` = 18;
RELATE artifact:basilisk_ii->in_thread->thread:t_revival_preservation SET `position` = 19;
RELATE artifact:sheepshaver->in_thread->thread:t_revival_preservation SET `position` = 20;
RELATE artifact:mini_vmac->in_thread->thread:t_revival_preservation SET `position` = 21;
RELATE artifact:internet_archive->in_thread->thread:t_revival_preservation SET `position` = 22;
RELATE event:e_compuserve_library_1988->in_thread->thread:t_stackware_economy SET `position` = 19;
RELATE artifact:cosmic_osmo->in_thread->thread:t_stackware_economy SET `position` = 20;
RELATE artifact:spelunx->in_thread->thread:t_stackware_economy SET `position` = 21;
RELATE artifact:filemaker->in_thread->thread:t_stackware_economy SET `position` = 22;
RELATE concept:stack_idioms->in_thread->thread:t_stackware_economy SET `position` = 23;
RELATE concept:visual_culture->in_thread->thread:t_stackware_economy SET `position` = 24;
RELATE artifact:afternoon->in_thread->thread:t_hypertext_lineage SET `position` = 16;
RELATE artifact:authorware->in_thread->thread:t_xtalk_diaspora SET `position` = 12;
RELATE artifact:flash->in_thread->thread:t_xtalk_diaspora SET `position` = 13;
RELATE artifact:scratch->in_thread->thread:t_xtalk_diaspora SET `position` = 14;
RELATE artifact:twine->in_thread->thread:t_xtalk_diaspora SET `position` = 15;
RELATE concept:anti_successor->in_thread->thread:t_modernization_tensions SET `position` = 28;

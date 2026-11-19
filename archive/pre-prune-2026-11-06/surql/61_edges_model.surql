-- Structural edges: composition, implementation, enablement, motivation, application.

-- part_of (with position for the message path chain) ------------------------
RELATE concept:part->part_of->concept:message_path SET `position` = 1, note = "the receiving part is first in the chain";
RELATE concept:card->part_of->concept:message_path SET `position` = 2;
RELATE concept:background->part_of->concept:message_path SET `position` = 3;
RELATE concept:stack->part_of->concept:message_path SET `position` = 4;
RELATE concept:home_stack->part_of->concept:message_path SET `position` = 6, note = "after start-using stacks";
RELATE concept:xcmd->part_of->concept:message_path SET `position` = 8, note = "last stop before the OS";
RELATE concept:target_me_it->part_of->concept:message_path SET note = "self-knowledge primitives of the dispatch system";
RELATE concept:card->part_of->concept:stack SET `position` = 1;
RELATE concept:background->part_of->concept:stack SET `position` = 2;
RELATE concept:part->part_of->concept:stack SET `position` = 3;
RELATE concept:script_property->part_of->concept:stack SET `position` = 4, note = "source travels inside the file";
RELATE concept:script_property->part_of->concept:property_model SET note = "behavior is one more property";
RELATE concept:chunk_expression->part_of->concept:hypertalk SET `position` = 1, note = "core expression grammar";
RELATE concept:message->part_of->concept:hypertalk SET `position` = 2;
RELATE concept:natural_language_syntax->part_of->concept:hypertalk SET `position` = 3;
RELATE concept:tool_palette->part_of->concept:home_stack SET note = "the palette chrome is home-stack-provided";
RELATE concept:find_mark_sort->part_of->concept:stack SET note = "stack-level data operations";

-- implements ---------------------------------------------------------------
RELATE artifact:hypercard->implements->concept:stack SET confidence = 'high';
RELATE artifact:hypercard->implements->concept:message_path SET confidence = 'high';
RELATE artifact:hypercard->implements->concept:script_property SET confidence = 'high';
RELATE artifact:hypercard->implements->concept:hypertext SET note = "the minimal shippable subset", confidence = 'high';
RELATE artifact:hypercard->implements->concept:end_user_programming SET confidence = 'high';
RELATE artifact:hypercard->implements->concept:immediate_persistence SET confidence = 'high';
RELATE artifact:hypercard->implements->concept:memex SET note = "partial: cards and find, not trails", confidence = 'medium';
RELATE artifact:decker->implements->concept:single_file_unity SET confidence = 'high';
RELATE artifact:decker->implements->concept:stack SET note = "re-imagined", confidence = 'medium';
RELATE artifact:world_wide_web->implements->concept:hypertext SET confidence = 'high';
RELATE concept:html_hyperlink->implements->concept:hypertext SET confidence = 'high';
RELATE concept:region_link->implements->concept:hypertext SET note = "the region-shaped subset", confidence = 'high';
RELATE concept:local_first->implements->concept:immediate_persistence SET confidence = 'medium';
RELATE concept:javascript_dom->implements->concept:message_path SET note = "bubbling as domesticated message path", confidence = 'high';
RELATE concept:tiddlywiki->implements->concept:single_file_unity SET confidence = 'high';
RELATE concept:single_file_html_app->implements->concept:single_file_unity SET confidence = 'high';
RELATE concept:wiki->implements->concept:end_user_programming SET note = "the authoring subset", confidence = 'high';

-- exemplifies --------------------------------------------------------------
RELATE artifact:whole_earth_catalog->exemplifies->concept:monolithic_file SET confidence = 'high';
RELATE artifact:whole_earth_catalog->exemplifies->concept:hypermedia_publishing SET confidence = 'high';
RELATE artifact:myst->exemplifies->concept:stackware SET note = "the commercial shadow of stackware", confidence = 'medium';
RELATE artifact:hard_days_night->exemplifies->concept:hypermedia_publishing SET confidence = 'high';
RELATE artifact:marble_springs->exemplifies->concept:art_exhibition SET note = "contrast case (Storyspace)", confidence = 'medium';
RELATE artifact:compileit->exemplifies->concept:stackware SET confidence = 'high';
RELATE artifact:focal_point->exemplifies->concept:prototyping SET confidence = 'medium';
RELATE artifact:oracle_card->exemplifies->concept:lightweight_database SET confidence = 'medium';
RELATE artifact:beehive->exemplifies->concept:lightweight_database SET note = "stacks as control planes", confidence = 'medium';
RELATE artifact:ibooks_author->exemplifies->concept:author_player_politics SET confidence = 'high';
RELATE artifact:hypercard_player->exemplifies->concept:author_player_politics SET confidence = 'high';
RELATE artifact:decker->exemplifies->concept:stackware SET note = "living stackware, 2020s", confidence = 'high';
RELATE artifact:wyldcard->exemplifies->concept:q_preservation SET confidence = 'medium';
RELATE artifact:stacksmith->exemplifies->concept:q_preservation SET confidence = 'high';
RELATE artifact:hypercard_3->exemplifies->concept:succession_failure SET confidence = 'high';
RELATE artifact:sk8->exemplifies->concept:succession_failure SET confidence = 'high';
RELATE artifact:javascript->exemplifies->concept:q_message_path_modern SET confidence = 'medium';

-- enables ------------------------------------------------------------------
RELATE concept:script_property->enables->concept:end_user_programming SET confidence = 'high';
RELATE concept:forgiving_semantics->enables->concept:end_user_programming SET confidence = 'high';
RELATE concept:message_box->enables->concept:learnability_first SET confidence = 'high';
RELATE concept:chunk_expression->enables->concept:lightweight_database SET confidence = 'high';
RELATE concept:find_mark_sort->enables->concept:lightweight_database SET confidence = 'high';
RELATE concept:card->enables->concept:lightweight_database SET note = "card-as-record", confidence = 'high';
RELATE concept:free_distribution->enables->concept:stackware SET confidence = 'high';
RELATE concept:single_file_unity->enables->concept:stackware SET note = "distributable by copying", confidence = 'high';
RELATE concept:tool_palette->enables->concept:art_exhibition SET confidence = 'medium';
RELATE concept:fixed_canvas->enables->concept:presentation SET note = "card-per-slide", confidence = 'medium';
RELATE concept:xcmd->enables->concept:prototyping SET confidence = 'medium';
RELATE concept:home_stack->enables->concept:art_exhibition SET note = "kiosk re-skinning", confidence = 'medium';
RELATE concept:immediate_persistence->enables->concept:education SET confidence = 'medium';

-- motivates ----------------------------------------------------------------
RELATE concept:end_user_programming->motivates->concept:hypertalk SET confidence = 'high';
RELATE concept:learnability_first->motivates->concept:message_box SET confidence = 'high';
RELATE concept:learnability_first->motivates->concept:natural_language_syntax SET confidence = 'high';
RELATE concept:learnability_first->motivates->concept:forgiving_semantics SET confidence = 'high';
RELATE concept:end_user_programming->motivates->concept:free_distribution SET confidence = 'high';
RELATE concept:property_model->motivates->concept:prototyping SET confidence = 'medium';
RELATE concept:learnability_first->motivates->concept:q_constraints SET confidence = 'medium';

-- applied_to ---------------------------------------------------------------
RELATE artifact:hypercard->applied_to->concept:education SET confidence = 'high';
RELATE artifact:hypercard->applied_to->concept:hypermedia_publishing SET confidence = 'high';
RELATE artifact:hypercard->applied_to->concept:prototyping SET confidence = 'high';
RELATE artifact:hypercard->applied_to->concept:lightweight_database SET confidence = 'high';
RELATE artifact:hypercard->applied_to->concept:art_exhibition SET confidence = 'high';
RELATE artifact:hypercard->applied_to->concept:presentation SET confidence = 'medium';
RELATE concept:hypertalk->applied_to->concept:education SET confidence = 'medium';
RELATE artifact:decker->applied_to->concept:art_exhibition SET confidence = 'medium';
RELATE artifact:beehive->applied_to->concept:prototyping SET confidence = 'medium';
RELATE artifact:oracle_card->applied_to->concept:lightweight_database SET confidence = 'high';
RELATE artifact:hyperstudio->applied_to->concept:education SET confidence = 'high';

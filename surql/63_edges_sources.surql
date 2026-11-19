-- Provenance: sources about nodes; nodes documented in sources.

RELATE source:s_wiki_hypercard->about->artifact:hypercard;
RELATE source:s_wiki_hypercard->about->artifact:hypercard_2;

RELATE source:s_wiki_hypercard->about->artifact:hypercard_3;
RELATE source:s_wiki_hypercard->about->artifact:hypercard_player;
RELATE source:s_wiki_hypercard->about->artifact:sk8;
RELATE source:s_wiki_hypercard->about->artifact:myst;
RELATE source:s_wiki_hypercard->about->artifact:ibooks_author;
RELATE source:s_wiki_hypercard->about->concept:macro_virus;
RELATE source:s_wiki_hypercard->about->concept:succession_failure;
RELATE source:s_wiki_hypercard->about->org:claris;
RELATE source:s_wiki_hypertalk->about->concept:hypertalk;
RELATE source:s_wiki_hypertalk->about->concept:xtalk_family;
RELATE source:s_wiki_hypertalk->about->concept:hypertalk_compiler;
RELATE source:s_wiki_hypertalk->about->artifact:javascript;
RELATE source:s_wired_2002->about->person:bill_atkinson;
RELATE source:s_wired_2002->about->concept:web_displacement;
RELATE source:s_wired_2002->about->concept:free_distribution;
RELATE source:s_wired_2002->about->event:e_atkinson_wired_2002;
RELATE source:s_beyondloom->about->artifact:decker;
RELATE source:s_beyondloom->about->concept:message_path;
RELATE source:s_beyondloom->about->concept:single_file_unity;
RELATE source:s_beyondloom->about->person:john_earnest;
RELATE source:s_hypercard_org->about->artifact:hypercard_org;
RELATE source:s_hypercard_org->about->concept:q_preservation;
RELATE source:s_loc_stack->about->concept:q_preservation;
RELATE source:s_loc_stack->about->concept:stack;



RELATE source:s_dobbs_xcmd->about->concept:xcmd;
RELATE source:s_dobbs_xcmd->about->concept:interpreter_performance;

RELATE source:s_goodman_handbook->about->concept:learnability_first;
RELATE source:s_goodman_handbook->about->concept:hypertalk;


RELATE source:s_victor_learnable->about->concept:learnability_first;
RELATE source:s_victor_learnable->about->concept:forgiving_semantics;
RELATE source:s_nielsen_critique->about->person:jakob_nielsen;
RELATE source:s_nielsen_critique->about->concept:no_inline_text_links;
RELATE source:s_nielsen_critique->about->concept:region_link;





RELATE source:s_hypercard_center->about->concept:stackware;
RELATE source:s_hypercard_center->about->artifact:decker;





RELATE source:s_internet_archive_hc->about->event:e_emulation_2017;
RELATE source:s_internet_archive_hc->about->concept:q_preservation;
RELATE source:s_stack_format_lorenzi->about->artifact:hypercardpreview;
RELATE source:s_stack_format_lorenzi->about->person:pierre_lorenzi;







RELATE artifact:hypercard->documented_in->source:s_wiki_hypercard;
RELATE artifact:hypercard->documented_in->source:s_wired_2002;
RELATE concept:hypertalk->documented_in->source:s_wiki_hypertalk;
RELATE concept:stack->documented_in->source:s_loc_stack;
RELATE concept:stack->documented_in->source:s_hypercard_org;

RELATE concept:xcmd->documented_in->source:s_dobbs_xcmd;
RELATE concept:learnability_first->documented_in->source:s_goodman_handbook;
RELATE concept:no_inline_text_links->documented_in->source:s_nielsen_critique;
RELATE concept:web_displacement->documented_in->source:s_wired_2002;
RELATE concept:q_preservation->documented_in->source:s_hypercard_org;
RELATE concept:q_preservation->documented_in->source:s_internet_archive_hc;

RELATE artifact:decker->documented_in->source:s_beyondloom;
RELATE concept:message_path->documented_in->source:s_beyondloom;
RELATE concept:macro_virus->documented_in->source:s_wiki_hypercard;

RELATE concept:forgiving_semantics->documented_in->source:s_victor_learnable;
RELATE concept:stackware->documented_in->source:s_hypercard_center;






-- Reference + code-corpus sources --------------------------------------------
RELATE source:s_hc_script_guide->about->concept:hypertalk;
RELATE source:s_jaed_tricks->about->concept:hypertalk;
RELATE source:s_mystextract->about->artifact:myst;
RELATE source:s_antlr_hypertalk->about->concept:hypertalk;
RELATE concept:hypertalk->documented_in->source:s_antlr_hypertalk;
RELATE concept:hypertalk->documented_in->source:s_hc_script_guide;
RELATE concept:chunk_expression->documented_in->source:s_hc_script_guide;
RELATE concept:target_me_it->documented_in->source:s_jaed_tricks;
RELATE concept:message_path->documented_in->source:s_jaed_tricks;
RELATE artifact:myst->documented_in->source:s_mystextract;

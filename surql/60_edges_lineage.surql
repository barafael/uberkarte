-- Lineage edges: influence, derivation, succession, authorship, ownership.

-- Extra people surfaced by the artifact/source layer

UPSERT person:pierre_lorenzi SET name = "Pierre Lorenzi", role = "HyperCardPreview author; format researcher", confidence = 'medium',
  bio = "Author of HyperCardPreview (a macOS stack viewer) and independent stack-format documentation cross-checking hypercard.org.";


-- influenced ---------------------------------------------------------------
RELATE concept:memex->influenced->concept:hypertext SET note = "Bush's associative trails are Nelson's explicit starting point", confidence = 'high';
RELATE person:vannevar_bush->influenced->concept:memex SET note = "As We May Think (1945)", confidence = 'high';
RELATE person:ted_nelson->influenced->concept:hypertext SET note = "coined and programmed it, 1965", confidence = 'high';




RELATE concept:hypertext->influenced->artifact:hypercard SET confidence = 'high';





RELATE person:alan_kay->influenced->artifact:hypercard SET note = "Smalltalk/Dynabook environment worldview; Kay at Apple in the era", confidence = 'medium';
RELATE person:alan_kay->influenced->concept:end_user_programming SET confidence = 'high';
RELATE person:doug_engelbart->influenced->concept:end_user_programming SET confidence = 'high';
RELATE event:e_demo_1968->influenced->concept:end_user_programming SET confidence = 'medium';



RELATE artifact:hypercard->influenced->artifact:myst SET note = "prototype idiom, and the original 1993 Mac release was HyperCard stacks (see the code layer); later ports reimplemented the engine", confidence = 'medium';
RELATE artifact:hypercard->influenced->artifact:supercard SET confidence = 'high';

RELATE artifact:hypercard->influenced->artifact:toolbook SET confidence = 'high';
RELATE artifact:hypercard->influenced->artifact:director SET note = "Lingo (1990) is HyperTalk-derived", confidence = 'medium';
RELATE concept:hypertalk->influenced->artifact:javascript SET note = "Eich cites HyperTalk directly; onmouseup et al.", confidence = 'high';
RELATE concept:hypertalk->influenced->concept:applescript SET confidence = 'medium';
RELATE artifact:hypercard->influenced->concept:wiki SET note = "Cunningham's wiki grew from HyperCard stack browsing", confidence = 'high';
RELATE artifact:hypercard->influenced->concept:tiddlywiki SET confidence = 'medium';
RELATE artifact:hypercard->influenced->concept:local_first SET note = "conceptual ancestry, via the stack's properties", confidence = 'medium';
RELATE artifact:hypercard->influenced->concept:single_file_html_app SET confidence = 'medium';
RELATE artifact:hypercard->influenced->concept:dynamicland SET note = "explicit lineage claim", confidence = 'medium';
RELATE artifact:hypercard->influenced->artifact:decker SET confidence = 'high';
RELATE artifact:hypercard->influenced->artifact:sk8 SET note = "Apple Advanced Technology Group environment, HyperTalk-influenced; never shipped", confidence = 'high';
RELATE artifact:hypercard->influenced->artifact:ibooks_author SET note = "Atkinson, 2017 (as cited by Wikipedia): 'I think it's the closest thing we have today to what HyperCard was', with superior video and distribution tools", confidence = 'medium';
RELATE artifact:hypercard->influenced->artifact:livecode SET note = "MetaCard was HyperCard-inspired; LiveCode imports stacks", confidence = 'high';
RELATE concept:xtalk_family->influenced->concept:q_modern_stack SET confidence = 'medium';

-- derived_from -------------------------------------------------------------
RELATE artifact:hypercard_2->derived_from->artifact:hypercard SET confidence = 'high';

RELATE artifact:hypercard_3->derived_from->artifact:hypercard_2 SET confidence = 'high';
RELATE artifact:hypercard_player->derived_from->artifact:hypercard SET note = "stripped runtime of the same codebase", confidence = 'high';
RELATE artifact:sk8->derived_from->concept:hypertalk SET note = "HyperTalk descendant language", confidence = 'medium';
RELATE artifact:supercard->derived_from->artifact:hypercard SET confidence = 'high';

RELATE artifact:toolbook->derived_from->artifact:hypercard SET confidence = 'high';
RELATE artifact:livecode->derived_from->artifact:hypercard SET note = "MetaCard lineage + stack import", confidence = 'medium';
RELATE artifact:openxtalk->derived_from->artifact:livecode SET confidence = 'high';
RELATE artifact:decker->derived_from->artifact:hypercard SET note = "re-imagining, not clone", confidence = 'high';
RELATE artifact:wyldcard->derived_from->artifact:hypercard SET note = "behavior-faithful reimplementation of 2.4.1", confidence = 'high';
RELATE artifact:stacksmith->derived_from->artifact:hypercard SET note = "built on the format", confidence = 'high';
RELATE artifact:hypercardpreview->derived_from->artifact:hypercard SET confidence = 'high';


RELATE artifact:javascript->derived_from->concept:hypertalk SET note = "language-level derivation", confidence = 'high';
RELATE concept:applescript->derived_from->concept:hypertalk SET confidence = 'medium';

-- succeeded ----------------------------------------------------------------
RELATE artifact:hypercard_2->succeeded->artifact:hypercard SET confidence = 'high';
RELATE artifact:hypercard_3->succeeded->artifact:hypercard_2 SET note = "cancelled before shipping", confidence = 'high';
RELATE artifact:sk8->succeeded->artifact:hypercard SET note = "never shipped; public domain", confidence = 'high';
RELATE artifact:ibooks_author->succeeded->artifact:hypercard SET note = "partial heir, per Atkinson's 2017 remark (via Wikipedia; the primary video was not reviewed here)", confidence = 'medium';

-- created_by / contributed_to / owned_by -----------------------------------
RELATE artifact:hypercard->created_by->person:bill_atkinson SET note = "creator code WILD", confidence = 'high';
RELATE concept:hypertalk->created_by->person:dan_winkler SET confidence = 'high';
RELATE artifact:hypercard_2->created_by->person:kevin_calhoun SET note = "engineering lead", confidence = 'high';
RELATE artifact:hypercard_3->created_by->person:dan_crow SET confidence = 'high', note = "led development through the late 1990s (Wikipedia); the 1996 alpha was shown by Calhoun";











RELATE artifact:decker->created_by->person:john_earnest SET confidence = 'high';
RELATE artifact:stacksmith->created_by->person:uli_kusterer SET confidence = 'high';
RELATE artifact:hypercardpreview->created_by->person:pierre_lorenzi SET confidence = 'medium';

RELATE concept:memex->created_by->person:vannevar_bush SET confidence = 'high';

RELATE concept:hypertext->created_by->person:ted_nelson SET confidence = 'high';

RELATE person:kevin_calhoun->contributed_to->artifact:hypercard SET confidence = 'high';
RELATE person:dan_crow->contributed_to->artifact:hypercard SET confidence = 'high';
RELATE person:dan_winkler->contributed_to->artifact:hypercard SET confidence = 'high';



RELATE artifact:hypercard->owned_by->org:apple;
RELATE artifact:hypercard_2->owned_by->org:apple;

RELATE artifact:hypercard_3->owned_by->org:apple;
RELATE artifact:sk8->owned_by->org:apple;
RELATE artifact:hypercard_player->owned_by->org:claris;
RELATE artifact:livecode->owned_by->org:livecode;

-- located_in (content living inside a runtime) ------------------------------
RELATE artifact:whole_earth_catalog->located_in->artifact:hypercard SET note = "shipped as a stack", confidence = 'high';


RELATE artifact:compileit->located_in->artifact:hypercard SET note = "ran as an XCMD", confidence = 'high';





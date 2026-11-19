-- HyperTalk language-semantics subgraph: the operational layer that sits under
-- the design-level concepts. These are the nodes a reimplementation consults
-- when the design docs run out.

UPSERT concept:container_model SET
  title    = "Container model (value / text / formattedText)",
  kind     = 'language',
  year     = 1987,
  confidence = 'high',
  tags     = ['semantics','values','reference'],
  summary  = "One abstraction owns every writable thing in HyperTalk: the container. Fields, buttons, variables, the selection, menus, even files hold a value; text and formattedText are the plain and style-carrying faces of that value.",
  body     = "HyperTalk has no streams, no pointers, no getters: everything you can read or write is a container, and every container answers to the same verbs — put (into/before/after), get, set, select, and the chunk expressions that address inside it. The Script Language Guide's chapter on Values lists the members: fields, buttons, variables, the pseudo-variable it, the selection, menus, the message box. Containers have faces: the value is what put and get trade by default; the text property is the plain-text contents; the formattedText property (HyperCard 2.x) returns the text plus the style commands needed to rebuild its fonts and styles — rich text as round-trippable source. The faces matter at the seams: putting into the text of a field replaces plain text, formattedText carries styling, and the value of a container is what a bare container name means in an expression. Chunk expressions make containers recursive — a field contains lines contain words contain characters, and every level is itself a container you can put into. The modern descendant is the DOM's value / textContent / innerHTML trichotomy on form elements and nodes: the same three questions (what is selected, what is shown, what is the markup) answered three times, with the same footguns about which face an assignment touches.";

UPSERT concept:evaluation_modes SET
  title    = "Evaluation modes (the, variables, and the evaluator)",
  kind     = 'language',
  year     = 1987,
  confidence = 'high',
  tags     = ['semantics','reference','parsing'],
  summary  = "How HyperTalk decides what a bare word means: with the it is a property or zero-argument function; without it a variable, and failing that a literal — plus value() and do as the explicit doors back into the evaluator.",
  body     = "HyperTalk's natural-language surface hides a resolution order. A bare identifier is a variable if one by that name exists, otherwise a literal word; properties and zero-argument functions must be prefixed with the (put the date into fld \"X\"), while parameterized functions must NOT be (get sin(22), never get the sin(22)). Omit the where it is required and HyperTalk does not error — it silently treats the word as a variable or literal, the single most beginner-hostile behavior in the language, documented at length in Tricks of the HyperTalk Masters. In container position (the target of put/set/select) an expression may also be a container description returned as text — the selectedLine property yields \"line 2 to 3 of card field 'Listbox'\", which select then accepts as a container. The message box runs the same evaluator but treats every name as global. And two doors let text re-enter the machine on purpose: value(\"2+2\") evaluates an expression, do executes statements — the mechanism behind click-to-run fields, generated code, and the security naivete the corpus records. Every reimplementation of HyperTalk has to re-decide each of these micro-rules; they are nowhere in the marketing descriptions of the language, and they are exactly what the specifies edges from wyldcard, SK8, and the xTalk descendants point at.";

UPSERT concept:coercion_ladder SET
  title    = "Coercion ladder (implicit string/number conversion)",
  kind     = 'language',
  year     = 1987,
  confidence = 'high',
  tags     = ['semantics','types','reference'],
  summary  = "Every value is text that may also be a number: arithmetic coerces text to number, concatenation coerces number to text, and the is a predicates let scripts inspect what a value can become.",
  body     = "HyperTalk values carry no fixed type. A container holds text; when a number is needed the text is parsed (the string \"3\" times 5 is 15), and when text is needed the number is rendered (5 & \"3\" is \"53\", 5 && \"3\" is \"5 3\"). Conversion is invisible and total in both directions until it is not: HyperTalk raises an error only when the text cannot be read as the number a numeric operator demands. Comparison follows the same logic — two numeric-looking operands compare numerically, everything else compares as text, which is why sort offers explicit text/numeric/datetime styles. The is a / is not a predicates (is a number, integer, point, rect, date, logical) are the introspection half: scripts ask what a value could become, not what it is. The chunk system rides the same ladder — character 2 of 1234 is legal and yields \"2\", because numbers coerce to text before chunking (a forgiving-semantics showcase). The design bargain is learnability-first: authors never declare, cast, or box anything, and in exchange every operator carries a hidden coercion step. JavaScript inherited the bargain almost exactly, including the surprise of context-dependent comparison.";

UPSERT concept:operator_precedence SET
  title    = "Operator precedence table",
  kind     = 'language',
  year     = 1987,
  confidence = 'high',
  tags     = ['semantics','reference'],
  summary  = "HyperTalk's one non-English corner: a ten-level precedence ladder where concatenation binds tighter than comparison and and/or bind loosest of all.",
  body     = "Appendix E of the HyperCard Script Language Guide fixes the order: (1) grouping; (2) unary minus, not, there is a / there is not an; (3) ^; (4) * / div mod; (5) + -; (6) & and &&; (7) > < <= >= and the verbal comparisons contains, is in, is within, is a; (8) = is / is not <>; (9) and; (10) or. Two consequences authors meet early: concatenation binds tighter than comparison, so a & b = c compares (a&b) to c — usually what the sentence meant; and and/or are not short-circuiting — both operands always evaluate, so a guard like if x is not empty and value(x) > 0 evaluates value() even on empty text. The antlr grammar mirrors the table in its expression rule alternatives, which makes the grammar a second, machine-readable copy of the same semantics. For any reimplementation this table is load-bearing: it is the part of HyperTalk that is not English and must be specified rather than intuited.";

-- Composition: these are the operational layer of the language ---------------
RELATE concept:container_model->part_of->concept:hypertalk SET `position` = 4, note = "the value layer of the language";
RELATE concept:evaluation_modes->part_of->concept:hypertalk SET `position` = 5, note = "how expressions resolve";
RELATE concept:coercion_ladder->part_of->concept:hypertalk SET `position` = 6, note = "the type story behind the natural-language surface";
RELATE concept:operator_precedence->part_of->concept:hypertalk SET `position` = 7, note = "the non-English corner of the grammar";
RELATE concept:chunk_expression->part_of->concept:container_model SET note = "chunks are how containers nest inside containers";
RELATE concept:coercion_ladder->part_of->concept:evaluation_modes SET note = "coercion is one rung of the resolution story";
RELATE concept:forgiving_semantics->motivates->concept:container_model SET note = "everything-is-a-container is what makes lenient input possible", confidence = 'medium';

-- Modern analogs --------------------------------------------------------------
RELATE concept:container_model->maps_to->concept:javascript_dom SET note = "value / textContent / innerHTML answer the same three questions about a DOM node", confidence = 'medium';
RELATE concept:coercion_ladder->maps_to->concept:javascript_dom SET note = "JS implicit coercion is the direct heir of the string/number ladder", confidence = 'medium';

-- Documentation and specification --------------------------------------------
RELATE concept:container_model->documented_in->source:s_hc_script_guide;
RELATE concept:evaluation_modes->documented_in->source:s_hc_script_guide;
RELATE concept:evaluation_modes->documented_in->source:s_jaed_tricks;
RELATE concept:coercion_ladder->documented_in->source:s_hc_script_guide;
RELATE concept:operator_precedence->documented_in->source:s_hc_script_guide;
RELATE concept:operator_precedence->documented_in->source:s_antlr_hypertalk;

-- Reimplementations that pin these semantics down -----------------------------
RELATE artifact:wyldcard->specifies->concept:hypertalk SET note = "behavior-faithful reimplementation of 2.4.1 forces an operational reading of every rule", confidence = 'high';
RELATE artifact:sk8->specifies->concept:hypertalk SET note = "a from-scratch HyperTalk in Java re-decides each micro-rule", confidence = 'high';
RELATE artifact:livecode->specifies->concept:hypertalk SET note = "Transcript is a superset; its divergences mark where HyperTalk semantics were under-specified", confidence = 'medium';
RELATE artifact:openxtalk->specifies->concept:hypertalk SET note = "the open-source xTalk fork keeps the semantics alive", confidence = 'medium';
RELATE artifact:compileit->specifies->concept:hypertalk_compiler SET note = "compiles a HyperTalk dialect to 68k machine code", confidence = 'high';

-- Thread wiring ---------------------------------------------------------------
RELATE concept:container_model->in_thread->thread:t_revival_preservation SET `position` = 17;
RELATE concept:evaluation_modes->in_thread->thread:t_revival_preservation SET `position` = 18;
RELATE concept:coercion_ladder->in_thread->thread:t_revival_preservation SET `position` = 19;
RELATE concept:operator_precedence->in_thread->thread:t_revival_preservation SET `position` = 20;

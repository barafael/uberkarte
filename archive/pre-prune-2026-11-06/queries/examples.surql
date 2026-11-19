-- Example traversals. Run with:
--   /home/rafael/.surrealdb/surreal sql --endpoint ws://127.0.0.1:8123 \
--     --username root --password root --namespace hypercard --database corpus < queries/examples.surql

-- 1. Counts by table
SELECT count() FROM concept GROUP ALL;
SELECT count() FROM artifact GROUP ALL;
SELECT count() FROM person GROUP ALL;
SELECT count() FROM event GROUP ALL;
SELECT count() FROM source GROUP ALL;
SELECT count() FROM thread GROUP ALL;

-- 2. Who created HyperCard? (edge points artifact ->created_by-> person)
SELECT ->created_by->person.* AS creators FROM artifact:hypercard;

-- 3. The message path, in order
SELECT meta::id(`in`) AS level, `position`, note FROM part_of
  WHERE `out` = concept:message_path AND `position` != NONE ORDER BY `position`;

-- 4. What influenced HyperCard, and what did HyperCard influence?
SELECT <-influenced<-influenced.concept.* AS ancestors FROM artifact:hypercard;
SELECT ->influenced->influenced.artifact.* AS descendants FROM artifact:hypercard;

-- 5. Full-text search across bodies
SELECT meta::id(id) AS id, string::slice(summary, 0, 80) AS s
  FROM concept WHERE body @@ 'hypertext' LIMIT 10;

-- 6. Tensions and the questions they raise
SELECT meta::id(`in`) AS tension, meta::id(`out`) AS question FROM raises;

-- 7. Which modern analogs address which tensions
SELECT meta::id(`in`) AS analog, meta::id(`out`) AS tension FROM addresses;

-- 8. Thread membership, ordered
SELECT meta::id(`in`) AS member, `position` FROM in_thread
  WHERE `out` = thread:t_hypertext_lineage ORDER BY `position`;

-- 9. Two-hop lineage walk from the memex (pure edge following)
SELECT ->influenced->influenced->influenced.concept.* AS three_hops FROM concept:memex;

-- 10. Everything about the stack concept, one hop in every direction
SELECT
  <-part_of.part.* AS components,
  <-limits.limits.* AS limited_by,
  <-maps_to.maps_to.* AS mapped_from,
  ->part_of.part_of.* AS belongs_to,
  ->documented_in.documented_in.source.* AS docs
FROM concept:stack;

-- == Reference + code corpus =================================================

-- 11. Counts for the new tables
SELECT count() FROM keyword GROUP ALL;
SELECT count() FROM code GROUP ALL;
SELECT count() FROM uses GROUP ALL;
SELECT count() FROM demonstrates GROUP ALL;

-- 12. Which code samples use a keyword (and what they demonstrate)
SELECT word, <-uses<-code.title AS samples FROM keyword:cmd_put;

-- 13. Which samples demonstrate a concept, with attribution
SELECT <-demonstrates<-code.{title, kind, attribution} AS samples
  FROM concept:chunk_expression;

-- 14. Full-text search over the verbatim scripts, with highlighting
SELECT title, search::highlight('<em>','</em>',1) AS h
  FROM code WHERE script @@ 'mouseUp';

-- 15. The keyword vocabulary of one category
SELECT word, summary FROM keyword WHERE category = 'message' ORDER BY word;

-- 16. Keywords a single sample uses, grouped by category
SELECT title, ->uses->keyword.word AS words, ->uses->keyword.category AS cats
  FROM code:myst_pillars;

-- 17. Where a code sample came from
SELECT title, ->extracted_from->source.{title, url} AS provenance
  FROM code:wp_replacestr;

-- 18. The semantics layer of the language, in reading order
SELECT meta::id(`in`) AS concept, `position` FROM part_of
  WHERE `out` = concept:hypertalk AND `position` >= 4 ORDER BY `position`;

-- 19. Which reimplementations pin down which semantics
SELECT meta::id(`in`) AS artifact, meta::id(`out`) AS concept, note
  FROM specifies;

-- 20. Keywords that instantiate a concept
SELECT <-relates_to<-keyword.word AS keywords, note
  FROM concept:target_me_it;

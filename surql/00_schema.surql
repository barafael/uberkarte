-- HyperCard / HyperTalk knowledge corpus — schema
-- SurrealDB 4.0.0 · namespace `hypercard` · database `corpus`
-- Node tables: concept, person, org, artifact, event, source, thread
-- Edge tables: RELATE-based, listed at the bottom.

-- Full-text analyzer (BM25 over summary/body) ------------------------------
DEFINE ANALYZER OVERWRITE en TOKENIZERS blank, class FILTERS lowercase, snowball(english);

-- Node tables ---------------------------------------------------------------

DEFINE TABLE OVERWRITE concept SCHEMAFULL;
DEFINE FIELD OVERWRITE title      ON concept TYPE string;
DEFINE FIELD OVERWRITE kind       ON concept TYPE string ASSERT $value IN ['object_model','language','practice','philosophy','idea','tension','modern_analog','domain','question'];
DEFINE FIELD OVERWRITE summary    ON concept TYPE string;
DEFINE FIELD OVERWRITE body       ON concept TYPE option<string>;
DEFINE FIELD OVERWRITE confidence ON concept TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE year       ON concept TYPE option<int>;
DEFINE FIELD OVERWRITE tags       ON concept TYPE option<array<string>>;
DEFINE INDEX OVERWRITE concept_kind        ON concept FIELDS kind;
DEFINE INDEX OVERWRITE concept_summary_ft  ON concept FIELDS summary FULLTEXT ANALYZER en BM25(1.2, 0.75) HIGHLIGHTS;
DEFINE INDEX OVERWRITE concept_body_ft     ON concept FIELDS body FULLTEXT ANALYZER en BM25(1.2, 0.75) HIGHLIGHTS;

DEFINE TABLE OVERWRITE person SCHEMAFULL;
DEFINE FIELD OVERWRITE name       ON person TYPE string;
DEFINE FIELD OVERWRITE role       ON person TYPE string;
DEFINE FIELD OVERWRITE bio        ON person TYPE option<string>;
DEFINE FIELD OVERWRITE birth      ON person TYPE option<int>;
DEFINE FIELD OVERWRITE confidence ON person TYPE option<string> ASSERT $value IN ['high','medium','low'];

DEFINE TABLE OVERWRITE org SCHEMAFULL;
DEFINE FIELD OVERWRITE name ON org TYPE string;
DEFINE FIELD OVERWRITE note ON org TYPE option<string>;

DEFINE TABLE OVERWRITE artifact SCHEMAFULL;
DEFINE FIELD OVERWRITE title      ON artifact TYPE string;
DEFINE FIELD OVERWRITE kind       ON artifact TYPE string ASSERT $value IN ['software','language','file_format','publication','archive','hardware','website'];
DEFINE FIELD OVERWRITE summary    ON artifact TYPE string;
DEFINE FIELD OVERWRITE body       ON artifact TYPE option<string>;
DEFINE FIELD OVERWRITE year       ON artifact TYPE option<int>;
DEFINE FIELD OVERWRITE status     ON artifact TYPE option<string>;
DEFINE FIELD OVERWRITE confidence ON artifact TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE INDEX OVERWRITE artifact_summary_ft ON artifact FIELDS summary FULLTEXT ANALYZER en BM25(1.2, 0.75) HIGHLIGHTS;

DEFINE TABLE OVERWRITE event SCHEMAFULL;
DEFINE FIELD OVERWRITE title      ON event TYPE string;
DEFINE FIELD OVERWRITE when       ON event TYPE option<string>;
DEFINE FIELD OVERWRITE summary    ON event TYPE string;
DEFINE FIELD OVERWRITE body       ON event TYPE option<string>;
DEFINE FIELD OVERWRITE confidence ON event TYPE option<string> ASSERT $value IN ['high','medium','low'];

DEFINE TABLE OVERWRITE source SCHEMAFULL;
DEFINE FIELD OVERWRITE title      ON source TYPE string;
DEFINE FIELD OVERWRITE url        ON source TYPE option<string>;
DEFINE FIELD OVERWRITE kind       ON source TYPE string ASSERT $value IN ['article','book','interview','docs','repo','talk','paper','archive_site'];
DEFINE FIELD OVERWRITE reliability ON source TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE note       ON source TYPE option<string>;

DEFINE TABLE OVERWRITE thread SCHEMAFULL;
DEFINE FIELD OVERWRITE title    ON thread TYPE string;
DEFINE FIELD OVERWRITE question ON thread TYPE string;
DEFINE FIELD OVERWRITE body     ON thread TYPE option<string>;

-- Edge tables ---------------------------------------------------------------
-- `in` / `out` are keywords, so they are backticked.

DEFINE TABLE OVERWRITE part_of SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`      ON part_of TYPE record<concept>;
DEFINE FIELD OVERWRITE `out`     ON part_of TYPE record<concept>;
DEFINE FIELD OVERWRITE `position` ON part_of TYPE option<int>;
DEFINE FIELD OVERWRITE note      ON part_of TYPE option<string>;

DEFINE TABLE OVERWRITE implements SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON implements TYPE record;
DEFINE FIELD OVERWRITE `out` ON implements TYPE record<concept>;
DEFINE FIELD OVERWRITE note  ON implements TYPE option<string>;

DEFINE TABLE OVERWRITE exemplifies SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON exemplifies TYPE record<artifact>;
DEFINE FIELD OVERWRITE `out` ON exemplifies TYPE record<concept>;
DEFINE FIELD OVERWRITE note  ON exemplifies TYPE option<string>;

DEFINE TABLE OVERWRITE influenced SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`       ON influenced TYPE record;
DEFINE FIELD OVERWRITE `out`      ON influenced TYPE record;
DEFINE FIELD OVERWRITE note       ON influenced TYPE option<string>;
DEFINE FIELD OVERWRITE confidence ON influenced TYPE option<string> ASSERT $value IN ['high','medium','low'];

DEFINE TABLE OVERWRITE derived_from SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON derived_from TYPE record;
DEFINE FIELD OVERWRITE `out` ON derived_from TYPE record;
DEFINE FIELD OVERWRITE note  ON derived_from TYPE option<string>;

DEFINE TABLE OVERWRITE succeeded SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON succeeded TYPE record<artifact>;
DEFINE FIELD OVERWRITE `out` ON succeeded TYPE record<artifact>;
DEFINE FIELD OVERWRITE note  ON succeeded TYPE option<string>;

DEFINE TABLE OVERWRITE created_by SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON created_by TYPE record;
DEFINE FIELD OVERWRITE `out` ON created_by TYPE record<person>;
DEFINE FIELD OVERWRITE note  ON created_by TYPE option<string>;

DEFINE TABLE OVERWRITE contributed_to SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON contributed_to TYPE record<person>;
DEFINE FIELD OVERWRITE `out` ON contributed_to TYPE record<artifact>;
DEFINE FIELD OVERWRITE note  ON contributed_to TYPE option<string>;

DEFINE TABLE OVERWRITE owned_by SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON owned_by TYPE record<artifact>;
DEFINE FIELD OVERWRITE `out` ON owned_by TYPE record<org>;

DEFINE TABLE OVERWRITE applied_to SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON applied_to TYPE record;
DEFINE FIELD OVERWRITE `out` ON applied_to TYPE record;
DEFINE FIELD OVERWRITE note  ON applied_to TYPE option<string>;

DEFINE TABLE OVERWRITE about SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON about TYPE record;
DEFINE FIELD OVERWRITE `out` ON about TYPE record;

DEFINE TABLE OVERWRITE documented_in SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON documented_in TYPE record;
DEFINE FIELD OVERWRITE `out` ON documented_in TYPE record<source>;

DEFINE TABLE OVERWRITE maps_to SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON maps_to TYPE record<concept>;
DEFINE FIELD OVERWRITE `out` ON maps_to TYPE record<concept>;
DEFINE FIELD OVERWRITE note  ON maps_to TYPE option<string>;

DEFINE TABLE OVERWRITE limits SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON limits TYPE record<concept>;
DEFINE FIELD OVERWRITE `out` ON limits TYPE record<concept>;
DEFINE FIELD OVERWRITE note  ON limits TYPE option<string>;

DEFINE TABLE OVERWRITE motivates SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON motivates TYPE record<concept>;
DEFINE FIELD OVERWRITE `out` ON motivates TYPE record<concept>;
DEFINE FIELD OVERWRITE note  ON motivates TYPE option<string>;

DEFINE TABLE OVERWRITE addresses SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON addresses TYPE record;
DEFINE FIELD OVERWRITE `out` ON addresses TYPE record<concept>;
DEFINE FIELD OVERWRITE note  ON addresses TYPE option<string>;

DEFINE TABLE OVERWRITE raises SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON raises TYPE record<concept>;
DEFINE FIELD OVERWRITE `out` ON raises TYPE record<concept>;
DEFINE FIELD OVERWRITE note  ON raises TYPE option<string>;

DEFINE TABLE OVERWRITE contrasts_with SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON contrasts_with TYPE record;
DEFINE FIELD OVERWRITE `out` ON contrasts_with TYPE record;
DEFINE FIELD OVERWRITE note  ON contrasts_with TYPE option<string>;

DEFINE TABLE OVERWRITE in_thread SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON in_thread TYPE record;
DEFINE FIELD OVERWRITE `out` ON in_thread TYPE record<thread>;
DEFINE FIELD OVERWRITE `position` ON in_thread TYPE option<int>;

DEFINE TABLE OVERWRITE enables SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON enables TYPE record;
DEFINE FIELD OVERWRITE `out` ON enables TYPE record;
DEFINE FIELD OVERWRITE note  ON enables TYPE option<string>;

DEFINE TABLE OVERWRITE located_in SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON located_in TYPE record<artifact>;
DEFINE FIELD OVERWRITE `out` ON located_in TYPE record<artifact>;
DEFINE FIELD OVERWRITE note  ON located_in TYPE option<string>;

-- confidence on all edge tables
DEFINE FIELD OVERWRITE confidence ON implements TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON exemplifies TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON derived_from TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON succeeded TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON created_by TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON contributed_to TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON applied_to TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON about TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON documented_in TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON maps_to TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON limits TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON motivates TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON addresses TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON raises TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON contrasts_with TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON enables TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE confidence ON located_in TYPE option<string> ASSERT $value IN ['high','medium','low'];

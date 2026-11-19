-- Social / institutional relations and provenance annotations.
-- Added in response to notes/coverage-critique.md §5 (schema and edge-level gaps).
-- Loaded after 01 and before any data: all later files may use these tables.

-- worked_with: person <-> person (direction is incidental; query both ways)
DEFINE TABLE OVERWRITE worked_with SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`        ON worked_with TYPE record<person>;
DEFINE FIELD OVERWRITE `out`       ON worked_with TYPE record<person>;
DEFINE FIELD OVERWRITE note        ON worked_with TYPE option<string>;
DEFINE FIELD OVERWRITE confidence  ON worked_with TYPE option<string> ASSERT $value IN ['high','medium','low'];

-- employed_by: person -> org (note carries the span / role)
DEFINE TABLE OVERWRITE employed_by SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`        ON employed_by TYPE record<person>;
DEFINE FIELD OVERWRITE `out`       ON employed_by TYPE record<org>;
DEFINE FIELD OVERWRITE note        ON employed_by TYPE option<string>;
DEFINE FIELD OVERWRITE confidence  ON employed_by TYPE option<string> ASSERT $value IN ['high','medium','low'];

-- participated_in: person|org -> event (actors finally act)
DEFINE TABLE OVERWRITE participated_in SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`        ON participated_in TYPE record<person|org>;
DEFINE FIELD OVERWRITE `out`       ON participated_in TYPE record<event>;
DEFINE FIELD OVERWRITE note        ON participated_in TYPE option<string>;
DEFINE FIELD OVERWRITE confidence  ON participated_in TYPE option<string> ASSERT $value IN ['high','medium','low'];

-- org <-> org
DEFINE TABLE OVERWRITE spun_off_from SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`        ON spun_off_from TYPE record<org>;
DEFINE FIELD OVERWRITE `out`       ON spun_off_from TYPE record<org>;
DEFINE FIELD OVERWRITE note        ON spun_off_from TYPE option<string>;
DEFINE FIELD OVERWRITE confidence  ON spun_off_from TYPE option<string> ASSERT $value IN ['high','medium','low'];

DEFINE TABLE OVERWRITE renamed_to SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`        ON renamed_to TYPE record<org>;
DEFINE FIELD OVERWRITE `out`       ON renamed_to TYPE record<org>;
DEFINE FIELD OVERWRITE note        ON renamed_to TYPE option<string>;
DEFINE FIELD OVERWRITE confidence  ON renamed_to TYPE option<string> ASSERT $value IN ['high','medium','low'];

-- published_by: artifact -> org (distribution actor, distinct from owned_by / created_by)
DEFINE TABLE OVERWRITE published_by SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`        ON published_by TYPE record<artifact>;
DEFINE FIELD OVERWRITE `out`       ON published_by TYPE record<org>;
DEFINE FIELD OVERWRITE note        ON published_by TYPE option<string>;
DEFINE FIELD OVERWRITE confidence  ON published_by TYPE option<string> ASSERT $value IN ['high','medium','low'];

-- institutional authorship: created_by may now point at an org (Cyan made Myst)
DEFINE FIELD OVERWRITE `out` ON created_by TYPE record<person|org>;

-- provenance edges may now say how strong the support is, and why
DEFINE FIELD OVERWRITE note ON about         TYPE option<string>;
DEFINE FIELD OVERWRITE note ON documented_in TYPE option<string>;

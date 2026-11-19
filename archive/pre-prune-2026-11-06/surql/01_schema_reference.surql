-- Extension schema: HyperTalk language reference (keywords) + code corpus.
-- Loaded after 00_schema.surql. Node tables: keyword, code.
-- Edge tables: uses, demonstrates, extracted_from, defined_in, relates_to, specifies.

-- keyword: one-line reference entries for the HyperTalk vocabulary.
DEFINE TABLE OVERWRITE keyword SCHEMAFULL;
DEFINE FIELD OVERWRITE word      ON keyword TYPE string;
DEFINE FIELD OVERWRITE category  ON keyword TYPE string ASSERT $value IN ['command','function','control','chunk','container','operator','constant','message','property','special'];
DEFINE FIELD OVERWRITE summary   ON keyword TYPE string;
DEFINE FIELD OVERWRITE aliases   ON keyword TYPE option<array<string>>;
DEFINE FIELD OVERWRITE body      ON keyword TYPE option<string>;
DEFINE FIELD OVERWRITE since     ON keyword TYPE option<string>;
DEFINE FIELD OVERWRITE confidence ON keyword TYPE option<string> ASSERT $value IN ['high','medium','low'];
DEFINE FIELD OVERWRITE tags      ON keyword TYPE option<array<string>>;
DEFINE INDEX OVERWRITE keyword_category   ON keyword FIELDS category;
DEFINE INDEX OVERWRITE keyword_summary_ft ON keyword FIELDS summary FULLTEXT ANALYZER en BM25(1.2, 0.75) HIGHLIGHTS;
DEFINE INDEX OVERWRITE keyword_body_ft    ON keyword FIELDS body FULLTEXT ANALYZER en BM25(1.2, 0.75) HIGHLIGHTS;

-- code: HyperTalk programs/handlers recovered from the internet, each with
-- attribution and a source node.
DEFINE TABLE OVERWRITE code SCHEMAFULL;
DEFINE FIELD OVERWRITE title       ON code TYPE string;
DEFINE FIELD OVERWRITE kind        ON code TYPE string ASSERT $value IN ['handler','function','pattern','idiom','script'];
DEFINE FIELD OVERWRITE script      ON code TYPE string;
DEFINE FIELD OVERWRITE summary     ON code TYPE string;
DEFINE FIELD OVERWRITE notes       ON code TYPE option<string>;
DEFINE FIELD OVERWRITE attribution ON code TYPE option<string>;
DEFINE FIELD OVERWRITE year        ON code TYPE option<int>;
DEFINE FIELD OVERWRITE tags        ON code TYPE option<array<string>>;
DEFINE INDEX OVERWRITE code_kind        ON code FIELDS kind;
DEFINE INDEX OVERWRITE code_summary_ft  ON code FIELDS summary FULLTEXT ANALYZER en BM25(1.2, 0.75) HIGHLIGHTS;
DEFINE INDEX OVERWRITE code_script_ft   ON code FIELDS script FULLTEXT ANALYZER en BM25(1.2, 0.75) HIGHLIGHTS;

-- Edge tables ---------------------------------------------------------------

-- code --uses--> keyword
DEFINE TABLE OVERWRITE uses SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON uses TYPE record<code>;
DEFINE FIELD OVERWRITE `out` ON uses TYPE record<keyword>;
DEFINE FIELD OVERWRITE note  ON uses TYPE option<string>;

-- code --demonstrates--> concept
DEFINE TABLE OVERWRITE demonstrates SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON demonstrates TYPE record<code>;
DEFINE FIELD OVERWRITE `out` ON demonstrates TYPE record<concept>;
DEFINE FIELD OVERWRITE note  ON demonstrates TYPE option<string>;
DEFINE FIELD OVERWRITE confidence ON demonstrates TYPE option<string> ASSERT $value IN ['high','medium','low'];

-- code --extracted_from--> source
DEFINE TABLE OVERWRITE extracted_from SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON extracted_from TYPE record<code>;
DEFINE FIELD OVERWRITE `out` ON extracted_from TYPE record<source>;
DEFINE FIELD OVERWRITE note  ON extracted_from TYPE option<string>;

-- keyword --defined_in--> concept:hypertalk (the language that defines them)
DEFINE TABLE OVERWRITE defined_in SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON defined_in TYPE record<keyword>;
DEFINE FIELD OVERWRITE `out` ON defined_in TYPE record<concept>;
DEFINE FIELD OVERWRITE note  ON defined_in TYPE option<string>;

-- keyword --relates_to--> concept (which corpus concept a keyword instantiates)
DEFINE TABLE OVERWRITE relates_to SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON relates_to TYPE record<keyword>;
DEFINE FIELD OVERWRITE `out` ON relates_to TYPE record<concept>;
DEFINE FIELD OVERWRITE note  ON relates_to TYPE option<string>;

-- artifact --specifies--> concept (a reimplementation/extension that pins down semantics)
DEFINE TABLE OVERWRITE specifies SCHEMAFULL TYPE RELATION;
DEFINE FIELD OVERWRITE `in`  ON specifies TYPE record<artifact>;
DEFINE FIELD OVERWRITE `out` ON specifies TYPE record<concept>;
DEFINE FIELD OVERWRITE note  ON specifies TYPE option<string>;
DEFINE FIELD OVERWRITE confidence ON specifies TYPE option<string> ASSERT $value IN ['high','medium','low'];

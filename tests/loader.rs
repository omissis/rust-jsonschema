use jsonschema::loader::{FileLoader, Loader};

macro_rules! assert_loader_from_path_fail {
    ($name:ident, $i:expr, $o:expr) => {
        #[test]
        fn $name() {
            let loadr = FileLoader::new();

            let res = loadr.load($i);

            assert!(res.is_err());
            assert_eq!(res.err().unwrap().to_string(), $o);
        }
    };
}

assert_loader_from_path_fail!(
    loader_from_path_empty_path,
    "",
    "Failed to open the file: No such file or directory (os error 2)"
);

assert_loader_from_path_fail!(
    loader_from_path_wrong_path,
    "/wrong.txt",
    "Failed to open the file: No such file or directory (os error 2)"
);

assert_loader_from_path_fail!(
    loader_from_path_wrong_schema_format,
    "tests/test_data/loader/schema.wrong.format",
    "Failed to parse minimal schema from file: Unsupported file format: tests/test_data/loader/schema.wrong"
);

assert_loader_from_path_fail!(
    loader_from_path_broken_schema_json,
    "tests/test_data/loader/schema.broken.json",
    "Failed to parse minimal schema from file: Failed to parse the json file: control character (\\u0000-\\u001F) found while parsing a string at line 3 column 0"
);

assert_loader_from_path_fail!(
    loader_from_path_broken_schema_yaml,
    "tests/test_data/loader/schema.broken.yaml",
    "Failed to parse minimal schema from file: Failed to parse the yaml file: found unexpected end of stream at line 2 column 1, while scanning a quoted scalar at line 1 column 14"
);

assert_loader_from_path_fail!(
    loader_from_path_wrong_schema_json,
    "tests/test_data/loader/schema.wrong.json",
    "Invalid schema draft uri: https://json-schema.org/draft/wrong/schema"
);

assert_loader_from_path_fail!(
    loader_from_path_not_implemented_schema_json,
    "tests/test_data/loader/schema.not_implemented.json",
    "Schema draft is not implemented yet: http://json-schema.org/draft-00/schema#"
);

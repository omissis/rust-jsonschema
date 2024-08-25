pub mod draft2020_12;

use std::fmt::Debug;

pub const DRAFT00: &str = "http://json-schema.org/draft-00/schema#";
pub const DRAFT00_ALT: &str = "https://json-schema.org/draft-00/schema";
pub const DRAFT01: &str = "http://json-schema.org/draft-01/schema#";
pub const DRAFT01_ALT: &str = "https://json-schema.org/draft-01/schema";
pub const DRAFT02: &str = "http://json-schema.org/draft-02/schema#";
pub const DRAFT02_ALT: &str = "https://json-schema.org/draft-02/schema";
pub const DRAFT03: &str = "http://json-schema.org/draft-03/schema#";
pub const DRAFT03_ALT: &str = "https://json-schema.org/draft-03/schema";
pub const DRAFT04: &str = "http://json-schema.org/draft-04/schema#";
pub const DRAFT04_ALT: &str = "https://json-schema.org/draft-04/schema";
pub const DRAFT05: &str = "http://json-schema.org/draft-05/schema#";
pub const DRAFT05_ALT: &str = "https://json-schema.org/draft-05/schema";
pub const DRAFT06: &str = "http://json-schema.org/draft-06/schema#";
pub const DRAFT06_ALT: &str = "https://json-schema.org/draft-06/schema";
pub const DRAFT07: &str = "http://json-schema.org/draft-07/schema#";
pub const DRAFT07_ALT: &str = "https://json-schema.org/draft-07/schema";
pub const DRAFT2019_09: &str = "https://json-schema.org/draft/2019-09/schema";
pub const DRAFT2020_12: &str = "https://json-schema.org/draft/2020-12/schema";

pub fn exists(draft: &str) -> bool {
    matches!(
        draft,
        DRAFT00
            | DRAFT00_ALT
            | DRAFT01
            | DRAFT01_ALT
            | DRAFT02
            | DRAFT02_ALT
            | DRAFT03
            | DRAFT03_ALT
            | DRAFT04
            | DRAFT04_ALT
            | DRAFT05
            | DRAFT05_ALT
            | DRAFT06
            | DRAFT06_ALT
            | DRAFT07
            | DRAFT07_ALT
            | DRAFT2019_09
            | DRAFT2020_12
    )
}

pub fn all() -> Vec<&'static str> {
    vec![
        DRAFT00,
        DRAFT00_ALT,
        DRAFT01,
        DRAFT01_ALT,
        DRAFT02,
        DRAFT02_ALT,
        DRAFT03,
        DRAFT03_ALT,
        DRAFT04,
        DRAFT04_ALT,
        DRAFT05,
        DRAFT05_ALT,
        DRAFT06,
        DRAFT06_ALT,
        DRAFT07,
        DRAFT07_ALT,
        DRAFT2019_09,
        DRAFT2020_12,
    ]
}

pub trait HasSchemaUri: Debug {
    fn schema_uri(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exists() {
        assert!(exists("http://json-schema.org/draft-00/schema#"));
        assert!(exists("https://json-schema.org/draft-00/schema"));
        assert!(exists("http://json-schema.org/draft-01/schema#"));
        assert!(exists("https://json-schema.org/draft-01/schema"));
        assert!(exists("http://json-schema.org/draft-02/schema#"));
        assert!(exists("https://json-schema.org/draft-02/schema"));
        assert!(exists("http://json-schema.org/draft-03/schema#"));
        assert!(exists("https://json-schema.org/draft-03/schema"));
        assert!(exists("http://json-schema.org/draft-04/schema#"));
        assert!(exists("https://json-schema.org/draft-04/schema"));
        assert!(exists("http://json-schema.org/draft-05/schema#"));
        assert!(exists("https://json-schema.org/draft-05/schema"));
        assert!(exists("http://json-schema.org/draft-06/schema#"));
        assert!(exists("https://json-schema.org/draft-06/schema"));
        assert!(exists("http://json-schema.org/draft-07/schema#"));
        assert!(exists("https://json-schema.org/draft-07/schema"));
        assert!(exists("https://json-schema.org/draft/2019-09/schema"));
        assert!(exists("https://json-schema.org/draft/2020-12/schema"));

        assert!(!exists("http://nonexistent-schema.org/schema#"));
    }

    #[test]
    fn test_all() {
        assert!(all().len() == 18);
        assert_eq!(
            all(),
            vec![
                DRAFT00,
                DRAFT00_ALT,
                DRAFT01,
                DRAFT01_ALT,
                DRAFT02,
                DRAFT02_ALT,
                DRAFT03,
                DRAFT03_ALT,
                DRAFT04,
                DRAFT04_ALT,
                DRAFT05,
                DRAFT05_ALT,
                DRAFT06,
                DRAFT06_ALT,
                DRAFT07,
                DRAFT07_ALT,
                DRAFT2019_09,
                DRAFT2020_12,
            ]
        );
    }
}

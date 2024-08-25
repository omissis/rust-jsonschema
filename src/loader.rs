use crate::draft::draft2020_12::Schema as Draft2020_12Schema;
use crate::draft::{self, HasSchemaUri};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::{fmt::Debug, fs::File, io::Seek, result::Result};

#[derive(Deserialize, Debug)]
struct MinimalSchema {
    #[serde(rename = "$schema")]
    pub schema: String,
}

impl draft::HasSchemaUri for MinimalSchema {
    fn schema_uri(&self) -> &str {
        self.schema.as_str()
    }
}

pub trait Loader {
    fn load(&self, path: &str) -> Result<Box<dyn draft::HasSchemaUri>, String>;
}

pub struct FakeLoader {
    draft_2020_12: Box<Draft2020_12Schema>,
    r#type: String,
}

impl FakeLoader {
    pub fn from_2020_12(schema: Draft2020_12Schema) -> Self {
        Self {
            draft_2020_12: Box::new(schema),
            r#type: draft::DRAFT2020_12.to_string(),
        }
    }
}

impl Loader for FakeLoader {
    fn load(&self, _: &str) -> Result<Box<dyn draft::HasSchemaUri>, String> {
        match self.r#type.as_str() {
            draft::DRAFT2020_12 => Ok(self.draft_2020_12.clone()),
            _ => Err(format!(
                "Schema draft is not implemented yet: {}",
                self.r#type.as_str()
            )),
        }
    }
}

pub struct FileLoader;

impl FileLoader {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for FileLoader {
    fn default() -> Self {
        FileLoader::new()
    }
}

impl Loader for FileLoader {
    fn load(&self, path: &str) -> Result<Box<dyn draft::HasSchemaUri>, String> {
        let mut file = match File::open(path) {
            Ok(file_handle) => file_handle,
            Err(error) => return Err(format!("Failed to open the file: {}", error)),
        };

        let res: MinimalSchema = match deserialize(path, &file) {
            Ok(value) => value,
            Err(error) => {
                return Err(format!(
                    "Failed to parse minimal schema from file: {}",
                    error
                ))
            }
        };

        if !draft::exists(res.schema_uri()) {
            return Err(format!("Invalid schema draft uri: {}", res.schema_uri()));
        }

        match file.rewind() {
            Ok(_) => (),
            Err(error) => return Err(format!("Failed to rewind the file: {}", error)),
        }

        match res.schema_uri() {
            draft::DRAFT2020_12 => match deserialize::<Draft2020_12Schema>(path, &file) {
                Ok(value) => Ok(Box::new(value)),
                Err(error) => Err(format!(
                    "Failed to parse full schema (draft 2020-12) from file: {}",
                    error
                )),
            },
            _ => Err(format!(
                "Schema draft is not implemented yet: {}",
                res.schema_uri()
            )),
        }
    }
}

fn deserialize<T: DeserializeOwned>(path: &str, file: &File) -> Result<T, String> {
    if path.ends_with(".json") {
        match serde_json::from_reader::<&File, T>(file) {
            Ok(value) => return Ok(value),
            Err(error) => return Err(format!("Failed to parse the json file: {}", error)),
        };
    }

    if path.ends_with(".yaml") || path.ends_with(".yml") {
        match serde_yaml::from_reader::<&File, T>(file) {
            Ok(value) => return Ok(value),
            Err(error) => return Err(format!("Failed to parse the yaml file: {}", error)),
        };
    }

    Err(format!(
        "Unsupported file format: {}",
        path.rsplit_once(".").unwrap().0
    ))
}

#[macro_use]
extern crate serde;

use std::env;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct LicenseReplace {
    pub year: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct License {
    pub id: String,
    pub replace: Option<LicenseReplace>,
    pub copyright: Option<String>,
    pub optional: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Exception {
    pub id: String,
    pub with: Option<Vec<String>>,
    pub optional: Option<Vec<String>>,
}

impl fmt::Debug for License {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let optional = if let Some(optional) = &self.optional {
            format!("Some(&{:?})", optional)
        } else {
            "None".to_owned()
        };
        write!(
            f,
            "License {{ id: {:?}, replace: {:?}, copyright: {:?}, optional: {} }}",
            &self.id, &self.replace, &self.copyright, optional
        )
    }
}

impl fmt::Debug for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let with = if let Some(with) = &self.with {
            format!("Some(&{:?})", with)
        } else {
            "None".to_owned()
        };
        let optional = if let Some(optional) = &self.optional {
            format!("Some(&{:?})", optional)
        } else {
            "None".to_owned()
        };
        write!(
            f,
            "Exception {{ id: {:?}, with: {}, optional: {} }}",
            &self.id, with, optional
        )
    }
}

pub fn get_root_path() -> PathBuf {
    let cargo_manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not defined");
    let mut root_path = PathBuf::from(cargo_manifest_dir)
        .canonicalize()
        .expect("Invalid Cargo manifest directory");
    if !root_path.pop() {
        panic!("Can't find root path.");
    }
    root_path
}

pub fn get_resources_path() -> PathBuf {
    let mut resources_path = get_root_path();
    resources_path.push("resources");
    resources_path
}

pub fn parse_licenses() -> Vec<License> {
    eprintln!("Parsing licenses.json...");
    let mut licenses_json_path = get_resources_path();
    licenses_json_path.push("licenses.json");
    let licenses_json = File::open(&licenses_json_path).expect("Can't read licenses.json");
    serde_json::from_reader(licenses_json).expect("Can't parse licenses.json")
}

pub fn parse_exceptions() -> Vec<Exception> {
    eprintln!("Parsing exceptions.json...");
    let mut exceptions_json_path = get_resources_path();
    exceptions_json_path.push("exceptions.json");
    let exceptions_json = File::open(&exceptions_json_path).expect("Can't read exceptions.json");
    serde_json::from_reader(exceptions_json).expect("Can't parse exceptions.json")
}

#[cfg(test)]
mod tests {
    mod parse {
        use crate::{parse_exceptions, parse_licenses};

        #[test]
        fn works() {
            let licenses = parse_licenses();
            let exceptions = parse_exceptions();

            assert!(!licenses.is_empty());
            assert!(!exceptions.is_empty());
        }
    }
}

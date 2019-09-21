#[macro_use]
extern crate serde;

use flate2::write::GzEncoder;
use flate2::Compression;
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct LicenseReplace {
    year: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct License {
    id: String,
    replace: Option<LicenseReplace>,
    copyright: Option<Vec<usize>>,
}

fn fetch_url_to_string(url: &str, client: &Client, token: &Option<String>) -> String {
    let request = client.get(url);
    if let Some(token) = token {
        request.header("Authorization", format!("token {}", token))
    } else {
        request
    }
    .send()
    .expect("Can't fetch url")
    .text()
    .expect("Can't convert fetched contents to string")
}

fn gz_encode_str(src: &str, level: Compression) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), level);
    encoder
        .write_all(src.as_bytes())
        .expect("Can't write to gzip encoder");
    encoder.finish().expect("Can't finish gzip encoding")
}

fn main() {
    let mut module_contents =
        "{ let mut hm: HashMap<&'static str, &'static [u8]> = HashMap::new(); ".to_owned();

    let licenses_file_contents =
        fs::read_to_string("./resources/licenses.json").expect("Can't read licenses.json");
    let licenses: Vec<License> =
        serde_json::from_str(&licenses_file_contents).expect("Can't deserialized licenses.json");

    let http_client = Client::new();
    let gh_token = if let Ok(token) = env::var("GITHUB_TOKEN") {
        Some(token)
    } else {
        None
    };

    for license in licenses {
        eprintln!("Processing info for {}...", &license.id);

        let mut contents = String::new();
        match license.source {
            LicenseSource::Url => {
                let fetched_contents = fetch_url_to_string(&license.value, &http_client, &None);
                contents.push_str(&fetched_contents);
            }
            LicenseSource::Github => {
                let fetched_contents = fetch_url_to_string(
                    &format!("https://api.github.com/licenses/{}", &license.value),
                    &http_client,
                    &gh_token,
                );
                let fetched_json: Value = serde_json::from_str(&fetched_contents)
                    .expect("Can't deserialized fetched GitHub license info");
                contents.push_str(
                    fetched_json["body"]
                        .as_str()
                        .expect("Can't find body field on fetched GitHub license info"),
                );
            }
        }
        if let Some(replace) = license.replace {
            if let Some(year) = replace.year {
                contents = contents.replace(&year, "{{ year }}");
            }
            if let Some(name) = replace.name {
                contents = contents.replace(&name, "{{ name }}");
            }
        }
        contents = contents.replace("\r", "");
        if !contents.ends_with("\n") {
            contents.push('\n');
        }

        let gz_contents = gz_encode_str(&contents, Compression::best());
        let to_insert = format!(
            "hm.insert({:?}, &{:?});",
            &license.id,
            gz_contents.as_slice()
        );
        module_contents.push_str(&to_insert);
    }

    module_contents.push_str(" hm };");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not defined");
    let dest_path = Path::new(&out_dir).join("licenses.rs");
    let mut out_file = File::create(&dest_path).expect("Can't create output file");
    out_file
        .write_all(module_contents.as_bytes())
        .expect("Can't write to output file");

    eprint!("{}", &module_contents);
}

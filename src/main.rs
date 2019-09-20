#[macro_use]
extern crate lazy_static;

use chrono::{Datelike, Utc};
use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::env;
use std::io;
use std::io::Read;
use std::process;

#[derive(Debug)]
struct LaunchArgs {
    help: bool,
    list: bool,
    id: Option<String>,
    name: Option<String>,
}

lazy_static! {
    static ref EMPTY_STRING: String = String::new();
    static ref LICENSES: HashMap<&'static str, &'static [u8]> =
        include!(concat!(env!("OUT_DIR"), "/licenses.rs"));
}

fn gz_decode_bytes(src: &[u8]) -> io::Result<String> {
    let mut decoder = GzDecoder::new(src);
    let mut result = String::new();
    decoder.read_to_string(&mut result)?;
    Ok(result)
}

fn main() {
    let help = env::args().any(|arg| arg == "-h".to_owned() || arg == "--help".to_owned());
    let list = env::args().any(|arg| arg == "-l".to_owned() || arg == "--list".to_owned());

    let id_and_name: Vec<String> = env::args()
        .skip(1)
        .filter(|arg| !arg.starts_with("-") && !arg.starts_with("--"))
        .collect();
    let (id, name) = id_and_name.split_first().unwrap_or((&EMPTY_STRING, &[]));
    let id = {
        if id.len() > 0 {
            Some(id.clone())
        } else {
            None
        }
    };
    let name = {
        if name.len() > 0 {
            Some(name.join(" "))
        } else {
            None
        }
    };

    let parsed_args = LaunchArgs {
        help,
        list,
        id,
        name,
    };

    if parsed_args.help {
        print!(include_str!("../resources/help.txt"));
    } else if parsed_args.list {
        for key in LICENSES.keys() {
            println!("{}", key);
        }
    } else if let Some(id) = parsed_args.id {
        if let Some(gz_contents) = LICENSES.get(id.as_str()) {
            if let Ok(contents) = gz_decode_bytes(gz_contents) {
                if let Some(name) = parsed_args.name {
                    print!(
                        "{}",
                        &contents
                            .replace("{{ name }}", &name)
                            .replace("{{ year }}", &Utc::today().year().to_string())
                    )
                } else {
                    print!("{}", &contents);
                }
            } else {
                eprintln!("Couldn't decode specified license. Please open an issue at <https://github.com/raftario/licensor>");
                process::exit(1);
            }
        } else {
            eprintln!("Invalid license ID.");

            let similar: Vec<String> = LICENSES
                .keys()
                .filter_map(|key| {
                    let id_lower = id.to_lowercase();
                    let key_lower = key.to_lowercase();

                    if id_lower.contains(&key_lower) || key_lower.contains(&id_lower) {
                        Some(key.to_string())
                    } else {
                        None
                    }
                })
                .collect();
            if similar.len() > 0 {
                eprintln!("Similar IDs: {}.", similar.join(", "));
            }

            process::exit(1);
        }
    } else {
        eprintln!("Invalid arguments. Use -h or --help for usage.");
        process::exit(1);
    }
}

mod codegen;
mod types;

#[macro_use]
extern crate calm_io;

use crate::codegen::{EXCEPTIONS, EXCEPTIONS_INFO, LICENSES, LICENSES_INFO};
use chrono::{Datelike, Utc};
use flate2::read::GzDecoder;
use phf::OrderedMap;
use std::io;
use std::io::Read;
use std::process;
use structopt::clap::ArgGroup;
use structopt::StructOpt;

fn actions_arg_group() -> ArgGroup<'static> {
    ArgGroup::with_name("actions").required(true)
}

#[derive(StructOpt)]
#[structopt(group = actions_arg_group())]
/// write licenses to stdout
///
/// Write a license to standard output given a SPDX expression.
/// A name for the copyright holder can optionally be provided for licenses where a notice is included.
/// If the provided ID isn't found, similar ones will be suggested.
/// Licenses are all compiled into the binary.
struct Args {
    /// Lists available licenses
    #[structopt(short = "l", long = "licenses", group = "actions")]
    list_licenses: bool,

    /// Lists available exceptions
    #[structopt(short = "e", long = "exceptions", group = "actions")]
    list_exceptions: bool,

    /// Keeps the copyright notice even if no name is specified
    #[structopt(short = "p", long = "placeholder")]
    placeholder: bool,

    /// SPDX license ID or expression
    ///
    /// WITH expressions are supported, i.e. "Apache-2.0 WITH LLVM-exception", just make sure to surround them with quotes.
    #[structopt(name = "LICENSE", group = "actions")]
    spdx_expr: Option<String>,

    /// Name of the copyright holder
    ///
    /// Will be used in the copyright notice in licenses where it is present.
    /// Has no effect if the license has no copyright notice.
    #[structopt(name = "NAME")]
    copyright_holder: Option<String>,
}

struct SPDXExpr {
    license: String,
    exception: Option<String>,
}

fn parse_spdx_expr(expr: String) -> io::Result<SPDXExpr> {
    let expr: Vec<&str> = expr.split(" ").collect();
    let len = expr.len();
    if len == 1 {
        Ok(SPDXExpr {
            license: expr[0].to_owned(),
            exception: None,
        })
    } else if len == 3 {
        if expr[1] == "WITH" {
            Ok(SPDXExpr {
                license: expr[0].to_owned(),
                exception: Some(expr[2].to_owned()),
            })
        } else {
            stderrln!(
                "Invalid SPDX expression. Did you mean \"{} WITH {}\"?",
                expr[0],
                expr[2]
            )?;
            process::exit(1)
        }
    } else {
        stderrln!("Invalid SPDX expression.")?;
        process::exit(1)
    }
}

fn gz_decode_bytes(src: &[u8]) -> io::Result<String> {
    let mut decoder = GzDecoder::new(src);
    let mut result = String::new();
    decoder.read_to_string(&mut result)?;
    Ok(result)
}

fn list_licenses() -> io::Result<()> {
    for id in LICENSES.keys() {
        stdoutln!("{}", id)?;
    }
    Ok(())
}

fn list_exceptions() -> io::Result<()> {
    for id in EXCEPTIONS.keys() {
        stdoutln!("{}", id)?;
    }
    Ok(())
}

fn is_similar(l: &str, r: &str) -> bool {
    let ll = l.to_lowercase();
    let lr = r.to_lowercase();
    ll.contains(&lr) || lr.contains(&ll)
}

fn get_similar_keys(id: &str, iter: &OrderedMap<&'static str, &'static [u8]>) -> Vec<String> {
    iter.keys()
        .filter_map(|key| {
            let key = key.to_string();
            if is_similar(id, &key) {
                Some(key)
            } else {
                None
            }
        })
        .collect()
}

fn parse_license(id: &str) -> io::Result<String> {
    if let Some(gz_contents) = LICENSES.get(id) {
        if let Ok(contents) = gz_decode_bytes(gz_contents) {
            Ok(contents)
        } else {
            stdoutln!("Can't decode license.")?;
            unexpected()?;
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Can't decode license.",
            ))
        }
    } else {
        stderrln!("Invalid license ID.")?;

        let similar = get_similar_keys(&id, &LICENSES);
        if similar.len() > 0 {
            stderrln!("Similar IDs: {}.", similar.join(", "))?;
        }

        process::exit(1);
    }
}

fn parse_exception(id: &str) -> io::Result<String> {
    if let Some(gz_contents) = EXCEPTIONS.get(id) {
        if let Ok(contents) = gz_decode_bytes(gz_contents) {
            Ok(contents)
        } else {
            stdoutln!("Can't decode exception.")?;
            unexpected()?;
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Can't decode exception.",
            ))
        }
    } else {
        stderrln!("Invalid exception ID.")?;

        let similar = get_similar_keys(&id, &EXCEPTIONS);
        if similar.len() > 0 {
            stderrln!("Similar IDs: {}.", similar.join(", "))?;
        }

        process::exit(1);
    }
}

fn unexpected() -> io::Result<()> {
    stderrln!("This shouldn't have happened. Please open an issue with the command you entered: <https://github.com/raftario/licensor/issues>.")?;
    process::exit(1);
}

#[pipefail]
fn main() -> io::Result<()> {
    let args = Args::from_args();

    if args.list_licenses {
        return list_licenses();
    }
    if args.list_exceptions {
        return list_exceptions();
    }
    if let Some(expr) = args.spdx_expr {
        let expr = parse_spdx_expr(expr)?;
        let mut valid_exception = true;

        let mut license = parse_license(&expr.license)?;
        license = license.replace('\r', "");
        if !license.ends_with('\n') {
            license.push('\n');
        }

        if let Some(exception_id) = &expr.exception {
            let mut exception = parse_exception(exception_id)?;
            exception = exception.replace('\r', "");

            let max_length = {
                let license_lines: Vec<&str> = license.split('\n').collect();
                let mut max_length = 80;
                for line in license_lines {
                    let length = line.len();
                    if length > max_length {
                        max_length = length;
                    }
                }
                max_length
            };
            exception = textwrap::fill(&exception, max_length);

            if !exception.ends_with('\n') {
                exception.push('\n');
            }

            license.push('\n');
            license.push('\n');
            license.push_str(&exception);

            if let Some(exception_info) = EXCEPTIONS_INFO.get(exception_id.as_str()) {
                if let Some(with) = &exception_info.with {
                    if !with.iter().any(|l| l == &expr.license) {
                        valid_exception = false;
                    }
                }
            }
        }

        if let Some(license_info) = LICENSES_INFO.get(expr.license.as_str()) {
            if let Some(name) = args.copyright_holder {
                if let Some(replace) = &license_info.replace {
                    if let Some(replace_year) = replace.year {
                        let year = Utc::today().year().to_string();
                        license = license.replace(replace_year, &year);
                    }
                    if let Some(replace_name) = replace.name {
                        license = license.replace(replace_name, &name);
                    }
                }
            } else if let Some(copyright) = license_info.copyright {
                if !args.placeholder {
                    license = license.replace(copyright, "");
                }
            }
        }

        stdout!("{}", license)?;

        if !valid_exception {
            stderrln!("This exception wasn't designed to be used with this license. Please consider using another license or removing it.")?;
        }
    } else {
        stderrln!("Invalid arguments.")?;
        return unexpected();
    }

    Ok(())
}

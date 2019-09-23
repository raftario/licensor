mod codegen;

#[macro_use]
extern crate calm_io;

use crate::codegen::{EXCEPTIONS, EXCEPTIONS_INFO, LICENSES, LICENSES_INFO};
use chrono::{Datelike, Utc};
use flate2::read::GzDecoder;
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

    /// Skips the copyright notice in licenses where it is present
    ///
    /// The Berne convention makes that notice optional.
    #[structopt(short = "c", long = "no-copyright")]
    no_copyright: bool,

    /// SPDX license ID or expression
    ///
    /// WITH expressions are supported, i.e. "Apache-2.0 WITH LLVM-exception", just make sure to surround them with quotes.
    #[structopt(name = "LICENSE", group = "actions")]
    spdx_expr: Option<String>,

    /// Name of the copyright holder
    ///
    /// Will be used in the copyright notice in licenses where it is present.
    /// If the license has no copyright notice or the no-copyright option is specified, has no effect.
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
            // TODO: use stderrln! once it's fixed
            stdoutln!(
                "Invalid SPDX expression. Did you mean \"{} WITH {}\"?",
                expr[0],
                expr[2]
            )?;
            process::exit(1)
        }
    } else {
        // TODO: use stderrln! once it's fixed
        stdoutln!("Invalid SPDX expression.")?;
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

fn unexpected() -> io::Result<!> {
    // TODO: use stderrln! once it's fixed
    stdoutln!("This shouldn't have happened. Please open an issue with the command you entered: <https://github.com/raftario/licensor/issues>.")?;
    process::exit(1)
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
        if let Some(exception) = expr.exception {
            // TODO
        } else {
            // TODO
        }
    } else {
        // TODO: use stderrln! once it's fixed
        stdoutln!("Invalid arguments.")?;
        return unexpected();
    }

    Ok(())
}

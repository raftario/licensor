mod codegen;

use crate::codegen::{EXCEPTIONS, EXCEPTIONS_INFO, LICENSES, LICENSES_INFO};
use chrono::{Datelike, Utc};
use flate2::read::GzDecoder;
use std::io;
use std::io::Read;
use std::process;
use structopt::clap::ArgGroup;
use structopt::StructOpt;

fn gz_decode_bytes(src: &[u8]) -> io::Result<String> {
    let mut decoder = GzDecoder::new(src);
    let mut result = String::new();
    decoder.read_to_string(&mut result)?;
    Ok(result)
}

fn actions_arg_group() -> ArgGroup<'static> {
    ArgGroup::with_name("actions").required(true)
}

#[derive(Debug, StructOpt)]
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
    spdx_expression: Option<String>,

    /// Name of the copyright holder
    ///
    /// Will be used in the copyright notice in licenses where it is present.
    /// If the license has no copyright notice or the no-copyright option is specified, has no effect.
    #[structopt(name = "NAME")]
    copyright_holder: Option<String>,
}

fn main() {
    let args = Args::from_args();
    println!("{:?}", args);
}

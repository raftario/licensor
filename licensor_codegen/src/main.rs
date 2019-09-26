use phf_codegen::OrderedMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn get_path() -> PathBuf {
    let mut path = licensor_common::get_root_path();
    path.push("src");
    path.push("codegen.rs");
    path
}

fn main() {
    eprintln!("Writing codegen.rs...");

    let path = get_path();
    let mut file = File::create(&path).expect("Can't create codegen.rs.");

    write!(
        &mut file,
        "// This is an automatically generated file, do not edit it.\n\n"
    )
    .expect("Can't write to codegen.rs.");
    write!(&mut file, "#![allow(clippy::unreadable_literal)]\n\n")
        .expect("Can't write to codegen.rs.");
    write!(
        &mut file,
        "use crate::types::{{License, Exception, LicenseReplace}};\nuse phf::OrderedMap;\n\n"
    )
    .expect("Can't write to codegen.rs.");

    let licenses = licensor_common::parse_licenses();
    let exceptions = licensor_common::parse_exceptions();

    let mut licenses_builder = OrderedMap::new();
    let mut licenses_info_builder = OrderedMap::new();
    for license in &licenses {
        licenses_builder.entry(
            license.id.as_str(),
            &format!(
                "include_bytes!(\"../resources/licenses/{}.txt.gz\")",
                license.id
            ),
        );
        licenses_info_builder.entry(license.id.as_str(), &format!("{:?}", license));
    }

    write!(
        &mut file,
        "pub static LICENSES: OrderedMap<&'static str, &'static [u8]> = "
    )
    .expect("Can't write to codegen.rs.");
    licenses_builder
        .build(&mut file)
        .expect("Can't write to codegen.rs.");
    write!(&mut file, ";\n").expect("Can't write to codegen.rs.");
    write!(
        &mut file,
        "pub static LICENSES_INFO: OrderedMap<&'static str, License> = "
    )
    .expect("Can't write to codegen.rs.");
    licenses_info_builder
        .build(&mut file)
        .expect("Can't write to codegen.rs.");
    write!(&mut file, ";\n").expect("Can't write to codegen.rs.");

    let mut exceptions_builder = OrderedMap::new();
    let mut exceptions_info_builder = OrderedMap::new();
    for exception in &exceptions {
        exceptions_builder.entry(
            exception.id.as_str(),
            &format!(
                "include_bytes!(\"../resources/exceptions/{}.txt.gz\")",
                exception.id
            ),
        );
        exceptions_info_builder.entry(exception.id.as_str(), &format!("{:?}", exception));
    }

    write!(
        &mut file,
        "pub static EXCEPTIONS: OrderedMap<&'static str, &'static [u8]> = "
    )
    .expect("Can't write to codegen.rs.");
    exceptions_builder
        .build(&mut file)
        .expect("Can't write to codegen.rs.");
    write!(&mut file, ";\n").expect("Can't write to codegen.rs.");
    write!(
        &mut file,
        "pub static EXCEPTIONS_INFO: OrderedMap<&'static str, Exception> = "
    )
    .expect("Can't write to codegen.rs.");
    exceptions_info_builder
        .build(&mut file)
        .expect("Can't write to codegen.rs.");
    write!(&mut file, ";\n").expect("Can't write to codegen.rs.");
}

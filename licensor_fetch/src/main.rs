use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use sha3::{Digest, Sha3_256};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::{env, fs, io, process};
use tar::Archive;

static LLD_ARCHIVE_URL: &str = "https://github.com/spdx/license-list-data/archive/v3.6.tar.gz";
static LLD_ARCHIVE_HASH: &[u8] = &[
    145, 22, 173, 180, 54, 59, 55, 113, 254, 131, 175, 24, 225, 159, 97, 56, 149, 179, 117, 193,
    32, 251, 216, 100, 23, 102, 251, 47, 57, 96, 160, 121,
];

fn hash_file<P: AsRef<Path> + Debug>(src: P) -> Vec<u8> {
    eprintln!("Hashing {:?}...", src);
    let mut file = File::open(src).expect("Can't read file for hashing");
    let mut hasher = Sha3_256::new();
    io::copy(&mut file, &mut hasher).expect("Can't hash file");
    hasher.result().to_vec()
}

fn download_file<P: AsRef<Path> + Debug>(url: &str, dest: P) {
    eprintln!("Downloading {} to {:?}...", url, dest);
    let mut file = File::create(dest).expect("Can't create destination file");
    let mut response = reqwest::get(url).expect("Can't download file");
    io::copy(&mut response, &mut file).expect("Can't write response to file");
}

fn decode_gz_file<P: AsRef<Path> + Debug, W: Write>(src: P, dest: &mut W) {
    eprintln!("Decoding {:?}...", src);
    let file = File::open(src).expect("Can't read file for decoding");
    let mut decoder = GzDecoder::new(file);
    io::copy(&mut decoder, dest).expect("Can't decode file");
}

fn encode_file_gz<P: AsRef<Path> + Debug, W: Write>(src: P, dest: &mut W, level: Compression) {
    eprintln!("Encoding {:?}...", src);
    let mut file = File::open(src).expect("Can't read file for encoding");
    let mut encoder = GzEncoder::new(dest, level);
    io::copy(&mut file, &mut encoder).expect("Can't encode file");
}

fn main() {
    let cargo_manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not defined");
    let mut resources_path = PathBuf::from(cargo_manifest_dir)
        .canonicalize()
        .expect("Invalid Cargo manifest directory");
    resources_path.pop();
    resources_path.push("resources");

    let mut lld_archive_path = resources_path.clone();
    lld_archive_path.push("license-list-data-3.6.tar.gz");
    let mut lld_archive_ok = false;
    if lld_archive_path.is_file() {
        eprintln!("Found license list archive. Checking hash...");
        let hash = hash_file(&lld_archive_path);
        if hash.as_slice() == LLD_ARCHIVE_HASH {
            lld_archive_ok = true;
        } else {
            eprintln!("License list archive hash doesn't match expected one.");
        }
    }

    if !lld_archive_ok {
        download_file(LLD_ARCHIVE_URL, &lld_archive_path);
        let hash = hash_file(&lld_archive_path);
        if hash.as_slice() != LLD_ARCHIVE_HASH {
            eprintln!("Downloaded license list archive hash doesn't match expected one. Please try again and report the issue if it reappears.");
            process::exit(1);
        }
    }

    let mut decoded_archive: Vec<u8> = Vec::new();
    decode_gz_file(&lld_archive_path, &mut decoded_archive);

    let licenses = licensor_common::parse_licenses();
    let exceptions = licensor_common::parse_exceptions();

    let mut parsed_licenses: Vec<String> = Vec::new();
    let mut parsed_exceptions: Vec<String> = Vec::new();

    let mut lld_archive = Archive::new(decoded_archive.as_slice());
    let mut licenses_path = resources_path.clone();
    licenses_path.push("licenses");
    let mut exceptions_path = resources_path.clone();
    exceptions_path.push("exceptions");
    for file in lld_archive.entries().expect("Can't read archive") {
        let mut file = file.expect("Can't read archive file");

        let filepath = file
            .header()
            .path()
            .expect("Can't read path from archive file")
            .to_path_buf();
        let components: Vec<Component> = filepath.components().collect();
        if components.len() == 3 && components[1].as_os_str() == OsStr::new("text") {
            let filename = components[2]
                .as_os_str()
                .to_str()
                .expect("Can't convert archive file filename to string");

            for license in &licenses {
                if &format!("{}.txt", license.id) == filename
                    || &format!("deprecated_{}.txt", license.id) == filename
                {
                    eprintln!("Parsing license {}", license.id);

                    let mut license_path = licenses_path.clone();
                    license_path.push(format!("{}.txt", license.id));
                    let mut license_file =
                        File::create(&license_path).expect("Can't create license file");
                    io::copy(&mut file, &mut license_file).expect("Can't write license to file");

                    let mut encoded_license_path = licenses_path.clone();
                    encoded_license_path.push(format!("{}.txt.gz", license.id));
                    let mut encoded_license_file = File::create(&encoded_license_path)
                        .expect("Can't create encoded license file");
                    encode_file_gz(
                        &license_path,
                        &mut encoded_license_file,
                        Compression::best(),
                    );

                    parsed_licenses.push(license.id.clone());
                }
            }

            for exception in &exceptions {
                if &format!("{}.txt", exception.id) == filename
                    || &format!("deprecated_{}.txt", exception.id) == filename
                {
                    eprintln!("Parsing exception {}", exception.id);

                    let mut exception_path = exceptions_path.clone();
                    exception_path.push(format!("{}.txt", exception.id));
                    let mut exception_file =
                        File::create(&exception_path).expect("Can't create exception file");
                    io::copy(&mut file, &mut exception_file)
                        .expect("Can't write exception to file");

                    let mut encoded_exception_path = exceptions_path.clone();
                    encoded_exception_path.push(format!("{}.txt.gz", exception.id));
                    let mut encoded_exception_file = File::create(&encoded_exception_path)
                        .expect("Can't create encoded exception file");
                    encode_file_gz(
                        &exception_path,
                        &mut encoded_exception_file,
                        Compression::best(),
                    );

                    parsed_exceptions.push(exception.id.clone());
                }
            }
        }
    }

    if licenses.len() != parsed_licenses.len() {
        eprintln!("Some licenses couldn't be parsed. Check for error in licenses.json.");
        process::exit(1);
    }
    if exceptions.len() != parsed_exceptions.len() {
        eprintln!("Some exceptions couldn't be parsed. Check for error in exceptions.json.");
        process::exit(1);
    }

    eprintln!("Writing list file...");

    let mut list_path = resources_path.clone();
    list_path.pop();
    list_path.push("LIST.md");

    let mut list_contents = "# Available licenses and exceptions\n\n[//]: # (This is an automatically generated file, do not edit it.)\n\n".to_owned();
    list_contents.push_str("## Licenses\n\n");
    for license in &licenses {
        list_contents.push_str(&format!("* {}\n", license.id));
    }
    list_contents.push_str("\n## Exceptions\n\n");
    for exception in &exceptions {
        list_contents.push_str(&format!("* {}\n", exception.id));
    }

    fs::write(&list_path, list_contents.as_bytes()).expect("Couldn't write to list file.");
}

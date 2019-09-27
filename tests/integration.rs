use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn list_licenses() {
    Command::cargo_bin("licensor")
        .unwrap()
        .args(&["-l"])
        .assert()
        .success()
        .stdout(predicates::str::contains("\n"))
        .stderr(predicates::str::is_empty());

    Command::cargo_bin("licensor")
        .unwrap()
        .args(&["--licenses"])
        .assert()
        .success()
        .stdout(predicates::str::contains("\n"))
        .stderr(predicates::str::is_empty());
}

#[test]
fn list_exceptions() {
    Command::cargo_bin("licensor")
        .unwrap()
        .args(&["-e"])
        .assert()
        .success()
        .stdout(predicates::str::contains("\n"))
        .stderr(predicates::str::is_empty());

    Command::cargo_bin("licensor")
        .unwrap()
        .args(&["--exceptions"])
        .assert()
        .success()
        .stdout(predicates::str::contains("\n"))
        .stderr(predicates::str::is_empty());
}

#[test]
fn license() {
    Command::cargo_bin("licensor")
        .unwrap()
        .args(&["MIT"])
        .assert()
        .success()
        .stdout(predicates::str::starts_with("MIT License \n"))
        .stderr(predicates::str::is_empty());
}

#[test]
fn license_invalid() {
    Command::cargo_bin("licensor")
        .unwrap()
        .args(&["mit"])
        .assert()
        .failure()
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::starts_with("Invalid license ID.\n"));
}

#[test]
fn license_and_name() {
    Command::cargo_bin("licensor")
        .unwrap()
        .args(&["MIT", "Raphaël Thériault"])
        .assert()
        .success()
        .stdout(predicates::str::starts_with(
            "MIT License Copyright (c) 2019 Raphaël Thériault\n",
        ))
        .stderr(predicates::str::is_empty());
}

#[test]
fn license_with_exception() {
    Command::cargo_bin("licensor")
        .unwrap()
        .args(&["Apache-2.0 WITH LLVM-exception"])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "limitations under the License.\n\n\nLLVM Exceptions to the Apache 2.0 License",
        ))
        .stderr(predicates::str::is_empty());
}

#[test]
fn license_with_exception_invalid() {
    Command::cargo_bin("licensor")
        .unwrap()
        .args(&["Apache-2.0 WITH llvm"])
        .assert()
        .failure()
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::starts_with("Invalid exception ID.\n"));
}

#[test]
fn expr_invalid() {
    Command::cargo_bin("licensor")
        .unwrap()
        .args(&["Apache-2.0 with LLVM-exception"])
        .assert()
        .failure()
        .stdout(predicates::str::is_empty())
        .stderr("Invalid SPDX expression. Did you mean \"Apache-2.0 WITH LLVM-exception\"?\n");
}

#[test]
fn license_keep_placeholder() {
    Command::cargo_bin("licensor")
        .unwrap()
        .args(&["MIT", "-p"])
        .assert()
        .success()
        .stdout(predicates::str::starts_with(
            "MIT License Copyright (c) <year> <copyright holders>\n",
        ))
        .stderr(predicates::str::is_empty());

    Command::cargo_bin("licensor")
        .unwrap()
        .args(&["MIT", "--keep-placeholder"])
        .assert()
        .success()
        .stdout(predicates::str::starts_with(
            "MIT License Copyright (c) <year> <copyright holders>\n",
        ))
        .stderr(predicates::str::is_empty());
}

#[test]
fn license_skip_optional() {
    Command::cargo_bin("licensor")
        .unwrap()
        .args(&["MIT", "-o"])
        .assert()
        .success()
        .stdout(predicates::str::starts_with(
            "Permission is hereby granted, free of charge",
        ))
        .stderr(predicates::str::is_empty());

    Command::cargo_bin("licensor")
        .unwrap()
        .args(&["MIT", "--skip-optional"])
        .assert()
        .success()
        .stdout(predicates::str::starts_with(
            "Permission is hereby granted, free of charge",
        ))
        .stderr(predicates::str::is_empty());
}

#[test]
fn invalid() {
    Command::cargo_bin("licensor")
        .unwrap()
        .assert()
        .failure()
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::contains("USAGE"));
}

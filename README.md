# licensor

write licenses to stdout

[![GitHub Actions](https://github.com/raftario/licensor/workflows/Build/badge.svg)](https://github.com/raftario/licensor/actions?workflowID=Build)

## About

Write a license to standard output given its SPDX ID. A name for the copyright holder can optionally be provided for licenses where it is included. If the provided ID isn't found, similar ones will be suggested. Licenses are all compiled into the binary.

## Why

I just got tired of losing time looking for license files for my new projects.

## Available licenses

* AGPL-3.0
* BSD-3-Clause
* GPL-3.0
* LGPL-2.1
* MPL-2.0
* LGPL-3.0
* MIT
* GPL-2.0
* Apache-2.0
* EPL-2.0
* BSD-2-Clause
* Unlicense

## Adding licenses

If you'd like a license to be added to this list, you can either open an issue for it or add it yourself, which is fairly easy.

To add a license, just add it to [`resources/licenses.json`](./resources/licenses.json) following the schema provided in [`resources/licenses-schema.json`](./resources/licenses-schema.json). If the license is officially available in raw text format, prefer using a direct url. If it isn't use the GitHub license key.

You don't even need to install the Rust toolchain in order to get your newly added license built in, as release binaries are automatically built and archived using GitHub Actions.

## How it works

The build script parses the licenses, then generates code to include them directly into the binary at compile time. You can read more about this [here](https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation).

This makes the binary really light, however, the main downside is that additional licenses cannot be parsed at runtime.

## Licensing

`licensor` is licensed under the [MIT License](./LICENSE).

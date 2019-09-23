// This is an automatically generated file, do not edit it.

use licensor_common::{Exception, License, LicenseReplace};
use phf::OrderedMap;

pub static LICENSES: OrderedMap<&'static str, &'static [u8]> = ::phf::OrderedMap {
    key: 794399669663935756,
    disps: ::phf::Slice::Static(&[(1, 0), (14, 13), (2, 12)]),
    idxs: ::phf::Slice::Static(&[13, 7, 10, 3, 8, 4, 6, 11, 1, 14, 5, 2, 0, 12, 9]),
    entries: ::phf::Slice::Static(&[
        (
            "AGPL-3.0",
            include_bytes!("../resources/licenses/AGPL-3.0.txt.gz"),
        ),
        (
            "Apache-2.0",
            include_bytes!("../resources/licenses/Apache-2.0.txt.gz"),
        ),
        (
            "BSD-2-Clause",
            include_bytes!("../resources/licenses/BSD-2-Clause.txt.gz"),
        ),
        (
            "BSD-3-Clause",
            include_bytes!("../resources/licenses/BSD-3-Clause.txt.gz"),
        ),
        (
            "CDDL-1.0",
            include_bytes!("../resources/licenses/CDDL-1.0.txt.gz"),
        ),
        (
            "EPL-2.0",
            include_bytes!("../resources/licenses/EPL-2.0.txt.gz"),
        ),
        (
            "GPL-2.0",
            include_bytes!("../resources/licenses/GPL-2.0.txt.gz"),
        ),
        (
            "GPL-3.0",
            include_bytes!("../resources/licenses/GPL-3.0.txt.gz"),
        ),
        ("ISC", include_bytes!("../resources/licenses/ISC.txt.gz")),
        (
            "LGPL-2.1",
            include_bytes!("../resources/licenses/LGPL-2.1.txt.gz"),
        ),
        (
            "LGPL-3.0",
            include_bytes!("../resources/licenses/LGPL-3.0.txt.gz"),
        ),
        ("MIT", include_bytes!("../resources/licenses/MIT.txt.gz")),
        (
            "MPL-2.0",
            include_bytes!("../resources/licenses/MPL-2.0.txt.gz"),
        ),
        (
            "Unlicense",
            include_bytes!("../resources/licenses/Unlicense.txt.gz"),
        ),
        ("Zlib", include_bytes!("../resources/licenses/Zlib.txt.gz")),
    ]),
};
pub static LICENSES_INFO: OrderedMap<&'static str, License> = ::phf::OrderedMap {
    key: 794399669663935756,
    disps: ::phf::Slice::Static(&[(1, 0), (14, 13), (2, 12)]),
    idxs: ::phf::Slice::Static(&[13, 7, 10, 3, 8, 4, 6, 11, 1, 14, 5, 2, 0, 12, 9]),
    entries: ::phf::Slice::Static(&[
        (
            "AGPL-3.0",
            License {
                id: "AGPL-3.0",
                replace: None,
                copyright: None,
            },
        ),
        (
            "Apache-2.0",
            License {
                id: "Apache-2.0",
                replace: None,
                copyright: None,
            },
        ),
        (
            "BSD-2-Clause",
            License {
                id: "BSD-2-Clause",
                replace: Some(LicenseReplace {
                    year: Some("<year>"),
                    name: Some("<owner>"),
                }),
                copyright: Some(&[1, 2]),
            },
        ),
        (
            "BSD-3-Clause",
            License {
                id: "BSD-3-Clause",
                replace: Some(LicenseReplace {
                    year: Some("<year>"),
                    name: Some("<owner>"),
                }),
                copyright: Some(&[1, 2]),
            },
        ),
        (
            "CDDL-1.0",
            License {
                id: "CDDL-1.0",
                replace: None,
                copyright: None,
            },
        ),
        (
            "EPL-2.0",
            License {
                id: "EPL-2.0",
                replace: None,
                copyright: None,
            },
        ),
        (
            "GPL-2.0",
            License {
                id: "GPL-2.0",
                replace: None,
                copyright: None,
            },
        ),
        (
            "GPL-3.0",
            License {
                id: "GPL-3.0",
                replace: None,
                copyright: None,
            },
        ),
        (
            "ISC",
            License {
                id: "ISC",
                replace: Some(LicenseReplace {
                    year: Some("1995-2003"),
                    name: Some("by Internet Software Consortium"),
                }),
                copyright: Some(&[1, 2, 3, 4]),
            },
        ),
        (
            "LGPL-2.1",
            License {
                id: "LGPL-2.1",
                replace: None,
                copyright: None,
            },
        ),
        (
            "LGPL-3.0",
            License {
                id: "LGPL-3.0",
                replace: None,
                copyright: None,
            },
        ),
        (
            "MIT",
            License {
                id: "MIT",
                replace: Some(LicenseReplace {
                    year: Some("<year>"),
                    name: Some("<copyright holders>"),
                }),
                copyright: Some(&[1, 2]),
            },
        ),
        (
            "MPL-2.0",
            License {
                id: "MPL-2.0",
                replace: None,
                copyright: None,
            },
        ),
        (
            "Unlicense",
            License {
                id: "Unlicense",
                replace: None,
                copyright: None,
            },
        ),
        (
            "Zlib",
            License {
                id: "Zlib",
                replace: Some(LicenseReplace {
                    year: Some("<year>"),
                    name: Some("<copyright holders>"),
                }),
                copyright: Some(&[1, 2]),
            },
        ),
    ]),
};
pub static EXCEPTIONS: OrderedMap<&'static str, &'static [u8]> = ::phf::OrderedMap {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[(0, 0)]),
    idxs: ::phf::Slice::Static(&[0]),
    entries: ::phf::Slice::Static(&[(
        "LLVM-exception",
        include_bytes!("../resources/exceptions/LLVM-exception.txt.gz"),
    )]),
};
pub static EXCEPTIONS_INFO: OrderedMap<&'static str, Exception> = ::phf::OrderedMap {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[(0, 0)]),
    idxs: ::phf::Slice::Static(&[0]),
    entries: ::phf::Slice::Static(&[(
        "LLVM-exception",
        Exception {
            id: "LLVM-exception",
            with: Some(&["Apache-2.0"]),
        },
    )]),
};

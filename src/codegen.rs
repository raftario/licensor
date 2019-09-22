// This is an automatically generated file, do not edit it.

use licensor_common::{License, Exception, LicenseReplace};
use phf::OrderedMap;

static LICENSES: OrderedMap<&'static str, &'static [u8]> = ::phf::OrderedMap {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[(0, 8), (2, 0), (3, 0)]),
    idxs: ::phf::Slice::Static(&[9, 4, 11, 10, 1, 2, 3, 0, 7, 5, 8, 6]),
    entries: ::phf::Slice::Static(
        &[
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
        ],
    ),
};
static LICENSES_INFO: OrderedMap<&'static str, License> = ::phf::OrderedMap {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[(0, 8), (2, 0), (3, 0)]),
    idxs: ::phf::Slice::Static(&[9, 4, 11, 10, 1, 2, 3, 0, 7, 5, 8, 6]),
    entries: ::phf::Slice::Static(
        &[
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
                    copyright: Some([1, 2]),
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
                    copyright: Some([1, 2]),
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
                    copyright: Some([1, 2]),
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
        ],
    ),
};
static EXCEPTIONS: OrderedMap<&'static str, &'static [u8]> = ::phf::OrderedMap {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[]),
    idxs: ::phf::Slice::Static(&[]),
    entries: ::phf::Slice::Static(&[]),
};
static EXCEPTIONS_INFO: OrderedMap<&'static str, Exception> = ::phf::OrderedMap {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[]),
    idxs: ::phf::Slice::Static(&[]),
    entries: ::phf::Slice::Static(&[]),
};

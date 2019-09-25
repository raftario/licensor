// This is an automatically generated file, do not edit it.

use crate::types::{Exception, License, LicenseReplace};
use phf::OrderedMap;

pub static LICENSES: OrderedMap<&'static str, &'static [u8]> = ::phf::OrderedMap {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[(0, 12), (0, 0), (3, 11)]),
    idxs: ::phf::Slice::Static(&[1, 9, 11, 10, 13, 7, 8, 4, 12, 5, 3, 6, 2, 14, 0]),
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
            "CDDL-1.1",
            include_bytes!("../resources/licenses/CDDL-1.1.txt.gz"),
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
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[
        (0, 12),
        (0, 0),
        (3, 11),
    ]),
    idxs: ::phf::Slice::Static(&[
        1,
        9,
        11,
        10,
        13,
        7,
        8,
        4,
        12,
        5,
        3,
        6,
        2,
        14,
        0,
    ]),
    entries: ::phf::Slice::Static(&[
        ("AGPL-3.0", License { id: "AGPL-3.0", replace: None, copyright: None, optional: Some(&["GNU AFFERO GENERAL PUBLIC LICENSE\n\nVersion 3, 19 November 2007", " END OF\nTERMS AND CONDITIONS\n\nHow to Apply These Terms to Your New Programs\n\nIf you develop a new program, and you want it to be of the greatest possible\nuse to the public, the best way to achieve this is to make it free software\nwhich everyone can redistribute and change under these terms.\n\nTo do so, attach the following notices to the program. It is safest to attach\nthem to the start of each source file to most effectively state the exclusion\nof warranty; and each file should have at least the \"copyright\" line and a\npointer to where the full notice is found.\n\n<one line to give the program\'s name and a brief idea of what it does.>\n\nCopyright (C) <year> <name of author>\n\nThis program is free software: you can redistribute it and/or modify it under\nthe terms of the GNU Affero General Public License as published by the Free\nSoftware Foundation, either version 3 of the License, or (at your option)\nany later version.\n\nThis program is distributed in the hope that it will be useful, but WITHOUT\nANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS\nFOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more\ndetails.\n\nYou should have received a copy of the GNU Affero General Public License along\nwith this program. If not, see <https://www.gnu.org/licenses/>.\n\nAlso add information on how to contact you by electronic and paper mail.\n\nIf your software can interact with users remotely through a computer network,\nyou should also make sure that it provides a way for users to get its source.\nFor example, if your program is a web application, its interface could display\na \"Source\" link that leads users to an archive of the code. There are many\nways you could offer source, and different solutions will be better for different\nprograms; see section 13 for the specific requirements.\n\nYou should also get your employer (if you work as a programmer) or school,\nif any, to sign a \"copyright disclaimer\" for the program, if necessary. For\nmore information on this, and how to apply and follow the GNU AGPL, see <https://www.gnu.org/licenses/>."]) }),
        ("Apache-2.0", License { id: "Apache-2.0", replace: None, copyright: None, optional: Some(&["Apache License\n\nVersion 2.0, January 2004\n\nhttp://www.apache.org/licenses/ TERMS AND CONDITIONS FOR USE, REPRODUCTION,\nAND DISTRIBUTION", " END OF TERMS AND CONDITIONS\n\nAPPENDIX: How to apply the Apache License to your work.\n\nTo apply the Apache License to your work, attach the following boilerplate\nnotice, with the fields enclosed by brackets \"[]\" replaced with your own identifying\ninformation. (Don\'t include the brackets!) The text should be enclosed in\nthe appropriate comment syntax for the file format. We also recommend that\na file or class name and description of purpose be included on the same \"printed\npage\" as the copyright notice for easier identification within third-party\narchives.\n\nCopyright [yyyy] [name of copyright owner]\n\nLicensed under the Apache License, Version 2.0 (the \"License\");\n\nyou may not use this file except in compliance with the License.\n\nYou may obtain a copy of the License at\n\nhttp://www.apache.org/licenses/LICENSE-2.0\n\nUnless required by applicable law or agreed to in writing, software\n\ndistributed under the License is distributed on an \"AS IS\" BASIS,\n\nWITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n\nSee the License for the specific language governing permissions and\n\nlimitations under the License."]) }),
        ("BSD-2-Clause", License { id: "BSD-2-Clause", replace: Some(LicenseReplace { year: Some("<year>"), name: Some("<owner>") }), copyright: Some("Copyright (c) <year> <owner>. All rights reserved."), optional: None }),
        ("BSD-3-Clause", License { id: "BSD-3-Clause", replace: Some(LicenseReplace { year: Some("<year>"), name: Some("<owner>") }), copyright: Some("Copyright (c) <year> <owner>. All rights reserved."), optional: None }),
        ("CDDL-1.1", License { id: "CDDL-1.1", replace: None, copyright: None, optional: Some(&["COMMON DEVELOPMENT AND DISTRIBUTION LICENSE (CDDL)\n\nVersion 1.1"]) }),
        ("EPL-2.0", License { id: "EPL-2.0", replace: None, copyright: None, optional: Some(&["Eclipse Public License - v 2.0"]) }),
        ("GPL-2.0", License { id: "GPL-2.0", replace: None, copyright: None, optional: Some(&["GNU GENERAL PUBLIC LICENSE\n\nVersion 2, June 1991", "END OF TERMS AND CONDITIONS\n\nHow to Apply These Terms to Your New Programs\n\nIf you develop a new program, and you want it to be of the greatest possible\nuse to the public, the best way to achieve this is to make it free software\nwhich everyone can redistribute and change under these terms.\n\nTo do so, attach the following notices to the program. It is safest to attach\nthem to the start of each source file to most effectively convey the exclusion\nof warranty; and each file should have at least the \"copyright\" line and a\npointer to where the full notice is found.\n\n<one line to give the program\'s name and an idea of what it does.>\n\nCopyright (C) < yyyy> <name of author>\n\nThis program is free software; you can redistribute it and/or modify it under\nthe terms of the GNU General Public License as published by the Free Software\nFoundation; either version 2 of the License, or (at your option) any later\nversion.\n\nThis program is distributed in the hope that it will be useful, but WITHOUT\nANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS\nFOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.\n\nYou should have received a copy of the GNU General Public License along with\nthis program; if not, write to the Free Software Foundation, Inc., 51 Franklin\nStreet, Fifth Floor, Boston, MA 02110-1301, USA.\n\nAlso add information on how to contact you by electronic and paper mail.\n\nIf the program is interactive, make it output a short notice like this when\nit starts in an interactive mode:\n\nGnomovision version 69, Copyright (C) year name of author Gnomovision comes\nwith ABSOLUTELY NO WARRANTY; for details type `show w\'. This is free software,\nand you are welcome to redistribute it under certain conditions; type `show\nc\' for details.\n\nThe hypothetical commands `show w\' and `show c\' should show the appropriate\nparts of the General Public License. Of course, the commands you use may be\ncalled something other than `show w\' and `show c\'; they could even be mouse-clicks\nor menu items--whatever suits your program.\n\nYou should also get your employer (if you work as a programmer) or your school,\nif any, to sign a \"copyright disclaimer\" for the program, if necessary. Here\nis a sample; alter the names:\n\nYoyodyne, Inc., hereby disclaims all copyright interest in the program `Gnomovision\'\n(which makes passes at compilers) written by James Hacker.\n\n<signature of Ty Coon>, 1 April 1989 Ty Coon, President of Vice This General\nPublic License does not permit incorporating your program into proprietary\nprograms. If your program is a subroutine library, you may consider it more\nuseful to permit linking proprietary applications with the library. If this\nis what you want to do, use the GNU Lesser General Public License instead\nof this License."]) }),
        ("GPL-3.0", License { id: "GPL-3.0", replace: None, copyright: None, optional: Some(&["GNU GENERAL PUBLIC LICENSE\n\nVersion 3, 29 June 2007", "END OF\nTERMS AND CONDITIONS\n\nHow to Apply These Terms to Your New Programs\n\nIf you develop a new program, and you want it to be of the greatest possible\nuse to the public, the best way to achieve this is to make it free software\nwhich everyone can redistribute and change under these terms.\n\nTo do so, attach the following notices to the program. It is safest to attach\nthem to the start of each source file to most effectively state the exclusion\nof warranty; and each file should have at least the \"copyright\" line and a\npointer to where the full notice is found.\n\n<one line to give the program\'s name and a brief idea of what it does.>\n\nCopyright (C) <year> <name of author>\n\nThis program is free software: you can redistribute it and/or modify it under\nthe terms of the GNU General Public License as published by the Free Software\nFoundation, either version 3 of the License, or (at your option) any later\nversion.\n\nThis program is distributed in the hope that it will be useful, but WITHOUT\nANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS\nFOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.\n\nYou should have received a copy of the GNU General Public License along with\nthis program. If not, see <https://www.gnu.org/licenses/>.\n\nAlso add information on how to contact you by electronic and paper mail.\n\nIf the program does terminal interaction, make it output a short notice like\nthis when it starts in an interactive mode:\n\n<program> Copyright (C) <year> <name of author>\n\nThis program comes with ABSOLUTELY NO WARRANTY; for details type `show w\'.\n\nThis is free software, and you are welcome to redistribute it under certain\nconditions; type `show c\' for details.\n\nThe hypothetical commands `show w\' and `show c\' should show the appropriate\nparts of the General Public License. Of course, your program\'s commands might\nbe different; for a GUI interface, you would use an \"about box\".\n\nYou should also get your employer (if you work as a programmer) or school,\nif any, to sign a \"copyright disclaimer\" for the program, if necessary. For\nmore information on this, and how to apply and follow the GNU GPL, see <https://www.gnu.org/licenses/>.\n\nThe GNU General Public License does not permit incorporating your program\ninto proprietary programs. If your program is a subroutine library, you may\nconsider it more useful to permit linking proprietary applications with the\nlibrary. If this is what you want to do, use the GNU Lesser General Public\nLicense instead of this License. But first, please read <https://www.gnu.org/\nlicenses /why-not-lgpl.html>."]) }),
        ("ISC", License { id: "ISC", replace: Some(LicenseReplace { year: Some("1995-2003"), name: Some("by Internet Software Consortium") }), copyright: Some("Copyright (c) 1995-2003 by Internet Software Consortium"), optional: Some(&["ISC License Copyright (c) 2004-2010 by Internet Systems Consortium, Inc. (\"ISC\")"]) }),
        ("LGPL-2.1", License { id: "LGPL-2.1", replace: None, copyright: None, optional: Some(&["GNU LESSER GENERAL PUBLIC LICENSE\n\nVersion 2.1, February 1999", "END OF TERMS AND CONDITIONS\n\nHow to Apply These Terms to Your New Libraries\n\nIf you develop a new library, and you want it to be of the greatest possible\nuse to the public, we recommend making it free software that everyone can\nredistribute and change. You can do so by permitting redistribution under\nthese terms (or, alternatively, under the terms of the ordinary General Public\nLicense).\n\nTo apply these terms, attach the following notices to the library. It is safest\nto attach them to the start of each source file to most effectively convey\nthe exclusion of warranty; and each file should have at least the \"copyright\"\nline and a pointer to where the full notice is found.\n\n< one line to give the library\'s name and an idea of what it does. >\n\nCopyright (C) < year > < name of author >\n\nThis library is free software; you can redistribute it and/or modify it under\nthe terms of the GNU Lesser General Public License as published by the Free\nSoftware Foundation; either version 2.1 of the License, or (at your option)\nany later version.\n\nThis library is distributed in the hope that it will be useful, but WITHOUT\nANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS\nFOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more\ndetails.\n\nYou should have received a copy of the GNU Lesser General Public License along\nwith this library; if not, write to the Free Software Foundation, Inc., 51\nFranklin Street, Fifth Floor, Boston, MA 02110-1301 USA Also add information\non how to contact you by electronic and paper mail.\n\nYou should also get your employer (if you work as a programmer) or your school,\nif any, to sign a \"copyright disclaimer\" for the library, if necessary. Here\nis a sample; alter the names:\n\nYoyodyne, Inc., hereby disclaims all copyright interest in\n\nthe library `Frob\' (a library for tweaking knobs) written\n\nby James Random Hacker.\n\n< signature of Ty Coon > , 1 April 1990\n\nTy Coon, President of Vice\n\nThat\'s all there is to it!"]) }),
        ("LGPL-3.0", License { id: "LGPL-3.0", replace: None, copyright: None, optional: Some(&["GNU LESSER GENERAL PUBLIC LICENSE\n\nVersion 3, 29 June 2007"]) }),
        ("MIT", License { id: "MIT", replace: Some(LicenseReplace { year: Some("<year>"), name: Some("<copyright holders>") }), copyright: Some("Copyright (c) <year> <copyright holders>"), optional: Some(&["MIT License "]) }),
        ("MPL-2.0", License { id: "MPL-2.0", replace: None, copyright: None, optional: Some(&["Mozilla Public License Version 2.0", " Exhibit A - Source Code Form\nLicense Notice\n\nThis Source Code Form is subject to the terms of the Mozilla Public License,\nv. 2.0. If a copy of the MPL was not distributed with this file, You can obtain\none at http://mozilla.org/MPL/2.0/.\n\nIf it is not possible or desirable to put the notice in a particular file,\nthen You may include the notice in a location (such as a LICENSE file in a\nrelevant directory) where a recipient would be likely to look for such a notice.\n\nYou may add additional accurate notices of copyright ownership.\n\nExhibit B - \"Incompatible With Secondary Licenses\" Notice\n\nThis Source Code Form is \"Incompatible With Secondary Licenses\", as defined\nby the Mozilla Public License, v. 2.0."]) }),
        ("Unlicense", License { id: "Unlicense", replace: None, copyright: None, optional: Some(&[" For more information,\nplease refer to <https://unlicense.org/>"]) }),
        ("Zlib", License { id: "Zlib", replace: Some(LicenseReplace { year: Some("<year>"), name: Some("<copyright holders>") }), copyright: Some("Copyright (c) <year> <copyright holders>"), optional: Some(&["zlib License "]) }),
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
            optional: Some(&["LLVM Exceptions to the Apache 2.0 License "]),
        },
    )]),
};

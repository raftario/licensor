pub struct LicenseReplace {
    pub year: Option<&'static str>,
    pub name: Option<&'static str>,
}

pub struct License {
    pub id: &'static str,
    pub replace: Option<LicenseReplace>,
    pub copyright: Option<&'static [usize]>,
}

pub struct Exception {
    pub id: &'static str,
    pub with: Option<&'static [&'static str]>,
}
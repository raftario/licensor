#[macro_use]
extern crate serde;

#[cfg_attr(feature = "deserialize", derive(Debug, Deserialize))]
pub struct LicenseReplace {
    pub year: Option<String>,
    pub name: Option<String>,
}

#[cfg_attr(feature = "deserialize", derive(Debug, Deserialize))]
pub struct License {
    pub id: String,
    pub replace: Option<LicenseReplace>,
    pub copyright: Option<Vec<usize>>,
}

#[cfg_attr(feature = "deserialize", derive(Debug, Deserialize))]
pub struct Exception {
    pub id: String,
}

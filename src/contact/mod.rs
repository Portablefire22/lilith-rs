use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct ContactInformation {
    /// Where the information pertains to e.g. Discord, Email.
    pub(crate) location_name: String,
    /// The contact info e.g. Username, email address.
    pub(crate) value: Option<String>,
    /// Relevant direct links or the website it relates to. 
    pub(crate) url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct ContactUrl {
    /// Destination URL
    pub(crate) link: String,
    /// Displayed text
    pub(crate) name: String,
}

#[derive(Debug)]
pub(crate) enum ContactError {
    Io(std::io::Error),
    Serde(serde_yaml::Error)
}

impl ContactInformation {
    pub(crate) fn from_file(path: PathBuf) -> Result<Vec<Self>, ContactError> {
        let yaml = match fs::read_to_string(path) {
            Ok(yaml) => yaml,
            Err(err) => return Err(ContactError::Io(err)),
        };
        let information = match serde_yaml::from_str(&yaml) {
            Ok(info) => info,
            Err(err) => return Err(ContactError::Serde(err)),
        };
        Ok(information)
    }
}

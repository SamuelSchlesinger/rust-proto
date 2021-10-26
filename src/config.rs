use log::{debug};
use serde::{Deserialize, Serialize};
use crate::error::{Error};
/// The static configuration of the program. This will be
/// held constant through the execution of this program.
///
/// NB: As far as I can tell, serde's derivations are
/// backwards compatible with removing fields.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    host_name: String,
}

impl Config {
    /// The default configuration for the program. This should always
    /// contain sensible enough values to test with.
    #[allow(dead_code)]
    pub fn default() -> Self {
        Config {
            host_name: String::from("samuel"),
        }
    }

    pub fn from_file(file_name: &'static str) -> Result<Self, Error> {
        debug!("Opening {:?} to create Config", file_name);
        let config_file = std::fs::OpenOptions::new()
            .read(true)
            .open(file_name)
            .map_err(|_e| Error::ConfigFileBad(file_name))?;
        debug!("Opened {:?} to create Config", file_name);
        let config = serde_json::from_reader(config_file)
            .map_err(|_e| Error::ConfigFileUnreadable(file_name))?;
        debug!("Created config {:?}", config);
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    fn test_default() {
        assert_eq!(Config::default(), Config { host_name: String::from("samuel") });
    }
}

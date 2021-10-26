use log::error;

/// The central error type for the application. These should be converted by the main program into
/// readable error messages for the user.
#[derive(Debug)]
pub enum Error {
    ConfigFileBad(&'static str),
    ConfigFileUnreadable(&'static str),
}

impl Error {
    /// Log the error, consuming it. Owning it isn't strictly necessary; we could copy them, but its
    /// nice for the interface to work this way, as it means that errors will probably only get
    /// reported once.
    pub fn report(self) -> () {
        match self {
            Error::ConfigFileBad(config_file) => {
                // TODO(sam) Search this file for "Log Configs Note"
                error!("Error opening {:?} for reading the Config", config_file)
            }
            Error::ConfigFileUnreadable(config_file) => {
                // TODO(sam) Search this file for "Log Configs Note"
                error!("Error parsing the Config from {:?}", config_file)
            }
        }
    }
}

// Log Configs Note
// ================
//
// Is it bad to log configs in general? Perhaps one could use a newtype for obfuscating
// certain fields of : Debug impl neatly.

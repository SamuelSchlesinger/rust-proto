extern crate env_logger;
extern crate log;
extern crate serde;
extern crate serde_json;

use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};

/// The static configuration of the program. This will be
/// held constant through the execution of this program.
///
/// NB: As far as I can tell, serde's derivations are
/// backwards compatible with removing fields.
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    host_name: String,
}

impl Config {
    /// The default configuration for the program. This should always
    /// contain sensible enough values to test with.
    fn default() -> Self {
        Config {
            host_name: String::from("samuel"),
        }
    }

    fn from_file(file_name: &'static str) -> Result<Self, Error> {
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

/// The dynamic context that the program needs to run. This
/// might include client handles to other programs, a pool
/// of database connections, a logging service, or whatever
/// else.
struct Context {
    config: Config,
}

impl Context {
    /// Create a [`Context`] from a [`Config`]
    fn new(config: Config) -> Result<Self, Error> {
        debug!("Creating new Context from {:?}", config);
        let context = Context { config };
        debug!("Created new Context");
        Ok(context)
    }

    fn from_file(file_name: &'static str) -> Result<Self, Error> {
        debug!("Creating a Context from the Config at file {:?}", file_name);
        let config = Config::from_file(file_name)?;
        Context::new(config)
    }
}

/// The central error type for the application. These should be converted by the main program into
/// readable error messages for the user.
#[derive(Debug)]
enum Error {
    ConfigFileBad(&'static str),
    ConfigFileUnreadable(&'static str),
}

fn setup(config_file: &'static str) -> Result<Context, Error> {
    env_logger::init();
    Context::from_file(config_file)
}

fn report_errors(error: Error) -> () {
    match error {
        Error::ConfigFileBad(config_file) => {
            error!("Error opening {:?} for reading the Config", config_file)
        }
        Error::ConfigFileUnreadable(config_file) => {
            error!("Error parsing the Config from {:?}", config_file)
        }
    }
}

/// A wrapper around the [`program`] which prints readable error logs when it fails.
fn main() {
    match setup("config.json").map_err(report_errors) {
        Err(()) => std::process::exit(1),
        Ok(context) => match program(context).map_err(report_errors) {
            Err(()) => std::process::exit(1),
            Ok(()) => std::process::exit(0),
        },
    }
}

/// The main entrypoint to the program, where I would write my actual logic, augmenting
/// the [`Context`] as is necessary to enable the type of development I want to achieve,
/// and suitably modifying the [`Config`] to parameterize the new capabilities I need hanging
/// around.
fn program(context: Context) -> Result<(), Error> {
    Ok(())
}

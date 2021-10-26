use log::{debug};

use crate::error::{Error};
use crate::config::{Config};

/// The dynamic context that the program needs to run. This
/// might include client handles to other programs, a pool
/// of database connections, a logging service, or whatever
/// else. Sadly, it also includes some global variables that
/// we set, in particular initializing env_logger.
#[allow(dead_code)]
pub struct Context {
    config: Config,
}

impl Context {
    /// Create a [`Context`] from a [`Config`]
    pub fn new(config: Config) -> Result<Self, Error> {
        debug!("Creating new Context from {:?}", config);
        let context = Context { config };
        debug!("Created new Context");
        Ok(context)
    }

    pub fn from_file(file_name: &'static str) -> Result<Self, Error> {
        debug!("Creating a Context from the Config at file {:?}", file_name);
        let config = Config::from_file(file_name)?;
        Context::new(config)
    }
}

pub fn setup(config_file: &'static str) -> Result<Context, Error> {
    env_logger::init();
    Context::from_file(config_file)
}

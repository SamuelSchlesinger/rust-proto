use log::debug;

use crate::config::Config;
use crate::error::Error;

/// The dynamic context that the program needs to run. This
/// might include client handles to other programs, a pool
/// of database connections, a logging service, or whatever
/// else. Sadly, it also includes some global variables that
/// we set, in particular initializing env_logger.
#[allow(dead_code)]
pub struct Context {
    /// The configuration which was used to create this [`Context`].
    config: Config,
}

impl Context {
    /// Create a new [`Context`] from a file located at the given path.
    pub fn from_file(file_name: &'static str) -> Result<Self, Error> {
        debug!("Creating a Context from the Config at file {:?}", file_name);
        let config = Config::from_file(file_name)?;
        Context::new(config)
    }

    /// Create a new [`Context`] from a [`Config`]
    pub fn new(config: Config) -> Result<Self, Error> {
        debug!("Creating new Context from {:?}", config);
        let context = Context { config };
        debug!("Created new Context");
        Ok(context)
    }

    pub fn default() -> Result<Self, Error> {
        debug!("Creating default Context");
        let config = Config::default();
        Context::new(config)
    }
}

/// Set up all of the necessary lowercase c context and create the uppercase
/// C [`Context`]. Namespaced this way for comprehension purposes, to separate
/// the things the [`Context`] knows about from the general context needed
/// for this program to function, such as the call to [`env_logger::init`].
pub fn setup(config_file: &'static str) -> Result<Context, Error> {
    env_logger::init();
    Context::from_file(config_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_context_has_a_sane_config() -> Result<(), Error> {
        let context = Context::default()?;
        assert_eq!(Config::default(), context.config);
        Ok(())
    }
}

extern crate env_logger;
extern crate log;
extern crate serde;
extern crate serde_json;

mod config;
mod context;
mod error;
mod program;

use crate::error::Error;

/// A wrapper around the [`program::entrypoint`] which prints readable error logs when it fails.
fn main() {
    // Sort of awkward, we map the error-specific reporting function over the error
    // and then treat the two cases as generic failure and generic success.
    match context::setup("config.json").map_err(Error::report) {
        Err(()) => std::process::exit(1),
        // We do the same awkward mapping here. It might even be nice.
        Ok(context) => match program::entrypoint(context).map_err(Error::report) {
            Err(()) => std::process::exit(1),
            Ok(()) => std::process::exit(0),
        },
    }
}

use crate::error::{Error};
use crate::context::{Context};

/// The main entrypoint to the program, where I would write my actual logic, augmenting
/// the [`Context`] as is necessary to enable the type of development I want to achieve,
/// and suitably modifying the [`Config`] to parameterize the new capabilities I need hanging
/// around.
pub fn entrypoint(_context: Context) -> Result<(), Error> {
    Ok(())
}

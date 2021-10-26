use crate::error::Error;
// By "the power of the dot", importing this alone is enough to bring in all of the Config
// accessors
use crate::context::Context;

/// The main entrypoint to the program, where I would write my actual logic.
pub fn entrypoint(_context: Context) -> Result<(), Error> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entrypoint_does_not_error_on_default_config() -> Result<(), Error> {
        let context = Context::default()?;
        entrypoint(context)

    }
}

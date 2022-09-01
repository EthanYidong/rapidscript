use crate::prelude::*;

pub fn script_init() -> AnyResult<()> {
    dotenv::dotenv()?;
    Ok(())
}

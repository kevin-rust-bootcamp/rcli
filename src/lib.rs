mod cli;
use anyhow::Result;
pub use cli::*;
use enum_dispatch::enum_dispatch;
mod process;

/// A trait for objects that can execute commands.
///
/// # Parameters
///
/// * `self` - A reference to the object implementing the `CmdExecutor` trait.
///
/// # Returns
///
/// A `Result` containing a unit type (`()`), which indicates whether the command was executed successfully or not.
#[enum_dispatch]
pub trait CmdExecutor {
    /// Executes the command.
    fn execute(self) -> Result<()>;
}

#[macro_use]
extern crate failure;

use std::process::Command;

#[derive(Debug, Fail)]
pub enum ExternalCommandError {
    #[fail(display = "Command not found: {}", command)]
    CommandNotFound { command: String },
    #[fail(display = "Non-zero exit-code = {}: {}", exit_code, command)]
    NonZeroExitCode { command: String, exit_code: i32 },
}

/// Run `std::process::Command` and check error into `Fail` trait
pub trait CheckRun {
    /// Run `std::process::Command` and check error
    fn check_run(&mut self) -> Result<(), ExternalCommandError>;
}

impl CheckRun for Command {
    fn check_run(&mut self) -> Result<(), ExternalCommandError> {
        let st = self.status().map_err(|_error| {
            let command = format!("{:?}", self);
            ExternalCommandError::CommandNotFound { command }
        })?;
        match st.code() {
            Some(exit_code) => {
                if exit_code != 0 {
                    let command = format!("{:?}", self);
                    Err(ExternalCommandError::NonZeroExitCode { command, exit_code })
                } else {
                    Ok(())
                }
            }
            None => Ok(()),
        }
    }
}

use std::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Command not found: {command}")]
    CommandNotFound { command: String },

    #[error("Non-zero exit-code = {exit_code}: {command}")]
    NonZeroExitCode { command: String, exit_code: i32 },
}

/// Run `std::process::Command` and check error into `Fail` trait
pub trait CheckRun {
    /// Run `std::process::Command` and check error
    fn check_run(&mut self) -> Result<(), CommandError>;
}

impl CheckRun for Command {
    fn check_run(&mut self) -> Result<(), CommandError> {
        let st = self.status().map_err(|_error| {
            let command = format!("{:?}", self);
            CommandError::CommandNotFound { command }
        })?;
        match st.code() {
            Some(exit_code) => {
                if exit_code != 0 {
                    let command = format!("{:?}", self);
                    Err(CommandError::NonZeroExitCode { command, exit_code })
                } else {
                    Ok(())
                }
            }
            None => Ok(()),
        }
    }
}

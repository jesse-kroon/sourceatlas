use crate::command::Command;

pub struct Config {
    pub command: Command,
}

impl Config {
    pub fn parse(args: &[String]) -> Result<Self, String> {
        let command = Command::parse(args)?;

        Ok(Self { command })
    }
}

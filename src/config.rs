use crate::Command;

pub struct Config {
    pub command: Command,
    pub file_path: String,
}

impl Config {
    pub fn parse(args: &[String]) -> Result<Self, String> {
        if args.len() != 3 {
            return Err(String::from("Usage: repolens <command> <directory>"));
        };

        let command = Command::parse(&args[1])?;
        let file_path = args[2].clone();

        Ok(Self { command, file_path })
    }
}

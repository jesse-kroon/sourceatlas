#[derive(Debug, PartialEq)]
pub enum Command {
    Scan,
}

impl Command {
    pub fn parse(input: &str) -> Result<Self, String> {
        match input {
            "scan" => Ok(Self::Scan),
            _ => Err(format!("unknown command")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Scan { directory: String },
    Help,
}

impl Command {
    pub fn parse(args: &[String]) -> Result<Self, String> {
        let Some(command) = args.get(1) else {
            return Err(String::from(
                "No command provided.\n\
                 Usage:\n\
                 sourceatlas scan <directory>\n\
                 sourceatlas help",
            ));
        };

        match command.as_str() {
            "scan" => {
                if args.len() != 3 {
                    return Err(String::from("Usage: sourceatlas scan <directory>"));
                }

                Ok(Self::Scan {
                    directory: args[2].clone(),
                })
            }
            "help" => {
                if args.len() != 2 {
                    return Err(String::from("Usage: sourceatlas help"));
                }
                Ok(Self::Help)
            }
            unknown => Err(format!(
                "Unknown command: {unknown}. Use \"sourceatlas help\" to see available commands"
            )),
        }
    }
}

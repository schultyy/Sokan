use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone)]
pub struct Command {
    pub sudo: bool,
    pub command: String,
    pub args: Vec<String>
}

impl Display for Command {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let arguments = self.args.iter().rev()
                                 .map(String::as_ref)
                                 .collect::<Vec<_>>()
                                 .connect(" ");

        if self.sudo {
            write!(formatter, "sudo {} {}", self.command, arguments)
        } else {
            write!(formatter, "{} {}", self.command, arguments)
        }
    }
}

pub fn parse(command: String) -> Command {
    let mut words: Vec<&str> = command.split_whitespace().collect();
    let mut cmd:String = String::new();
    let mut sudo = false;

    words.reverse();

    let mut first = words.pop();

    if first == Some("sudo") {
        sudo = true;
        first = words.pop();
    }

    match first {
        Some(val) => cmd = val.to_string(),
        None => { }
    }


    let mut arguments = words.iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>();

    arguments.reverse();

    return Command {
        sudo: sudo,
        command: cmd,
        args: arguments
    };
}

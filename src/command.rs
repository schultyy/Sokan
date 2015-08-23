use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::clone::Clone;

pub struct Command {
    pub sudo: bool,
    pub command: String,
    pub args: Vec<String>
}

impl Clone for Command {
    fn clone(&self) -> Command {
        Command {
            sudo: self.sudo,
            command: self.command.clone(),
            args: self.args.iter()
                            .map(|e| e.to_string())
                            .collect::<Vec<_>>()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.sudo = source.sudo;
        self.command = source.command.clone();
        self.args = self.args.iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>();
    }
}

impl Display for Command {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let mut arguments = self.args.clone();
        arguments.reverse();

        if self.sudo {
            write!(formatter, "sudo {} {}", self.command, arguments.connect(" "))
        } else {
            write!(formatter, "{} {}", self.command, arguments.connect(" "))
        }
    }
}

pub fn parse(command: String) -> Command {
    let mut words: Vec<&str> = command.split_whitespace().collect();
    let arguments: Vec<String> = vec![];
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
        None => cmd = String::new()
    }


    let arguments = words.iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>();

    return Command {
        sudo: sudo,
        command: cmd,
        args: arguments
    };
}

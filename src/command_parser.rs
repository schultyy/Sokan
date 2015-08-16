pub struct Command {
    pub sudo: bool,
    pub command: String,
    pub args: Vec<String>
}

pub fn parse(command: String) -> Command {
    let mut words: Vec<&str> = command.split_whitespace().collect();
    let arguments: Vec<String> = vec![];
    let mut cmd:String = String::new();

    match words.pop() {
        Some(val) => cmd = val.to_string(),
        None => cmd = String::new()
    }

    return Command {
        sudo: false,
        command: cmd,
        args: arguments
    };
}

pub struct Command {
    pub sudo: bool,
    pub command: String,
    pub args: Vec<String>
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

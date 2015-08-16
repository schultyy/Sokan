#[path="../src/command_parser.rs"]
mod command_parser;

#[test]
fn parses_non_sudo_without_args() {
    let cmd_str = "ls";

    let cmd = command_parser::parse(cmd_str.to_string());

    assert_eq!(cmd.sudo, false);
    assert_eq!(cmd.command, "ls");
    assert_eq!(cmd.args.is_empty(), true);
}

#[test]
fn parses_non_sudo_with_args() {
    let cmd_str = "ls -al";

    let cmd = command_parser::parse(cmd_str.to_string());

    assert_eq!(cmd.sudo, false);
    assert_eq!(cmd.command, "ls");
    assert_eq!(cmd.args, vec!["-al"]);
}

#[test]
fn parses_sudo_without_args() {
    let cmd_str = "sudo su";

    let cmd = command_parser::parse(cmd_str.to_string());

    assert_eq!(cmd.sudo, true);
    assert_eq!(cmd.command, "su");
    assert_eq!(cmd.args.is_empty(), true);
}

#[test]
fn parses_sudo_with_args() {
    let cmd_str = "sudo make cake";

    let cmd = command_parser::parse(cmd_str.to_string());

    assert_eq!(cmd.sudo, true);
    assert_eq!(cmd.command, "make");
    assert_eq!(cmd.args, vec!["cake"]);
}

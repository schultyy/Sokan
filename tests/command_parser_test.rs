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

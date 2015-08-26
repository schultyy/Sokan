#[path="../src/command.rs"]
mod command;

#[test]
fn parses_non_sudo_without_args() {
    let cmd_str = "ls";

    let cmd = command::parse(cmd_str.to_string());

    assert_eq!(cmd.sudo, false);
    assert_eq!(cmd.command, "ls");
    assert_eq!(cmd.args.is_empty(), true);
}

#[test]
fn parses_non_sudo_with_args() {
    let cmd_str = "ls -al";

    let cmd = command::parse(cmd_str.to_string());

    assert_eq!(cmd.sudo, false);
    assert_eq!(cmd.command, "ls");
    assert_eq!(cmd.args, vec!["-al"]);
}

#[test]
fn parses_sudo_without_args() {
    let cmd_str = "sudo su";

    let cmd = command::parse(cmd_str.to_string());

    assert_eq!(cmd.sudo, true);
    assert_eq!(cmd.command, "su");
    assert_eq!(cmd.args.is_empty(), true);
}

#[test]
fn parses_sudo_with_args() {
    let cmd_str = "sudo make cake";

    let cmd = command::parse(cmd_str.to_string());

    assert_eq!(cmd.sudo, true);
    assert_eq!(cmd.command, "make");
    assert_eq!(cmd.args, vec!["cake"]);
}

#[test]
fn parse_args_are_in_correct_order() {
    let cmd_str = "yum clean all";

    let cmd = command::parse(cmd_str.to_string());

    assert_eq!(cmd.args, vec!["clean", "all"]);
}

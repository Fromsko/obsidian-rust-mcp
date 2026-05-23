use obsidian_mcp::command::registry::{find_command, COMMANDS};

#[test]
fn all_commands_registered() {
    assert_eq!(COMMANDS.len(), 7);
    assert!(find_command("obsidian.guide").is_some());
    assert!(find_command("obsidian.search").is_some());
    assert!(find_command("obsidian.write").is_some());
    assert!(find_command("obsidian.read").is_some());
    assert!(find_command("obsidian.index").is_some());
    assert!(find_command("obsidian.delete").is_some());
    assert!(find_command("obsidian.semantic_search").is_some());
}

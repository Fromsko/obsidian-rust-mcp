use std::env;

pub const VAULT_ROOT: &str = r"D:\notes\Fromsko";

pub const VALID_DIRS: &[&str] = &[
    "tech",
    "ai",
    "projects",
    "methods",
    "career",
    "ideas",
    "cheatsheet",
    "journal",
];

pub const VALID_STATUSES: &[&str] = &["active", "archived", "draft"];

pub const WRITE_NOTE_TIPS: &str = include_str!("../write-note-tips.md");

pub fn get_vault_root() -> String {
    env::var("OBSIDIAN_VAULT_ROOT").unwrap_or_else(|_| VAULT_ROOT.to_string())
}

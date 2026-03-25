use std::path::Path;

pub fn build_file_tree(root: &Path) -> String {
    let mut lines = Vec::new();
    tree_recursive(root, "", &mut lines);
    lines.join("\n")
}

fn tree_recursive(dir: &Path, prefix: &str, lines: &mut Vec<String>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_name().to_string_lossy().starts_with('.'))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let total = entries.len();
    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == total - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();

        if path.is_dir() {
            lines.push(format!("{prefix}{connector}{name}/"));
            let child_prefix = if is_last {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│   ")
            };
            tree_recursive(&path, &child_prefix, lines);
        } else {
            lines.push(format!("{prefix}{connector}{name}"));
        }
    }
}

use std::path::Path;

use walkdir::WalkDir;

use crate::note::frontmatter::{body_excerpt, parse_frontmatter};
use crate::types::{NoteEntry, VaultIndex};

const BODY_EXCERPT_MAX: usize = 4096;

pub fn build_index(root: &Path) -> VaultIndex {
    let mut index = VaultIndex::default();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() && e.path().extension().map(|ext| ext == "md").unwrap_or(false)
        })
    {
        let path = entry.path();
        let rel = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

        let title = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let content = std::fs::read_to_string(path).unwrap_or_default();
        let (tags, aliases, status) = parse_frontmatter(&content);
        let excerpt = body_excerpt(&content, BODY_EXCERPT_MAX);

        let idx = index.entries.len();
        index.entries.push(NoteEntry {
            rel_path: rel,
            tags: tags.clone(),
            aliases,
            status,
            title: title.clone(),
            body_excerpt: excerpt,
        });
        index.name_map.insert(title.to_lowercase(), idx);

        for tag in &tags {
            index
                .tag_map
                .entry(tag.to_lowercase())
                .or_default()
                .push(idx);
        }
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_build_index_finds_md_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("sub")).unwrap();
        fs::write(dir.path().join("note1.md"), "---\ntags:\n  - a\n---\nHello").unwrap();
        fs::write(
            dir.path().join("sub/note2.md"),
            "---\ntags:\n  - b\n---\nWorld",
        )
        .unwrap();
        fs::write(dir.path().join("skip.txt"), "not markdown").unwrap();

        let index = build_index(dir.path());
        assert_eq!(index.entries.len(), 2);
        assert!(index.tag_map.contains_key("a"));
        assert!(index.tag_map.contains_key("b"));
    }

    #[test]
    fn test_build_index_body_excerpt() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("note.md"),
            "---\ntags: []\n---\n# Title\n\nSemantic content here.",
        )
        .unwrap();
        let index = build_index(dir.path());
        assert!(index.entries[0].body_excerpt.contains("Semantic"));
    }
}

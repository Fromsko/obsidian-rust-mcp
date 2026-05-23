//! Local weighted semantic search over vault index (no external model).

use crate::types::{NoteEntry, VaultIndex};

const TITLE_WEIGHT: f32 = 3.0;
const TAG_WEIGHT: f32 = 2.0;
const ALIAS_WEIGHT: f32 = 2.0;
const PATH_WEIGHT: f32 = 1.5;
const BODY_WEIGHT: f32 = 1.0;

#[derive(Debug, Clone)]
pub struct ScoredHit {
    pub index: usize,
    pub score: f32,
}

pub fn semantic_search(index: &VaultIndex, query: &str, limit: usize) -> Vec<ScoredHit> {
    let terms: Vec<String> = tokenize(query);
    if terms.is_empty() {
        return Vec::new();
    }

    let mut hits: Vec<ScoredHit> = index
        .entries
        .iter()
        .enumerate()
        .map(|(i, entry)| ScoredHit {
            index: i,
            score: score_entry(entry, &terms),
        })
        .filter(|h| h.score > 0.0)
        .collect();

    hits.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    hits.truncate(limit.max(1));
    hits
}

fn tokenize(query: &str) -> Vec<String> {
    query
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '-')
        .filter(|t| t.len() >= 2)
        .map(|t| t.to_string())
        .collect()
}

fn score_entry(entry: &NoteEntry, terms: &[String]) -> f32 {
    let title = entry.title.to_lowercase();
    let path = entry.rel_path.to_lowercase();
    let body = entry.body_excerpt.to_lowercase();
    let tags: Vec<String> = entry.tags.iter().map(|t| t.to_lowercase()).collect();
    let aliases: Vec<String> = entry.aliases.iter().map(|a| a.to_lowercase()).collect();

    let mut score = 0.0f32;
    for term in terms {
        if title.contains(term) {
            score += TITLE_WEIGHT;
        }
        if path.contains(term) {
            score += PATH_WEIGHT;
        }
        if body.contains(term) {
            score += BODY_WEIGHT;
        }
        for tag in &tags {
            if tag.contains(term) {
                score += TAG_WEIGHT;
            }
        }
        for alias in &aliases {
            if alias.contains(term) {
                score += ALIAS_WEIGHT;
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::NoteEntry;

    fn sample_index() -> VaultIndex {
        let mut index = VaultIndex::default();
        index.entries.push(NoteEntry {
            rel_path: "tech/docker.md".into(),
            tags: vec!["docker".into(), "linux".into()],
            aliases: vec!["容器".into()],
            status: "active".into(),
            title: "docker-guide".into(),
            body_excerpt: "nginx reverse proxy setup".into(),
        });
        index.entries.push(NoteEntry {
            rel_path: "ai/rust.md".into(),
            tags: vec!["rust".into()],
            aliases: vec![],
            status: "active".into(),
            title: "rust-notes".into(),
            body_excerpt: "ownership and borrowing".into(),
        });
        index
    }

    #[test]
    fn ranks_docker_higher_for_docker_query() {
        let index = sample_index();
        let hits = semantic_search(&index, "docker nginx", 5);
        assert!(!hits.is_empty());
        assert_eq!(hits[0].index, 0);
    }

    #[test]
    fn empty_query_returns_nothing() {
        let index = sample_index();
        assert!(semantic_search(&index, "  ", 5).is_empty());
    }
}

mod file_tree;
mod frontmatter;
mod index;
pub mod semantic;

pub use file_tree::build_file_tree;
pub use frontmatter::{generate_frontmatter, parse_frontmatter, update_frontmatter_date};
pub use index::build_index;
pub use semantic::{semantic_search, ScoredHit};

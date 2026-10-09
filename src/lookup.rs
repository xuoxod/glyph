// ==============================================================================
// Sovereign Glyph Engine - Microsecond Lookup & Search Resolver
// High-Speed Multi-Index Resolver (Shortcodes, Emoticons, Tags, Fuzzy)
// ==============================================================================

use crate::catalog::{Catalog, EmojiEntry};
use crate::sanitize::sanitize_input;

pub struct LookupEngine;

impl LookupEngine {
    /// Resolves a single keyword, shortcode, or emoticon into a glyph.
    /// Priority:
    /// 1. Exact match on shortcode
    /// 2. Exact match on emoticon
    /// 3. Exact match on synonym tag
    /// 4. Substring / prefix match
    pub fn resolve(query: &str) -> Option<&'static EmojiEntry> {
        let clean = sanitize_input(query).trim().to_lowercase();
        if clean.is_empty() {
            return None;
        }

        let raw_clean = query.trim();

        // 1. Check exact emoticon first (case-sensitive or exact string like ":)", "<3")
        for entry in Catalog::all() {
            for &emo in entry.emoticons {
                if emo == raw_clean || emo.eq_ignore_ascii_case(raw_clean) {
                    return Some(entry);
                }
            }
        }

        // 2. Exact match on shortcode (ignoring leading/trailing colons like :smile:)
        let stripped = clean.trim_matches(':');
        for entry in Catalog::all() {
            if entry.shortcode.eq_ignore_ascii_case(stripped) {
                return Some(entry);
            }
        }

        // 3. Exact match on tags
        for entry in Catalog::all() {
            for &tag in entry.tags {
                if tag.eq_ignore_ascii_case(stripped) {
                    return Some(entry);
                }
            }
        }

        // 4. Prefix match on shortcode
        for entry in Catalog::all() {
            if entry.shortcode.starts_with(stripped) {
                return Some(entry);
            }
        }

        // 5. Substring match on name or tags
        for entry in Catalog::all() {
            if entry.name.contains(stripped) {
                return Some(entry);
            }
            for &tag in entry.tags {
                if tag.contains(stripped) {
                    return Some(entry);
                }
            }
        }

        None
    }

    /// Searches the catalog and returns all matching results with a match score.
    pub fn search(query: &str) -> Vec<SearchResult> {
        let clean = sanitize_input(query).trim().to_lowercase();
        if clean.is_empty() {
            return Vec::new();
        }

        let stripped = clean.trim_matches(':');
        let mut results = Vec::new();

        for entry in Catalog::all() {
            let mut score = 0;

            // Check emoticon
            for &emo in entry.emoticons {
                if emo.eq_ignore_ascii_case(query.trim()) {
                    score = score.max(100);
                }
            }

            // Check exact shortcode
            if entry.shortcode.eq_ignore_ascii_case(stripped) {
                score = score.max(95);
            } else if entry.shortcode.starts_with(stripped) {
                score = score.max(75);
            } else if entry.shortcode.split('_').any(|w| w.eq_ignore_ascii_case(stripped)) {
                score = score.max(70);
            } else if entry.shortcode.contains(stripped) {
                score = score.max(25);
            }

            // Check exact tags
            for &tag in entry.tags {
                if tag.eq_ignore_ascii_case(stripped) {
                    score = score.max(90);
                } else if tag.starts_with(stripped) {
                    score = score.max(65);
                } else if tag.split_whitespace().any(|w| w.eq_ignore_ascii_case(stripped)) {
                    score = score.max(60);
                } else if tag.contains(stripped) {
                    score = score.max(20);
                }
            }

            // Check full name
            if entry.name.eq_ignore_ascii_case(stripped) {
                score = score.max(85);
            } else if entry.name.split_whitespace().any(|w| w.eq_ignore_ascii_case(stripped)) {
                score = score.max(80);
            } else if entry.name.contains(stripped) {
                score = score.max(20);
            }

            if score > 0 {
                results.push(SearchResult { entry, score });
            }
        }

        // Sort descending by score, then ascending alphabetically by shortcode
        results.sort_by(|a, b| {
            b.score
                .cmp(&a.score)
                .then_with(|| a.entry.shortcode.cmp(b.entry.shortcode))
        });

        results
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub entry: &'static EmojiEntry,
    pub score: u32,
}

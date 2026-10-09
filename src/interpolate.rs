// ==============================================================================
// Sovereign Glyph Engine - Text & Sentence Shortcode Interpolator
// Replaces :shortcode: and ASCII emoticons inline within sentences
// ==============================================================================

use crate::lookup::LookupEngine;
use crate::sanitize::sanitize_input;

pub struct Interpolator;

impl Interpolator {
    /// Interpolates a text string, replacing `:shortcode:` and common ASCII emoticons with Unicode emojis.
    pub fn process(text: &str) -> String {
        let clean = sanitize_input(text);
        let mut result = String::with_capacity(clean.len());

        // 1. Process :shortcode: tags first
        let mut buffer = String::new();
        let mut in_colon = false;

        for ch in clean.chars() {
            if ch == ':' {
                if in_colon {
                    // Reached closing colon
                    if !buffer.is_empty() {
                        if let Some(entry) = LookupEngine::resolve(&buffer) {
                            result.push_str(entry.glyph);
                        } else {
                            // Not found, preserve original :buffer:
                            result.push(':');
                            result.push_str(&buffer);
                            result.push(':');
                        }
                        buffer.clear();
                        in_colon = false;
                    } else {
                        // Double colon ::
                        result.push(':');
                    }
                } else {
                    in_colon = true;
                    buffer.clear();
                }
            } else if in_colon {
                // If whitespace or punctuation terminates a tag unexpectedly
                if ch.is_whitespace() || ch == '\n' {
                    result.push(':');
                    result.push_str(&buffer);
                    result.push(ch);
                    buffer.clear();
                    in_colon = false;
                } else {
                    buffer.push(ch);
                }
            } else {
                result.push(ch);
            }
        }

        // Flush dangling colon buffer if open
        if in_colon {
            result.push(':');
            result.push_str(&buffer);
        }

        // 2. Process whitespace-delimited ASCII emoticons (like ":)" or "<3")
        Self::replace_emoticons(&result)
    }

    fn replace_emoticons(text: &str) -> String {
        let mut words = Vec::new();
        for word in text.split(' ') {
            // Check if this exact word is an emoticon (like ":-)", ";)", "<3")
            if let Some(entry) = LookupEngine::resolve(word) {
                // Only replace if the matched entry actually listed this exact word in its emoticons
                let is_emoticon = entry.emoticons.iter().any(|&e| e.eq_ignore_ascii_case(word));
                if is_emoticon {
                    words.push(entry.glyph.to_string());
                } else {
                    words.push(word.to_string());
                }
            } else {
                words.push(word.to_string());
            }
        }
        words.join(" ")
    }
}

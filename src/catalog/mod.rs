// ==============================================================================
// Sovereign Glyph Engine - Emoji Catalog Types & Interfaces
// 100% In-Memory, Zero-Cloud, Microsecond Unicode Catalog
// ==============================================================================

pub mod data;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Smileys,
    Gestures,
    People,
    Animals,
    Nature,
    Food,
    Travel,
    Activities,
    Objects,
    Symbols,
    Flags,
}

impl Category {
    pub const ALL: &'static [Category] = &[
        Category::Smileys,
        Category::Gestures,
        Category::People,
        Category::Animals,
        Category::Nature,
        Category::Food,
        Category::Travel,
        Category::Activities,
        Category::Objects,
        Category::Symbols,
        Category::Flags,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Smileys => "smileys",
            Category::Gestures => "gestures",
            Category::People => "people",
            Category::Animals => "animals",
            Category::Nature => "nature",
            Category::Food => "food",
            Category::Travel => "travel",
            Category::Activities => "activities",
            Category::Objects => "objects",
            Category::Symbols => "symbols",
            Category::Flags => "flags",
        }
    }

    pub fn parse(s: &str) -> Option<Category> {
        let lower = s.to_ascii_lowercase();
        match lower.as_str() {
            "smileys" | "smiley" | "emotions" | "emotion" => Some(Category::Smileys),
            "gestures" | "gesture" | "hands" | "hand" => Some(Category::Gestures),
            "people" | "person" => Some(Category::People),
            "animals" | "animal" | "fauna" => Some(Category::Animals),
            "nature" | "flora" | "weather" => Some(Category::Nature),
            "food" | "drink" => Some(Category::Food),
            "travel" | "places" | "transport" => Some(Category::Travel),
            "activities" | "activity" | "sports" => Some(Category::Activities),
            "objects" | "object" | "tools" | "tech" => Some(Category::Objects),
            "symbols" | "symbol" | "signs" => Some(Category::Symbols),
            "flags" | "flag" => Some(Category::Flags),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmojiEntry {
    pub glyph: &'static str,
    pub name: &'static str,
    pub shortcode: &'static str,
    pub category: Category,
    pub tags: &'static [&'static str],
    pub emoticons: &'static [&'static str],
}

pub struct Catalog;

impl Catalog {
    #[inline]
    pub fn all() -> &'static [EmojiEntry] {
        data::CATALOG
    }

    pub fn by_category(category: Category) -> impl Iterator<Item = &'static EmojiEntry> {
        Self::all().iter().filter(move |e| e.category == category)
    }

    pub fn count() -> usize {
        Self::all().len()
    }
}

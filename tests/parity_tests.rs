// ==============================================================================
// Sovereign Parity Test Suite: glyph / em
// Asserts behavioral and format parity with industry standard shortcodes,
// Slack/GitHub/Discord syntax, ASCII emoticons, and RFC 4648 Base64 OSC 52.
// ==============================================================================

use glyph::catalog::Catalog;
use glyph::clipboard::Clipboard;
use glyph::interpolate::Interpolator;
use glyph::lookup::LookupEngine;

#[test]
fn test_parity_github_slack_shortcodes() {
    let test_cases = vec![
        ("fire", "🔥"),
        ("rocket", "🚀"),
        ("shield", "🛡️"),
        ("thumbsup", "👍"),
        ("thumbsdown", "👎"),
        ("100", "💯"),
        ("check", "✅"),
        ("cross", "❌"),
        ("heart", "❤️"),
        ("coffee", "☕"),
        ("party", "🎉"),
        ("zap", "⚡"),
        ("crab", "🦀"),
        ("bug", "🐛"),
        ("skull", "💀"),
        ("warning", "⚠️"),
        ("lock", "🔒"),
        ("key", "🔑"),
        ("package", "📦"),
    ];

    for (code, expected) in test_cases {
        let entry = LookupEngine::resolve(code)
            .unwrap_or_else(|| panic!("Failed to resolve shortcode '{}'", code));
        assert_eq!(
            entry.glyph, expected,
            "Shortcode '{}' expected '{}', got '{}'",
            code, expected, entry.glyph
        );

        // Also test colon-wrapped syntax
        let colon_wrapped = format!(":{}", code);
        let entry_colon = LookupEngine::resolve(&colon_wrapped)
            .unwrap_or_else(|| panic!("Failed to resolve colon shortcode '{}'", colon_wrapped));
        assert_eq!(entry_colon.glyph, expected);
    }
}

#[test]
fn test_parity_ascii_emoticons() {
    let emoticon_cases = vec![
        (":-)", "😄"),
        (":)", "😄"),
        (";-)", "😉"),
        (";)", "😉"),
        (":D", "😀"),
        (":-D", "😀"),
        ("<3", "❤️"),
        ("</3", "💔"),
        ("B-)", "😎"),
        ("8-)", "😎"),
        (">:(", "😡"),
        ("-_", "😑"),
        ("¯\\_(ツ)_/¯", "🤷"),
    ];

    for (emoticon, expected) in emoticon_cases {
        let entry = LookupEngine::resolve(emoticon);
        assert!(
            entry.is_some(),
            "Emoticon '{}' failed resolution",
            emoticon
        );
        assert_eq!(
            entry.unwrap().glyph,
            expected,
            "Emoticon '{}' mapped to incorrect glyph",
            emoticon
        );
    }
}

#[test]
fn test_parity_sentence_interpolation() {
    let sentence = "Shipment ready :package: Deployment :rocket: Defense active :shield:!";
    let processed = Interpolator::process(sentence);
    assert_eq!(
        processed,
        "Shipment ready 📦 Deployment 🚀 Defense active 🛡️!"
    );

    let emotion_sentence = "Good morning :) Let's get it <3";
    let emotion_processed = Interpolator::process(emotion_sentence);
    assert_eq!(emotion_processed, "Good morning 😄 Let's get it ❤️");
}

#[test]
fn test_parity_osc52_rfc4648_base64() {
    let text = "🛡️";
    let seq = Clipboard::build_osc52_sequence(text);
    // Must follow: \x1b]52;c;<base64>\x07
    assert!(seq.starts_with("\x1b]52;c;"));
    assert!(seq.ends_with("\x07"));

    let b64_shield = Clipboard::base64_encode(text.as_bytes());
    assert_eq!(seq, format!("\x1b]52;c;{}\x07", b64_shield));

    // Standard RFC test vectors for base64:
    assert_eq!(Clipboard::base64_encode(b""), "");
    assert_eq!(Clipboard::base64_encode(b"f"), "Zg==");
    assert_eq!(Clipboard::base64_encode(b"fo"), "Zm8=");
    assert_eq!(Clipboard::base64_encode(b"foo"), "Zm9v");
    assert_eq!(Clipboard::base64_encode(b"foob"), "Zm9vYg==");
    assert_eq!(Clipboard::base64_encode(b"fooba"), "Zm9vYmE=");
    assert_eq!(Clipboard::base64_encode(b"foobar"), "Zm9vYmFy");
}

#[test]
fn test_parity_catalog_integrity() {
    let all = Catalog::all();
    assert!(
        all.len() >= 100,
        "Catalog size ({}) is smaller than expected enterprise baseline",
        all.len()
    );

    // Assert every entry has non-empty fields
    for entry in all {
        assert!(!entry.glyph.is_empty(), "Entry has empty glyph");
        assert!(!entry.name.is_empty(), "Entry has empty name");
        assert!(!entry.shortcode.is_empty(), "Entry has empty shortcode");
    }
}

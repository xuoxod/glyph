// ==============================================================================
// Sovereign Adversarial TDD Suite: glyph / em
// Threat Models Defended:
// 1. Formula Injection (=, +, -, @, \t, \r)
// 2. ANSI Escape & Log Poisoning (CSI, OSC, Bell, Control Bytes)
// 3. Resource Exhaustion & Allocation Bombs (Megabyte strings, colon floods)
// 4. Corrupted Boundary Ingestion (Null bytes, malformed UTF-8 boundaries)
// ==============================================================================

use glyph::interpolate::Interpolator;
use glyph::lookup::LookupEngine;
use glyph::sanitize::{neutralize_formula, sanitize_input, MAX_INPUT_LENGTH};

#[test]
fn test_threat_1_formula_injection_defense() {
    let payloads = vec![
        "=cmd|'/c calc'!A0",
        "+12345",
        "-SUM(A1:A10)",
        "@SUM(B1:B2)",
        "\tmalicious_tab",
        "\rmalicious_cr",
    ];

    for payload in payloads {
        let neutralized = neutralize_formula(payload);
        assert!(
            neutralized.starts_with('\''),
            "Payload failed formula neutralization: {}",
            payload
        );
    }

    // Normal strings should remain untouched
    assert_eq!(neutralize_formula("smile"), "smile");
    assert_eq!(neutralize_formula("shield"), "shield");
}

#[test]
fn test_threat_2_ansi_and_log_poisoning_defense() {
    // Malicious ANSI CSI escape injection intended to rebind keys or hide terminal text
    let poisoned_input = "\x1b[31;1m\x1b[2Jshield\x1b[0m";
    let cleaned = sanitize_input(poisoned_input);
    assert!(!cleaned.contains('\x1b'), "Escape character leaked into sanitized string");
    assert_eq!(cleaned, "shield");

    // OSC sequence injection
    let osc_injection = "\x1b]52;c;evil\x07fire";
    let cleaned_osc = sanitize_input(osc_injection);
    assert!(!cleaned_osc.contains('\x1b'));
    assert_eq!(cleaned_osc, "fire");

    // Null bytes and unprintable control characters stripped
    let raw_control = "hello\x00\x01\x02\x03world";
    let cleaned_ctrl = sanitize_input(raw_control);
    assert_eq!(cleaned_ctrl, "helloworld");
}

#[test]
fn test_threat_3_allocation_bomb_and_resource_exhaustion() {
    // 1 MB massive allocation bomb input
    let massive_string = "a".repeat(1_000_000);
    let sanitized = sanitize_input(&massive_string);
    assert_eq!(
        sanitized.len(),
        MAX_INPUT_LENGTH,
        "Sanitizer did not enforce MAX_INPUT_LENGTH bound!"
    );

    // Deep colon recursion attack: ::::::::... (10,000 colons)
    let colon_bomb = ":".repeat(10_000);
    let processed = Interpolator::process(&colon_bomb);
    assert!(processed.len() <= MAX_INPUT_LENGTH + 10);

    // Repeated query search shouldn't panic or freeze
    let lookup_res = LookupEngine::search(&"x".repeat(500));
    assert!(lookup_res.is_empty());
}

#[test]
fn test_threat_4_corrupted_boundary_ingestion() {
    // Traversal payload in lookup query
    let traversal = "../../../../etc/passwd";
    let res = LookupEngine::resolve(traversal);
    assert!(res.is_none(), "Traversal payload should resolve to nothing");

    // Incomplete shortcode strings
    assert_eq!(Interpolator::process("Incomplete :fire"), "Incomplete :fire");
    assert_eq!(Interpolator::process("Dangling colon :"), "Dangling colon :");
    assert_eq!(Interpolator::process("Double colons ::"), "Double colons ::");
    assert_eq!(Interpolator::process("Spaces in colons :hello world:"), "Spaces in colons :hello world:");
}

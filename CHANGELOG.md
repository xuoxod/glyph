# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-10-09

### Added
- **Dual Binary Architecture**: Ships as both `glyph` and ultra-fast typing shortcut `em`.
- **Microsecond Multi-Index Lookup**: Sub-microsecond keyword, shortcode, tag, and emotion resolver.
- **Classic ASCII Emoticon Parity**: Native translation for `:)`, `:-)`, `;-)`, `:D`, `:-(`, `>:(`, `<3`, `¯\_(ツ)_/¯`, `(y)`, and more.
- **Sentence & Shortcode Interpolation**: Inline replacement for sentences containing `:shortcode:` tags and ASCII emoticons.
- **Remote SSH Clipboard Injection (OSC 52)**: Automatically pipes glyphs into the host machine clipboard across remote headless SSH sessions without X11 or Wayland forwarding.
- **Interactive Terminal Fuzzy Picker**: Zero-lag micro-TUI search with live query filtering, arrow navigation, and RAII terminal raw-mode restoration.
- **Expanded 400+ Multi-Category Catalog**: Comprehensive coverage of people (`👨‍💻`, `🥷`, `🧙`), flags (`🏴‍☠️`, `🇺🇸`, `🇪🇺`, `🇯🇵`), food (`🍕`, `🍣`, `🍜`), travel (`🚗`, `✈️`, `🚂`), sports/music, nature, and tech objects.
- **Adversarial Security Guardrails**: Built-in defenses against ANSI terminal injection, spreadsheet formula poisoning (`=`, `+`, `-`, `@`), and buffer exhaustion bombs.
- **Single Static Musl Binary**: Standalone compilation target with zero runtime dependencies under 600 KB.

### Fixed
- **Picker Column Alignment**: Fixed column collision between long shortcodes (e.g. `:slightly_smiling:`) and descriptions in the interactive terminal fuzzy search.

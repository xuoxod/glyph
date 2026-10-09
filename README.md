# 🛡️ glyph (`em`)

> **Sovereign, ultra-fast emoji & symbol engine on demand for terminals, SSH sessions, and scripts.**  
> *Zero cloud. Zero telemetry. Sub-microsecond execution. Single static binary under 600 KB.*

```
   ____ _           __  __
  / __/(_)  __ _____/ /_/ /
 / _/ / / // // _  / __/ _ \
/_/  /_/\_, //_//_/\__/_//_/
       /___/
```

[![CI](https://github.com/xuoxod/glyph/actions/workflows/ci.yml/badge.svg)](https://github.com/xuoxod/glyph/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)

---

## ⚡ Why glyph?

Typing emojis in the terminal shouldn't require opening a heavy GUI picker, opening a browser tab to copy-paste, or memorizing raw Unicode code points.

And in a **headless remote SSH session**, GUI pickers don't exist at all.

`glyph` (invoked as `em` for maximum typing speed) gives you instant emoji output, text interpolation, and remote clipboard injection with zero friction:

```bash
$ em smile
😊

$ em shield
🛡️

$ em ":-)"
😊

$ em "Mission accomplished :rocket: :shield: :100:"
Mission accomplished 🚀 🛡️ 💯
```

---

## 🌟 Core Features

### 1. Instant Keyword & Emotion Lookup
Type what you feel or need:
```bash
$ em fire           # 🔥
$ em idea           # 💡
$ em check          # ✅
$ em cross          # ❌
$ em shrug          # 🤷
$ em coffee         # ☕
$ em crab           # 🦀
```

### 2. Classic ASCII Emoticon Translation
Natural translation of everyday internet emoticons:
```bash
$ em ":-)"          # 😊
$ em ";-)"          # 😉
$ em ":D"           # 😀
$ em ":-("          # 🙁
$ em ">:("          # 😡
$ em "<3"           # ❤️
$ em "¯\_(ツ)_/¯"   # 🤷
```

### 3. Sentence & Shortcode Interpolation
Pass whole sentences containing standard `:shortcode:` tags and emoticons:
```bash
$ em "Shipment ready :package: Deployment live :rocket: Fortress fortified :shield:!"
Shipment ready 📦 Deployment live 🚀 Fortress fortified 🛡️!
```

### 4. Remote SSH Clipboard Injection (OSC 52)
When logged into a remote Linux server or container over SSH, GUI clipboards (`xclip`, `wl-copy`) fail because there is no display server.

Passing `-c` or `--copy` uses **OSC 52 ANSI escape sequences** to pipe the glyph directly into your **local workstation's clipboard across the SSH tunnel**:
```bash
$ em -c shield
🛡️
📋 Copied to clipboard!
```
*(Supports Windows Terminal, Alacritty, Kitty, WezTerm, iTerm2, and modern tmux).*

### 5. Interactive Fuzzy Picker
Run `em` with no arguments in any terminal (or `em -i`):
```
╭── 🛡️  GLYPH EMOJI FINDER ──────────────────────────────────────────────╮
│ Search: shiel▌                                                         │
├───────────────────────────────────────────────────────────────────────┤
│ > 🛡️  :shield          shield [defense, protect, sovereign, security] │
╰────────────────────────────────────────────────────────────────────────╯
[↑/↓] Navigate  •  [Enter] Select & Emit  •  [Esc/Ctrl+C] Quit
```

### 6. Search & Category Exploration
```bash
# Search by keyword or synonym
$ em find cat
Found 6 glyphs matching 'cat':
  🐱  :cat                (cat face)
  😸  :smile_cat          (grinning cat with smiling eyes)
  🐯  :tiger              (tiger face)

# List catalog categories
$ em list
Available categories:
  • smileys      (83 glyphs)
  • gestures     (31 glyphs)
  • people       (6 glyphs)
  • animals      (19 glyphs)
  • nature       (8 glyphs)
  • food         (8 glyphs)
  • travel       (7 glyphs)
  • activities   (7 glyphs)
  • objects      (38 glyphs)
  • symbols      (37 glyphs)
  • flags        (4 glyphs)
```

### 7. Random Emoji on Demand
```bash
$ em -r
🎉
```

---

## 🛠️ CLI Reference

```
🛡️ GLYPH (em) - Sovereign Emoji & Symbol Engine on Demand

USAGE:
    em <keyword|emoticon>        Emit emoji by keyword or emotion (e.g. 'smile', ':-)')
    em "<text with :codes:>"     Interpolate shortcodes & emoticons within sentences
    em find <query>              Search glyphs by synonym, shortcode, or description
    em list [category]           List catalog emojis or view specific category
    em random                    Emit a random emoji
    em                           Launch interactive micro-TUI search (when in terminal)

FLAGS:
    -c, --copy                   Copy result to clipboard (supports SSH via OSC 52)
    -n, --no-newline             Print without trailing newline (useful for prompt/pipe)
    -i, --interactive            Open interactive fuzzy search picker
    -r, --random                 Pick a random emoji
    -h, --help                   Print this sovereign help manual
    -v, --version                Print version information
```

---

## 💻 Script & Shell Integration

Because `glyph` outputs clean UTF-8 to standard output, it integrates seamlessly into shell one-liners, prompts, and Git workflows:

### Git Commit Messages
```bash
git commit -m "$(em shield) Fortified radix firewall $(em rocket)"
# Produces: 🛡️ Fortified radix firewall 🚀
```

### PS1 / Prompt Status Indicator
```bash
# In your ~/.bashrc or ~/.zshrc:
export PS1="\$(em -n crab) \u@\h:\w\$ "
```

### Clean Piping
Use `-n` to emit without a trailing newline:
```bash
em -n check | xclip -selection clipboard
```

---

## 📦 Installation

### From Source
```bash
git clone https://github.com/xuoxod/glyph.git
cd glyph
cargo build --release
cp target/release/em target/release/glyph ~/.local/bin/
```

### One-Line Install Script
```bash
git clone https://github.com/xuoxod/glyph.git
cd glyph
./scripts/install.sh
```

---

## 🛡️ Sovereign Engineering & Security

`glyph` strictly adheres to sovereign engineering invariants:
1. **Radical Sovereignty**: 100% offline, zero network requests, zero telemetry, zero analytics tracking.
2. **Adversarial Input Sanitization**: 
   - State-machine neutralization of ANSI terminal injection attacks.
   - Spreadsheet formula injection protection (`=`, `+`, `-`, `@`).
   - Hard boundary ceilings preventing memory allocation exhaustion.
3. **Radical OJP (One-Job-Principle)**: Thin lifecycle entry points ($\le 80$ lines), decoupled domain lookup, clean modularity.
4. **Static Musl Distribution**: Compiles to a single static Musl binary under 600 KB with zero runtime dependencies.

---

## 📄 License

Dual-licensed under either:
- **MIT License** ([LICENSE](LICENSE))
- **Apache License, Version 2.0**

---

Crafted with care by **Rick Walker** (`xuoxod`).

// ==============================================================================
// Sovereign Glyph Engine - Interactive Terminal Emoji Picker
// Ultra-Fast Zero-Lag Fuzzy TUI Picker with Raw Mode RAII Guard
// ==============================================================================

use std::io::{self, stdout, Write};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use unicode_width::UnicodeWidthStr;

use crate::catalog::{Catalog, EmojiEntry};
use crate::lookup::LookupEngine;

struct TerminalGuard;

impl TerminalGuard {
    fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut out = stdout();
        execute!(out, cursor::Hide)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let mut out = stdout();
        let _ = execute!(out, cursor::Show);
        let _ = terminal::disable_raw_mode();
    }
}

pub struct InteractivePicker;

impl InteractivePicker {
    /// Launches interactive micro-TUI search in terminal.
    pub fn run() -> io::Result<Option<&'static EmojiEntry>> {
        let _guard = TerminalGuard::new()?;
        let mut query = String::new();
        let mut selected_idx = 0usize;

        loop {
            // Filter entries
            let matches: Vec<&'static EmojiEntry> = if query.is_empty() {
                Catalog::all().iter().take(30).collect()
            } else {
                LookupEngine::search(&query)
                    .into_iter()
                    .map(|r| r.entry)
                    .take(30)
                    .collect()
            };

            if selected_idx >= matches.len() && !matches.is_empty() {
                selected_idx = matches.len() - 1;
            }

            Self::render(&query, &matches, selected_idx)?;

            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match code {
                    KeyCode::Esc => return Ok(None),
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(None);
                    }
                    KeyCode::Char(c) => {
                        query.push(c);
                        selected_idx = 0;
                    }
                    KeyCode::Backspace => {
                        query.pop();
                        selected_idx = 0;
                    }
                    KeyCode::Up => {
                        selected_idx = selected_idx.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        if !matches.is_empty() && selected_idx < matches.len() - 1 {
                            selected_idx += 1;
                        }
                    }
                    KeyCode::Enter if !matches.is_empty() => {
                        return Ok(Some(matches[selected_idx]));
                    }
                    _ => {}
                }
            }
        }
    }

    fn render(
        query: &str,
        matches: &[&'static EmojiEntry],
        selected: usize,
    ) -> io::Result<()> {
        let mut out = stdout();
        queue!(out, cursor::MoveTo(0, 0), Clear(ClearType::FromCursorDown))?;

        // Header
        queue!(
            out,
            SetForegroundColor(Color::Cyan),
            Print("╭── 🛡️  GLYPH EMOJI FINDER ──────────────────────────────────────────────╮\r\n"),
            Print("│ "),
            SetForegroundColor(Color::White),
            Print("Search: "),
            SetForegroundColor(Color::Yellow),
            Print(query),
            Print("▌"),
            ResetColor,
            cursor::MoveToColumn(74),
            SetForegroundColor(Color::Cyan),
            Print("│\r\n"),
            Print("├──"),
            SetForegroundColor(Color::DarkGrey),
            Print("─────────────────────────────────────────────────────────────────────"),
            SetForegroundColor(Color::Cyan),
            Print("┤\r\n")
        )?;

        // Results list (up to 10 rows visible)
        let visible_count = 10;
        let start_idx = if selected >= visible_count {
            selected - visible_count + 1
        } else {
            0
        };
        let end_idx = (start_idx + visible_count).min(matches.len());

        for (i, &entry) in matches.iter().enumerate().take(end_idx).skip(start_idx) {
            let is_sel = i == selected;

            queue!(
                out,
                SetForegroundColor(Color::Cyan),
                Print("│ "),
                ResetColor
            )?;

            if is_sel {
                queue!(
                    out,
                    SetForegroundColor(Color::Green),
                    Print("> "),
                    ResetColor
                )?;
            } else {
                queue!(out, Print("  "))?;
            }

            // Print Glyph
            queue!(
                out,
                Print(entry.glyph),
                Print(" ")
            )?;

            // Print Shortcode
            let code_str = format!("{:<16}", format!(":{}", entry.shortcode));
            if is_sel {
                queue!(
                    out,
                    SetForegroundColor(Color::Yellow),
                    Print(&code_str),
                    ResetColor
                )?;
            } else {
                queue!(
                    out,
                    SetForegroundColor(Color::White),
                    Print(&code_str),
                    ResetColor
                )?;
            }

            // Print Name & Tags
            let tags_str = if !entry.tags.is_empty() {
                format!("[{}]", entry.tags.join(", "))
            } else {
                String::new()
            };
            let desc = format!("{} {}", entry.name, tags_str);
            let truncated = if desc.width() > 42 {
                format!("{}...", &desc[..39.min(desc.len())])
            } else {
                desc
            };

            queue!(
                out,
                SetForegroundColor(Color::DarkGrey),
                Print(format!("{:<45}", truncated)),
                ResetColor,
                cursor::MoveToColumn(74),
                SetForegroundColor(Color::Cyan),
                Print("│\r\n")
            )?;
        }

        if matches.is_empty() {
            queue!(
                out,
                SetForegroundColor(Color::Cyan),
                Print("│ "),
                SetForegroundColor(Color::Red),
                Print("  No matching glyphs found.                                          "),
                SetForegroundColor(Color::Cyan),
                Print("│\r\n")
            )?;
        }

        // Footer
        queue!(
            out,
            SetForegroundColor(Color::Cyan),
            Print("╰────────────────────────────────────────────────────────────────────────╯\r\n"),
            SetForegroundColor(Color::DarkGrey),
            Print("[↑/↓] Navigate  •  [Enter] Select & Emit  •  [Esc/Ctrl+C] Quit\r\n"),
            ResetColor
        )?;

        out.flush()?;
        Ok(())
    }
}

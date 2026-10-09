// ==============================================================================
// Sovereign Glyph Engine - Input Sanitization & Threat Neutralization
// Defends against ANSI poisoning, allocation bombs, and control byte attacks
// ==============================================================================

/// Maximum allowed input length for queries or shortcodes (prevents allocation bombs)
pub const MAX_INPUT_LENGTH: usize = 4096;

#[derive(PartialEq)]
enum AnsiState {
    Normal,
    Esc,
    Csi,
    Osc,
}

/// Sanitizes user-provided string queries or inputs before internal processing.
/// - Truncates to MAX_INPUT_LENGTH
/// - Strips unprintable control characters and dangerous ANSI escape sequences (CSI and OSC)
pub fn sanitize_input(input: &str) -> String {
    let bounded = if input.len() > MAX_INPUT_LENGTH {
        &input[..MAX_INPUT_LENGTH]
    } else {
        input
    };

    let mut clean = String::with_capacity(bounded.len());
    let mut state = AnsiState::Normal;

    for ch in bounded.chars() {
        match state {
            AnsiState::Normal => {
                if ch == '\x1b' {
                    state = AnsiState::Esc;
                } else if ch == '\t' || ch == '\n' || ch == '\r' || !ch.is_control() {
                    clean.push(ch);
                }
            }
            AnsiState::Esc => {
                if ch == '[' {
                    state = AnsiState::Csi;
                } else if ch == ']' {
                    state = AnsiState::Osc;
                } else {
                    // 2-byte escape sequence completed
                    state = AnsiState::Normal;
                }
            }
            AnsiState::Csi => {
                // CSI terminates on alphabetic letter or '~'
                if ch.is_ascii_alphabetic() || ch == '~' {
                    state = AnsiState::Normal;
                }
            }
            AnsiState::Osc => {
                // OSC terminates on BEL (\x07) or String Terminator (\x1b\\)
                if ch == '\x07' || ch == '\\' {
                    state = AnsiState::Normal;
                }
            }
        }
    }

    clean
}

/// Neutralizes spreadsheet formula injection payloads for exported logs or data
pub fn neutralize_formula(input: &str) -> String {
    // If raw string starts with tab/cr or formula trigger chars:
    if input.starts_with('=')
        || input.starts_with('+')
        || input.starts_with('-')
        || input.starts_with('@')
        || input.starts_with('\t')
        || input.starts_with('\r')
    {
        return format!("'{}", input);
    }

    let trimmed = input.trim_start_matches(' ');
    if trimmed.starts_with('=')
        || trimmed.starts_with('+')
        || trimmed.starts_with('-')
        || trimmed.starts_with('@')
        || trimmed.starts_with('\t')
        || trimmed.starts_with('\r')
    {
        format!("'{}", input)
    } else {
        input.to_string()
    }
}

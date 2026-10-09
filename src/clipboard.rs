// ==============================================================================
// Sovereign Glyph Engine - Universal Clipboard & OSC 52 Engine
// Supports Zero-Cloud Local Clipboard and Remote SSH Clipboard Injection
// ==============================================================================

use std::io::{self, Write};
use std::process::{Command, Stdio};

pub struct Clipboard;

impl Clipboard {
    /// Copies text to the system clipboard using OSC 52 escape sequences and local tools.
    pub fn copy(text: &str) -> io::Result<()> {
        // 1. Emit OSC 52 sequence to stdout (works natively over SSH!)
        let osc52 = Self::build_osc52_sequence(text);
        let mut stdout = io::stdout().lock();
        stdout.write_all(osc52.as_bytes())?;
        stdout.flush()?;

        // 2. Best-effort fallback to local desktop clipboard tools
        let _ = Self::copy_local(text);

        Ok(())
    }

    /// Builds the standard terminal OSC 52 escape sequence: \x1b]52;c;<base64>\x07
    pub fn build_osc52_sequence(text: &str) -> String {
        let b64 = Self::base64_encode(text.as_bytes());
        format!("\x1b]52;c;{}\x07", b64)
    }

    /// Best-effort OS native clipboard command invocation
    fn copy_local(text: &str) -> bool {
        // Wayland
        if Self::pipe_to_command("wl-copy", &[], text) {
            return true;
        }
        // X11 xclip
        if Self::pipe_to_command("xclip", &["-selection", "clipboard"], text) {
            return true;
        }
        // X11 xsel
        if Self::pipe_to_command("xsel", &["--clipboard", "--input"], text) {
            return true;
        }
        // macOS pbcopy
        if Self::pipe_to_command("pbcopy", &[], text) {
            return true;
        }
        // Windows clip.exe
        if Self::pipe_to_command("clip", &[], text) {
            return true;
        }

        false
    }

    fn pipe_to_command(cmd: &str, args: &[&str], input: &str) -> bool {
        let mut child = match Command::new(cmd)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(c) => c,
            Err(_) => return false,
        };

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(input.as_bytes());
        }

        matches!(child.wait(), Ok(status) if status.success())
    }

    /// Pure Rust zero-dependency standard Base64 encoder for OSC 52 payloads
    #[allow(clippy::chunks_exact_to_as_chunks)]
    pub fn base64_encode(bytes: &[u8]) -> String {
        const CHARSET: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);

        let mut chunks = bytes.chunks_exact(3);
        for chunk in &mut chunks {
            let b0 = chunk[0] as usize;
            let b1 = chunk[1] as usize;
            let b2 = chunk[2] as usize;

            out.push(CHARSET[b0 >> 2] as char);
            out.push(CHARSET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);
            out.push(CHARSET[((b1 & 0x0F) << 2) | (b2 >> 6)] as char);
            out.push(CHARSET[b2 & 0x3F] as char);
        }

        let rem = chunks.remainder();
        if rem.len() == 1 {
            let b0 = rem[0] as usize;
            out.push(CHARSET[b0 >> 2] as char);
            out.push(CHARSET[(b0 & 0x03) << 4] as char);
            out.push('=');
            out.push('=');
        } else if rem.len() == 2 {
            let b0 = rem[0] as usize;
            let b1 = rem[1] as usize;
            out.push(CHARSET[b0 >> 2] as char);
            out.push(CHARSET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);
            out.push(CHARSET[(b1 & 0x0F) << 2] as char);
            out.push('=');
        }

        out
    }
}

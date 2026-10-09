// ==============================================================================
// Sovereign Glyph Engine - Command Line Interface Parser & Dispatcher
// Fast, Zero-Dependency Argument Parsing with Radical OJP Modularity
// ==============================================================================

use std::env;
use std::io::{self, IsTerminal, Write};

use crate::catalog::{Catalog, Category};
use crate::clipboard::Clipboard;
use crate::interpolate::Interpolator;
use crate::lookup::LookupEngine;
use crate::picker::InteractivePicker;

pub struct CliConfig {
    pub copy: bool,
    pub no_newline: bool,
    pub interactive: bool,
    pub random: bool,
    pub command: Option<Command>,
}

pub enum Command {
    Lookup(String),
    Interpolate(String),
    Search(String),
    List(Option<Category>),
    Random,
    Interactive,
    Help,
    Version,
}

pub struct Cli;

impl Cli {
    pub fn parse() -> CliConfig {
        let args: Vec<String> = env::args().skip(1).collect();

        let mut copy = false;
        let mut no_newline = false;
        let mut interactive = false;
        let mut random = false;
        let mut positional = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-c" | "--copy" => copy = true,
                "-n" | "--no-newline" => no_newline = true,
                "-i" | "--interactive" => interactive = true,
                "-r" | "--random" => random = true,
                "-h" | "--help" => {
                    return CliConfig {
                        copy: false,
                        no_newline: false,
                        interactive: false,
                        random: false,
                        command: Some(Command::Help),
                    };
                }
                "-v" | "--version" => {
                    return CliConfig {
                        copy: false,
                        no_newline: false,
                        interactive: false,
                        random: false,
                        command: Some(Command::Version),
                    };
                }
                _ => {
                    positional.push(arg);
                }
            }
        }

        if interactive {
            return CliConfig {
                copy,
                no_newline,
                interactive: true,
                random: false,
                command: Some(Command::Interactive),
            };
        }

        if random {
            return CliConfig {
                copy,
                no_newline,
                interactive: false,
                random: true,
                command: Some(Command::Random),
            };
        }

        if positional.is_empty() {
            // If TTY, default to interactive picker. Otherwise print help.
            if io::stdout().is_terminal() {
                return CliConfig {
                    copy,
                    no_newline,
                    interactive: true,
                    random: false,
                    command: Some(Command::Interactive),
                };
            } else {
                return CliConfig {
                    copy,
                    no_newline,
                    interactive: false,
                    random: false,
                    command: Some(Command::Help),
                };
            }
        }

        let first = positional[0].as_str();
        let cmd = match first {
            "search" | "find" => {
                let query = positional[1..].join(" ");
                Command::Search(query)
            }
            "list" => {
                let cat = positional.get(1).and_then(|c| Category::parse(c));
                Command::List(cat)
            }
            "random" => Command::Random,
            "pick" | "picker" => Command::Interactive,
            _ => {
                // If single word or short token, check if it directly resolves to an emoji
                if positional.len() == 1 {
                    let token = &positional[0];
                    if token.contains(':') && token.len() > 2 && !token.starts_with(':') {
                        // Looks like sentence interpolation with embedded colons
                        Command::Interpolate(token.clone())
                    } else {
                        Command::Lookup(token.clone())
                    }
                } else {
                    // Multiple words passed: sentence interpolation
                    Command::Interpolate(positional.join(" "))
                }
            }
        };

        CliConfig {
            copy,
            no_newline,
            interactive: false,
            random: false,
            command: Some(cmd),
        }
    }

    pub fn execute(config: &CliConfig) -> io::Result<()> {
        let cmd = match &config.command {
            Some(c) => c,
            None => return Ok(()),
        };

        match cmd {
            Command::Help => Self::print_help(),
            Command::Version => Self::print_version(),
            Command::Lookup(query) => {
                if let Some(entry) = LookupEngine::resolve(query) {
                    Self::emit(entry.glyph, config)?;
                } else {
                    // Try search suggestions if no direct match
                    let suggestions = LookupEngine::search(query);
                    if !suggestions.is_empty() {
                        eprintln!("No exact match for '{}'. Did you mean:", query);
                        for res in suggestions.iter().take(5) {
                            eprintln!("  {}  :{:<16} - {}", res.entry.glyph, res.entry.shortcode, res.entry.name);
                        }
                    } else {
                        eprintln!("No glyph found matching '{}'. Try 'em find <keyword>'", query);
                    }
                }
            }
            Command::Interpolate(text) => {
                let processed = Interpolator::process(text);
                Self::emit(&processed, config)?;
            }
            Command::Search(query) => {
                let results = LookupEngine::search(query);
                if results.is_empty() {
                    println!("No glyphs matching '{}'.", query);
                } else {
                    println!("Found {} glyphs matching '{}':", results.len(), query);
                    for res in results.iter().take(20) {
                        println!(
                            "  {}  :{:<18} ({})",
                            res.entry.glyph, res.entry.shortcode, res.entry.name
                        );
                    }
                }
            }
            Command::List(maybe_cat) => {
                if let Some(cat) = maybe_cat {
                    println!("Category: {}", cat.as_str());
                    for entry in Catalog::by_category(*cat) {
                        println!("  {}  :{:<18} {}", entry.glyph, entry.shortcode, entry.name);
                    }
                } else {
                    println!("Available categories:");
                    for cat in Category::ALL {
                        let count = Catalog::by_category(*cat).count();
                        println!("  • {:<12} ({} glyphs)", cat.as_str(), count);
                    }
                    println!("\nUse 'em list <category>' to inspect.");
                }
            }
            Command::Random => {
                let all = Catalog::all();
                if !all.is_empty() {
                    // Ultra-fast deterministic cycle or timestamp-based pseudo-random
                    let nanos = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.subsec_nanos() as usize)
                        .unwrap_or(0);
                    let idx = nanos % all.len();
                    Self::emit(all[idx].glyph, config)?;
                }
            }
            Command::Interactive => {
                if let Some(entry) = InteractivePicker::run()? {
                    Self::emit(entry.glyph, config)?;
                }
            }
        }

        Ok(())
    }

    fn emit(output: &str, config: &CliConfig) -> io::Result<()> {
        let mut out = io::stdout().lock();
        if config.no_newline {
            write!(out, "{}", output)?;
        } else {
            writeln!(out, "{}", output)?;
        }
        out.flush()?;

        if config.copy {
            let _ = Clipboard::copy(output);
            if io::stderr().is_terminal() {
                eprintln!("📋 Copied to clipboard!");
            }
        }

        Ok(())
    }

    fn print_version() {
        println!("glyph (em) v0.1.0 - Sovereign Emoji & Symbol Engine");
    }

    fn print_help() {
        println!(
            r#"🛡️ GLYPH (em) - Sovereign Emoji & Symbol Engine on Demand

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

EXAMPLES:
    $ em smile                   # 😊
    $ em ":-)"                   # 😊
    $ em idea                    # 💡
    $ em shield                  # 🛡️
    $ em -c fire                 # 🔥 (copied to clipboard across SSH!)
    $ em "Done :check: :rocket:" # Done ✅ 🚀
    $ git commit -m "$(em shield) Fortified firewall"
"#
        );
    }
}

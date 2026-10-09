// ==============================================================================
// Sovereign Glyph Engine - Primary CLI Entrypoint (`glyph`)
// Radical OJP: Thin Lifecycle Shell (<= 80 lines)
// ==============================================================================

use std::process::ExitCode;

fn main() -> ExitCode {
    let config = glyph::cli::Cli::parse();
    match glyph::cli::Cli::execute(&config) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("glyph error: {}", e);
            ExitCode::FAILURE
        }
    }
}

// src/bin/ftop.rs — `ftop` standalone binary
// Thin entry point that delegates to the FANCYBASH TUI system monitor.
// Build:  cargo build --bin ftop --features tools
// Usage:  ftop

#[cfg(feature = "tools")]
fn main() {
    if let Err(e) = fancybash_core::tools::system_monitor::run() {
        eprintln!("ftop: {e}");
        std::process::exit(1);
    }
}

#[cfg(not(feature = "tools"))]
fn main() {
    eprintln!(
        "ftop was compiled without the 'tools' feature.\n\
         Re-build with:  cargo install --path . --features tools"
    );
    std::process::exit(1);
}

// =============================================================================
//  src/bin/ftop.rs — Standalone `ftop` binary
//  Delegates to the shared gladeshell_core ftop engine (src/tools/ftop.rs).
//  Both `gladeshell sysmon` and `ftop` run identical code — zero duplication.
//  Auto-updated alongside gladeshell on every `cargo install --path . --force`.
// =============================================================================

fn main() {
    if let Err(e) = gladeshell_core::tools::ftop::run() {
        eprintln!("ftop: {e}");
        std::process::exit(1);
    }
}

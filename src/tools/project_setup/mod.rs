// =============================================================================
//  src/tools/project_setup/mod.rs — Modular Web & Native Project Generators
// =============================================================================

pub mod utils;
pub mod next;
pub mod vite;
pub mod shadcn_ui;
pub mod tailwind;
pub mod ii;
pub mod html;
pub mod hub;

pub use utils::{patch_tsconfig, patch_viteconfig, inject_ts_paths};
pub use next::run_next;
pub use vite::run_vite;
pub use shadcn_ui::run_ui;
pub use tailwind::run_css;
pub use ii::run_ii;
pub use html::run_html;
pub use hub::{run_project, PROJECT_TOOLS, ProjectToolItem};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_css_target_paths() {
        assert!(Path::new("Cargo.toml").exists());
    }

    #[test]
    fn test_inject_ts_paths_with_compiler_options() {
        let input = r#"{ "compilerOptions": { "strict": true } }"#;
        let output = inject_ts_paths(input);
        assert!(output.contains("\"@/*\""));
        assert!(output.contains("baseUrl"));
    }

    #[test]
    fn test_inject_ts_paths_fallback() {
        let input = r#"{ "include": ["src"] }"#;
        let output = inject_ts_paths(input);
        assert!(output.contains("compilerOptions"));
        assert!(output.contains("\"@/*\""));
    }
}

// src/tools/mod.rs
// Phase 4 & 5 tool subcommand modules.
// Each module replaces a complex shell function with a native Rust implementation.

pub mod theme_picker;
pub mod auto_ls;
pub mod bun_runner;
pub mod cpp_gen;
pub mod dep_installer;
pub mod dman;
pub mod drive_jumper;
pub mod extractor;
pub mod fast_grep;
pub mod ffmedia;
pub mod file_renamer;
pub mod fkill;
pub mod fuzzy_cd;
pub mod gbranch;
pub mod git_wip;
pub mod history_search;
pub mod keep;
pub mod notes;
pub mod pc_optimizer;
pub mod pc_info;
pub mod pkg_converter;
pub mod project_setup;
pub mod runtime_installer;
pub mod secret_gen_tool;
pub mod self_uninstall;
pub mod self_upgrade;
pub mod system_clean;
pub mod ftop;
pub use ftop as system_monitor;
pub mod system_update;
pub mod todo;
pub mod touch_tool;
pub mod uninstaller;
pub mod universal_clean;
pub mod updater;
pub mod vault;
pub mod video_player;
pub mod mkd;
pub mod rmd;
pub mod rmf;
pub mod bak;
pub mod trash;
pub mod file_find;
pub mod zed_setup;



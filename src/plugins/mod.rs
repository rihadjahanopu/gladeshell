// ============================================================================
//  src/plugins/mod.rs — Plugin module registry
// ============================================================================
//
//  Each sub-module is self-contained with zero cross-module shared state.
//  All public functions are panic-free and produce empty/None on any failure.

pub mod autocomplete;
pub mod autosuggest;
pub mod highlight;

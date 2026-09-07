// =============================================================================
//  src/core/aliases.rs — Parser for aliases.toml → multi-shell exporter
//
//  Data flow:
//    aliases.toml  →  Vec<AliasGroup>  →  target shell syntax string
//
//  Phase 1: TOML schema + serializer for all four shells.
//  Phase 3: full migration of 10,000-line shell config aliases.
// =============================================================================

use serde::Deserialize;

// ── TOML schema ───────────────────────────────────────────────────────────────

/// Top-level structure of `aliases.toml`.
#[derive(Debug, Deserialize)]
pub struct AliasFile {
    /// Named groups of aliases (e.g. "navigation", "git", "npm")
    pub group: Vec<AliasGroup>,
}

/// A logical category of aliases with optional per-group shell restrictions.
#[derive(Debug, Deserialize)]
pub struct AliasGroup {
    /// Human-readable category name
    pub name: String,
    /// List of alias entries in this group
    pub aliases: Vec<AliasEntry>,
    /// Optional: only emit for these shells (empty = all shells)
    #[serde(default)]
    pub only_shells: Vec<String>,
}

/// A single alias definition.
#[derive(Debug, Deserialize)]
pub struct AliasEntry {
    /// The short alias name (e.g. `gs`)
    pub key: String,
    /// The full command string (e.g. `git status -sb`)
    pub value: String,
    /// Optional human-readable description / comment
    #[serde(default)]
    pub description: String,
}

// ── Shell targets ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Pwsh,
}

impl Shell {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "bash" => Some(Shell::Bash),
            "zsh"  => Some(Shell::Zsh),
            "fish" => Some(Shell::Fish),
            "pwsh" | "powershell" => Some(Shell::Pwsh),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Shell::Bash => "bash",
            Shell::Zsh  => "zsh",
            Shell::Fish => "fish",
            Shell::Pwsh => "pwsh",
        }
    }
}

// ── Serializer ────────────────────────────────────────────────────────────────

impl AliasFile {
    /// Load from a TOML string.
    pub fn from_toml(src: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(src)
    }

    /// Render all aliases for `shell` as a valid shell script string.
    pub fn render(&self, shell: Shell) -> String {
        let mut out = String::with_capacity(4096);

        for group in &self.group {
            // Respect per-group shell restriction
            if !group.only_shells.is_empty()
                && !group
                    .only_shells
                    .iter()
                    .any(|s| Shell::from_str(s) == Some(shell))
            {
                continue;
            }

            // Section header comment
            match shell {
                Shell::Bash | Shell::Zsh => {
                    out.push_str(&format!("\n# ── {} ──\n", group.name));
                }
                Shell::Fish => {
                    out.push_str(&format!("\n# ── {} ──\n", group.name));
                }
                Shell::Pwsh => {
                    out.push_str(&format!("\n# ── {} ──\n", group.name));
                }
            }

            for entry in &group.aliases {
                match shell {
                    Shell::Bash | Shell::Zsh => {
                        if entry.description.is_empty() {
                            out.push_str(&format!(
                                "alias {}='{}'\n",
                                entry.key, entry.value
                            ));
                        } else {
                            out.push_str(&format!(
                                "alias {}='{}'  # {}\n",
                                entry.key, entry.value, entry.description
                            ));
                        }
                    }
                    Shell::Fish => {
                        // Fish uses `abbr` for expandable aliases or `alias`
                        // We default to `alias` for compatibility.
                        if entry.description.is_empty() {
                            out.push_str(&format!(
                                "alias {} '{}'\n",
                                entry.key, entry.value
                            ));
                        } else {
                            out.push_str(&format!(
                                "alias {} '{}'  # {}\n",
                                entry.key, entry.value, entry.description
                            ));
                        }
                    }
                    Shell::Pwsh => {
                        // PowerShell uses Set-Alias for simple command aliases;
                        // for commands with arguments we emit a function wrapper.
                        if entry.value.contains(' ') {
                            // Multi-word → function wrapper
                            let fname = pascal_case(&entry.key);
                            if entry.description.is_empty() {
                                out.push_str(&format!(
                                    "function {fname} {{ {} $args }}\n",
                                    entry.value
                                ));
                            } else {
                                out.push_str(&format!(
                                    "# {}\nfunction {fname} {{ {} $args }}\n",
                                    entry.description, entry.value
                                ));
                            }
                        } else {
                            // Single-command → Set-Alias
                            if entry.description.is_empty() {
                                out.push_str(&format!(
                                    "Set-Alias -Name {} -Value {}\n",
                                    entry.key, entry.value
                                ));
                            } else {
                                out.push_str(&format!(
                                    "# {}\nSet-Alias -Name {} -Value {}\n",
                                    entry.description, entry.key, entry.value
                                ));
                            }
                        }
                    }
                }
            }
        }

        out
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Convert a kebab/snake alias name to PascalCase for PowerShell function names.
/// `gs` → `Gs`, `make-cpp` → `MakeCpp`
fn pascal_case(s: &str) -> String {
    s.split(|c: char| c == '-' || c == '_')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut chars = p.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

// =============================================================================
//  Unit tests
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_TOML: &str = r#"
[[group]]
name = "Navigation"

[[group.aliases]]
key   = ".."
value = "cd .."
description = "Go up one directory"

[[group.aliases]]
key   = "..."
value = "cd ../.."

[[group]]
name = "Git"

[[group.aliases]]
key   = "gs"
value = "git status -sb"
description = "Git status (short)"

[[group.aliases]]
key   = "gp"
value = "git push"
"#;

    #[test]
    fn parses_toml() {
        let af = AliasFile::from_toml(SAMPLE_TOML).unwrap();
        assert_eq!(af.group.len(), 2);
        assert_eq!(af.group[0].aliases.len(), 2);
    }

    #[test]
    fn render_bash() {
        let af = AliasFile::from_toml(SAMPLE_TOML).unwrap();
        let out = af.render(Shell::Bash);
        assert!(out.contains("alias ..='cd ..'"));
        assert!(out.contains("alias gs='git status -sb'"));
    }

    #[test]
    fn render_fish() {
        let af = AliasFile::from_toml(SAMPLE_TOML).unwrap();
        let out = af.render(Shell::Fish);
        assert!(out.contains("alias .. 'cd ..'"));
    }

    #[test]
    fn render_pwsh_multi_word_is_function() {
        let af = AliasFile::from_toml(SAMPLE_TOML).unwrap();
        let out = af.render(Shell::Pwsh);
        // "git status -sb" has spaces → must be a function wrapper
        assert!(out.contains("function Gs {"));
    }

    #[test]
    fn pascal_case_works() {
        assert_eq!(pascal_case("gs"), "Gs");
        assert_eq!(pascal_case("make-cpp"), "MakeCpp");
        assert_eq!(pascal_case("my_long_alias"), "MyLongAlias");
    }

    #[test]
    fn parses_embedded_aliases_toml() {
        let raw = include_str!("../../aliases.toml");
        let af = AliasFile::from_toml(raw).expect("aliases.toml must be valid TOML");
        assert!(af.group.len() >= 10, "must have at least 10 groups, got {}", af.group.len());
    }
}

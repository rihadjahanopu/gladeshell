// =============================================================================
//  src/core/aliases/mod.rs — Modular multi-shell alias system
//
//  Data flow:
//    Built-in category modules (.rs) OR TOML → Vec<AliasGroup> → shell script
// =============================================================================

pub mod categories;

use serde::Deserialize;

// ── TOML / Rust Schema ────────────────────────────────────────────────────────

/// Top-level container of alias groups.
#[derive(Debug, Clone, Deserialize)]
pub struct AliasFile {
    /// Named groups of aliases (e.g. "Navigation & Filesystem", "Git", "npm")
    pub group: Vec<AliasGroup>,
}

/// A logical category of aliases with optional per-group shell restrictions.
#[derive(Debug, Clone, Deserialize)]
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
#[derive(Debug, Clone, Deserialize)]
pub struct AliasEntry {
    /// The short alias name (e.g. `gs`)
    pub key: String,
    /// The full command string (e.g. `git status -sb`)
    pub value: String,
    /// Optional human-readable description / comment
    #[serde(default)]
    pub description: String,
    /// Optional: only emit for these shells (empty = all shells)
    #[serde(default)]
    pub only_shells: Vec<String>,
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

// ── Constructors & Serializer ─────────────────────────────────────────────────

impl AliasFile {
    /// Return built-in aliases constructed from category Rust modules.
    pub fn builtin() -> Self {
        Self {
            group: categories::all_groups(),
        }
    }

    /// Load from a TOML string.
    pub fn from_toml(src: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(src)
    }

    /// Render all aliases for `shell` as a valid shell script string.
    pub fn render(&self, shell: Shell) -> String {
        let mut out = String::with_capacity(8192);

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

            let mut group_header_pushed = false;

            for entry in &group.aliases {
                // Respect per-entry shell restriction
                if !entry.only_shells.is_empty()
                    && !entry
                        .only_shells
                        .iter()
                        .any(|s| Shell::from_str(s) == Some(shell))
                {
                    continue;
                }

                // Push group header once if not pushed yet
                if !group_header_pushed {
                    out.push_str(&format!("\n# ── {} ──\n", group.name));
                    group_header_pushed = true;
                }

                match shell {
                    Shell::Bash | Shell::Zsh => {
                        let escaped_val = entry.value.replace('\'', "'\\''");
                        if shell == Shell::Zsh {
                            out.push_str(&format!("unfunction {} 2>/dev/null\n", entry.key));
                        }
                        if entry.description.is_empty() {
                            out.push_str(&format!(
                                "alias {}='{}'\n",
                                entry.key, escaped_val
                            ));
                        } else {
                            out.push_str(&format!(
                                "alias {}='{}'  # {}\n",
                                entry.key, escaped_val, entry.description
                            ));
                        }
                    }
                    Shell::Fish => {
                        let escaped_val = entry.value.replace('\'', "\\'");
                        if entry.description.is_empty() {
                            out.push_str(&format!(
                                "alias {} '{}'\n",
                                entry.key, escaped_val
                            ));
                        } else {
                            out.push_str(&format!(
                                "alias {} '{}'  # {}\n",
                                entry.key, escaped_val, entry.description
                            ));
                        }
                    }
                    Shell::Pwsh => {
                        if entry.value.contains(' ') {
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
    fn builtin_has_all_categories() {
        let af = AliasFile::builtin();
        assert!(af.group.len() >= 15, "Expected at least 15 categories, got {}", af.group.len());
    }

    #[test]
    fn render_bash() {
        let af = AliasFile::builtin();
        let out = af.render(Shell::Bash);
        assert!(out.contains("alias ..='cd ..'"));
        assert!(out.contains("alias gs='git status -sb'"));
    }

    #[test]
    fn render_fish() {
        let af = AliasFile::builtin();
        let out = af.render(Shell::Fish);
        assert!(out.contains("alias .. 'cd ..'"));
    }

    #[test]
    fn render_pwsh_multi_word_is_function() {
        let af = AliasFile::builtin();
        let out = af.render(Shell::Pwsh);
        assert!(out.contains("function Gs {"));
    }

    #[test]
    fn pascal_case_works() {
        assert_eq!(pascal_case("gs"), "Gs");
        assert_eq!(pascal_case("make-cpp"), "MakeCpp");
        assert_eq!(pascal_case("my_long_alias"), "MyLongAlias");
    }
}

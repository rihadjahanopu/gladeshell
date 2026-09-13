// =============================================================================
//  src/init/shared.rs — Reusable modular components for shell initializers
// =============================================================================

use crate::core::aliases::Shell;
use crate::core::env;
use std::env as std_env;
use std::fs;
use std::path::{Path, PathBuf};

/// Get current user's HOME directory path.
pub fn home_dir() -> String {
    std_env::var("HOME").unwrap_or_default()
}

/// Find first file that exists among candidates on host system.
pub fn find_first_existing(candidates: &[String]) -> Option<String> {
    for candidate in candidates {
        if Path::new(candidate).is_file() {
            return Some(candidate.clone());
        }
    }
    None
}

/// Native Rust resolution of the latest NVM Node.js binary path.
pub fn get_latest_nvm_node_bin() -> Option<String> {
    let home = home_dir();
    if home.is_empty() {
        return None;
    }
    let nvm_dir = std_env::var("NVM_DIR").unwrap_or_else(|_| format!("{}/.config/nvm", home));
    let nvm_path = if Path::new(&nvm_dir).is_dir() {
        PathBuf::from(nvm_dir)
    } else {
        PathBuf::from(format!("{}/.nvm", home))
    };

    let node_versions_dir = nvm_path.join("versions").join("node");
    if let Ok(entries) = fs::read_dir(node_versions_dir) {
        let mut versions: Vec<String> = entries
            .flatten()
            .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        versions.sort();
        if let Some(latest) = versions.last() {
            let bin_path = nvm_path
                .join("versions")
                .join("node")
                .join(latest)
                .join("bin");
            if bin_path.is_dir() {
                return Some(bin_path.to_string_lossy().into_owned());
            }
        }
    }
    None
}

/// Render double-sourcing guard snippet per shell.
pub fn render_guard(shell: Shell) -> String {
    match shell {
        Shell::Zsh => r#"
# Guard against double-sourcing
[[ -n "${_FANCYBASH_ZSH_LOADED:-}" ]] && return 0
typeset -g _FANCYBASH_ZSH_LOADED=1
"#.to_string(),
        Shell::Bash => r#"
# Guard against double-sourcing
if [[ -n "${_FANCYBASH_BASH_LOADED:-}" ]]; then
    return 0 2>/dev/null || true
fi
export _FANCYBASH_BASH_LOADED=1
"#.to_string(),
        Shell::Fish => r#"
# Guard against double-sourcing
if set -q _FANCYBASH_FISH_LOADED
    return
end
set -gx _FANCYBASH_FISH_LOADED 1
"#.to_string(),
        Shell::Pwsh => r#"
# Guard against double-sourcing
if ($env:_FANCYBASH_PWSH_LOADED -eq '1') { return }
$env:_FANCYBASH_PWSH_LOADED = '1'
"#.to_string(),
    }
}

/// Render canonical environment variables and PATH entries per shell.
pub fn render_env_and_path(shell: Shell) -> String {
    let mut out = String::new();

    out.push_str("\n# ── Environment variables ──\n");
    for var in env::canonical_env_vars() {
        match shell {
            Shell::Zsh | Shell::Bash => out.push_str(&env::render_env_bash(&var)),
            Shell::Fish => out.push_str(&env::render_env_fish(&var)),
            Shell::Pwsh => out.push_str(&env::render_env_pwsh(&var)),
        }
        out.push('\n');
    }

    out.push_str("\n# ── PATH ──\n");
    for entry in env::canonical_path_entries() {
        match shell {
            Shell::Zsh | Shell::Bash => out.push_str(&env::render_path_bash(&entry)),
            Shell::Fish => out.push_str(&env::render_path_fish(&entry)),
            Shell::Pwsh => out.push_str(&env::render_path_pwsh(&entry)),
        }
        out.push('\n');
    }

    out
}

/// Render NVM lazy-load snippet per shell.
pub fn render_nvm_lazy_load(shell: Shell) -> String {
    let mut out = String::new();

    match shell {
        Shell::Zsh => {
            out.push_str(r#"
# ======================================================
# 🟢 NVM & NODE.JS DYNAMIC LAZY-LOAD
# ======================================================
export NVM_DIR="${NVM_DIR:-$HOME/.config/nvm}"
[[ ! -d "$NVM_DIR" && -d "$HOME/.nvm" ]] && export NVM_DIR="$HOME/.nvm"
"#);
            if let Some(node_bin) = get_latest_nvm_node_bin() {
                out.push_str(&format!("export PATH=\"{}:$PATH\"\n", node_bin));
            }
            out.push_str(r#"
if [[ -o interactive ]]; then
    fancybash internal-clean-rc >/dev/null 2>&1 &!
fi

_fb_lazy_load_nvm() {
    unset -f nvm node npm npx 2>/dev/null
    fancybash internal-clean-rc >/dev/null 2>&1 &!
    if [ -s "$NVM_DIR/nvm.sh" ]; then
        \. "$NVM_DIR/nvm.sh"
    fi
    if [ -s "$NVM_DIR/bash_completion" ]; then
        autoload -Uz bashcompinit 2>/dev/null
        bashcompinit 2>/dev/null || true
        \. "$NVM_DIR/bash_completion"
    fi
}

nvm()  { _fb_lazy_load_nvm; nvm  "$@"; }
node() { _fb_lazy_load_nvm; node "$@"; }
npm()  { _fb_lazy_load_nvm; npm  "$@"; }
npx()  { _fb_lazy_load_nvm; npx  "$@"; }
"#);
        }
        Shell::Bash => {
            out.push_str(r#"
# ── NVM lazy-load (zero startup cost) ──
export NVM_DIR="${NVM_DIR:-$HOME/.config/nvm}"
[[ ! -d "$NVM_DIR" && -d "$HOME/.nvm" ]] && export NVM_DIR="$HOME/.nvm"
"#);
            if let Some(node_bin) = get_latest_nvm_node_bin() {
                out.push_str(&format!("export PATH=\"{}:$PATH\"\n", node_bin));
            }
            out.push_str(r#"
_fb_lazy_load_nvm() {
    unset -f nvm node npm npx
    [[ -s "$NVM_DIR/nvm.sh" ]] && \. "$NVM_DIR/nvm.sh"
    [[ -s "$NVM_DIR/bash_completion" ]] && \. "$NVM_DIR/bash_completion"
}
nvm()  { _fb_lazy_load_nvm; nvm  "$@"; }
node() { _fb_lazy_load_nvm; node "$@"; }
npm()  { _fb_lazy_load_nvm; npm  "$@"; }
npx()  { _fb_lazy_load_nvm; npx  "$@"; }
"#);
        }
        Shell::Fish => {
            out.push_str(r#"
# ── NVM (Fish-compatible lazy-load via nvm.fish plugin) ──
if functions -q nvm
    # nvm.fish is already loaded; nothing to do.
else
    set -gx NVM_DIR (test -d $HOME/.config/nvm && echo $HOME/.config/nvm || echo $HOME/.nvm)
end
"#);
        }
        Shell::Pwsh => {}
    }

    out
}

/// Render Bun setup snippet per shell.
pub fn render_bun_setup(shell: Shell) -> String {
    match shell {
        Shell::Zsh => r#"
# ======================================================
# 🥐 BUN ENVIRONMENT & AUTOCOMPLETION
# ======================================================
export BUN_INSTALL="${BUN_INSTALL:-$HOME/.bun}"

if [[ -d "$BUN_INSTALL/bin" && ":$PATH:" != *":$BUN_INSTALL/bin:"* ]]; then
    export PATH="$BUN_INSTALL/bin:$PATH"
fi

if [[ -s "$BUN_INSTALL/_bun" ]]; then
    [ -s "$BUN_INSTALL/_bun" ] && source "$BUN_INSTALL/_bun" 2>/dev/null
fi
"#.to_string(),
        Shell::Bash => r#"
# ── Bun environment ──
export BUN_INSTALL="${BUN_INSTALL:-$HOME/.bun}"
if [[ -d "$BUN_INSTALL/bin" && ":$PATH:" != *":$BUN_INSTALL/bin:"* ]]; then
    export PATH="$BUN_INSTALL/bin:$PATH"
fi
"#.to_string(),
        Shell::Fish => r#"
# ── Bun ──
set -gx BUN_INSTALL "$HOME/.bun"
fish_add_path --prepend "$BUN_INSTALL/bin"
"#.to_string(),
        Shell::Pwsh => "".to_string(),
    }
}

/// Render safe ls / eza fallback wrapper snippet per shell.
pub fn render_safe_ls_wrapper(shell: Shell) -> String {
    match shell {
        Shell::Bash | Shell::Zsh => r#"
# ── Safe ls / eza fallback wrapper ──
unalias ls ll la lt tree 2>/dev/null
if command -v eza &>/dev/null; then
    alias ls='eza --icons --group-directories-first'
    alias ll='eza -lah --icons --group-directories-first --git'
    alias la='eza -a --icons'
    alias lt='eza --tree --icons --level=2'
    alias tree='eza --tree --icons'
else
    alias ls='ls --color=auto'
    alias ll='ls -la --color=auto'
    alias la='ls -A --color=auto'
fi
"#.to_string(),
        Shell::Fish => r#"
# ── Safe ls / eza fallback wrapper ──
functions -e ls ll la lt tree 2>/dev/null
if command -v eza &>/dev/null
    alias ls 'eza --icons --group-directories-first'
    alias ll 'eza -lah --icons --group-directories-first --git'
    alias la 'eza -a --icons'
    alias lt 'eza --tree --icons --level=2'
    alias tree 'eza --tree --icons'
else
    alias ls 'ls --color=auto'
    alias ll 'ls -la --color=auto'
    alias la 'ls -A --color=auto'
end
"#.to_string(),
        Shell::Pwsh => r#"
# ── Safe ls / eza fallback wrapper ──
if (Get-Command eza -ErrorAction SilentlyContinue) {
    function global:ls { eza --icons --group-directories-first @args }
    function global:ll { eza -lah --icons --group-directories-first --git @args }
    function global:la { eza -a --icons @args }
} else {
    Remove-Item alias:ls -ErrorAction SilentlyContinue 2>$null
    function global:ls { Get-ChildItem @args }
    function global:ll { Get-ChildItem -Force @args }
    function global:la { Get-ChildItem -Force @args }
}
"#.to_string(),
    }
}

/// Render built-in aliases and safe ls wrapper snippet per shell.
pub fn render_aliases(shell: Shell) -> String {
    let mut out = String::new();
    out.push_str("\n# ── Aliases ──\n");
    out.push_str(&crate::core::aliases::AliasFile::builtin().render(shell));
    out.push_str(&render_safe_ls_wrapper(shell));
    out
}

/// Render modern CLI tool integrations (Zoxide, FZF, Bat, Sensors) per shell.
pub fn render_integrations(shell: Shell) -> String {
    match shell {
        Shell::Zsh => r#"
# ======================================================
# 🚀 INTEGRATIONS (Zoxide, FZF, Bat, Sensors)
# ======================================================
# Zoxide Init & Smart CD
if command -v zoxide &>/dev/null; then
    eval "$(zoxide init zsh 2>/dev/null)"
    alias cd='z' 2>/dev/null
fi

# FZF Integration
if command -v fzf &>/dev/null; then
    eval "$(fzf --zsh 2>/dev/null || fzf --completion 2>/dev/null)"
fi

# Bat / Batcat Aliases
if command -v batcat &>/dev/null; then
    alias bat='batcat'
    alias cat='batcat -p'
elif command -v bat &>/dev/null; then
    alias cat='bat -p'
fi

# Sensor Alias
if command -v sensors &>/dev/null; then
    alias temp='sensors'
fi
"#.to_string(),
        Shell::Bash => r#"
# ======================================================
# 🚀 INTEGRATIONS (Zoxide, FZF, Bat, Sensors)
# ======================================================
# Zoxide Init & Smart CD
if command -v zoxide &>/dev/null; then
    eval "$(zoxide init bash 2>/dev/null)"
    alias cd='z' 2>/dev/null
fi

# FZF Integration
if command -v fzf &>/dev/null; then
    eval "$(fzf --bash 2>/dev/null)"
fi

# Bat / Batcat Aliases
if command -v batcat &>/dev/null; then
    alias bat='batcat'
    alias cat='batcat -p'
elif command -v bat &>/dev/null; then
    alias cat='bat -p'
fi

# Sensor Alias
if command -v sensors &>/dev/null; then
    alias temp='sensors'
fi
"#.to_string(),
        Shell::Fish => r#"
# ======================================================
# 🚀 INTEGRATIONS (Zoxide, FZF, Bat, Sensors)
# ======================================================
if command -v zoxide &>/dev/null
    zoxide init fish | source 2>/dev/null
    alias cd='z'
end

if command -v fzf &>/dev/null
    fzf --fish | source 2>/dev/null
end

if command -v batcat &>/dev/null
    alias bat='batcat'
    alias cat='batcat -p'
else if command -v bat &>/dev/null
    alias cat='bat -p'
end

if command -v sensors &>/dev/null
    alias temp='sensors'
end
"#.to_string(),
        Shell::Pwsh => r#"
# ======================================================
# 🚀 INTEGRATIONS (Zoxide, FZF, Bat, Sensors)
# ======================================================
if (Get-Command zoxide -ErrorAction SilentlyContinue) {
    Invoke-Expression (& { (zoxide init powershell | Out-String) })
    Set-Alias -Name cd -Value z -Option AllScope -Force
}
if (Get-Command batcat -ErrorAction SilentlyContinue) {
    Set-Alias -Name bat -Value batcat
    function global:cat { batcat -p @args }
} elseif (Get-Command bat -ErrorAction SilentlyContinue) {
    function global:cat { bat -p @args }
}
"#.to_string(),
    }
}


/// Render Native Rust Auto-LS hook snippet per shell.
pub fn render_auto_ls_hook(shell: Shell) -> String {
    match shell {
        Shell::Zsh => r#"
# ── Native Rust Auto-LS on directory change ──
unalias accurate_auto_ls 2>/dev/null
accurate_auto_ls() {
    fancybash auto-ls 2>/dev/null
}
add-zsh-hook chpwd accurate_auto_ls
"#.to_string(),
        Shell::Bash => r#"
# ── Native Rust Auto-LS on directory change ──
_fb_last_pwd="$PWD"
_fb_auto_ls() {
    if [[ "$PWD" != "$_fb_last_pwd" ]]; then
        _fb_last_pwd="$PWD"
        fancybash auto-ls 2>/dev/null
    fi
}
"#.to_string(),
        Shell::Fish => r#"
# ── Native Rust Auto-LS on directory change ──
functions -e accurate_auto_ls 2>/dev/null
function accurate_auto_ls --on-variable PWD
    fancybash auto-ls 2>/dev/null
end
"#.to_string(),
        Shell::Pwsh => r#"
# ── Native Rust Auto-LS trigger ──
if ($cwd -ne $global:_fb_last_pwd) {
    $global:_fb_last_pwd = $cwd
    fancybash auto-ls 2>$null
}
"#.to_string(),
    }
}

/// Render shell function wrapper for `cf` fuzzy directory navigator.
pub fn render_cf_wrapper(shell: Shell) -> String {
    match shell {
        Shell::Zsh | Shell::Bash => r#"
# ── Interactive Fuzzy Directory Navigator (`cf`) shell wrapper ──
unalias cf 2>/dev/null
cf() {
    local target
    target="$(fancybash cf "$@")"
    if [[ -n "$target" && -d "$target" ]]; then
        cd "$target" || return
    elif [[ -n "$target" && -f "$target" ]]; then
        cd "$(dirname "$target")" || return
    fi
}
"#.to_string(),
        Shell::Fish => r#"
# ── Interactive Fuzzy Directory Navigator (`cf`) shell wrapper ──
functions -e cf 2>/dev/null
function cf
    set -l target (fancybash cf $argv)
    if test -n "$target" -a -d "$target"
        cd "$target"
    else if test -n "$target" -a -f "$target"
        cd (dirname "$target")
    end
end
"#.to_string(),
        Shell::Pwsh => r#"
# ── Interactive Fuzzy Directory Navigator (`cf`) shell wrapper ──
Remove-Item alias:cf -ErrorAction SilentlyContinue 2>$null
function cf {
    $target = fancybash cf @args
    if ($target -and (Test-Path -Path $target -PathType Container)) {
        Set-Location -Path $target
    } elseif ($target -and (Test-Path -Path $target -PathType Leaf)) {
        Set-Location -Path (Split-Path -Parent $target)
    }
}
"#.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_dir_not_empty() {
        assert!(!home_dir().is_empty());
    }

    #[test]
    fn test_render_guard() {
        assert!(render_guard(Shell::Zsh).contains("_FANCYBASH_ZSH_LOADED"));
        assert!(render_guard(Shell::Bash).contains("_FANCYBASH_BASH_LOADED"));
        assert!(render_guard(Shell::Fish).contains("_FANCYBASH_FISH_LOADED"));
        assert!(render_guard(Shell::Pwsh).contains("_FANCYBASH_PWSH_LOADED"));
    }

    #[test]
    fn test_render_env_and_path() {
        let env_path = render_env_and_path(Shell::Zsh);
        assert!(env_path.contains("Environment variables"));
        assert!(env_path.contains("PATH"));
    }

    #[test]
    fn test_render_safe_ls_wrapper() {
        assert!(render_safe_ls_wrapper(Shell::Zsh).contains("eza"));
        assert!(render_safe_ls_wrapper(Shell::Fish).contains("eza"));
        assert!(render_safe_ls_wrapper(Shell::Pwsh).contains("Get-Command"));
    }

    #[test]
    fn test_render_auto_ls_hook() {
        assert!(render_auto_ls_hook(Shell::Zsh).contains("fancybash auto-ls"));
        assert!(render_auto_ls_hook(Shell::Bash).contains("fancybash auto-ls"));
        assert!(render_auto_ls_hook(Shell::Fish).contains("fancybash auto-ls"));
        assert!(render_auto_ls_hook(Shell::Pwsh).contains("fancybash auto-ls"));
    }

    #[test]
    fn test_render_nvm_lazy_load() {
        assert!(render_nvm_lazy_load(Shell::Zsh).contains("NVM_DIR"));
        assert!(render_nvm_lazy_load(Shell::Bash).contains("NVM_DIR"));
    }

    #[test]
    fn test_render_bun_setup() {
        assert!(render_bun_setup(Shell::Zsh).contains("BUN_INSTALL"));
        assert!(render_bun_setup(Shell::Fish).contains("BUN_INSTALL"));
    }

    #[test]
    fn test_render_cf_wrapper() {
        assert!(render_cf_wrapper(Shell::Bash).contains("unalias cf"));
        assert!(render_cf_wrapper(Shell::Zsh).contains("unalias cf"));
        assert!(render_cf_wrapper(Shell::Fish).contains("functions -e cf"));
        assert!(render_cf_wrapper(Shell::Pwsh).contains("Remove-Item alias:cf"));
    }

    #[test]
    fn test_render_integrations() {
        assert!(render_integrations(Shell::Zsh).contains("zoxide"));
        assert!(render_integrations(Shell::Zsh).contains("fzf"));
        assert!(render_integrations(Shell::Bash).contains("zoxide"));
        assert!(render_integrations(Shell::Bash).contains("fzf"));
        assert!(render_integrations(Shell::Fish).contains("zoxide"));
        assert!(render_integrations(Shell::Pwsh).contains("zoxide"));
    }
}


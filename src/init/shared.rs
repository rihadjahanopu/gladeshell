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
pub fn render_guard(_shell: Shell) -> String {
    String::new()
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
    unalias z zi 2>/dev/null
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
    unalias z zi 2>/dev/null
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
unalias cf _cf_open 2>/dev/null || true

# File-type aware opener: videos → mpv/vlc, images → eog/feh, audio → mpv, docs → xdg-open
_cf_open() {
    local file="$1"
    [[ -z "$file" ]] && return
    local ext="${file##*.}"
    ext="$(echo "$ext" | tr '[:upper:]' '[:lower:]')"
    case "$ext" in
        mp4|mkv|avi|mov|webm|flv|wmv|m4v|ogv|ts|rmvb|3gp)
            if   command -v mpv    &>/dev/null; then mpv    "$file" >/dev/null 2>&1 &
            elif command -v vlc    &>/dev/null; then vlc    "$file" >/dev/null 2>&1 &
            elif command -v mplayer &>/dev/null; then mplayer "$file" >/dev/null 2>&1 &
            else xdg-open "$file" >/dev/null 2>&1 &
            fi ;;
        jpg|jpeg|png|gif|bmp|webp|svg|ico|tiff|tif|avif|heic|raw)
            if   command -v eog    &>/dev/null; then eog    "$file" >/dev/null 2>&1 &
            elif command -v feh    &>/dev/null; then feh    "$file" >/dev/null 2>&1 &
            elif command -v imv    &>/dev/null; then imv    "$file" >/dev/null 2>&1 &
            elif command -v sxiv   &>/dev/null; then sxiv   "$file" >/dev/null 2>&1 &
            elif command -v nomacs &>/dev/null; then nomacs "$file" >/dev/null 2>&1 &
            else xdg-open "$file" >/dev/null 2>&1 &
            fi ;;
        mp3|flac|ogg|wav|aac|m4a|opus|wma)
            if   command -v mpv &>/dev/null; then mpv "$file" >/dev/null 2>&1 &
            elif command -v vlc &>/dev/null; then vlc "$file" >/dev/null 2>&1 &
            else xdg-open "$file" >/dev/null 2>&1 &
            fi ;;
        pdf|docx|doc|odt|pptx|ppt|xlsx|xls|odp|ods|odf)
            xdg-open "$file" >/dev/null 2>&1 & ;;
        *)
            xdg-open "$file" >/dev/null 2>&1 & ;;
    esac
}

cf() {
    local result action target
    result="$(fancybash cf "$@")"
    [[ -z "$result" ]] && return

    action="${result%%:*}"
    target="${result#*:}"

    case "$action" in
        CD)
            [[ -d "$target" ]] && cd "$target" || return
            ;;
        CODE)
            local open_path="$target"
            [[ -f "$target" ]] && open_path="$(dirname "$target")"
            if   command -v code   &>/dev/null; then code   "$open_path" >/dev/null 2>&1 &
            elif command -v codium &>/dev/null; then codium "$open_path" >/dev/null 2>&1 &
            else [[ -d "$open_path" ]] && cd "$open_path" || return
            fi
            ;;
        OPEN)
            _cf_open "$target"
            ;;
        EXPLORE)
            local explore_path="$target"
            [[ -f "$target" ]] && explore_path="$(dirname "$target")"
            if   command -v xdg-open &>/dev/null; then xdg-open "$explore_path" >/dev/null 2>&1 &
            elif command -v open     &>/dev/null; then open     "$explore_path" >/dev/null 2>&1 &
            elif command -v explorer.exe &>/dev/null; then explorer.exe "$(wslpath -w "$explore_path" 2>/dev/null || echo "$explore_path")" >/dev/null 2>&1 &
            fi
            ;;
        *)
            # Legacy fallback: raw path (no tag)
            if   [[ -d "$result" ]]; then cd "$result" || return
            elif [[ -f "$result" ]]; then cd "$(dirname "$result")" || return
            fi
            ;;
    esac
}
"#.to_string(),
        Shell::Fish => r#"
# ── Interactive Fuzzy Directory Navigator (`cf`) shell wrapper ──
functions -e cf 2>/dev/null
function _cf_open
    set -l file $argv[1]
    test -z "$file"; and return
    set -l ext (string lower (string split -r -m1 . -- $file)[-1])
    switch $ext
        case mp4 mkv avi mov wmv flv webm m4v
            if type -q mpv; command mpv "$file" >/dev/null 2>&1 &
            else if type -q vlc; command vlc "$file" >/dev/null 2>&1 &
            else; xdg-open "$file" >/dev/null 2>&1 &
            end
        case png jpg jpeg gif webp bmp svg ico
            if type -q eog; command eog "$file" >/dev/null 2>&1 &
            else if type -q feh; command feh "$file" >/dev/null 2>&1 &
            else if type -q imv; command imv "$file" >/dev/null 2>&1 &
            else; xdg-open "$file" >/dev/null 2>&1 &
            end
        case mp3 flac ogg wav aac m4a opus wma
            if type -q mpv; command mpv "$file" >/dev/null 2>&1 &
            else if type -q vlc; command vlc "$file" >/dev/null 2>&1 &
            else; xdg-open "$file" >/dev/null 2>&1 &
            end
        case pdf docx doc odt pptx ppt xlsx xls odp ods odf
            xdg-open "$file" >/dev/null 2>&1 &
        case '*'
            xdg-open "$file" >/dev/null 2>&1 &
    end
end

function cf
    set -l result (fancybash cf $argv)
    test -z "$result"; and return

    set -l parts (string split -m1 ":" -- $result)
    set -l action $parts[1]
    set -l target $parts[2]

    switch $action
        case CD
            if test -d "$target"
                cd "$target"
            end
        case CODE
            set -l open_path "$target"
            if test -f "$target"
                set open_path (dirname "$target")
            end
            if type -q code
                code "$open_path" >/dev/null 2>&1 &
            else if type -q codium
                codium "$open_path" >/dev/null 2>&1 &
            else if test -d "$open_path"
                cd "$open_path"
            end
        case OPEN
            _cf_open "$target"
        case EXPLORE
            set -l explore_path "$target"
            if test -f "$target"
                set explore_path (dirname "$target")
            end
            if type -q xdg-open
                xdg-open "$explore_path" >/dev/null 2>&1 &
            else if type -q open
                open "$explore_path" >/dev/null 2>&1 &
            else if type -q explorer.exe
                explorer.exe "$explore_path" >/dev/null 2>&1 &
            end
        case '*'
            if test -d "$result"
                cd "$result"
            else if test -f "$result"
                cd (dirname "$result")
            end
    end
end
"#.to_string(),
        Shell::Pwsh => r#"
# ── Interactive Fuzzy Directory Navigator (`cf`) shell wrapper ──
Remove-Item alias:cf -ErrorAction SilentlyContinue 2>$null
function _cf_open($file) {
    if (-not $file) { return }
    $ext = [System.IO.Path]::GetExtension($file).ToLower().TrimStart('.')
    switch ($ext) {
        { $_ -in 'mp4','mkv','avi','mov','wmv','flv','webm','m4v' } {
            if (Get-Command mpv -ErrorAction SilentlyContinue) { Start-Process mpv -ArgumentList "`"$file`"" }
            elseif (Get-Command vlc -ErrorAction SilentlyContinue) { Start-Process vlc -ArgumentList "`"$file`"" }
            else { Start-Process "$file" }
            break
        }
        { $_ -in 'png','jpg','jpeg','gif','webp','bmp','svg','ico' } {
            if (Get-Command eog -ErrorAction SilentlyContinue) { Start-Process eog -ArgumentList "`"$file`"" }
            elseif (Get-Command feh -ErrorAction SilentlyContinue) { Start-Process feh -ArgumentList "`"$file`"" }
            elseif (Get-Command imv -ErrorAction SilentlyContinue) { Start-Process imv -ArgumentList "`"$file`"" }
            else { Start-Process "$file" }
            break
        }
        { $_ -in 'mp3','flac','ogg','wav','aac','m4a','opus','wma' } {
            if (Get-Command mpv -ErrorAction SilentlyContinue) { Start-Process mpv -ArgumentList "`"$file`"" }
            elseif (Get-Command vlc -ErrorAction SilentlyContinue) { Start-Process vlc -ArgumentList "`"$file`"" }
            else { Start-Process "$file" }
            break
        }
        default { Start-Process "$file" }
    }
}

function cf {
    $result = fancybash cf @args
    if (-not $result) { return }
    $parts = $result -split ':', 2
    if ($parts.Length -lt 2) {
        if (Test-Path -Path $result -PathType Container) { Set-Location -Path $result }
        elseif (Test-Path -Path $result -PathType Leaf) { Set-Location -Path (Split-Path -Parent $result) }
        return
    }
    $action = $parts[0]
    $target = $parts[1]
    switch ($action) {
        "CD" {
            if (Test-Path -Path $target -PathType Container) { Set-Location -Path $target }
        }
        "CODE" {
            $openPath = $target
            if (Test-Path -Path $target -PathType Leaf) { $openPath = Split-Path -Parent $target }
            if (Get-Command code -ErrorAction SilentlyContinue) { Start-Process code -ArgumentList "`"$openPath`"" }
            elseif (Get-Command codium -ErrorAction SilentlyContinue) { Start-Process codium -ArgumentList "`"$openPath`"" }
            elseif (Test-Path -Path $openPath -PathType Container) { Set-Location -Path $openPath }
        }
        "OPEN" {
            _cf_open $target
        }
        "EXPLORE" {
            $explorePath = $target
            if (Test-Path -Path $target -PathType Leaf) { $explorePath = Split-Path -Parent $target }
            if (Get-Command Invoke-Item -ErrorAction SilentlyContinue) { Invoke-Item -Path $explorePath }
            else { Start-Process explorer.exe -ArgumentList "`"$explorePath`"" }
        }
        default {
            if (Test-Path -Path $result -PathType Container) { Set-Location -Path $result }
            elseif (Test-Path -Path $result -PathType Leaf) { Set-Location -Path (Split-Path -Parent $result) }
        }
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
        let _ = render_guard(Shell::Zsh);
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
        assert!(render_cf_wrapper(Shell::Zsh).contains("_cf_open"));
        assert!(render_cf_wrapper(Shell::Fish).contains("_cf_open"));
        assert!(render_cf_wrapper(Shell::Pwsh).contains("_cf_open"));
        assert!(render_cf_wrapper(Shell::Bash).contains("OPEN)"));
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


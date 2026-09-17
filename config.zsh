
# ==============================================================================
#   ULTRA-THIN COMPACT PRO Zsh ENVIRONMENT
#   Author: [Rihad Jahan Opu]
#   Version: 2.0.0 Complete Multi-Distro Edition
#   Purpose: A fast, beautiful, and productive terminal for Web Development
#   Supports: Ubuntu/Debian, Fedora/RHEL/CentOS, Arch, macOS, Alpine, openSUSE
#   Verified: 2026 - Cross-platform compatibility
# ==============================================================================

# Clear terminal screen silently on interactive session startup
if [[ -o interactive ]]; then
    clear 2>/dev/null
fi

plugins=(
  git
  zsh-autosuggestions
  zsh-syntax-highlighting
)

setopt PROMPT_SUBST

# ======================================================
# ⚡ ZSH AUTOCOMPLETION ENGINE & PLUGINS (LOAD FIRST)
# ======================================================
if [[ -o interactive ]]; then
    # 1. Ensure fpath includes custom and system completion directories BEFORE compinit
    typeset -U fpath
    local -a _fb_fpaths=(
        "$HOME/.zsh/zsh-completions/src"
        "$HOME/.zsh/completion"
        "${BUN_INSTALL:-$HOME/.bun}"
        "$HOME/.bun"
        "/usr/local/share/zsh/site-functions"
        "/usr/share/zsh/site-functions"
        "/usr/share/zsh/vendor-completions"
    )
    local _fb_fp
    for _fb_fp in "${_fb_fpaths[@]}"; do
        [[ -d "$_fb_fp" ]] && fpath=("$_fb_fp" $fpath)
    done
    unset _fb_fpaths _fb_fp

    # 2. Configure completion options and styles BEFORE compinit
    setopt extendedglob 2>/dev/null || true
    setopt AUTO_LIST AUTO_MENU COMPLETE_IN_WORD ALWAYS_TO_END 2>/dev/null || true

    zstyle ':completion:*' menu select
    zstyle ':completion:*' list-colors "${(s.:.)LS_COLORS}"
    zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}' 'r:|[._-]=* r:|=*' 'l:|=* r:|=*'
    zstyle ':completion:*' rehash true

    # 3. Secure & Fast compinit execution with -i flag (silences insecure directory errors)
    autoload -Uz compinit bashcompinit
    local zcompdump="${ZSH_COMPDUMP:-$HOME/.zcompdump}"
    if [[ ! -f "$zcompdump" || ! -s "$zcompdump" || -n ${zcompdump}(#qN.m+1) ]]; then
        compinit -i -d "$zcompdump"
    else
        compinit -i -C -d "$zcompdump"
    fi
    bashcompinit 2>/dev/null || true

    # 4. Asynchronous zcompile of zcompdump with size validation
    [[ -f "$zcompdump" && -s "$zcompdump" && (! -f "${zcompdump}.zwc" || "$zcompdump" -nt "${zcompdump}.zwc") ]] && \
        ( zcompile "$zcompdump" 2>/dev/null &! )

    # 5. Multi-distro zsh-autocomplete plugin lookup
    local -a _fb_ac_paths=(
        "$HOME/.zsh/zsh-autocomplete/zsh-autocomplete.plugin.zsh"
        "/usr/share/zsh-autocomplete/zsh-autocomplete.plugin.zsh"
        "/usr/share/zsh/plugins/zsh-autocomplete/zsh-autocomplete.plugin.zsh"
        "/opt/homebrew/share/zsh-autocomplete/zsh-autocomplete.plugin.zsh"
        "/usr/local/share/zsh-autocomplete/zsh-autocomplete.plugin.zsh"
    )
    local _fb_ac
    for _fb_ac in "${_fb_ac_paths[@]}"; do
        if [[ -f "$_fb_ac" ]]; then
            source "$_fb_ac" 2>/dev/null
            break
        fi
    done
    unset _fb_ac_paths _fb_ac

    # 6. Multi-distro zsh-autosuggestions lookup
    local -a _fb_as_paths=(
        "$HOME/.zsh/zsh-autosuggestions/zsh-autosuggestions.zsh"
        "/usr/share/zsh-autosuggestions/zsh-autosuggestions.zsh"
        "/usr/share/zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh"
        "/opt/homebrew/share/zsh-autosuggestions/zsh-autosuggestions.zsh"
        "/usr/local/share/zsh-autosuggestions/zsh-autosuggestions.zsh"
    )
    local _fb_as
    for _fb_as in "${_fb_as_paths[@]}"; do
        if [[ -f "$_fb_as" ]]; then
            source "$_fb_as" 2>/dev/null
            break
        fi
    done
    unset _fb_as_paths _fb_as
    ZSH_AUTOSUGGEST_USE_ASYNC=true

    # 7. Multi-distro zsh-syntax-highlighting lookup
    local -a _fb_sh_paths=(
        "$HOME/.zsh/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"
        "/usr/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"
        "/usr/share/zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"
        "/opt/homebrew/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"
        "/usr/local/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"
    )
    local _fb_sh
    for _fb_sh in "${_fb_sh_paths[@]}"; do
        if [[ -f "$_fb_sh" ]]; then
            source "$_fb_sh" 2>/dev/null
            break
        fi
    done
    unset _fb_sh_paths _fb_sh
fi

# ======================================================
# 🟢 NVM & NODE.JS DYNAMIC LAZY-LOAD (LOADED AFTER AUTOCOMPLETE)
# ======================================================
export NVM_DIR="${NVM_DIR:-$HOME/.config/nvm}"
[[ ! -d "$NVM_DIR" && -d "$HOME/.nvm" ]] && export NVM_DIR="$HOME/.nvm"

if [[ -d "$NVM_DIR/versions/node" ]]; then
    _NODE_DEFAULT_BIN="$(ls -d "$NVM_DIR/versions/node"/* 2>/dev/null | tail -n 1)/bin"
    [[ -d "$_NODE_DEFAULT_BIN" && ":$PATH:" != *":$_NODE_DEFAULT_BIN:"* ]] && export PATH="$_NODE_DEFAULT_BIN:$PATH"
fi

# Internal marker refs (split to avoid grep false-positives in installed block)
_FB_ZSH_ID='fancy-zshrc'
_FB_ZSH_START="# >>> ${_FB_ZSH_ID} >>>"
_FB_ZSH_END="# <<< ${_FB_ZSH_ID} <<<"

_fb_ensure_bottom() {
    local rc_file="${1:-$HOME/.zshrc}"
    local start_marker="${2:-$_FB_ZSH_START}"
    local end_marker="${3:-$_FB_ZSH_END}"

    [[ -f "$rc_file" ]] || return 0
    grep -qF "$start_marker" "$rc_file" 2>/dev/null || return 0
    grep -qF "$end_marker" "$rc_file" 2>/dev/null || return 0

    local trailing_content
    trailing_content=$(sed -n "/${end_marker//\//\\/}/,\$p" "$rc_file" 2>/dev/null | tail -n +2 | grep -v '^[[:space:]]*$' || true)

    if [[ -n "$trailing_content" ]]; then
        local tmp_file
        tmp_file=$(mktemp 2>/dev/null || echo "/tmp/fb_reorder_$$")

        grep -n "$start_marker" "$rc_file" | head -1 | cut -d: -f1 | read -r _fb_start_ln
        [[ -z "$_fb_start_ln" ]] && { rm -f "$tmp_file"; return 0; }

        grep -n "$end_marker" "$rc_file" | tail -1 | cut -d: -f1 | read -r _fb_end_ln
        [[ -z "$_fb_end_ln" ]] && { rm -f "$tmp_file"; return 0; }

        awk -v s="$_fb_start_ln" -v e="$_fb_end_ln" 'NR < s || NR > e' "$rc_file" > "$tmp_file"
        awk -v s="$_fb_start_ln" -v e="$_fb_end_ln" 'NR >= s && NR <= e' "$rc_file" >> "$tmp_file"

        mv "$tmp_file" "$rc_file" 2>/dev/null || { cat "$tmp_file" > "$rc_file"; rm -f "$tmp_file"; }
    fi
}

_fb_clean_external_nvm_lines() {
    local rc_file="${1:-$HOME/.zshrc}"
    [[ ! -f "$rc_file" ]] && return 0
    local _sm="$_FB_ZSH_START" _em="$_FB_ZSH_END"
    if grep -qF "$_sm" "$rc_file" 2>/dev/null; then
        if [ "$(uname)" = "Darwin" ]; then
            sed -i '' "/${_sm//\//\\/}/,/${_em//\//\\/}/"'!{ /nvm\.sh/d; /bash_completion/d; /_bun/d; /bun completions/d; }' "$rc_file" 2>/dev/null || true
        else
            sed -i "/${_sm//\//\\/}/,/${_em//\//\\/}/"'!{ /nvm\.sh/d; /bash_completion/d; /_bun/d; /bun completions/d; }' "$rc_file" 2>/dev/null || true
        fi
    fi
}

# Auto-clean duplicate external NVM/Bun installer lines & reorder position in non-blocking background on shell boot
if [[ -o interactive ]]; then
    _fb_ensure_bottom "$HOME/.zshrc" "$_FB_ZSH_START" "$_FB_ZSH_END" 2>/dev/null &!
    _fb_clean_external_nvm_lines "$HOME/.zshrc" 2>/dev/null &!
fi

_fb_lazy_load_nvm() {
    unset -f nvm node npm npx 2>/dev/null
    _fb_clean_external_nvm_lines "$HOME/.zshrc" 2>/dev/null &!
    if [ -s "$NVM_DIR/nvm.sh" ]; then
        \. "$NVM_DIR/nvm.sh"
    fi
    if [ -s "$NVM_DIR/bash_completion" ]; then
        autoload -Uz bashcompinit 2>/dev/null
        bashcompinit 2>/dev/null || true
        \. "$NVM_DIR/bash_completion"
    fi
}

nvm() {
    _fb_lazy_load_nvm
    nvm "$@"
}

node() {
    _fb_lazy_load_nvm
    node "$@"
}

npm() {
    _fb_lazy_load_nvm
    npm "$@"
}

npx() {
    _fb_lazy_load_nvm
    npx "$@"
}

# ======================================================
# 🥐 BUN ENVIRONMENT & AUTOCOMPLETION (LAZY-LOADED)
# ======================================================
export BUN_INSTALL="${BUN_INSTALL:-$HOME/.bun}"

if [[ -d "$BUN_INSTALL/bin" && ":$PATH:" != *":$BUN_INSTALL/bin:"* ]]; then
    export PATH="$BUN_INSTALL/bin:$PATH"
fi

# Bun completions (Lazy loaded & safe sourced without conflicts)
if [[ -s "$BUN_INSTALL/_bun" ]]; then
    [ -s "$BUN_INSTALL/_bun" ] && source "$BUN_INSTALL/_bun" 2>/dev/null
fi


# --- Atomic Non-Blocking Zsh Bytecode Recompilation ---
_fb_zsh_compile_config() {
    local fb_dir="${FB_DIR:-$HOME/.fancybash}"
    local config_src="$fb_dir/config.zsh"
    if [[ ! -f "$config_src" ]]; then
        config_src="${(%):-%x}"
    fi
    local config_zwc="${config_src}.zwc"
    local lock_file="/tmp/fancybash_zcompile_${UID:-0}.lock"

    if [[ -f "$config_src" && -w "$(dirname "$config_src")" && ( ! -f "$config_zwc" || "$config_src" -nt "$config_zwc" ) ]]; then
        if mkdir "$lock_file" 2>/dev/null; then
            (
                zcompile -R "$config_zwc" "$config_src" 2>/dev/null
                rmdir "$lock_file" 2>/dev/null
            ) &!
        fi
    fi
}
_fb_zsh_compile_config



# ======================================================
# 🎨 RAINBOW COLOR & EMOJI SETUP (zsh compatible)
# ======================================================

# ======================================================
# 🌀 CONFIGS & ARRAYS (Zsh Pure Native Fix)
# ======================================================
typeset -g -a rainbow_colors
rainbow_colors=(31 32 33 34 35 36 91 92 93 94 95 96)

unalias rand_color 2>/dev/null
function rand_color {
  local idx=$(( (RANDOM % ${#rainbow_colors}) + 1 ))
  echo "${rainbow_colors[$idx]}"
}

unalias rand_emoji 2>/dev/null
function rand_emoji {
  local folder="${PWD:t}"
  case $folder in
    *web* )   echo "🌐" ;;
    *node* )  echo "🟢" ;;
    *bun* )   echo "🥐" ;;
    *py* )    echo "🐍" ;;
    * )
        local -a emojis
        emojis=(🔥 ⚡️ 🚀 💫 🌈 🌀 ✨ 🧠 🎯 🌟 👾 🦊 🎨 💎 🔮 👑 🦄 🐉)
        local idx=$(( (RANDOM % ${#emojis}) + 1 ))
        echo "${emojis[$idx]}" ;;
  esac
}

# ======================================================

_fb_git_cache_file="/tmp/.fb_git_cache_${USER}_$$"
_fb_update_git_async() {
  {
    (
      local branch=$(git branch --show-current 2>/dev/null)
      if [[ -z "$branch" ]]; then
        branch=$(git rev-parse --short HEAD 2>/dev/null)
        [[ -n "$branch" ]] && branch="➦ $branch"
      fi
      if [[ -n "$branch" ]]; then
        local dirty=""
        [[ -n $(git status --porcelain --untracked-files=no 2>/dev/null) ]] && dirty=" ❗"
        echo " [🌿 $branch$dirty]" > "$_fb_git_cache_file"
      else
        rm -f "$_fb_git_cache_file" 2>/dev/null
      fi
    ) &>/dev/null &!
  } 2>/dev/null
}

unalias parse_git_branch 2>/dev/null
function parse_git_branch {
  # Fast guard: Skip git subshell if not inside a git repository
  [[ ! -d .git ]] && ! git rev-parse --is-inside-work-tree &>/dev/null && { rm -f "$_fb_git_cache_file" 2>/dev/null; return; }
  if [[ ! -f "$_fb_git_cache_file" ]]; then
    local branch=$(git branch --show-current 2>/dev/null)
    [[ -z "$branch" ]] && branch=$(git rev-parse --short HEAD 2>/dev/null)
    if [[ -n "$branch" ]]; then
      local dirty=""
      [[ -n $(git status --porcelain --untracked-files=no 2>/dev/null) ]] && dirty=" ❗"
      echo " [🌿 $branch$dirty]" > "$_fb_git_cache_file"
    fi
  else
    _fb_update_git_async
  fi
  [[ -f "$_fb_git_cache_file" ]] && cat "$_fb_git_cache_file" 2>/dev/null
}

_fb_versions_cache_file="/tmp/.fb_versions_cache_${USER}"
_fb_last_path="$PATH"

_fb_update_tool_versions_async() {
  {
    (
      local n_ver="" p_ver="" b_ver=""
      command -v node >/dev/null 2>&1 && n_ver="🟢 $(node -v) │ "
      command -v npm  >/dev/null 2>&1 && p_ver="📦 v$(npm -v) │ "
      command -v bun  >/dev/null 2>&1 && b_ver="🥐 v$(bun -v) │ "

      cat <<EOF > "$_fb_versions_cache_file"
_fb_cached_node="$n_ver"
_fb_cached_npm="$p_ver"
_fb_cached_bun="$b_ver"
EOF
    ) &>/dev/null &!
  } 2>/dev/null
}

_fb_check_and_refresh_versions_cache() {
  if [[ ! -f "$_fb_versions_cache_file" || "$PATH" != "$_fb_last_path" ]]; then
    _fb_last_path="$PATH"
    _fb_update_tool_versions_async
    return
  fi
  local mtime=$(stat -c %Y "$_fb_versions_cache_file" 2>/dev/null || stat -f %m "$_fb_versions_cache_file" 2>/dev/null)
  local current_epoch=$(date +%s)
  if [[ -n "$mtime" && $((current_epoch - mtime)) -gt 60 ]]; then
    _fb_update_tool_versions_async
  fi
}

# Auto-load or generate version cache on startup
_fb_check_and_refresh_versions_cache
[[ -f "$_fb_versions_cache_file" ]] && source "$_fb_versions_cache_file" 2>/dev/null

unalias node_version 2>/dev/null
function node_version {
  _fb_check_and_refresh_versions_cache
  [[ -f "$_fb_versions_cache_file" ]] && source "$_fb_versions_cache_file" 2>/dev/null
  echo "$_fb_cached_node"
}
unalias npm_version 2>/dev/null
function npm_version {
  _fb_check_and_refresh_versions_cache
  [[ -f "$_fb_versions_cache_file" ]] && source "$_fb_versions_cache_file" 2>/dev/null
  echo "$_fb_cached_npm"
}
unalias bun_version 2>/dev/null
function bun_version {
  _fb_check_and_refresh_versions_cache
  [[ -f "$_fb_versions_cache_file" ]] && source "$_fb_versions_cache_file" 2>/dev/null
  echo "$_fb_cached_bun"
}
unalias time_date 2>/dev/null
function time_date { echo "📅 $(date +'%b %d')"; }

unalias sys_info 2>/dev/null
function sys_info {
  if [[ -f /proc/meminfo ]]; then
    local mem_total=$(awk '/MemTotal/ {print $2}' /proc/meminfo 2>/dev/null)
    local mem_avail=$(awk '/MemAvailable/ {print $2}' /proc/meminfo 2>/dev/null)
    if [[ -n "$mem_total" && -n "$mem_avail" ]]; then
      local mem_used=$(( (mem_total - mem_avail) / 1024 ))
      local mem_total_mb=$(( mem_total / 1024 ))
      echo "📟 🧠 ${mem_used}M/${mem_total_mb}M │ "
      return
    fi
  fi
  if command -v free >/dev/null 2>&1; then
    local RAM=$(free -h 2>/dev/null | awk '/^Mem/ {print $3 "/" $2}')
    [[ -n "$RAM" ]] && echo "📟 🧠 ${RAM} │ "
  fi
}

unalias battery_info 2>/dev/null
function battery_info {
  if [[ -f /sys/class/power_supply/BAT0/capacity ]]; then
    echo "🔋$(cat /sys/class/power_supply/BAT0/capacity)% │ "
  elif [[ -f /sys/class/power_supply/BAT1/capacity ]]; then
    echo "🔋$(cat /sys/class/power_supply/BAT1/capacity)% │ "
  fi
}

unalias kernel_version 2>/dev/null
function kernel_version { echo "🐧 $(uname -r | cut -d'-' -f1) │ "; }

unalias cpu_temp 2>/dev/null
function cpu_temp {
  local temp=""
  if [[ -f /sys/class/thermal/thermal_zone0/temp ]]; then
    local raw=$(cat /sys/class/thermal/thermal_zone0/temp 2>/dev/null)
    [[ -n "$raw" && "$raw" -gt 0 ]] && temp=$((raw / 1000))
  fi
  if [[ -z "$temp" ]] && command -v sensors >/dev/null 2>&1; then
    temp=$(sensors 2>/dev/null | grep -iE 'Package id 0|Core 0|temp1' | head -n1 | grep -oP '\+\K[0-9.]+' | head -n1 | cut -d. -f1)
  fi
  [[ -n "$temp" ]] && echo " 🌡️ ${temp}°C"
}

unalias folder_size 2>/dev/null
function folder_size {
  local size=""
  if command -v timeout >/dev/null 2>&1; then
    size=$(timeout 0.2s du -sh . 2>/dev/null | cut -f1)
  else
    size=$(du -sh . 2>/dev/null | cut -f1)
  fi
  [[ -n "$size" ]] && echo " 📂 ${size}" || echo " 📂 ~"
}

unalias disk_usage 2>/dev/null
function disk_usage {
  local disk=$(df -h / 2>/dev/null | awk 'NR==2 {print $4}')
  [[ -n "$disk" ]] && echo " 💽 ${disk} free"
}

unalias load_avg 2>/dev/null
function load_avg {
  local load=$(uptime 2>/dev/null | awk -F'load average:' '{ print $2 }' | cut -d',' -f1 | sed 's/ //g')
  [[ -n "$load" ]] && echo " ⚖️ ${load}"
}

typeset -g timer
unalias zsh_stats_preexec 2>/dev/null
function zsh_stats_preexec { timer=$SECONDS; }
unalias zsh_stats_precmd 2>/dev/null
function zsh_stats_precmd {
  if [ -n "$timer" ]; then
    local delta=$(( SECONDS - timer ))
    [ $delta -ge 1 ] && export CMD_DURATION=" ⏱️ ${delta}s" || export CMD_DURATION=""
    unset timer
  else
    export CMD_DURATION=""
  fi
}
autoload -Uz add-zsh-hook
add-zsh-hook preexec zsh_stats_preexec
add-zsh-hook precmd zsh_stats_precmd

unalias check_readonly 2>/dev/null
function check_readonly { [ ! -w . ] && echo " 🔒"; }
unalias pending_updates 2>/dev/null
function pending_updates {
  local updates=0
  if command -v checkupdates >/dev/null 2>&1; then
    updates=$(checkupdates 2>/dev/null | wc -l)
  elif command -v dnf >/dev/null 2>&1; then
    updates=$(dnf check-update -q 2>/dev/null | grep -c '^[a-zA-Z0-9]')
  elif [ -f /var/lib/update-notifier/updates-available ]; then
    updates=$(cat /var/lib/update-notifier/updates-available | grep -Po '^[0-9]+(?= updates? can be installed)' | head -n1)
  elif command -v apt-get >/dev/null 2>&1; then
    updates=$(apt-get -s upgrade 2>/dev/null | grep -iP '^[0-9]+ upgraded' | cut -d' ' -f1)
  fi
  [[ -n "$updates" && "$updates" -gt 0 ]] && echo " 🆙 $updates"
}

# ======================================================
# 🎨 PROMPT THEME ENGINE & CUSTOMIZATION (fancy_theme)
# ======================================================
# 🎨 PROMPT THEME ENGINE & CUSTOMIZATION (fancy_theme)
# ======================================================

setopt prompt_subst

# ⚡ Pre-cmd Telemetry Cache (Smart: inspect active theme function for required variables)
_fb_precmd() {
  # Reset all cache variables first
  _fb_git=""; _fb_emoji=""; _fb_color=""
  _fb_size=""; _fb_node=""; _fb_npm=""
  _fb_bun=""; _fb_temp=""; _fb_date=""

  local target_theme="${FANCYBASH_ZSH_THEME:-minimal}"
  if [[ -f ~/.fancybash_theme ]]; then
    local saved="${$(< ~/.fancybash_theme):-minimal}"
    saved="${saved//[$'\t\r\n ']}"
    [[ -n "$saved" ]] && target_theme="$saved"
  fi
  local theme_body="$(typeset -f fb_theme_${target_theme} 2>/dev/null)"
  [[ -z "$theme_body" ]] && theme_body="$(typeset -f fb_theme_minimal 2>/dev/null)"

  # Always refresh dynamic emoji and color for responsive prompt reload
  _fb_emoji=$(rand_emoji)
  _fb_color=$(rand_color)

  # Only run each expensive command if the active theme function body actually references it
  [[ "$theme_body" == *'_fb_git'* ]]   && _fb_git=$(parse_git_branch)
  [[ "$theme_body" == *'_fb_size'* ]]  && _fb_size=$(folder_size)
  [[ "$theme_body" == *'_fb_node'* ]]  && _fb_node=$(node_version)
  [[ "$theme_body" == *'_fb_npm'* ]]   && _fb_npm=$(npm_version)
  [[ "$theme_body" == *'_fb_bun'* ]]   && _fb_bun=$(bun_version)
  [[ "$theme_body" == *'_fb_temp'* ]]  && _fb_temp=$(cpu_temp)
  [[ "$theme_body" == *'_fb_date'* ]]  && _fb_date=$(time_date)
}

# --- Theme 1: Minimal (Default) ---
fb_theme_minimal() {
  PROMPT="${_fb_emoji} %F{147}%1~%f"$'\n'
  PROMPT+="%F{147}❯❯❯%f "
}

# --- Theme 2: Full (Detailed Two-Line) ---
fb_theme_full() {
  local col=${_fb_color:-$(rand_color)}
  PROMPT="${_fb_emoji} %F{$col}%1~%f ${_fb_size}${_fb_git}${_fb_temp}$(disk_usage)$(load_avg)\$CMD_DURATION$(check_readonly)$(pending_updates)"$'\n'
  PROMPT+="${_fb_node}${_fb_npm}${_fb_bun}$(kernel_version)$(sys_info)$(battery_info)${_fb_date}"$'\n'
  PROMPT+=$'%{\e[5m%}❯❯❯%{\e[25m%} '
}

# --- Theme 3: RobbyRussell (Oh My Zsh Default) ---
fb_theme_robbyrussell() {
  PROMPT="%F{green}➜ %f %F{cyan}%1~%f${_fb_git} %F{green}❯%f "
}

# --- Theme 4: Powerlevel10k (P10k Multi-Line) ---
fb_theme_p10k() {
  PROMPT="%F{blue}╭─%f 🐧 %F{green}%n@%m%f in %F{cyan}%1~%f ${_fb_size}${_fb_git}${_fb_temp}"$'\n'
  PROMPT+="%F{blue}╰─%F{green}❯%f "
}

# --- Theme 5: Agnoster (Oh My Zsh Powerline) ---
fb_theme_agnoster() {
  PROMPT="%K{blue}%F{black} 💻 %n@%m %K{green}%F{blue} %F{black}%1~ %K{yellow}%F{green} %F{black}${_fb_git} %k%F{yellow}%f"$'\n'
  PROMPT+="%F{cyan}❯%f "
}

# --- Theme 6: Catppuccin (Oh My Posh Mocha) ---
fb_theme_catppuccin() {
  PROMPT="%F{#ca9ee6}%K{#ca9ee6}%F{#1e1e2e} 🐱 catppuccin %K{#89b4fa}%F{#ca9ee6}%K{#89b4fa}%F{#1e1e2e} 📂 %1~ %k%F{#89b4fa}%f${_fb_git}"$'\n'
  PROMPT+="%F{#f5c2e7}❯❯❯%f "
}

# --- Theme 7: TokyoNight (Oh My Posh Cyber Neon) ---
fb_theme_tokyonight() {
  PROMPT="%F{#bb9af7}🌌 %F{#7dcfff}%1~%f %F{#7aa2f7}${_fb_git}%f ${_fb_node}"$'\n'
  PROMPT+="%F{#bb9af7}⚡ %f"
}

# --- Theme 8: Dracula (Oh My Posh Dark Pink/Purple) ---
fb_theme_dracula() {
  PROMPT="%F{#bd93f9}🧛 %n %F{#ff79c6}in %1~ %F{#8be9fd}${_fb_git}%f"$'\n'
  PROMPT+="%F{#50fa7b}❯ %f"
}

# --- Theme 9: Nord (Oh My Posh Arctic Frost) ---
fb_theme_nord() {
  PROMPT="%F{#88c0d0}❄️  %1~ %F{#81a1c1}${_fb_git}%f"$'\n'
  PROMPT+="%F{#8fbcbb}❯ %f"
}

# --- Theme 10: Bira (Oh My Zsh Classic) ---
fb_theme_bira() {
  PROMPT="%F{yellow}╭─%F{green}%n@%m %F{blue}%1~ %F{magenta}${_fb_git}%f"$'\n'
  PROMPT+="%F{yellow}╰─%F{green}\$ %f"
}

# --- Theme 11: Pure (Sindre Sorhus Quiet Minimalist) ---
fb_theme_pure() {
  PROMPT=""$'\n'"%F{blue}%1~%f %F{240}${_fb_git}%f"$'\n'"%F{green}❯%f "
}

# --- Theme 12: Starship (Cross-Shell Rocket) ---
fb_theme_starship() {
  PROMPT="%F{magenta}🚀 %n %F{green}in %F{cyan}%1~%f ${_fb_git}"$'\n'"%F{yellow}❯%f "
}

# --- Theme 13: Cyberpunk (Futuristic Neon Yellow/Cyan) ---
fb_theme_cyberpunk() {
  PROMPT="%K{#ffee00}%F{#000000} ⚡ CYBER %K{#00f0ff}%F{#000000} %1~ %k%f ${_fb_git}"$'\n'"%F{#ff0055}▶▶ %f"
}

# --- Theme 14: Synthwave (80s Retro Magenta/Purple) ---
fb_theme_synthwave() {
  PROMPT="%F{#ff007f}🌅 %F{#00ffff}%1~%f %F{#9d00ff}${_fb_git}%f"$'\n'"%F{#ff007f}❯❯ %f"
}

# --- Theme 15: Gruvbox (Retro Warm Gold/Green) ---
fb_theme_gruvbox() {
  PROMPT="%F{#fabd2f}🌴 %n %F{#8ec07c}in %1~ %F{#fe8019}${_fb_git}%f"$'\n'"%F{#fabd2f}❯ %f"
}

# --- Theme 16: OneDark (Atom/VSCode Classic Blue) ---
fb_theme_onedark() {
  PROMPT="%F{#61afef}🌐 %n %F{#c678dd}%1~ %F{#98c379}${_fb_git}%f"$'\n'"%F{#61afef}❯ %f"
}

# --- Theme 17: Sorin (Oh My Zsh Modern Arrow) ---
fb_theme_sorin() {
  PROMPT="%F{blue}%n@%m %F{cyan}%1~%f ${_fb_git}"$'\n'"%F{magenta}❯ %f"
}

# --- Theme 18: Spaceship (Popular ZSH Prompt) ---
fb_theme_spaceship() {
  PROMPT=""$'\n'"%F{blue}🚀 %n%f in %F{cyan}%1~%f ${_fb_git}"$'\n'"%F{green}➜ %f"
}

# --- Theme 19: HalfLife (Scientist Lambda) ---
fb_theme_halflife() {
  PROMPT="%F{#ff9800}λ %F{#ffeb3b}%1~ %F{#ff9800}${_fb_git}%f"$'\n'"%F{#ff9800}▶ %f"
}

# --- Theme 20: Paradox (Chevron Powerline) ---
fb_theme_paradox() {
  PROMPT="%K{blue}%F{white} ⚡ %1~ %K{magenta}%F{blue} %F{white}${_fb_git} %k%F{magenta}%f"$'\n'"%F{cyan}❯ %f"
}

# --- Theme 21: Bureau (Oh My Zsh Multi-Line) ---
fb_theme_bureau() {
  PROMPT="%F{240}[%F{yellow}%n@%m%F{240}] %F{cyan}%1~%f ${_fb_git}"$'\n'"%F{green}❯ %f"
}

# --- Theme 22: Gallifrey (Sci-Fi Tardis Gold) ---
fb_theme_gallifrey() {
  PROMPT="%F{#ffd700}⏳ %F{#00bfff}%1~ %F{#ffd700}${_fb_git}%f"$'\n'"%F{#00bfff}❯ %f"
}

# --- Theme 23: Material (Google Emerald Cyan) ---
fb_theme_material() {
  PROMPT="%F{#00e676}💎 %n %F{#00e5ff}%1~ %F{#d500f9}${_fb_git}%f"$'\n'"%F{#00e676}❯ %f"
}

# --- Theme 24: Monokai (Pro Sublime Neon) ---
fb_theme_monokai() {
  PROMPT="%F{#a6e22e}🔥 %n %F{#f92672}in %1~ %F{#e6db74}${_fb_git}%f"$'\n'"%F{#a6e22e}❯ %f"
}

# --- Theme 25: Palenight (Material Purple Lavender) ---
fb_theme_palenight() {
  PROMPT="%F{#c792ea}🍇 %1~ %F{#89ddff}${_fb_git}%f"$'\n'"%F{#ff5370}❯ %f"
}

# --- Theme 26: PowerlineClassic (Classic Statusline) ---
fb_theme_powerlineclassic() {
  PROMPT="%K{blue}%F{white} %n@%m %K{green}%F{blue} %F{black}%1~ %k%F{green}%f ${_fb_git}"$'\n'"%F{cyan}❯ %f"
}

# --- Theme 27: Lambda (Oh My Zsh Lambda) ---
fb_theme_lambda() {
  PROMPT="%F{green}λ %F{blue}%1~%f ${_fb_git} %F{green}❯%f "
}

# --- Theme 28: Hyper (Hyper Terminal Lightning) ---
fb_theme_hyper() {
  PROMPT="%F{#ff00ff}⚡ %F{#00ffff}%1~ %F{#ffff00}${_fb_git}%f"$'\n'"%F{#ff00ff}❯ %f"
}

# --- Theme 29: Slick (Single-Line Dot Indicator) ---
fb_theme_slick() {
  PROMPT="%F{green}● %F{cyan}%1~%f ${_fb_git} %F{green}❯%f "
}

# --- Theme 30: Matrix (Hacker Glowing Green) ---
fb_theme_matrix() {
  PROMPT="%F{#00ff00}📟 %n@%m:%1~ ${_fb_git}%f"$'\n'"%F{#00ff00}[matrix]❯ %f"
}

# --- Theme 31: Sunset (Coral Pink Orange) ---
fb_theme_sunset() {
  PROMPT="%F{#ff6b6b}🌇 %1~ %F{#feca57}${_fb_git}%f"$'\n'"%F{#5f27cd}❯ %f"
}

# --- Theme 32: Solarized (Warm Retro Cyan & Yellow) ---
fb_theme_solarized() {
  PROMPT="%F{#b58900}☀️ %n %F{#2aa198}in %1~ %F{#cb4b16}${_fb_git}%f"$'\n'"%F{#268bd2}❯ %f"
}

# --- Theme 33: RosePine (Muted Rose & Lavender) ---
fb_theme_rosepine() {
  PROMPT="%F{#eb6f92}🌹 %1~ %F{#c4a7e7}${_fb_git}%f"$'\n'"%F{#f6c177}❯ %f"
}

# --- Theme 34: Everforest (Deep Forest Green & Amber) ---
fb_theme_everforest() {
  PROMPT="%F{#a7c080}🌲 %n %F{#e2b76e}%1~ %F{#7fbbb3}${_fb_git}%f"$'\n'"%F{#a7c080}❯ %f"
}

# --- Theme 35: Kanagawa (Japanese Ink & Dragon Gold) ---
fb_theme_kanagawa() {
  PROMPT="%F{#7e9cd8}🌊 %n %F{#e06d76}in %1~ %F{#e09e72}${_fb_git}%f"$'\n'"%F{#98bb75}❯ %f"
}

# --- Theme 36: NightOwl (Midnight Navy & Turquoise) ---
fb_theme_nightowl() {
  PROMPT="%F{#82aaff}🦉 %n %F{#7fdbca}%1~ %F{#c792ea}${_fb_git}%f"$'\n'"%F{#ffcb6b}❯ %f"
}

# --- Theme 37: Cobalt2 (Electric Yellow & Blue) ---
fb_theme_cobalt2() {
  PROMPT="%F{#ffc400}⚡ %n %F{#0088ff}in %1~ %F{#00e5ff}${_fb_git}%f"$'\n'"%F{#ffc400}❯ %f"
}

# --- Theme 38: ShadesOfPurple (Royal Purple & Yellow) ---
fb_theme_shadesofpurple() {
  PROMPT="%F{#bd93f9}🍇 %1~ %F{#ffd600}${_fb_git}%f"$'\n'"%F{#ff8000}❯❯ %f"
}

# --- Theme 39: Ayu (Clean Orange & Bright Teal) ---
fb_theme_ayu() {
  PROMPT="%F{#ff8f40}🎨 %n %F{#95e6cb}%1~ %F{#ffd700}${_fb_git}%f"$'\n'"%F{#ff8f40}❯ %f"
}

# --- Theme 40: Snazzy (Vivid Magenta & Electric Cyan) ---
fb_theme_snazzy() {
  PROMPT="%F{#ff5c57}✨ %1~ %F{#5ffaef}${_fb_git}%f"$'\n'"%F{#ff6ac1}❯ %f"
}

# --- Theme 41: Outrun (80s Neon Sunset Pink & Cyan) ---
fb_theme_outrun() {
  PROMPT="%F{#ff007f}🌆 %n %F{#00f0ff}in %1~ %F{#ffc800}${_fb_git}%f"$'\n'"%F{#ff007f}▶▶ %f"
}

# --- Theme 42: Oceanic (Deep Sea Cyan & Coral) ---
fb_theme_oceanic() {
  PROMPT="%F{#6699cc}🌊 %1~ %F{#ec5f67}${_fb_git}%f"$'\n'"%F{#5fb3b3}❯ %f"
}

# --- Theme 43: Moonlight (Deep Indigo & Sky Blue) ---
fb_theme_moonlight() {
  PROMPT="%F{#82aaff}🌙 %n %F{#b4a0ff}%1~ %F{#87ddff}${_fb_git}%f"$'\n'"%F{#c586c0}❯ %f"
}

# --- Theme 44: PaperColor (High Contrast Monochrome Accent) ---
fb_theme_papercolor() {
  PROMPT="%F{#5faf87}📜 %n@%m %F{#d78700}%1~ %F{#af005f}${_fb_git}%f"$'\n'"%F{#005f87}❯ %f"
}

# --- Theme 45: Horizon (Warm Coral & Peach Sunset) ---
fb_theme_horizon() {
  PROMPT="%F{#e95c72}🌄 %1~ %F{#f0907a}${_fb_git}%f"$'\n'"%F{#fac591}❯ %f"
}

# --- Theme 46: CatppuccinFrappe (Cool Muted Pastel Lavender) ---
fb_theme_catppuccin_frappe() {
  PROMPT="%F{#ca9ee6}☕ %n %F{#bac2de}in %1~ %F{#99d1db}${_fb_git}%f"$'\n'"%F{#f4b8e4}❯❯ %f"
}

# --- Theme 47: DraculaPro (Neon Violet & Emerald Blade) ---
fb_theme_dracula_pro() {
  PROMPT="%F{#a27aff}🗡️  %n %F{#ff80bf}%1~ %F{#80ffea}${_fb_git}%f"$'\n'"%F{#50fa7b}▶ %f"
}

# --- Theme 48: CyberSamurai (Crimson Red & Neon Cyan) ---
fb_theme_cyber_samurai() {
  PROMPT="%F{#ff2a6d}🥷 %n %F{#05d9e8}%1~ %F{#fff200}${_fb_git}%f"$'\n'"%F{#ff2a6d}❯❯❯ %f"
}

# --- Theme 49: Evergreen (Fresh Mint & Leaf Green) ---
fb_theme_evergreen() {
  PROMPT="%F{#2ecc71}🍃 %1~ %F{#1abc9c}${_fb_git}%f"$'\n'"%F{#3498db}❯ %f"
}

# --- Theme 50: Ghost (Translucent Grey & Phantom White) ---
fb_theme_ghost() {
  PROMPT="%F{#bdc3c7}👻 %n %F{#95a5a6}in %1~ %F{#ecf0f1}${_fb_git}%f"$'\n'"%F{#7f8c8d}❯ %f"
}

# --- Theme 51: Oxide (Burnt Orange & Copper Rust) ---
fb_theme_oxide() {
  PROMPT="%F{#d35400}⚙️  %1~ %F{#e67e22}${_fb_git}%f"$'\n'"%F{#f1c40f}❯ %f"
}

# --- Theme 52: NeonPulse (Vivid Lime & Electric Pink) ---
fb_theme_neon_pulse() {
  PROMPT="%F{#39ff14}🔮 %n %F{#ff1493}%1~ %F{#00ffff}${_fb_git}%f"$'\n'"%F{#39ff14}⚡ %f"
}

# --- Theme 53: Volcano (Fiery Red & Obsidian Magma) ---
fb_theme_volcano() {
  PROMPT="%F{#e74c3c}🌋 %1~ %F{#c0392b}${_fb_git}%f"$'\n'"%F{#f39c12}▶ %f"
}

# --- Theme 54: Sakura (Cherry Blossom Pink & Pastel Green) ---
fb_theme_sakura() {
  PROMPT="%F{#ffb7b2}🌸 %n %F{#ffda09}in %1~ %F{#e2f0cb}${_fb_git}%f"$'\n'"%F{#ff9aa2}❯ %f"
}

# --- Theme 55: Galaxy (Deep Space Indigo & Nebula Purple) ---
fb_theme_galaxy() {
  PROMPT="%F{#9b59b6}🌌 %n %F{#8e44ad}%1~ %F{#3498db}${_fb_git}%f"$'\n'"%F{#f1c40f}✨ ❯ %f"
}





# Global variable to store current ZSH theme
typeset -g FANCYBASH_ZSH_THEME="minimal"

# Main ZSH prompt hook
unalias build_prompt 2>/dev/null
function build_prompt {
  _fb_precmd
  if typeset -f "fb_theme_${FANCYBASH_ZSH_THEME}" >/dev/null 2>&1; then
    "fb_theme_${FANCYBASH_ZSH_THEME}"
  else
    fb_theme_minimal
  fi
}
add-zsh-hook precmd build_prompt

# --- Theme Gallery Preview Function ---
_fb_preview_all_themes() {
  local available_themes=($(typeset +f | grep -o 'fb_theme_[a-zA-Z0-9_]*' | sed 's/fb_theme_//' | sort -u))
  local count=${#available_themes}

  (
    _fb_precmd 2>/dev/null
    echo -e "\033[1;35m🎨 FANCYBASH PROMPT THEME GALLERY (${count} Themes)\033[0m"
    echo -e "\033[36m──────────────────────────────────────────────────────────\033[0m"

    local old_prompt="$PROMPT"
    local idx=1
    for t in "${available_themes[@]}"; do
      echo -e "\033[1;33m📌 [$idx/$count] $t\033[0m"
      "fb_theme_${t}" 2>/dev/null
      print -P "$PROMPT"
      echo -e "\033[38;2;60;60;80m──────────────────────────────────────────────────────────\033[0m"
      idx=$((idx + 1))
    done
    PROMPT="$old_prompt"
  ) | less -RFX
}

# --- Fancybash Auto-Upgrade & Shell Reload ---
unalias fancy_upgrade 2>/dev/null
unalias fancy_update 2>/dev/null
function fancy_upgrade {
  echo -e "\033[1;35m⚡ Upgrading fancybash to latest version...\033[0m"
  if curl -fsSL https://fancybash.netlify.app/i.sh | bash 2>/dev/null || curl -fsSL https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main/i.sh | bash; then
    _fb_ensure_bottom "$HOME/.zshrc" "$_FB_ZSH_START" "$_FB_ZSH_END" 2>/dev/null
    echo -e "\033[1;32m✨ fancybash upgraded successfully! Auto-reloading shell...\033[0m"
    local current_shell="${SHELL:-zsh}"
    exec "$current_shell"
  else
    echo -e "\033[1;31m❌ Upgrade failed. Please check your network connection.\033[0m"
    return 1
  fi
}
function fancy_update {
  fancy_upgrade "$@"
}

# --- Interactive Theme Switcher Function ---
unalias fancy_theme 2>/dev/null
function fancy_theme {
  local chosen_theme="$1"

  # Handle 'preview' or 'list' subcommand
  if [[ "$chosen_theme" == "preview" || "$chosen_theme" == "list" ]]; then
    _fb_preview_all_themes
    return 0
  fi

  # Handle 'upgrade' or 'update' subcommand
  if [[ "$chosen_theme" == "upgrade" || "$chosen_theme" == "update" ]]; then
    fancy_upgrade "$@"
    return $?
  fi

  # Handle 'sync' or 'bottom' subcommand
  if [[ "$chosen_theme" == "sync" || "$chosen_theme" == "bottom" ]]; then
    _fb_ensure_bottom "$HOME/.zshrc" "$_FB_ZSH_START" "$_FB_ZSH_END"
    echo -e "\033[1;32m✨ Fancybash position synced to absolute bottom of ~/.zshrc!\033[0m"
    return 0
  fi

  # Auto-discover all functions starting with fb_theme_
  local available_themes
  available_themes=$(typeset +f | grep -o 'fb_theme_[a-zA-Z0-9_]*' | sed 's/fb_theme_//' | sort -u)

  if [[ -z "$available_themes" ]]; then
    echo "❌ No themes found!"
    return 1
  fi

  # Interactive picker if no argument provided
  if [[ -z "$chosen_theme" ]]; then
    if command -v gum >/dev/null 2>&1; then
      chosen_theme=$(echo "$available_themes" | gum choose --header="🎨 Select Fancybash Prompt Theme:")
    elif command -v fzf >/dev/null 2>&1; then
      chosen_theme=$(echo "$available_themes" | fzf \
        --prompt="🎨 Select Theme: " \
        --height=50% \
        --layout=reverse \
        --border \
        --preview='zsh -c "source ~/.zshrc 2>/dev/null; _fb_precmd 2>/dev/null; fb_theme_{}; print -P \"\$PROMPT\""' \
        --preview-window=right:50%:wrap)
    else
      echo "🎨 Available Themes:"
      select theme in ${(f)available_themes}; do
        chosen_theme="$theme"
        break
      done
    fi
  fi

  # Exit if selection cancelled
  [[ -z "$chosen_theme" ]] && return 0

  # Apply & Persist Theme
  if typeset -f "fb_theme_${chosen_theme}" >/dev/null 2>&1; then
    FANCYBASH_ZSH_THEME="$chosen_theme"
    _fb_precmd 2>/dev/null
    "fb_theme_${chosen_theme}"
    echo "$chosen_theme" > ~/.fancybash_theme
    echo "✅ Theme switched to: $chosen_theme"
  else
    echo "❌ Invalid theme '$chosen_theme'."
    echo "Available themes: $(echo $available_themes | tr '\n' ' ')"
    return 1
  fi
}

# Shortcut function: typing 'fancy' opens interactive TUI theme picker
unalias fancy 2>/dev/null
function fancy {
  if command -v fancybash >/dev/null 2>&1; then
    fancybash theme "$@"
  else
    fancy_theme "$@"
  fi
}

# --- Auto-load Saved Theme on Startup ---
if [[ -f ~/.fancybash_theme ]]; then
  __fb_saved_theme=$(cat ~/.fancybash_theme 2>/dev/null | tr -d ' \n\r')
  if typeset -f "fb_theme_${__fb_saved_theme}" >/dev/null 2>&1; then
    FANCYBASH_ZSH_THEME="$__fb_saved_theme"
  fi
fi



# ======================================================
#  ⚡ INTERACTIVE SETUP SCRIPTS
# ======================================================

# --- Interactive Project Setup Hub TUI ---
unalias project 2>/dev/null
function project {
  if command -v fancybash &>/dev/null; then
    fancybash project "$@"
    return
  fi
  echo "🚀 Select Project Tool:"
  echo "  1) ⚡ Vite (React/Vue)"
  echo "  2) 🚀 Next.js"
  echo "  3) 🎨 Shadcn UI"
  echo "  4) 📦 Tailwind CSS v4"
  echo "  5) 🥐 Initialize Project (ii)"
  echo "  6) ⚙️ C/C++ Project (makecpp)"
  echo "  7) 🏃 Run JS/TS File (run)"
  echo "  8) 🔄 Package Converter (pg)"
  read "choice?Select option [1-8]: "
  case "$choice" in
    1) vite "$@" ;;
    2) next "$@" ;;
    3) ui "$@" ;;
    4) css "$@" ;;
    5) ii "$@" ;;
    6) makecpp "$@" ;;
    7) run "$@" ;;
    8) pg "$@" ;;
    *) echo "Invalid choice" ;;
  esac
}

# --- Initialize a Project (Bun or NPM) ---
unalias ii 2>/dev/null
function ii {
  if command -v fancybash &>/dev/null; then
    fancybash ii "$@"
    return
  fi
  local has_bun=0 has_npm=0 has_pnpm=0 has_yarn=0
  command -v bun  >/dev/null 2>&1 && has_bun=1
  command -v npm  >/dev/null 2>&1 && has_npm=1
  command -v pnpm >/dev/null 2>&1 && has_pnpm=1
  command -v yarn >/dev/null 2>&1 && has_yarn=1

  echo "🚀 Select Package Manager:"
  [[ $has_bun  -eq 1 ]] && echo "1) 🥐 Bun (Fast)"       || echo "1) 🥐 Bun (Not installed)"
  [[ $has_npm  -eq 1 ]] && echo "2) 📦 NPM (Standard)"   || echo "2) 📦 NPM (Not installed)"
  [[ $has_pnpm -eq 1 ]] && echo "3) 🟡 PNPM (Strict)"    || echo "3) 🟡 PNPM (Not installed)"
  [[ $has_yarn -eq 1 ]] && echo "4) 🧶 Yarn (Classic)"   || echo "4) 🧶 Yarn (Not installed)"

  # ✅ Zsh compatible
  read "choice?Enter choice [1-4]: "

  case "$choice" in
    1)
      [[ $has_bun -eq 0 ]] && { echo "❌ Bun not installed."; return 1; }
      bun init -y
      ;;
    2)
      [[ $has_npm -eq 0 ]] && { echo "❌ NPM not installed."; return 1; }
      npm init -y
      ;;
    3)
      [[ $has_pnpm -eq 0 ]] && { echo "❌ PNPM not installed."; return 1; }
      pnpm init
      ;;
    4)
      [[ $has_yarn -eq 0 ]] && { echo "❌ Yarn not installed."; return 1; }
      yarn init -y
      ;;
    *) echo "❌ Cancelled."; return 1 ;;
  esac

  if [ ! -f .gitignore ]; then
    cat > .gitignore << 'GITIGNORE'
# See https://help.github.com/articles/ignoring-files/ for more about ignoring files.

# Dependency directories
node_modules/
jspm_packages/
web_modules/
/.pnp
.pnp.*
.yarn/*
!.yarn/patches
!.yarn/plugins
!.yarn/releases
!.yarn/versions

# Debug logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*
lerna-debug.log*

# Diagnostic reports (https://nodejs.org/api/report.html)
report.[0-9]*.[0-9]*.[0-9]*.[0-9]*.json

# Runtime data
pids
*.pid
*.seed
*.pid.lock

# Directory for instrumented libs generated by jscoverage/JSCover or coveralls
lib-cov
coverage
.nyc_output

# Grunt intermediate storage (https://gruntjs.com/creating-plugins#storing-task-files)
.grunt

# Bower dependency directory (https://bower.io/)
bower_components

# node-gyp's directory when local npm dependencies are compiled
build/Release

# Dependency directories for instrumented code
.lock-wscript

# Optional npm cache directory
.npm

# Optional eslint cache
.eslintcache

# Optional stylelint cache
.stylelintcache

# Microbundle cache
.rpt2_cache/
.rts2_cache_cjs/
.rts2_cache_es/
.rts2_cache_umd/

# Optional REPL history
.node_repl_history

# Output of 'npm pack'
*.tgz

# Yarn Integrity file
.yarn-integrity

# dotenv environment variable files
.env
.env.development.local
.env.test.local
.env.production.local
.env.local

# parcel-bundler cache (https://parceljs.org/)
.cache
.parcel-cache

# Next.js build images and page cache
.next
out

# Nuxt.js build project
.nuxt
dist

# Gatsby files
.cache/
public

# vuepress build output
.vuepress/dist

# Serverless Webpack directories
.webpack/

# Service stability presets
.svelte-kit

# IDEs and editors
.idea/
.vscode/
*.suo
*.ntvs*
*.njsproj
*.sln
*.sw?

# OS-specific files
.DS_Store
.DS_Store?
._*
Thumbs.db
ehthumbs.db
desktop.ini



# testing
/coverage

# next.js
/.next/
/out/

# production
/build

# misc
.DS_Store
*.pem

# debug
npm-debug.log*
yarn-debug.log*
yarn-error.log*
.pnpm-debug.log*

# env files (can opt-in for committing if needed)
.env*

# vercel
.vercel

# typescript
*.tsbuildinfo
next-env.d.ts

GITIGNORE
    echo "✅ .gitignore created."
  else
    echo "ℹ️  .gitignore already exists."
  fi

  echo "✅ Project initialized!"
}


# --- Setup Next.js Project ---
unalias next 2>/dev/null
function next() {
  if command -v fancybash &>/dev/null; then
    fancybash next "$@"
    return
  fi
  echo "⚡ Setup Next.js with:"
  echo "1) Bun"
  echo "2) NPM"
  vared -p "Choice: " -c c
  case "$c" in
    1) bunx create-next-app@latest . ;;
    2) npx create-next-app@latest . ;;
    *) echo "Invalid choice" ;;
  esac
}

# --- Setup Vite shadcn ui ---

# Auto-patch tsconfig/jsconfig with baseUrl and @/* paths
# Priority: tsconfig.app.json (Vite TS) → tsconfig.json (Next.js TS) → jsconfig.json (JS)
unalias _ui_patch_tsconfig 2>/dev/null
function _ui_patch_tsconfig {
  local tsconfig

  if [[ -f "tsconfig.app.json" ]]; then
    tsconfig="tsconfig.app.json"
    echo "  ℹ️  Vite (TS) detected → patching tsconfig.app.json"

  elif [[ -f "tsconfig.json" ]]; then
    tsconfig="tsconfig.json"
    echo "  ℹ️  TypeScript project → patching tsconfig.json"

  elif [[ -f "jsconfig.json" ]]; then
    tsconfig="jsconfig.json"
    echo "  ℹ️  JavaScript project → patching jsconfig.json"

  else
    # No config file found — check if JS project and create jsconfig.json
    if [[ -f "package.json" ]] && ! grep -q '"typescript"' package.json 2>/dev/null; then
      echo "  📝 JavaScript project detected — creating jsconfig.json with @/* alias..."
      node -e "
        const fs = require('fs');
        const jsconfig = {
          compilerOptions: {
            baseUrl: '.',
            paths: { '@/*': ['./src/*'] }
          }
        };
        fs.writeFileSync('jsconfig.json', JSON.stringify(jsconfig, null, 2));
        console.log('  → jsconfig.json created with @/* alias');
      "
      return
    else
      echo "⚠️  No tsconfig/jsconfig found, skipping..."
      return
    fi
  fi

  # Check if paths already set
  if grep -q '"@/\*"' "$tsconfig"; then
    echo "  ✅ $tsconfig paths already configured."
    return
  fi

  echo "🔧 Patching $tsconfig with baseUrl & @/* paths..."

  # Use node to safely patch JSON
  node -e "
    const fs = require('fs');
    const raw = fs.readFileSync('$tsconfig', 'utf8');
    const json = JSON.parse(raw);
    if (!json.compilerOptions) json.compilerOptions = {};
    json.compilerOptions.baseUrl = '.';
    json.compilerOptions.paths = { '@/*': ['./src/*'] };
    fs.writeFileSync('$tsconfig', JSON.stringify(json, null, 2));
    console.log('  → baseUrl & paths written to ' + '$tsconfig');
  "
}

# Auto-patch vite.config.ts with path alias and tailwind import
unalias _ui_patch_viteconfig 2>/dev/null
function _ui_patch_viteconfig {
  local viteconfig
  viteconfig=$(ls vite.config.ts vite.config.js 2>/dev/null | head -n1)

  if [[ -z "$viteconfig" ]]; then
    echo "⚠️  vite.config.ts/js not found, skipping..."
    return
  fi

  echo "🔧 Patching $viteconfig with path alias & tailwind..."

  local content
  content=$(cat "$viteconfig")

  # Add: import path from "path"
  if ! echo "$content" | grep -q 'import path from'; then
    _fb_sed_i '1s|^|import path from "path"\n|' "$viteconfig"
    echo "  → Added: import path from \"path\""
  else
    echo "  ✅ path import already exists."
  fi

  # Add: import tailwindcss from "@tailwindcss/vite"
  if ! grep -q '@tailwindcss/vite' "$viteconfig"; then
    _fb_sed_i '1s|^|import tailwindcss from "@tailwindcss/vite"\n|' "$viteconfig"
    echo "  → Added: import tailwindcss from \"@tailwindcss/vite\""
  else
    echo "  ✅ tailwindcss import already exists."
  fi

  # Add tailwindcss() to plugins array if missing
  if ! grep -q 'tailwindcss()' "$viteconfig"; then
    _fb_sed_i 's/plugins: \[/plugins: [tailwindcss(), /' "$viteconfig"
    echo "  → Added: tailwindcss() to plugins"
  else
    echo "  ✅ tailwindcss() plugin already exists."
  fi

  # Add resolve.alias if missing (no leading comma — JS syntax fix)
  if ! grep -q '"@"' "$viteconfig" && ! grep -q "@:" "$viteconfig"; then
    # Insert resolve block before closing }) of defineConfig
    _fb_sed_i '/^})/i\  resolve: {\n    alias: {\n      "@": path.resolve(__dirname, "./src"),\n    },\n  }' "$viteconfig"
    echo "  → Added: resolve.alias @/* → ./src"
  else
    echo "  ✅ resolve.alias already exists."
  fi
}

unalias ui 2>/dev/null
function ui {
  if command -v fancybash &>/dev/null; then
    fancybash ui "$@"
    return
  fi
  echo "🎨 Setup Shadcn UI"
  echo ""

  # Auto-detect project type
  local project_type
  if [[ -f "tsconfig.app.json" ]]; then
    project_type="vite"
    echo "  🔍 Detected: Vite project"
  elif [[ -f "next.config.js" || -f "next.config.ts" || -f "next.config.mjs" ]]; then
    project_type="nextjs"
    echo "  🔍 Detected: Next.js project"
  elif grep -q '"next"' package.json 2>/dev/null; then
    project_type="nextjs"
    echo "  🔍 Detected: Next.js project (via package.json)"
  else
    echo "  ⚠️  Could not auto-detect project type."
    echo "  1) Vite (React)"
    echo "  2) Next.js"
    read "pt?Choose manually: "
    case "$pt" in
      1) project_type="vite" ;;
      2) project_type="nextjs" ;;
      *) echo "Invalid choice"; return ;;
    esac
  fi

  echo ""
  echo "Package manager:"
  echo "1) Bun"
  echo "2) NPM"
  read "pm?Choice: "

  read "components?Add specific components? (e.g. button card input): "

  # STEP 1: Patch tsconfig FIRST — shadcn init requires @/* path alias
  echo ""
  echo "⚙️  Pre-configuring path aliases before shadcn init..."
  _ui_patch_tsconfig

  if [[ "$project_type" == "vite" ]]; then
    case "$pm" in
      1)
        echo ""
        echo "🧱 Initializing Shadcn UI with Bun (Vite)..."
        bunx --bun shadcn@latest init -t vite
        if [[ -n "$components" ]]; then
          echo "🔘 Adding components: $components..."
          bunx --bun shadcn@latest add $components
        else
          echo "🔘 Adding default Button component..."
          bunx --bun shadcn@latest add button
        fi
        ;;
      2)
        echo ""
        echo "🧱 Initializing Shadcn UI with NPM (Vite)..."
        npx shadcn@latest init -t vite
        if [[ -n "$components" ]]; then
          echo "🔘 Adding components: $components..."
          npx shadcn@latest add $components
        else
          echo "🔘 Adding default Button component..."
          npx shadcn@latest add button
        fi
        ;;
      *) echo "Invalid package manager choice"; return ;;
    esac

    # STEP 2: Patch vite.config only for Vite projects
    echo ""
    echo "⚙️  Patching vite.config with alias & tailwind..."
    _ui_patch_viteconfig

  elif [[ "$project_type" == "nextjs" ]]; then
    case "$pm" in
      1)
        echo ""
        echo "🧱 Initializing Shadcn UI with Bun (Next.js)..."
        bunx --bun shadcn@latest init
        if [[ -n "$components" ]]; then
          echo "🔘 Adding components: $components..."
          bunx --bun shadcn@latest add $components
        else
          echo "🔘 Adding default Button component..."
          bunx --bun shadcn@latest add button
        fi
        ;;
      2)
        echo ""
        echo "🧱 Initializing Shadcn UI with NPM (Next.js)..."
        npx shadcn@latest init
        if [[ -n "$components" ]]; then
          echo "🔘 Adding components: $components..."
          npx shadcn@latest add $components
        else
          echo "🔘 Adding default Button component..."
          npx shadcn@latest add button
        fi
        ;;
      *) echo "Invalid package manager choice"; return ;;
    esac
    echo "  ℹ️  Next.js detected — vite.config patch skipped."
  fi

  echo ""
  echo "---------------------------------------------------"
  echo "✅ Shadcn UI setup complete!"
  echo "🚀 Happy coding with Shadcn!"
  echo "---------------------------------------------------"
}


# --- Setup Vite (React/Vue) Project ---
unalias vite 2>/dev/null
function vite {
  if command -v fancybash &>/dev/null; then
    fancybash vite "$@"
    return
  fi
  echo "⚡ Setup Vite with:"
  echo "1) Bun"
  echo "2) NPM"
  read "c?Choice: "

  read "tw?Add Tailwind CSS v4? (y/n): "

  case "$c" in
    1)
      bunx create-vite@latest .
      if [[ "$tw" == "y" ]]; then
        if ! bun add tailwindcss @tailwindcss/vite; then
          echo "❌ Install failed with Bun."
          read "force?Try with --force? (y/n): "
          [[ "$force" == "y" ]] && bun add tailwindcss @tailwindcss/vite --force
        fi
      fi
      ;;
    2)
      npx create-vite@latest .
      if [[ "$tw" == "y" ]]; then
        if ! npm install tailwindcss @tailwindcss/vite; then
          echo "❌ Install failed with NPM (Peer Dependency Conflict likely)."
          read "legacy?Try with --legacy-peer-deps? (y/n): "
          [[ "$legacy" == "y" ]] && npm install tailwindcss @tailwindcss/vite --legacy-peer-deps
        fi
      fi
      ;;
    *) echo "Invalid choice"; return ;;
  esac

  if [[ "$tw" == "y" ]]; then
    mkdir -p src
    CSS_FILE="src/index.css"
    [ -f "src/style.css" ] && CSS_FILE="src/style.css"

    echo '@import "tailwindcss";' > "$CSS_FILE"

    echo "---------------------------------------------------"
    echo "✅ Tailwind CSS v4 packages installed!"
    echo "✅ Added '@import \"tailwindcss\";' to $CSS_FILE"
    echo ""

    _ui_patch_tsconfig
    _ui_patch_viteconfig

    echo "---------------------------------------------------"
    echo "🎉 Tailwind CSS v4 full setup complete!"
    echo "---------------------------------------------------"
  fi
}




# ======================================================
# 🚀 Install Tailwind CSS + Helpers
# ======================================================


unalias css 2>/dev/null
function css {
  if command -v fancybash &>/dev/null; then
    fancybash css "$@"
    return
  fi
  # Check package.json
  if [[ ! -f package.json ]]; then
    echo "❌ Error: package.json not found!"
    return 1
  fi

  # Auto-detect package manager
  local pm="npm"
  [[ -f bun.lockb ]] && pm="bun"

  # Detect project type
  local has_vite=0 has_next=0 has_postcss=0
  [[ -f vite.config.ts || -f vite.config.js ]] && has_vite=1
  [[ -f next.config.ts || -f next.config.js || -f next.config.mjs ]] && has_next=1
  [[ -f postcss.config.js || -f postcss.config.mjs ]] && has_postcss=1

  echo "📦 Installing Tailwind CSS v4 via $pm..."

  if [[ "$pm" == "bun" ]]; then
    if [[ $has_vite -eq 1 ]]; then
      bun add -D tailwindcss @tailwindcss/vite
    else
      bun add -D tailwindcss @tailwindcss/postcss postcss
    fi
    bun add -D clsx tailwind-merge
  else
    if [[ $has_vite -eq 1 ]]; then
      npm install -D tailwindcss @tailwindcss/vite
    else
      npm install -D tailwindcss @tailwindcss/postcss postcss
    fi
    npm install -D clsx tailwind-merge
  fi

  # Find main CSS file
  local css_file=""
  for f in "src/index.css" "src/style.css" "src/app/globals.css" "app/globals.css" "styles/globals.css" "src/styles.css"; do
    [[ -f "$f" ]] && css_file="$f" && break
  done

  # Create or update CSS file
  if [[ -n "$css_file" ]]; then
    if ! grep -q '@import "tailwindcss"' "$css_file" 2>/dev/null; then
      echo '@import "tailwindcss";' | cat - "$css_file" > /tmp/tw_css && mv /tmp/tw_css "$css_file"
      echo "✅ Added @import to $css_file"
    fi
  else
    mkdir -p src
    echo '@import "tailwindcss";' > src/index.css
    echo "✅ Created src/index.css"
    css_file="src/index.css"
  fi

  # Vite config patch (if Vite project)
  if [[ $has_vite -eq 1 ]]; then
    local vite_config=""
    for f in "vite.config.ts" "vite.config.js" "vite.config.mjs"; do
      [[ -f "$f" ]] && vite_config="$f" && break
    done

    if [[ -n "$vite_config" ]] && ! grep -q "@tailwindcss/vite" "$vite_config" 2>/dev/null; then
      _fb_sed_i '1i import tailwindcss from "@tailwindcss/vite";' "$vite_config"
      _fb_sed_i 's/plugins: \[/plugins: [tailwindcss(), /' "$vite_config"
      echo "✅ Patched $vite_config"
    fi
  fi

  # PostCSS config (if not Vite/Next)
  if [[ $has_vite -eq 0 && $has_next -eq 0 ]]; then
    if [[ ! -f postcss.config.js && ! -f postcss.config.mjs ]]; then
      cat > postcss.config.mjs << 'EOF'
/** @type {import('postcss-load-config').Config} */
const config = {
  plugins: {
    "@tailwindcss/postcss": {},
  },
};

export default config;
EOF
      echo "✅ Created postcss.config.mjs"
    fi
  fi

  echo ""
  echo "🎉 Tailwind CSS v4 Ready!"
  echo "   CSS: $css_file"
  [[ $has_vite -eq 1 ]] && echo "   Plugin: @tailwindcss/vite"
  [[ $has_vite -eq 0 && $has_next -eq 0 ]] && echo "   Plugin: @tailwindcss/postcss"
}




#  Kill Port (Usage: kp 3000)
unalias kp 2>/dev/null
function kp {
  if [ -z "$1" ]; then echo "❌ Port number required!"; return; fi
  lsof -ti:$1 | xargs kill -9 > /dev/null 2>&1 && echo "✅ Port $1 killed." || echo "❌ Port $1 not in use."
}



# ======================================================
# 🚀 Universal Extractor (Usage: ex file.zip)
# ======================================================


unalias ex 2>/dev/null
function ex {
  if [ -f "$1" ] ; then
    case "$1" in
      *.tar.bz2|*.tbz2) tar xjf "$1" ;;
      *.tar.gz|*.tgz)   tar xzf "$1" ;;
      *.tar.xz)         tar xJf "$1" ;;
      *.tar.zst|*.zst)  unzstd "$1" 2>/dev/null || tar --zstd -xf "$1" ;;
      *.bz2)            bunzip2 "$1" ;;
      *.rar)            unrar x "$1" 2>/dev/null || 7z x "$1" ;;
      *.gz)             gunzip "$1" ;;
      *.tar)            tar xf "$1" ;;
      *.zip)            unzip "$1" ;;
      *.7z)             7z x "$1" ;;
      *)                echo "❌ Unknown archive format" ;;
    esac
  else
    echo "❌ '$1' is not a valid file"
  fi
}


# Fast File Search Engine (Native Rust implementation)
unalias ff 2>/dev/null
function ff {
  fancybash ff "$@"
}

#  Secret Key Generator (Usage: gen 32)
unalias gen 2>/dev/null
function gen {
  local len="${1:-24}"
  echo -e "🔑 Base64: \033[1;32m$(openssl rand -base64 "$len" 2>/dev/null | cut -c1-"$len")\033[0m"
  echo -e "🔑 Hex:    \033[1;36m$(openssl rand -hex "$len" 2>/dev/null | cut -c1-"$len")\033[0m"
}

#  Backup File (Usage: bak .env)
unalias bak 2>/dev/null
function bak {
  cp "$1" "$1.bak" && echo "✅ Created: $1.bak"
}

# Global IP & Location Details
alias iploc='curl -s ipinfo.io/json | grep -E "ip|city|region|org"'

# Search Command History
# Usage: h git
alias h='history | grep'

# FZF History Search (Usage: fh - Dynamic fzf auto-installer)
unalias fh 2>/dev/null
function fh {
  fancybash ensure-dep fzf fzf fzf fzf || return 1
  local cmd=$(history | awk '{$1=""; print $0}' | fzf --reverse +s)
  [[ -n "$cmd" ]] && eval "$cmd"
}

# Dynamic Auto-installing Wrappers for Modern CLI Tools
unalias bat 2>/dev/null
function bat {
  fancybash ensure-dep bat bat bat bat || return 1
  if command -v batcat &>/dev/null; then
    command batcat "$@"
  else
    command bat "$@"
  fi
}

unalias eza 2>/dev/null
function eza {
  fancybash ensure-dep eza eza eza eza || return 1
  command eza "$@"
}

unalias z 2>/dev/null
function z {
  if ! command -v zoxide &>/dev/null; then
    fancybash ensure-dep zoxide zoxide zoxide zoxide || return 1
    eval "$(zoxide init zsh)"
  fi
  zoxide "$@"
}

unalias tree 2>/dev/null
function tree {
  if ! command -v tree &>/dev/null && ! command -v eza &>/dev/null; then
    fancybash ensure-dep tree tree tree tree || return 1
  fi
  if command -v eza &>/dev/null; then
    eza --tree "$@"
  else
    command tree "$@"
  fi
}

unalias tldr 2>/dev/null
function tldr {
  fancybash ensure-dep tldr tldr tldr tldr || return 1
  command tldr "$@"
}

# Safe Delete - moves to system trash
unalias trash 2>/dev/null
function trash {
  if command -v gio >/dev/null 2>&1; then
    gio trash "$@" && echo "🗑 Moved to Trash via GIO."
  else
    mkdir -p ~/.local/share/Trash/files/ 2>/dev/null
    mv "$@" ~/.local/share/Trash/files/ 2>/dev/null || mv "$@" ~/.Trash/ 2>/dev/null && echo "🗑 Moved to Trash."
  fi
}


# ======================================================
# 🚀 INTERACTIVE GIT WIP & PUSH
# ======================================================


unalias gwip 2>/dev/null
unalias gcommit 2>/dev/null

function gwip {
    if command -v fancybash &>/dev/null; then
        fancybash gwip "$@"
    else
        if ! command -v git &>/dev/null; then
            echo "❌ Git is not installed."
            return 1
        fi

        if ! git rev-parse --is-inside-work-tree &>/dev/null; then
            echo "❌ Not a git repository."
            return 1
        fi

        git add .
        local cur_branch
        cur_branch=$(git branch --show-current 2>/dev/null)
        git commit -m "🚧 WIP: Save point ($(date +'%Y-%m-%d %H:%M'))" || return 1

        if [ -n "$cur_branch" ]; then
            git push -u origin "$cur_branch"
        else
            git push -u
        fi
    fi
}

alias gcommit=gwip



# ======================================================
#  📦 universal remove
# ======================================================


unalias uu 2>/dev/null
function uu {
    local RED='\033[1;31m' GRN='\033[1;32m' YLW='\033[1;33m' CYN='\033[1;36m' BOLD='\033[1m' NC='\033[0m'

    # --- OS & Package Manager Detection ---
    local PKG_MGR=""
    if command -v apt-get &>/dev/null; then
        PKG_MGR="apt"
    elif command -v pacman &>/dev/null; then
        PKG_MGR="pacman"
    elif command -v dnf &>/dev/null; then
        PKG_MGR="dnf"
    else
        echo -e "${RED}Unsupported package manager! Cannot proceed.${NC}"
        return 1
    fi

    # --- ZSH FIX: Bulletproof fzf detection ---
    local FZF_CMD=""
    FZF_CMD=$(whence -p fzf 2>/dev/null)
    [[ -z "$FZF_CMD" ]] && FZF_CMD=$(command -v fzf 2>/dev/null)
    [[ -z "$FZF_CMD" ]] && FZF_CMD=$(command -v fzf 2>/dev/null)

    if [[ -z "$FZF_CMD" ]]; then
        local fzf_paths=(
            /usr/bin/fzf
            /usr/local/bin/fzf
            /bin/fzf
            /usr/share/doc/fzf/bin/fzf
            /usr/share/fzf/bin/fzf
            /opt/fzf/bin/fzf
            ~/.fzf/bin/fzf
            "$HOME/.fzf/bin/fzf"
        )
        for p in "${fzf_paths[@]}"; do
            [[ -x "$p" ]] && { FZF_CMD="$p"; break; }
        done
    fi

    if [[ -z "$FZF_CMD" ]]; then
        echo -e "${YLW}fzf is missing. Installing...${NC}"
        case "$PKG_MGR" in
            apt) sudo apt update && sudo apt install -y fzf ;;
            pacman) sudo pacman -Sy --noconfirm fzf ;;
            dnf) sudo dnf install -y fzf ;;
        esac
        rehash 2>/dev/null || true
        FZF_CMD=$(whence -p fzf 2>/dev/null)
        [[ -z "$FZF_CMD" ]] && FZF_CMD=$(command -v fzf 2>/dev/null)
        [[ -z "$FZF_CMD" ]] && FZF_CMD=$(command -v fzf 2>/dev/null)
    fi

    if [[ -z "$FZF_CMD" ]] || [[ ! -x "$FZF_CMD" ]]; then
        echo -e "${RED}fzf installation failed or not in PATH.${NC}"
        return 1
    fi

    echo -e "${GRN}✔ Using fzf: $FZF_CMD${NC}"

    [[ -f /usr/share/fzf/key-bindings.zsh ]] && source /usr/share/fzf/key-bindings.zsh 2>/dev/null
    [[ -f /usr/share/doc/fzf/examples/key-bindings.zsh ]] && source /usr/share/doc/fzf/examples/key-bindings.zsh 2>/dev/null
    [[ -f ~/.fzf.zsh ]] && source ~/.fzf.zsh 2>/dev/null

    sudo -v || { echo -e "${RED}Sudo authentication failed.${NC}"; return 1; }

    sync
    local START_KB=$(df -k / | awk 'NR==2 {print $4}')
    local APPS_RAW=""
    local idx=1

    echo -e "${CYN}🔍 Harvesting System Assets...${NC}"

    unalias shred_animation 2>/dev/null
    function shred_animation {
        local PID=$1 pkg=$2
        local i=0 exit_status=0

        # ZSH FIX: inner functions don't inherit outer local variables —
        # define colors locally so printf format works correctly
        local _R="\033[1;31m" _G="\033[1;32m" _C="\033[1;36m" _B="\033[1m" _N="\033[0m"

        # ZSH FIX: use array for spinner so indexing is unambiguous
        local -a sp=('/' '-' '\' '|')
        local BAR_WIDTH=20

        tput civis 2>/dev/null || true

        while kill -0 "$PID" 2>/dev/null; do
            local filled=$(( i % (BAR_WIDTH + 1) ))
            local empty=$(( BAR_WIDTH - filled ))

            # ZSH FIX: Use printf width trick instead of C-style for loops.
            # C-style ((j++)) evaluates to 0 when j reaches the limit,
            # zsh returns exit code 1, and the j counter leaks to stdout.
            local bar="" e_bar=""
            [[ $filled -gt 0 ]] && bar=$(printf '%*s' "$filled" '' | tr ' ' '█')
            [[ $empty  -gt 0 ]] && e_bar=$(printf '%*s' "$empty"  '' | tr ' ' '▒')

            local sc="${sp[$(( (i % 4) + 1 ))]}"

            # \r moves to column 0; no trailing newline so next frame overwrites
            printf "\r${_C}⚡ Processing ${_B}%s${_N}: ${_R}[${_G}%s${_R}%s${_R}]${_N} %s " \
                "$pkg" "$bar" "$e_bar" "$sc"

            # ZSH FIX: avoid ((i++)) which returns exit-code 1 when i==0
            i=$(( i + 1 ))
            sleep 0.1
        done

        wait "$PID" 2>/dev/null
        exit_status=$?

        # Clear the animation line cleanly
        local cols
        cols=$(tput cols 2>/dev/null || echo 80)
        printf "\r%${cols}s\r" ""
        tput cnorm 2>/dev/null || true

        return $exit_status
    }

    unalias format_name 2>/dev/null
    function format_name {
        echo "$1" | sed -E 's/(google-chrome-stable|google-chrome)/chrome/g; s/(brave-browser)/brave/g; s/code/vscode/g; s/(-stable|-bin|-desktop)//g; s/\.[a-zA-Z0-9]+$//' | cut -c1-18
    }

    # ============================================
    # --- SNAP COLLECTION (FIXED) ---
    # ============================================
    if command -v snap &>/dev/null; then
        echo -e "${CYN}  → Scanning Snap packages...${NC}"
        local snap_list
        snap_list=$(snap list 2>/dev/null | tail -n +2)
        if [[ -n "$snap_list" ]]; then
            echo "$snap_list" | while read -r line; do
                [[ -z "$line" ]] && continue
                local pkg=$(echo "$line" | awk '{print $1}')
                local ver=$(echo "$line" | awk '{print $2}')
                [[ "$pkg" =~ ^(Name|core|snapd|bare|gtk|gnome|kf5|qt) ]] && continue
                local name=$(format_name "$pkg")
                local size=$(du -sh /var/lib/snapd/snaps/"${pkg}"_*.snap 2>/dev/null | tail -1 | awk '{print $1}')
                local inst_date=$(snap info "$pkg" 2>/dev/null | grep "installed:" | awk '{print $2}')
                APPS_RAW+="$(printf "%-4s | %-18s | %-10s | %-12s | %-8s | %-10s | %s\n" "$idx" "$name" "snap" "$ver" "${size:-N/A}" "${inst_date:-N/A}" "$pkg")"$'\n'
                ((idx++))
            done
        fi
    fi

    # ============================================
    # --- FLATPAK COLLECTION (FIXED) ---
    # ============================================
    if command -v flatpak &>/dev/null; then
        echo -e "${CYN}  → Scanning Flatpak packages...${NC}"
        local flat_list
        flat_list=$(flatpak list --app --columns=application,name,version 2>/dev/null)
        if [[ -n "$flat_list" ]]; then
            echo "$flat_list" | while IFS=$'\t' read -r id name ver; do
                [[ -z "$id" ]] && continue
                local clean_n=$(format_name "$name")
                local fp_path="/var/lib/flatpak/app/$id"
                [[ ! -d "$fp_path" ]] && fp_path="$HOME/.local/share/flatpak/app/$id"
                local size=$(du -sh "$fp_path" 2>/dev/null | awk '{print $1}')
                local inst_date=$(stat -c %y "$fp_path" 2>/dev/null | awk '{print $1}')
                APPS_RAW+="$(printf "%-4s | %-18s | %-10s | %-12s | %-8s | %-10s | %s\n" "$idx" "$clean_n" "flatpak" "$ver" "${size:-~MB}" "$inst_date" "$id")"$'\n'
                ((idx++))
            done
        fi
    fi

    # ============================================
    # --- APPIMAGE COLLECTION (FIXED) ---
    # ============================================
    echo -e "${CYN}  → Scanning AppImage files...${NC}"
    local appimage_paths=(
        "$HOME/Downloads"
        "$HOME/Applications"
        "/opt"
        "$HOME/.local/bin"
        "$HOME/bin"
    )
    local find_paths=""
    for d in "${appimage_paths[@]}"; do
        [[ -d "$d" ]] && find_paths+="$d "
    done

    if [[ -n "$find_paths" ]]; then
        local appimage_list
        appimage_list=$(find $find_paths -maxdepth 3 -name "*.AppImage" -type f 2>/dev/null)
        if [[ -n "$appimage_list" ]]; then
            echo "$appimage_list" | while IFS= read -r path; do
                [[ -z "$path" ]] && continue
                # ZSH FIX: Use ${path:t} instead of basename
                local filename="${path:t}"
                local name=$(format_name "$filename")
                local size=$(du -sh "$path" 2>/dev/null | awk '{print $1}')
                local inst_date=$(stat -c %y "$path" 2>/dev/null | awk '{print $1}')
                APPS_RAW+="$(printf "%-4s | %-18s | %-10s | %-12s | %-8s | %-10s | %s\n" "$idx" "$name" "appimage" "Local" "${size:-N/A}" "${inst_date:-N/A}" "$path")"$'\n'
                ((idx++))
            done
        fi
    fi

    # ============================================
    # --- APT/DEB COLLECTION (FIXED) ---
    # ============================================
    case "$PKG_MGR" in
        apt)
            echo -e "${CYN}  → Scanning APT packages...${NC}"
            local manual_pkgs
            manual_pkgs=$(apt-mark showmanual 2>/dev/null)
            if [[ -n "$manual_pkgs" ]]; then
                # ZSH FIX: Process one by one to avoid word splitting issues
                echo "$manual_pkgs" | while IFS= read -r pkg; do
                    [[ -z "$pkg" ]] && continue
                    [[ "$pkg" =~ ^(linux-|grub|systemd|lib|python|gir1) ]] && continue

                    local ver=$(dpkg-query -W -f='${Version}' "$pkg" 2>/dev/null)
                    [[ -z "$ver" ]] && continue

                    local name=$(format_name "$pkg")
                    local size_kb=$(dpkg-query -W -f='${Installed-Size}' "$pkg" 2>/dev/null)
                    local size="N/A"
                    if [[ -n "$size_kb" && "$size_kb" =~ ^[0-9]+$ ]]; then
                        if (( size_kb >= 1048576 )); then
                            size=$(awk "BEGIN {printf \"%.1fGB\", $size_kb/1048576}")
                        else
                            size=$(awk "BEGIN {printf \"%.1fMB\", $size_kb/1024}")
                        fi
                    fi
                    local inst_date=$(stat -c %y "/var/lib/dpkg/info/${pkg}.list" 2>/dev/null | awk '{print $1}' || echo "N/A")
                    APPS_RAW+="$(printf "%-4s | %-18s | %-10s | %-12s | %-8s | %-10s | %s\n" "$idx" "$name" "apt" "${ver:0:10}" "$size" "$inst_date" "$pkg")"$'\n'
                    ((idx++))
                done
            fi
            ;;
        pacman)
            echo -e "${CYN}  → Scanning Pacman packages...${NC}"
            local pacman_list
            pacman_list=$(pacman -Qe 2>/dev/null)
            if [[ -n "$pacman_list" ]]; then
                echo "$pacman_list" | while IFS=' ' read -r pkg ver; do
                    [[ -z "$pkg" ]] && continue
                    [[ "$pkg" =~ ^(linux|grub|systemd|lib) ]] && continue
                    local name=$(format_name "$pkg")
                    APPS_RAW+="$(printf "%-4s | %-18s | %-10s | %-12s | %-8s | %-10s | %s\n" "$idx" "$name" "pacman" "${ver:0:10}" "N/A" "N/A" "$pkg")"$'\n'
                    ((idx++))
                done
            fi
            ;;
        dnf)
            echo -e "${CYN}  → Scanning DNF packages...${NC}"
            local dnf_list
            dnf_list=$(rpm -qa --qf '%{NAME} %{VERSION}\n' 2>/dev/null)
            if [[ -n "$dnf_list" ]]; then
                echo "$dnf_list" | while IFS=' ' read -r pkg ver; do
                    [[ -z "$pkg" ]] && continue
                    [[ "$pkg" =~ ^(kernel|grub|systemd|lib) ]] && continue
                    local name=$(format_name "$pkg")
                    APPS_RAW+="$(printf "%-4s | %-18s | %-10s | %-12s | %-8s | %-10s | %s\n" "$idx" "$name" "dnf" "${ver:0:10}" "N/A" "N/A" "$pkg")"$'\n'
                    ((idx++))
                done
            fi
            ;;
    esac

    APPS_RAW="${APPS_RAW%$'\n'}"

    # DEBUG: Show count
    local total_apps=$(echo "$APPS_RAW" | grep -c '^[0-9]' 2>/dev/null || echo "0")
    echo -e "${GRN}✔ Found $total_apps applications${NC}"

    [[ -z "$APPS_RAW" ]] && { echo -e "${YLW}No applications found.${NC}"; return; }

    local SELECTED
    SELECTED=$(echo "$APPS_RAW" | "$FZF_CMD" \
        --ansi --multi --layout=reverse --border=rounded \
        --prompt="🎯 Asset Target: " \
        --delimiter=' \| ' --with-nth=1,2,3 \
        --header="$(printf "%-5s %-20s %-11s " "IDX" "NAME" "SOURCE")" \
        --preview-window='right,45%,border-rounded,wrap' \
        --preview='
            RED="\033[1;31m"; GRN="\033[1;32m"; YLW="\033[1;33m"; CYN="\033[1;36m"; BOLD="\033[1m"; NC="\033[0m"
            name=$(echo {2}); src=$(echo {3}); ver=$(echo {4}); size=$(echo {5}); idate=$(echo {6})
            printf "\n ${BOLD}${CYN}┌─ Package Details  ─────────────┐${NC}"
            printf "\n ${CYN}│${NC} ${YLW}%-12s${NC} : %-15s ${CYN}│${NC}" "Name" "$name"
            printf "\n ${CYN}│${NC} ${YLW}%-12s${NC} : %-15s ${CYN}│${NC}" "Source" "$src"
            printf "\n ${CYN}│${NC} ${YLW}%-12s${NC} : %-15s ${CYN}│${NC}" "Version" "$ver"
            printf "\n ${CYN}│${NC} ${YLW}%-12s${NC} : ${RED}%-15s${NC} ${CYN}│${NC}" "Disk Size" "$size"
            printf "\n ${CYN}│${NC} ${YLW}%-12s${NC} : ${GRN}%-15.10s${NC} ${CYN}│${NC}" "Inst. Date" "$idate"
            printf "\n ${CYN}└────────────────────────────────┘${NC}\n"
            printf "\n ${CYN}┌─ Description ────────────────┐${NC}\n"
            printf " ${CYN}│${NC} Managed via %-16s ${CYN}│${NC}\n" "$src"
            printf " ${CYN}│${NC} Total space: ${RED}%-14s${NC} ${CYN} │${NC}\n" "$size"
            printf " ${CYN}└──────────────────────────────┘${NC}\n"
            printf " ${RED} [TAB] Select  [ENTER] Purge ${NC}"
        ')

    [[ -z "$SELECTED" ]] && { echo -e "${YLW}No selection made.${NC}"; return; }

    local count
    count=$(echo "$SELECTED" | wc -l)
    echo -e "\n${YLW}⚠️ You have selected ${BOLD}$count${NC} ${YLW}apps to uninstall:${NC}"
    echo -e "${CYN}┌──────────────────────────────────────────┐${NC}"
    echo "$SELECTED" | awk -F ' \| ' '{printf "│ • %-38s │\n", $2}'
    echo -e "${CYN}└──────────────────────────────────────────┘${NC}"

    echo -n "Are you sure you want to proceed? (y/N): "
    local confirm
    read -r confirm
    [[ ! "$confirm" =~ ^[Yy]$ ]] && { echo -e "${RED}Aborted.${NC}"; return; }
    sudo -v || { echo -e "${RED}Sudo authentication failed.${NC}"; return 1; }

    local OLD_SET="+m"
    [[ $- == *m* ]] && OLD_SET="-m"
    set +m

    local failed_apps=""

    while IFS= read -r line; do
        [[ -z "$line" ]] && continue
        local pkg_display=$(echo "$line" | awk -F ' \| ' '{print $2}' | xargs)
        local src_type=$(echo "$line" | awk -F ' \| ' '{print $3}' | xargs)
        local orig_id=$(echo "$line" | awk -F ' \| ' '{print $7}' | xargs)

        [[ -z "$src_type" || -z "$orig_id" ]] && continue

        (
            local exit_code=0
            case "$src_type" in
                snap)
                    sudo snap remove "$orig_id" &>/dev/null || exit_code=1
                    local snap_name="${orig_id:t}"
                    rm -rf ~/snap/"$snap_name" 2>/dev/null
                    ;;
                flatpak)
                    flatpak uninstall -y --delete-data "$orig_id" &>/dev/null || exit_code=1
                    rm -rf ~/.var/app/"$orig_id" 2>/dev/null
                    rm -rf ~/.local/share/flatpak/app/"$orig_id" 2>/dev/null
                    ;;
                appimage)
                    if [[ -f "$orig_id" ]]; then
                        rm -f "$orig_id" &>/dev/null || exit_code=1
                    else
                        exit_code=1
                    fi
                    local appimage_name="${orig_id:t:r}"
                    [[ -n "$appimage_name" ]] && {
                        rm -f ~/.local/share/applications/appimagekit_*"${appimage_name}"*.desktop 2>/dev/null
                        rm -f ~/.local/share/applications/"${appimage_name}".desktop 2>/dev/null
                        rm -rf ~/.local/share/icons/hicolor/*/apps/appimagekit_*"${appimage_name}"* 2>/dev/null
                        rm -f ~/.config/AppImageLauncher/entries/"${appimage_name}"* 2>/dev/null
                    }
                    ;;
                apt)
                    sudo apt purge -y "$orig_id" &>/dev/null || exit_code=1
                    if [[ "$orig_id" =~ ^[a-z0-9-]+$ ]] && [[ ! "$orig_id" =~ ^(python|lib|systemd|xorg|gtk|gnome|kde|qt) ]]; then
                        [[ -d ~/.config/"$orig_id" ]] && rm -rf ~/.config/"$orig_id" 2>/dev/null
                        [[ -d ~/.cache/"$orig_id" ]] && rm -rf ~/.cache/"$orig_id" 2>/dev/null
                        [[ -d ~/.local/share/"$orig_id" ]] && rm -rf ~/.local/share/"$orig_id" 2>/dev/null
                        local base_name=$(echo "$orig_id" | sed 's/-desktop//g; s/-stable//g; s/-git//g; s/-bin//g')
                        if [[ "$base_name" != "$orig_id" ]]; then
                            [[ -d ~/.config/"$base_name" ]] && rm -rf ~/.config/"$base_name" 2>/dev/null
                            [[ -d ~/.cache/"$base_name" ]] && rm -rf ~/.cache/"$base_name" 2>/dev/null
                            [[ -d ~/.local/share/"$base_name" ]] && rm -rf ~/.local/share/"$base_name" 2>/dev/null
                        fi
                        [[ -d ~/."${base_name}" ]] && rm -rf ~/."${base_name}" 2>/dev/null
                    fi
                    ;;
                pacman)
                    sudo pacman -Rns --noconfirm "$orig_id" &>/dev/null || exit_code=1
                    if [[ "$orig_id" =~ ^[a-z0-9-]+$ ]] && [[ ! "$orig_id" =~ ^(python|lib|systemd|xorg|gtk|gnome|kde|qt) ]]; then
                        [[ -d ~/.config/"$orig_id" ]] && rm -rf ~/.config/"$orig_id" 2>/dev/null
                        [[ -d ~/.cache/"$orig_id" ]] && rm -rf ~/.cache/"$orig_id" 2>/dev/null
                        [[ -d ~/.local/share/"$orig_id" ]] && rm -rf ~/.local/share/"$orig_id" 2>/dev/null
                        local base_name=$(echo "$orig_id" | sed 's/-desktop//g; s/-stable//g; s/-git//g; s/-bin//g')
                        if [[ "$base_name" != "$orig_id" ]]; then
                            [[ -d ~/.config/"$base_name" ]] && rm -rf ~/.config/"$base_name" 2>/dev/null
                            [[ -d ~/.cache/"$base_name" ]] && rm -rf ~/.cache/"$base_name" 2>/dev/null
                            [[ -d ~/.local/share/"$base_name" ]] && rm -rf ~/.local/share/"$base_name" 2>/dev/null
                        fi
                        [[ -d ~/."${base_name}" ]] && rm -rf ~/."${base_name}" 2>/dev/null
                    fi
                    ;;
                dnf)
                    sudo dnf remove -y "$orig_id" &>/dev/null || exit_code=1
                    if [[ "$orig_id" =~ ^[a-z0-9-]+$ ]] && [[ ! "$orig_id" =~ ^(python|lib|systemd|xorg|gtk|gnome|kde|qt) ]]; then
                        [[ -d ~/.config/"$orig_id" ]] && rm -rf ~/.config/"$orig_id" 2>/dev/null
                        [[ -d ~/.cache/"$orig_id" ]] && rm -rf ~/.cache/"$orig_id" 2>/dev/null
                        [[ -d ~/.local/share/"$orig_id" ]] && rm -rf ~/.local/share/"$orig_id" 2>/dev/null
                        local base_name=$(echo "$orig_id" | sed 's/-desktop//g; s/-stable//g; s/-git//g; s/-bin//g')
                        if [[ "$base_name" != "$orig_id" ]]; then
                            [[ -d ~/.config/"$base_name" ]] && rm -rf ~/.config/"$base_name" 2>/dev/null
                            [[ -d ~/.cache/"$base_name" ]] && rm -rf ~/.cache/"$base_name" 2>/dev/null
                            [[ -d ~/.local/share/"$base_name" ]] && rm -rf ~/.local/share/"$base_name" 2>/dev/null
                        fi
                        [[ -d ~/."${base_name}" ]] && rm -rf ~/."${base_name}" 2>/dev/null
                    fi
                    ;;
            esac
            exit $exit_code
        ) &
        local PID=$!

        if shred_animation "$PID" "$pkg_display"; then
            echo -e "${GRN}✔ $pkg_display has been shredded.${NC}"
        else
            echo -e "${RED}✘ $pkg_display failed to uninstall.${NC}"
            failed_apps+="$pkg_display ($src_type), "
        fi
    done <<< "$SELECTED"

    if [[ "$OLD_SET" == "-m" ]]; then
        set -m
    else
        set +m
    fi

    [[ -n "$failed_apps" ]] && echo -e "\n${RED}Failed: ${failed_apps%, }${NC}"

    # --- Turbo Clean ---
    echo -e "\n${CYN}➜ Initializing Turbo Clean Protocol...${NC}\n"

    if command -v snap &>/dev/null; then
        echo -ne "${YLW}➜ Purging old Snap revisions...${NC} "
        LANG=en_US.UTF-8 snap list --all 2>/dev/null | awk '/disabled/{print $1, $3}' | while read -r snapname revision; do
            [[ -n "$snapname" && -n "$revision" ]] && sudo snap remove "$snapname" --revision="$revision" &>/dev/null
        done
        echo -e "${GRN}OK${NC}"
    fi

    echo -ne "${YLW}➜ Cleaning AppImage artifacts...${NC} "
    find ~/.local/share/applications -name "*appimage*" -type f 2>/dev/null | while IFS= read -r file; do
        local exec_path
        exec_path=$(grep "^Exec=" "$file" 2>/dev/null | head -1 | cut -d'=' -f2 | cut -d' ' -f1)
        if [[ -n "$exec_path" && ! -f "$exec_path" ]]; then
            rm -f "$file"
        fi
    done
    echo -e "${GRN}OK${NC}"

    if command -v flatpak &>/dev/null; then
        echo -ne "${YLW}➜ Removing unused Flatpak data...${NC} "
        flatpak uninstall --unused -y &>/dev/null
        echo -e "${GRN}OK${NC}"
    fi

    echo -ne "${YLW}➜ Purging unused system configs & cache ($PKG_MGR)...${NC} "
    case "$PKG_MGR" in
        apt)
            sudo apt autoremove -y &>/dev/null
            sudo apt autoclean -y &>/dev/null
            sudo apt clean &>/dev/null
            ;;
        pacman)
            local orphans
            orphans=$(pacman -Qtdq 2>/dev/null)
            [[ -n "$orphans" ]] && sudo pacman -Rns --noconfirm $orphans &>/dev/null
            sudo pacman -Sc --noconfirm &>/dev/null
            ;;
        dnf)
            sudo dnf autoremove -y &>/dev/null
            sudo dnf clean all &>/dev/null
            ;;
    esac
    echo -e "${GRN}OK${NC}"

    sync
    sleep 1
    local END_KB=$(df -k / | awk 'NR==2 {print $4}')
    local SAVED_MB=$(( (START_KB - END_KB) / 1024 ))

    echo -e "\n${GRN}✅ Cleanup Successful!${NC}"
    if (( SAVED_MB > 0 )); then
        echo -e "${CYN}🚀 Total Space Recovered: ${BOLD}${SAVED_MB} MB${NC}\n"
    elif (( SAVED_MB == 0 )); then
        echo -e "${CYN}📊 No significant space change${NC}\n"
    else
        echo -e "${YLW}⚠️  Space calculation shows negative value (disk activity during cleanup)${NC}\n"
    fi
}


# ======================================================
#  📦 Universal Update pack
# ======================================================


unalias uup 2>/dev/null
alias uup="fancybash uup"



# ======================================================
#  🆘 HELP MENU — Modern UI/UX Edition
# ======================================================
unalias keep 2>/dev/null
function keep {
    # Modern Color Palette
    local RESET BOLD DIM CYAN PINK PURPLE GREEN YELLOW ORANGE BLUE RED WHITE GRAY BG_DARK BG_CARD
    local ICON_ROCKET ICON_FOLDER ICON_FILE ICON_GEAR ICON_PACKAGE ICON_BUN ICON_GIT ICON_LIGHTNING ICON_TERMINAL ICON_WARNING ICON_STAR ICON_SEARCH ICON_PRISMA
    RESET='\033[0m'
    BOLD='\033[1m'
    DIM='\033[2m'

    # Primary Colors
    CYAN='\033[38;5;51m'      # Electric Cyan
    PINK='\033[38;5;213m'     # Hot Pink
    PURPLE='\033[38;5;141m'   # Soft Purple
    GREEN='\033[38;5;82m'     # Neon Green
    YELLOW='\033[38;5;220m'   # Gold Yellow
    ORANGE='\033[38;5;208m'   # Orange
    BLUE='\033[38;5;75m'      # Sky Blue
    RED='\033[38;5;203m'      # Soft Red
    WHITE='\033[38;5;255m'    # Pure White
    GRAY='\033[38;5;245m'     # Gray

    # Background Colors
    BG_DARK='\033[48;5;234m'  # Dark background
    BG_CARD='\033[48;5;236m'  # Card background

    # Icons
    ICON_ROCKET='🚀'
    ICON_FOLDER='📂'
    ICON_FILE='📄'
    ICON_GEAR='⚙️'
    ICON_PACKAGE='📦'
    ICON_BUN='🥐'
    ICON_GIT='🌿'
    ICON_LIGHTNING='⚡'
    ICON_TERMINAL='💻'
    ICON_WARNING='⚠️'
    ICON_STAR='✨'
    ICON_SEARCH='🔍'
    ICON_PRISMA='💎'

    # Clear screen for clean look
    clear

    # Header with gradient effect

    echo -e "${CYAN} ╔══════════════════════════════════════════════════════════════════════════╗${RESET}"
    echo -e "${CYAN} ║${RESET}  ${BOLD}${PINK}${ICON_ROCKET}  MASTER COMMAND CENTER ${RESET}${CYAN}│${RESET} ${GRAY}Developer Rihad's Ultimate Bash Environment${RESET}      ${CYAN} ${RESET}"
    echo -e "${CYAN} ╚══════════════════════════════════════════════════════════════════════════╝${RESET}"
    echo -e "${GRAY}  v2.0 • Modern Terminal UX • $(date +'%B %d, %Y')${RESET}\n"

    # Function to print category headers
    unalias print_category 2>/dev/null
    function print_category {
        local icon=$1
        local title=$2
        local color=$3
        echo -e "\n  ${color}┌─────────────────────────────────────────────────────────────────────┐${RESET}"
        echo -e "  ${color}│${RESET} ${BOLD}${icon}  ${title}${RESET}${color}                                    ${RESET}"
        echo -e "  ${color}└─────────────────────────────────────────────────────────────────────┘${RESET}"
    }

    # Function to print command row
    unalias print_cmd 2>/dev/null
    function print_cmd {
        local cmd=$1
        local desc=$2
        local example=$3
        local cmd_color=$4

        if [ -z "$example" ]; then
            printf "     ${BOLD}${cmd_color}%-12s${RESET} ${GRAY}│${RESET} %s\n" "$cmd" "$desc"
        else
            printf "     ${BOLD}${cmd_color}%-12s${RESET} ${GRAY}│${RESET} %-35s ${DIM}%s${RESET}\n" "$cmd" "$desc" "$example"
        fi
    }

    # Function to print alias row
    unalias print_alias 2>/dev/null
    function print_alias {
        local alias=$1
        local equals=$2
        local full=$3
        local color=$4
        printf "     ${BOLD}${color}%-6s${RESET} ${GRAY}%s${RESET} ${DIM}%s${RESET}\n" "$alias" "$equals" "$full"
    }

    # ==================== NAVIGATION ====================
    print_category "$ICON_FOLDER" "NAVIGATION & MOVEMENT" "$CYAN"
    print_cmd ".." "Parent directory" "" "$YELLOW"
    print_cmd "..." "Two levels up" "" "$YELLOW"
    print_cmd "...." "Three levels up" "" "$YELLOW"
    print_cmd "dev" "Go to ~/Development" "" "$GREEN"
    print_cmd "fr / ba / fu" "Frontend / Backend / Fullstack" "" "$GREEN"
    print_cmd "fig / ar / de" "Figma / Archive / Dev folders" "" "$GREEN"
    print_cmd "des / doc / dow" "Desktop / Documents / Downloads" "" "$GREEN"
    print_cmd "bv / ch / gp" "Brave / Chrome / Photos Downloads" "" "$GREEN"

    # ==================== FILE OPERATIONS ====================
    print_category "$ICON_FILE" "FILE & FOLDER MANAGEMENT" "$PINK"
    print_cmd "mkd <name>" "Create & enter directory" "mkd new-project" "$YELLOW"
    print_cmd "t <file>" "Create file with feedback" "t index.html" "$YELLOW"
    print_cmd "rmd <name>" "Force remove directory" "rmd old-folder" "$RED"
    print_cmd "rmf <file>" "Remove file (safe)" "rmf file.txt" "$RED"
    print_cmd "bak <file>" "Create backup copy" "bak .env" "$BLUE"
    print_cmd "trash <file>" "Move to system trash" "trash junk.txt" "$ORANGE"
    print_cmd "to" "Open current folder in VS Code" "" "$CYAN"

    # ==================== NPM ====================
    print_category "$ICON_PACKAGE" "NPM COMMANDS" "$GREEN"
    print_alias "ni" "→" "npm install" "$GREEN"
    print_alias "nid" "→" "npm install -D" "$GREEN"
    print_alias "nr" "→" "npm run" "$GREEN"
    print_alias "nrd" "→" "npm run dev" "$YELLOW"
    print_alias "nrb" "→" "npm run build" "$YELLOW"
    print_alias "nrs" "→" "npm run start" "$YELLOW"

    # ==================== BUN ====================
    print_category "$ICON_BUN" "BUN COMMANDS (Ultra Fast)" "$YELLOW"
    print_alias "bi" "→" "bun install" "$YELLOW"
    print_alias "br" "→" "bun run" "$YELLOW"
    print_alias "brd" "→" "bun run dev" "$GREEN"
    print_alias "bhot" "→" "bun --hot" "$CYAN"
    print_alias "w" "→" "bun --watch" "$CYAN"
    print_alias "brb" "→" "bun run build" "$GREEN"
    print_alias "brs" "→" "bun run start" "$GREEN"
    print_alias "html" "→" "bun run index.html" "$CYAN"

    # ==================== GIT ====================
    print_category "$ICON_GIT" "GIT VERSION CONTROL" "$PURPLE"
    print_cmd "gi" "Initialize new repository" "" "$GREEN"
    print_cmd "gs" "Check status (short format)" "" "$BLUE"
    print_cmd "ga" "Stage all files" "" "$YELLOW"
    print_cmd "gcm <msg>" "Commit with message" "gcm 'feat: add login'" "$GREEN"
    print_cmd "gps / gpl" "Push / Pull from remote" "" "$PINK"
    print_cmd "gl" "View beautiful git log" "" "$CYAN"
    print_cmd "gco <branch>" "Checkout branch" "gco main" "$YELLOW"
    print_cmd "gcb <name>" "Create & checkout new branch" "gcb feature-x" "$GREEN"
    print_cmd "gd" "View diff" "" "$ORANGE"
    print_cmd "gst / gsta / gpop" "Stash / Apply / Pop" "" "$BLUE"
    print_cmd "gwip" "Quick WIP commit + auto push" "" "$PINK"

    # ==================== PROJECT SETUP ====================
    print_category "$ICON_LIGHTNING" "PROJECT INITIALIZATION" "$ORANGE"
    print_cmd "ii" "Initialize project (Bun/NPM choice)" "" "$GREEN"
    print_cmd "next" "Setup Next.js project" "" "$CYAN"
    print_cmd "ui" "Setup Shadcn UI with components" "ui + select button,card" "$BLUE"
    print_cmd "vite" "Setup Vite project with Tailwind" "" "$PURPLE"
    print_cmd "css" "Auto-install Tailwind CSS" "" "$BLUE"
    print_cmd "run" "Bun Run JS & TS File (Interactive)" "" "$YELLOW"

    # ==================== C / C++ ====================
    print_category "$ICON_GEAR" "C/C++ DEVELOPMENT" "$CYAN"
    print_cmd "makecpp" "C/C++ boilerplate (cd, git, vscode)" "makecpp proj_name" "$BLUE"
    print_cmd "make run" "Compile and run the C/C++ project" "" "$GREEN"
    print_cmd "make clean" "Remove compiled binary file" "" "$RED"

    # ==================== SYSTEM ====================
    print_category "$ICON_GEAR" "SYSTEM & MAINTENANCE" "$BLUE"
    print_cmd "update" "Update system packages" "" "$GREEN"
    print_cmd "clean" "Clean apt cache & orphans" "" "$YELLOW"
    print_cmd "uup" "MEGA UPDATE: Apt+Snap+Flatpak+Bun+Node" "" "$PINK"
    print_cmd "uu" "UNINSTALLER: Remove apps interactively" "" "$RED"
    print_cmd "uc" "Universal Clean" "" "$YELLOW"
    print_cmd "setuppc" "Setup new PC with all tools" "" "$CYAN"
    print_cmd "rel" "Reload .bashrc configuration" "" "$GREEN"
    print_cmd "myip / iploc" "Show IP / Location info" "" "$BLUE"
    print_cmd "ports" "Show open ports" "" "$YELLOW"
    print_cmd "kp <port>" "Kill process on port" "kp 3000" "$RED"
    print_cmd "serve" "Start Python HTTP server" "" "$GREEN"
    print_cmd "ut" "Setup cli tool for pc Optimized" "" "$CYAN"
    print_cmd "rt" " Install Node(nvm) , Bun , Deno" "" "$YELLOW"
    print_cmd "rn" "Renamed All file @ & % * # @" "" "$PINK"


    # ==================== UTILITIES ====================
    print_category "$ICON_TERMINAL" "UTILITY TOOLS" "$CYAN"
    print_cmd "ex <file>" "Extract any archive" "ex file.zip" "$GREEN"
    print_cmd "ff <name>" "Find file (excludes node_modules)" "ff config" "$YELLOW"
    print_cmd "gen <len>" "Generate random secret key" "gen 32" "$PURPLE"
    print_cmd "h <word>" "Search command history" "h git" "$BLUE"
    print_cmd "c / cls" "Clear terminal screen" "" "$GRAY"
    print_cmd "v" "Interactive video player for directory" "" "$PINK"
    print_cmd "pg" "Generate package.json for current project" "" "$PURPLE"

    # ==================== DOCKER & CONTAINERS ====================
    print_category "$ICON_BUN" "DOCKER & CONTAINERS" "$YELLOW"
    print_cmd "dman" "Docker Desktop interactive TUI manager" "dman" "$CYAN"
    print_cmd "dstats" "Live Docker resource usage dashboard" "dstats" "$BLUE"
    print_cmd "dps" "List running containers" "" "$GREEN"
    print_cmd "dpsa" "List all containers" "" "$YELLOW"
    print_cmd "di" "Show Docker images" "" "$CYAN"
    print_cmd "dvl" "List Docker volumes" "" "$CYAN"
    print_cmd "dnl" "List Docker networks" "" "$CYAN"
    print_cmd "dsize" "Inspect Docker disk usage" "" "$BLUE"
    print_cmd "dtop" "Show container resource usage" "" "$BLUE"
    print_cmd "dstop <name>" "Stop container" "dstop myapp" "$RED"
    print_cmd "drm <name>" "Remove container" "drm myapp" "$RED"
    print_cmd "drmi <name>" "Remove image" "drmi myimage" "$RED"
    print_cmd "drestart <name>" "Restart container" "drestart myapp" "$ORANGE"
    print_cmd "dkill <name>" "Force remove container" "dkill myapp" "$ORANGE"
    print_cmd "dstopall" "Stop all running containers" "" "$ORANGE"
    print_cmd "drmall" "Remove all containers" "" "$ORANGE"
    print_cmd "dbuild <tag>" "Build Docker image" "dbuild myapp ." "$GREEN"
    print_cmd "dbuild-nocache <tag>" "Build without cache" "dbuild-nocache myapp ." "$GREEN"
    print_cmd "dhist <image>" "Show image history" "dhist myimage" "$CYAN"
    print_cmd "dports <name>" "Inspect container ports" "dports myapp" "$CYAN"
    print_cmd "dsh <name>" "Shell into container" "dsh myapp bash" "$BLUE"
    print_cmd "dlogs <name>" "Follow logs" "dlogs myapp" "$CYAN"
    print_cmd "dcup / dcdn" "Compose up/down" "dcup / dcdn" "$PURPLE"
    print_cmd "dclogs" "Follow compose logs" "dclogs" "$PURPLE"
    print_cmd "dcupb" "Compose up with build" "dcupb" "$PURPLE"
    print_cmd "dtest-ubuntu / dtest-node / dtest-alpine" "Launch test containers" "dtest-node" "$GREEN"
    print_cmd "dfind <term>" "Search containers/images" "dfind nginx" "$YELLOW"
    print_cmd "droot <name>" "Enter container as root" "droot myapp" "$YELLOW"
    print_cmd "dip <name>" "Show container IP" "dip myapp" "$BLUE"
    print_cmd "dwatch <name>" "Watch container changes" "dwatch myapp" "$CYAN"
    print_cmd "dnetstat <name>" "Inspect container network" "dnetstat myapp" "$CYAN"
    print_cmd "dtop-proc <name>" "Show process tree" "dtop-proc myapp" "$PURPLE"
    print_cmd "dbackup <name> <archive>" "Backup volume to tar" "dbackup data data.tar" "$GREEN"
    print_cmd "dkill-force" "Force remove all containers" "dkill-force" "$RED"
    print_cmd "dclean" "Clean unused Docker resources" "dclean" "$RED"
    print_cmd "dstart" "Start Docker service" "" "$GREEN"
    print_cmd "doff" "Stop Docker service" "" "$RED"
    print_cmd "dstatus" "Check Docker service status" "" "$BLUE"
    print_cmd "denable" "Enable Docker auto-start on boot" "" "$GREEN"
    print_cmd "ddisable" "Disable Docker auto-start on boot" "" "$ORANGE"

    # ==================== POSTGRESQL ====================
    print_category "$ICON_GEAR" "POSTGRESQL DATABASE" "$BLUE"
    print_cmd "pgstart / pgstop" "Start / Stop PostgreSQL service" "" "$GREEN"
    print_cmd "pgrestart" "Restart PostgreSQL service" "" "$YELLOW"
    print_cmd "pgstatus" "Check PostgreSQL service status" "" "$CYAN"
    print_cmd "pgenable / pgdisable" "Enable / Disable auto-start on boot" "" "$ORANGE"
    print_cmd "pgl" "Login as postgres user (psql)" "" "$BLUE"
    print_cmd "pgdb <name>" "Connect to a specific database" "pgdb mydb" "$BLUE"
    print_cmd "pgls" "List all databases" "" "$CYAN"
    print_cmd "pgtables" "List all tables in current DB" "" "$CYAN"
    print_cmd "pgusers" "List all users / roles" "" "$PURPLE"
    print_cmd "pgsize" "Show size of each database" "" "$YELLOW"
    print_cmd "pgver" "Show PostgreSQL version" "" "$GRAY"
    print_cmd "pgconn" "Show active connections count" "" "$BLUE"
    print_cmd "pgcreate <db>" "Create a new database" "pgcreate mydb" "$GREEN"
    print_cmd "pgdrop <db>" "Drop / delete a database" "pgdrop mydb" "$RED"
    print_cmd "pgdump <db>" "Dump/Backup a database" "pgdump mydb > b.sql" "$ORANGE"
    print_cmd "pgrestore <db>" "Restore database from file" "pgrestore mydb < b.sql" "$ORANGE"
    print_cmd "pglogs" "Follow PostgreSQL log file" "" "$RED"

    # ==================== PRISMA ORM ====================
    print_category "$ICON_PRISMA" "PRISMA ORM" "$CYAN"
    print_cmd "np / bp" "npx/bunx prisma (base)" "np" "$GREEN"
    print_cmd "npi / bpi" "prisma init" "npi" "$BLUE"
    print_cmd "npg / bpg" "prisma generate" "npg" "$YELLOW"
    print_cmd "nps / bps" "prisma studio" "nps" "$PINK"
    print_cmd "npmd / bpmd" "prisma migrate dev" "npmd" "$CYAN"
    print_cmd "npmdn / bpmdn <n>" "prisma migrate dev --name" "npmdn add_users" "$CYAN"
    print_cmd "npmr / bpmr" "prisma migrate reset" "npmr" "$RED"
    print_cmd "npmdp / bpmdp" "prisma migrate deploy" "npmdp" "$PURPLE"
    print_cmd "npms / bpms" "prisma migrate status" "npms" "$GRAY"
    print_cmd "npdp / bpdp" "prisma db push" "npdp" "$ORANGE"
    print_cmd "npdl / bpdl" "prisma db pull" "npdl" "$BLUE"
    print_cmd "npds / bpds" "prisma db seed" "npds" "$GREEN"
    print_cmd "npf / bpf" "prisma format" "npf" "$YELLOW"
    print_cmd "npv / bpv" "prisma version" "npv" "$GRAY"



    # ==================== ADVANCED INTERACTIVE TOOLS ====================
    print_category "$ICON_LIGHTNING" "ADVANCED INTERACTIVE TOOLS" "$PURPLE"
    print_cmd "todo" "Interactive Todo Task Manager" "todo | todo add | todo done" "$GREEN"
    print_cmd "notes" "Fuzzy Notes Manager with live preview" "notes | notes add | notes search" "$PINK"
    print_cmd "ffmedia" "24-in-1 FFmpeg Multimedia Suite" "ffmedia | ffstudio | fftool" "$CYAN"
    print_cmd "vault" "Hardened AES-256 Multi-Vault Manager" "vault | secvault | fvault" "$RED"
    print_cmd "cf" "Fuzzy find & navigate directories" "" "$CYAN"
    print_cmd "   ↳ ENTER" "cd to selected folder" "" "$GREEN"
    print_cmd "   ↳ CTRL+O" "Open in VS Code/Cursor/Nvim" "" "$BLUE"
    print_cmd "   ↳ CTRL+Y" "Copy path to clipboard" "" "$YELLOW"
    print_cmd "   ↳ CTRL+H" "Navigate to parent directory" "" "$PURPLE"
    print_cmd "gbranch" "Modern interactive Git branch manager (fzf/gum)" "gbranch [-l|-r|-a]" "$PURPLE"
    print_cmd "   ↳ ENTER" "Checkout selected branch" "" "$GREEN"
    print_cmd "   ↳ CTRL+D" "Delete local branch (git branch -D)" "" "$RED"
    print_cmd "   ↳ CTRL+R" "Rebase onto selected branch" "" "$YELLOW"
    print_cmd "   ↳ CTRL+M" "Merge selected branch into current" "" "$BLUE"
    print_cmd "fkill" "Advanced interactive process killer (fzf/gum)" "fkill [query|port]" "$RED"
    print_cmd "   ↳ Tab" "Multi-select multiple processes" "" "$YELLOW"
    print_cmd "   ↳ ENTER" "Soft kill selected process(es) (SIGTERM)" "" "$ORANGE"
    print_cmd "   ↳ CTRL+X" "Force kill selected process(es) (SIGKILL -9)" "" "$RED"
    print_cmd "   ↳ CTRL+R" "Reload live process list" "" "$BLUE"
    print_cmd "fcd" "Fuzzy quick directory jump (fzf/gum)" "" "$GREEN"
    print_cmd "mkd <name>" "Create & enter new directory" "mkd my-project" "$GREEN"
    print_cmd "rmd <name>" "Force remove directory" "rmd old-folder" "$RED"
    print_cmd "rmf <file>" "Safe remove single file" "rmf file.txt" "$ORANGE"
    print_cmd "bak <file>" "Create backup of file" "bak config.js" "$BLUE"
    print_cmd "trash <file>" "Move file to system trash" "trash junk.txt" "$YELLOW"

    # ==================== FILE MANAGEMENT ====================
    print_category "$ICON_FILE" "FOLDER UTILITIES" "$BLUE"
    print_cmd "mkd / rmd / rmf" "Create/Remove directories/files" "" "$YELLOW"
    print_cmd "bak / trash" "Backup or trash files safely" "" "$ORANGE"
    print_cmd "cd <folder>" "Smart cd with auto-list files" "" "$CYAN"

    # ==================== FOOTER ====================
    echo -e "\n  ${PURPLE}┌─────────────────────────────────────────────────────────────────────┐${RESET}"
    echo -e "  ${PURPLE}│${RESET}  ${ICON_STAR} ${BOLD}PRO TIPS:${RESET}                                                        ${PURPLE} ${RESET}"
    echo -e "  ${PURPLE}│${RESET}    • ${YELLOW}cd <folder>${RESET} automatically lists files with colors             ${PURPLE} ${RESET}"
    echo -e "  ${PURPLE}│${RESET}    • Type ${CYAN}folder name only${RESET} to auto-cd (autocd enabled)            ${PURPLE} ${RESET}"
    echo -e "  ${PURPLE}│${RESET}    • ${GRAY}Prompt shows:${RESET} Git status │ Node/Bun versions │ System stats    ${PURPLE} ${RESET}"
    echo -e "  ${PURPLE}└─────────────────────────────────────────────────────────────────────┘${RESET}"



    echo ""
    echo -e "        \e[1;36m========================================================================\e[0m"
    echo -e "        \e[1;33m          🚀 MY LINUX SETUP LIST         \e[0m"
    echo -e "        \e[1;36m========================================================================\e[0m"

    echo -e "           \e[1;32m[📦 FLATPAK APPS]\e[0m"
    echo -e "           • Brave, Flatseal, ytDownloader, Packet"
    echo -e "           • Inkscape, Bazaar, Vlc, Zed"
    echo ""

    echo -e "           \e[1;34m[⚙️ CORE DEB & TOOLS]\e[0m"
    echo -e "           • VS Code, Chrome"
    echo -e "           • Zram, Fzf, Preload, Earlyoom, ls-sensors"
    echo ""

    echo -e "           \e[1;35m[🛠️ DEV TOOLS]\e[0m"
    echo -e "           • Git, Nodejs, Bun, Curl, Wget"

    echo -e "        \e[1;36m========================================================================\e[0m"
    echo ""



    # Dynamic stats
    echo -e "\n  ${DIM}$(date +'%H:%M:%S') • Bash v${BASH_VERSION:0:3} • $(whoami)@$(hostname) • $PWD${RESET}\n"
}

# ======================================================
#  📦 Run ts / js file on terminal
# ======================================================

unalias run 2>/dev/null
function run {
    # Color Codes
    local CYAN='\033[0;36m'
    local YELLOW='\033[1;33m'
    local BLUE='\033[1;34m'
    local GREEN='\033[1;32m'
    local RED='\033[0;31m'
    local BOLD='\033[1m'
    local NC='\033[0m'

    # zsh: nullglob enable (local scope)
    setopt localoptions nullglob

    # Safe file collection
    local files=(*.js *.ts)

    if (( ${#files} == 0 )); then
        echo -e "${RED}󱓇 No .js or .ts files found!${NC}"
        return 1
    fi

    # Modern Header
    echo -e "\n${CYAN}╭──────────────────────────────────────────╮${NC}"
    echo -e "${CYAN}│${NC}  ${BOLD}⚡ BUN INTERACTIVE RUNNER${NC}               ${CYAN}│${NC}"
    echo -e "${CYAN}╰──────────────────────────────────────────╯${NC}"

    # List Display with Icons
    local i ext icon
    # zsh: array index 1-based
    for (( i = 1; i <= ${#files}; i++ )); do
        ext="${files[$i]##*.}"
        if [[ "$ext" == "ts" ]]; then
            icon="${BLUE}📘${NC}"
        else
            icon="${YELLOW}📒${NC}"
        fi
        printf "${CYAN}  [%2d]${NC}  %b  %-30s\n" "$i" "$icon" "${files[$i]}"
    done

    echo -e "${CYAN}────────────────────────────────────────────${NC}"

    # Smart Input Prompt
    echo -e "${YELLOW}👉 Enter file number (or Ctrl+C):${NC}"
    echo -ne "${YELLOW}❯ ${NC}"
    local choice
    read -r choice

    # Strict validation
    if [[ "$choice" =~ ^[0-9]+$ ]] && (( choice >= 1 && choice <= ${#files} )); then
        # zsh: 1-based index, so direct access
        local selected_file="${files[$choice]}"

        echo -e "\n${GREEN}✔ Selected:${NC} ${BOLD}$selected_file${NC}"
        echo -e "${CYAN}────────────────────────────────────────────${NC}"

        # Mode Selection Menu
        echo -e "\n${YELLOW}👉 Choose run mode:${NC}"
        echo -e "${CYAN}  [1]${NC}  🚀  ${BOLD}bun run${NC}     (default)"
        echo -e "${CYAN}  [2]${NC}  🔥  ${BOLD}bun --hot${NC}   (hot reload)"
        echo -e "${CYAN}  [3]${NC}  👁  ${BOLD}bun --watch${NC} (watch mode)"
        echo -e "${CYAN}────────────────────────────────────────────${NC}"
        echo -ne "${YELLOW}❯ ${NC}"
        local mode
        read -r mode

        local bun_action mode_label mode_color
        case "$mode" in
            2)
                bun_action="--hot"
                mode_label="HOT RELOAD"
                mode_color="${RED}"
                ;;
            3)
                bun_action="--watch"
                mode_label="WATCH MODE"
                mode_color="${YELLOW}"
                ;;
            1|"")
                bun_action="run"
                mode_label="RUN"
                mode_color="${GREEN}"
                ;;
            *)
                echo -e "\n${RED}✘ Error: Invalid mode! Defaulting to 'bun run'.${NC}"
                bun_action="run"
                mode_label="RUN"
                mode_color="${GREEN}"
                ;;
        esac

        echo -e "\n${mode_color}⚙ $mode_label:${NC} ${BOLD}$selected_file${NC}\n"

        # Execute
        bun $bun_action "$selected_file"
    else
        echo -e "\n${RED}✘ Error: Invalid selection!${NC}"
        return 1
    fi
}


# ======================================================
#  📦 VIDEO FILLTER AND OPEN
# ======================================================



unalias v 2>/dev/null
function v {
    local TARGET="${1:-$PWD}"

    # 🎥 Smart Player Detection (fastest first)
    local PLAYER_CMD
    if command -v flatpak &>/dev/null && flatpak list 2>/dev/null | grep -q 'org.videolan.VLC'; then
        PLAYER_CMD="flatpak run org.videolan.VLC"
    elif command -v vlc &>/dev/null; then
        PLAYER_CMD="vlc"
    elif command -v mpv &>/dev/null; then
        PLAYER_CMD="mpv --fs --no-terminal"
    elif command -v totem &>/dev/null; then
        PLAYER_CMD="totem"
    elif command -v celluloid &>/dev/null; then
        PLAYER_CMD="celluloid"
    elif command -v xdg-open &>/dev/null; then
        PLAYER_CMD="xdg-open"
    else
        echo -e "\e[1;31m❌ কোনো ভিডিও প্লেয়ার পাওয়া যায়নি। VLC বা mpv ইন্সটল করুন।\e[0m"
        return 1
    fi

    # Direct file play support if target is a single video file
    if [ -f "$TARGET" ]; then
        echo -e "\e[1;35m🎬 Playing video:\e[0m $(basename "$TARGET")"
        # Zsh array split on spaces for multi-word player commands
        local -a player_args=("${(s/ /)PLAYER_CMD}")
        "${player_args[@]}" "$TARGET" >/dev/null 2>&1 &
        disown $! 2>/dev/null || true
        return 0
    fi

    # 🔍 Find Videos inside directory
    local DIR="$TARGET"
    local RAW_LIST
    RAW_LIST=$(find "$DIR" -type f \( -iname "*.mp4" -o -iname "*.mkv" -o -iname "*.avi" -o -iname "*.mov" -o -iname "*.webm" -o -iname "*.flv" -o -iname "*.m4v" \) 2>/dev/null | sort)

    [ -z "$RAW_LIST" ] && echo "❌ No videos found" && return 1

    # 🎨 Modern UI Header
    local HEADER_STR=$(printf "\e[1;36m%-5s \e[1;33m%-22s \e[1;35m%-s\e[0m" "IDX" "FOLDER" "VIDEO FILE NAME")

    local SELECTED_LINE
    SELECTED_LINE=$(echo "$RAW_LIST" | awk -F/ '{
        idx = NR;
        folder = $(NF-1);
        filename = $NF;
        folder_with_icon = "📁 " folder;
        if (length(filename) > 55) filename = substr(filename, 1, 52) "...";
        if (length(folder_with_icon) > 19) folder_with_icon = substr(folder_with_icon, 1, 16) "...";
        printf "\033[36m%02d   \033[33m%-22s \033[1;37m%s\033[0m\n", idx, folder_with_icon, filename
    }' | fzf \
        --ansi \
        --reverse \
        --height=70% \
        --border=double \
        --border-label=" 🎬 FANCYBASH VIDEO VAULT & PLAYER " \
        --border-label-pos=3 \
        --header="$HEADER_STR" \
        --header-first \
        --prompt="🔍 Filter: " \
        --pointer="▶ " \
        --color="bg+:#1e1e2e,fg+:white,hl:#f9e2af,hl+:#89dceb,header:#cba6f7,prompt:#89b4fa,pointer:#a6e3a1,border:#cba6f7,label:#f5c2e7")

    if [ -n "$SELECTED_LINE" ]; then
        local INDEX=$(echo "$SELECTED_LINE" | sed 's/\x1b\[[0-9;]*m//g' | awk '{print $1}')

        if ! [[ "$INDEX" =~ ^[0-9]+$ ]]; then
            echo "❌ Invalid selection"
            return 1
        fi

        local FULL_PATH=$(echo "$RAW_LIST" | sed -n "${INDEX}p")

        if [ -z "$FULL_PATH" ] || [ ! -f "$FULL_PATH" ]; then
            echo "❌ Video file not found"
            return 1
        fi

        echo -e "\e[1;92m▶ Playing:\e[0m $(basename "$FULL_PATH")"

        # ✅ Zsh-এর পারফেক্ট ডিটাচড মেথড (&!)
        ${=PLAYER_CMD} "$FULL_PATH" >/dev/null 2>&1 &!
    else
        echo "👋 Exit"
    fi
}

# ======================================================
#  📦universal clean
# ======================================================


unalias uc 2>/dev/null
unfunction uc 2>/dev/null
alias uc="fancybash uc"

function _legacy_uc {
    # ==============================
    # 🎨 COLORS & SAFETY
    # ==============================
    local RED="\033[0;31m" GREEN="\033[0;32m" YELLOW="\033[1;33m"
    local BLUE="\033[0;34m" CYAN="\033[0;36m" MAGENTA="\033[0;35m" NC="\033[0m"

    set -o pipefail
    # ZSH FIX: errexit DISABLED — it fires the EXIT trap before LOG_FILE is
    # initialized (leaving it empty), which causes: _log:1: no such file or directory:
    # set -o errexit
    # ZSH FIX: nounset causes issues with unset variables in Zsh
    # set -o nounset  # DISABLED for Zsh compatibility

    # ==============================
    # 🔐 TRAP & LOG SETUP
    # ==============================
    local LOG_FILE="/tmp/uc-optimizer-$(id -u).log"

    # Try to use HOME first, fallback to /tmp
    if [[ -d "${HOME:-}" ]]; then
        local cache_dir="${HOME}/.cache/uc-optimizer"
        if mkdir -p "$cache_dir" 2>/dev/null; then
            LOG_FILE="${cache_dir}/optimizer.log"
        fi
    fi

    # Security: Remove symlink if exists
    if [[ -L "$LOG_FILE" ]]; then
        rm -f "$LOG_FILE" 2>/dev/null || LOG_FILE="/dev/null"
    fi

    # Create log file
    if ! touch "$LOG_FILE" 2>/dev/null; then
        LOG_FILE="/dev/null"
    fi

    unalias _log 2>/dev/null
    function _log {
        # Guard: never write to empty path (happens if trap fires before LOG_FILE is set)
        [[ -z "${LOG_FILE:-}" ]] && return 0
        echo "$(date '+%Y-%m-%d %H:%M:%S') - $1" >> "$LOG_FILE" 2>/dev/null || true
    }

    unalias _trap_exit 2>/dev/null
    function _trap_exit {
        local exit_code=$?
        trap - INT TERM EXIT
        echo -e "\n${RED}⚠️  Interrupted (Exit code: $exit_code)${NC}" >&2
        _log "Session interrupted with code $exit_code"
        # ZSH FIX: use 'return' not 'exit' inside a function — 'exit' kills the
        # entire shell and triggers "you have running jobs" from process substitutions.
        return 130
    }
    trap _trap_exit INT TERM EXIT

    _log "Session started [UID: $(id -u), PID: $$]"

    # ==============================
    # 🐧 DISTRO DETECTION
    # ==============================
    unalias _detect_distro 2>/dev/null
    function _detect_distro {
        local d_id="unknown" d_name="Unknown Linux" key="" value=""

        if [[ -r /etc/os-release ]]; then
            # Source safely with restricted scope
            while IFS='=' read -r key value; do
                case "$key" in
                    ID) d_id="${value//\"/}" ;;
                    NAME) d_name="${value//\"/}" ;;
                esac
            done < /etc/os-release
            d_id=$(echo "$d_id" | tr '[:upper:]' '[:lower:]')
        elif [[ -r /etc/redhat-release ]]; then
            d_id="rhel"
            d_name="Red Hat Enterprise Linux"
        elif [[ -r /etc/arch-release ]]; then
            d_id="arch"
            d_name="Arch Linux"
        elif [[ -r /etc/alpine-release ]]; then
            d_id="alpine"
            d_name="Alpine Linux"
        fi

        # Validate ID format
        if [[ ! "$d_id" =~ ^[a-z0-9._-]+$ ]]; then
            d_id="unknown"
        fi

        printf '%s|%s' "$d_id" "$d_name"
    }

    local distro_info
    distro_info=$(_detect_distro)
    local DISTRO_ID="${distro_info%%|*}"
    local DISTRO_NAME="${distro_info##*|}"

    # Fallback if parsing failed
    [[ -z "$DISTRO_ID" ]] && DISTRO_ID="unknown"
    [[ -z "$DISTRO_NAME" ]] && DISTRO_NAME="Unknown"

    echo -e "${CYAN}╔════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║${NC}   🚀 System Optimizer v3.0         ${CYAN}║${NC}"
    echo -e "${CYAN}╚════════════════════════════════════╝${NC}"
    echo -e "${CYAN}OS:${NC} $DISTRO_NAME"
    echo -e "${CYAN}ID:${NC} $DISTRO_ID"
    echo ""

    _log "Distro detected: $DISTRO_NAME ($DISTRO_ID)"

    # ==============================
    # 🔒 PACKAGE MANAGER
    # ==============================
    unalias _sudo_check 2>/dev/null
    function _sudo_check {
        if [[ $EUID -eq 0 ]]; then
            return 0  # Already root
        fi
        if ! sudo -n true 2>/dev/null; then
            echo -e "${YELLOW}🔐 Sudo authentication required...${NC}"
            if ! sudo -v; then
                echo -e "${RED}❌ Sudo authentication failed${NC}" >&2
                return 1
            fi
        fi
        return 0
    }

    # Prompt for sudo authentication upfront
    _sudo_check

    unalias _pkg_install 2>/dev/null
    function _pkg_install {
        local pkgs=("$@")
        local pkg_manager=""

        _sudo_check || return 1

        # Detect package manager
        case "$DISTRO_ID" in
            ubuntu|debian|linuxmint|pop|elementary|zorin)
                pkg_manager="apt"
                ;;
            fedora|rhel|centos|rocky|almalinux|nobara)
                if command -v dnf >/dev/null 2>&1; then
                    pkg_manager="dnf"
                else
                    pkg_manager="yum"
                fi
                ;;
            arch|manjaro|endeavouros|garuda|cachyos)
                pkg_manager="pacman"
                ;;
            opensuse*|suse*|tumbleweed|leap)
                pkg_manager="zypper"
                ;;
            alpine)
                pkg_manager="apk"
                ;;
            void)
                pkg_manager="xbps"
                ;;
            gentoo)
                pkg_manager="emerge"
                ;;
            nixos)
                pkg_manager="nix"
                ;;
            *)
                echo -e "${RED}❌ Unsupported distro: $DISTRO_ID${NC}" >&2
                return 1
                ;;
        esac

        echo -e "${BLUE}📦 Installing: ${pkgs[*]}${NC}"

        local exit_code=0
        case "$pkg_manager" in
            apt)
                sudo apt-get update -qq && \
                sudo DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends "${pkgs[@]}"
                exit_code=$?
                ;;
            dnf)
                sudo dnf install -y --setopt=install_weak_deps=False "${pkgs[@]}"
                exit_code=$?
                ;;
            yum)
                sudo yum install -y "${pkgs[@]}"
                exit_code=$?
                ;;
            pacman)
                sudo pacman -Sy --noconfirm --needed "${pkgs[@]}"
                exit_code=$?
                ;;
            zypper)
                sudo zypper --non-interactive install --no-recommends "${pkgs[@]}"
                exit_code=$?
                ;;
            apk)
                sudo apk add --no-cache "${pkgs[@]}"
                exit_code=$?
                ;;
            xbps)
                sudo xbps-install -y "${pkgs[@]}"
                exit_code=$?
                ;;
            emerge)
                sudo emerge -av "${pkgs[@]}"
                exit_code=$?
                ;;
            nix)
                sudo nix-env -iA "${pkgs[@]/#/nixpkgs.}"
                exit_code=$?
                ;;
        esac

        return $exit_code
    }

    # ==============================
    # 🔄 PATH & ENV REFRESH
    # ==============================
    unalias _refresh_env 2>/dev/null
    function _refresh_env {
        # Reload PATH
        local paths=(
            "/usr/local/sbin"
            "/usr/local/bin"
            "/usr/sbin"
            "/usr/bin"
            "/sbin"
            "/bin"
            "${HOME}/.local/bin"
            "${HOME}/.bin"
            "${HOME}/.fzf/bin"
        )

        local new_path=""
        for p in "${paths[@]}"; do
            [[ -d "$p" ]] && new_path="${new_path:+$new_path:}$p"
        done

        # Preserve existing PATH entries not in our list
        local IFS=':'
        for p in ${=PATH}; do
            if [[ ":$new_path:" != *":$p:"* ]] && [[ -d "$p" ]]; then
                new_path="$new_path:$p"
            fi
        done
        unset IFS

        export PATH="$new_path"

        # Refresh hash table
        hash -r 2>/dev/null || true

        # Source bashrc if exists (for new completions)
        [[ -f "$HOME/.bashrc" ]] && source "$HOME/.bashrc" 2>/dev/null || true
    }

    # ==============================
    # 🛠️ TOOL INSTALLERS
    # ==============================
    unalias _install_fzf 2>/dev/null
    function _install_fzf {
        echo -e "${YELLOW}⚠️  fzf not found. Installing...${NC}"

        # Try package manager first
        local pkg_name="fzf"

        # Some distros have different names
        case "$DISTRO_ID" in
            alpine) pkg_name="fzf" ;;
        esac

        if _pkg_install "$pkg_name"; then
            _configure_fzf
            return 0
        fi

        # Fallback: Git installation
        echo -e "${YELLOW}📥 Package manager failed. Trying git install...${NC}"

        if ! command -v git >/dev/null 2>&1; then
            _pkg_install "git" || {
                echo -e "${RED}❌ git not available${NC}" >&2
                return 1
            }
        fi

        local fzf_dir="${HOME}/.fzf"
        rm -rf "$fzf_dir" 2>/dev/null || true

        if git clone --depth 1 https://github.com/junegunn/fzf.git "$fzf_dir" 2>/dev/null; then
            "$fzf_dir/install" --all --no-bash --no-fish --no-zsh 2>/dev/null || true
            _configure_fzf
            return 0
        fi

        echo -e "${RED}❌ fzf installation failed${NC}" >&2
        return 1
    }

    unalias _configure_fzf 2>/dev/null
    function _configure_fzf {
        echo -e "${BLUE}🔧 Configuring fzf...${NC}"

        _refresh_env

        # Verify fzf is available
        if ! command -v fzf >/dev/null 2>&1; then
            # Manual PATH addition
            [[ -f "${HOME}/.fzf/bin/fzf" ]] && export PATH="${HOME}/.fzf/bin:$PATH"
            [[ -f "/usr/bin/fzf" ]] && export PATH="/usr/bin:$PATH"
        fi

        # Setup shell integration
        local bashrc="${HOME}/.bashrc"
        local fzf_shell=""

        # Find fzf shell files
        for d in "/usr/share/doc/fzf/examples" "/usr/share/fzf" "/usr/share/fzf/shell" "${HOME}/.fzf/shell"; do
            [[ -f "$d/completion.bash" ]] && fzf_shell="$d" && break
        done

        if [[ -n "$fzf_shell" ]] && [[ -f "$bashrc" ]]; then
            # Add to bashrc if not present
            if ! grep -q "fzf" "$bashrc" 2>/dev/null; then
                {
                    echo ""
                    echo "# fzf configuration added by uc-optimizer"
                    echo "source $fzf_shell/completion.bash 2>/dev/null || true"
                    echo "source $fzf_shell/key-bindings.bash 2>/dev/null || true"
                } >> "$bashrc"
            fi
        fi

        # Load immediately
        [[ -n "$fzf_shell" ]] && source "$fzf_shell/completion.bash" 2>/dev/null || true
        [[ -n "$fzf_shell" ]] && source "$fzf_shell/key-bindings.bash" 2>/dev/null || true

        # Final verification
        if command -v fzf >/dev/null 2>&1; then
            echo -e "${GREEN}✅ fzf $(fzf --version | head -1) installed${NC}"
            return 0
        else
            echo -e "${RED}❌ fzf configuration incomplete${NC}" >&2
            return 1
        fi
    }

    unalias _install_sensors 2>/dev/null
    function _install_sensors {
        echo -e "${YELLOW}⚠️  sensors not found. Installing...${NC}"

        local pkg_name="lm-sensors"
        [[ "$DISTRO_ID" == "arch" || "$DISTRO_ID" == "manjaro" ]] && pkg_name="lm_sensors"
        [[ "$DISTRO_ID" == "alpine" ]] && pkg_name="lm-sensors"

        if ! _pkg_install "$pkg_name"; then
            echo -e "${RED}❌ lm-sensors installation failed${NC}" >&2
            return 1
        fi

        echo -e "${BLUE}🔧 Configuring sensors...${NC}"

        if ! command -v sensors-detect >/dev/null 2>&1; then
            echo -e "${YELLOW}⚠️  sensors-detect not found${NC}"
            return 0  # Partial success
        fi

        # Non-interactive configuration
        echo -e "${CYAN}🌡️  Detecting hardware sensors (this may take a moment)...${NC}"

        # ZSH FIX: C-style loop instead of {1..10}
        local answers=""
        local i
        for ((i=0; i<10; i++)); do
            answers="${answers}YES\n"
        done

        echo -e "$answers" | sudo sensors-detect --no-interactive 2>/dev/null || \
        echo -e "$answers" | sudo sensors-detect 2>/dev/null || true

        # Load common modules
        local modules=(coretemp nct6775 k10temp acpi_cpufreq it87)
        for mod in "${modules[@]}"; do
            sudo modprobe "$mod" 2>/dev/null || true
        done

        # Enable service
        if command -v systemctl >/dev/null 2>&1; then
            sudo systemctl enable --now lm-sensors 2>/dev/null || \
            sudo systemctl enable --now sensord 2>/dev/null || true
        fi

        # Generate sensors.conf if missing
        if [[ ! -f /etc/sensors3.conf ]] && [[ ! -f /etc/sensors.conf ]]; then
            sudo sensors -s 2>/dev/null || true
        fi

        # Test
        if sensors >/dev/null 2>&1; then
            echo -e "${GREEN}✅ sensors configured successfully${NC}"
        else
            echo -e "${YELLOW}⚠️  sensors configured but no sensors detected${NC}"
        fi

        return 0
    }

    unalias _install_zram 2>/dev/null
    function _install_zram {
        echo -e "${YELLOW}⚠️  zram tools not found. Installing...${NC}"

        local pkg_name="util-linux"
        local extra_pkgs=()

        case "$DISTRO_ID" in
            ubuntu|debian|linuxmint|pop)
                pkg_name="zram-tools"
                ;;
            fedora|rhel|centos|rocky|almalinux|nobara)
                pkg_name="zram-generator"
                ;;
            arch|manjaro|endeavouros)
                pkg_name="zram-generator"
                extra_pkgs=("util-linux")
                ;;
            alpine)
                pkg_name="zram-init"
                ;;
            opensuse*)
                pkg_name="systemd-zram-service"
                ;;
        esac

        if ! _pkg_install "$pkg_name" "${extra_pkgs[@]}"; then
            echo -e "${RED}❌ zram package installation failed${NC}" >&2
            return 1
        fi

        echo -e "${BLUE}🔧 Configuring zram...${NC}"

        # Check if zram already configured
        if [[ -e /dev/zram0 ]] && swapon -s 2>/dev/null | grep -q zram; then
            echo -e "${GREEN}✅ zram already active${NC}"
            return 0
        fi

        # Load module
        if ! lsmod 2>/dev/null | grep -q "^zram"; then
            sudo modprobe zram num_devices=1 2>/dev/null || {
                echo -e "${RED}❌ Cannot load zram module${NC}" >&2
                return 1
            }
        fi

        # Calculate size (50% of RAM)
        local mem_total zram_size
        mem_total=$(awk '/MemTotal/{print $2}' /proc/meminfo 2>/dev/null || echo "0")
        zram_size=$((mem_total * 512))  # KB to bytes / 2

        [[ "$zram_size" -lt 104857600 ]] && zram_size=536870912  # Minimum 512MB

        # Configure
        echo "$zram_size" | sudo tee /sys/block/zram0/disksize >/dev/null 2>/dev/null || {
            echo -e "${RED}❌ Cannot set zram size${NC}" >&2
            return 1
        }

        sudo mkswap /dev/zram0 >/dev/null 2>&1 || true
        sudo swapon /dev/zram0 -p 100 >/dev/null 2>&1 || {
            echo -e "${RED}❌ Cannot enable zram swap${NC}" >&2
            return 1
        }

        # Persistent config for systemd-based systems
        if [[ "$DISTRO_ID" == "arch" || "$DISTRO_ID" == "fedora" ]] && [[ -d /etc/systemd ]]; then
            local zram_conf="/etc/systemd/zram-generator.conf"
            if [[ ! -f "$zram_conf" ]]; then
                sudo tee "$zram_conf" >/dev/null <<'EOF'
[zram0]
zram-size = ram / 2
compression-algorithm = zstd
swap-priority = 100
EOF
            fi
        fi

        echo -e "${GREEN}✅ zram configured: $((zram_size / 1024 / 1024))MB${NC}"
        return 0
    }

    # ==============================
    # 🔍 DEPENDENCY CHECK
    # ==============================
    echo -e "${BLUE}🔍 Checking dependencies...${NC}"

    # ZSH FIX: Initialize arrays properly
    local missing_tools
    missing_tools=()
    local install_failed=0

    command -v fzf >/dev/null 2>&1 || missing_tools+=("fzf")
    command -v sensors >/dev/null 2>&1 || missing_tools+=("sensors")
    command -v zramctl >/dev/null 2>&1 || missing_tools+=("zram")

    if [[ ${#missing_tools[@]} -gt 0 ]]; then
        echo -e "${YELLOW}📋 Missing: ${missing_tools[*]}${NC}"
        echo -e "${CYAN}🚀 Installing...${NC}"
        echo ""

        for tool in "${missing_tools[@]}"; do
            case "$tool" in
                fzf)
                    if ! _install_fzf; then
                        install_failed=$((install_failed + 1))
                        echo -e "${RED}CRITICAL: fzf is required${NC}" >&2
                    fi
                    ;;
                sensors)
                    _install_sensors || install_failed=$((install_failed + 1))
                    ;;
                zram)
                    _install_zram || install_failed=$((install_failed + 1))
                    ;;
            esac
            echo ""
        done

        _refresh_env
    fi

    # Final verification
    local critical_fail=0
    if ! command -v fzf >/dev/null 2>&1; then
        echo -e "${RED}❌ CRITICAL: fzf not available${NC}" >&2
        critical_fail=1
    fi

    if ! command -v sensors >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠️  sensors not available (temp monitoring disabled)${NC}"
    fi

    if ! command -v zramctl >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠️  zramctl not available (zram monitoring disabled)${NC}"
    fi

    if [[ $critical_fail -eq 1 ]]; then
        echo -e "${RED}❌ Cannot continue without fzf${NC}" >&2
        _log "FAILED: Missing critical dependency fzf"
        return 1
    fi

    echo -e "${GREEN}✅ Dependencies ready!${NC}"
    _log "Dependencies satisfied"
    sleep 1
    clear

    # ==============================
    # 🔧 CORE FUNCTIONS
    # ==============================
    unalias _pkg_clean 2>/dev/null
    function _pkg_clean {
        _sudo_check || return 1
        case "$DISTRO_ID" in
            ubuntu|debian|linuxmint|pop|elementary)
                sudo apt-get autoremove --purge -y && sudo apt-get autoclean
                ;;
            fedora|rhel|centos|rocky|almalinux|nobara)
                if command -v dnf >/dev/null 2>&1; then
                    sudo dnf autoremove -y && sudo dnf clean all
                else
                    sudo yum autoremove -y && sudo yum clean all
                fi
                ;;
            arch|manjaro|endeavouros|garuda)
                sudo pacman -Sc --noconfirm
                command -v paccache >/dev/null 2>&1 && sudo paccache -r
                ;;
            opensuse*|suse*)
                sudo zypper clean
                ;;
            alpine)
                sudo apk cache clean
                ;;
            void)
                sudo xbps-remove -yo 2>/dev/null || true
                ;;
        esac
    }

    unalias _get_temp_zram 2>/dev/null
    function _get_temp_zram {
        local temp="N/A" zram_used="0"

        if command -v sensors >/dev/null 2>&1; then
            temp=$(sensors 2>/dev/null | awk '
                /°C/ {
                    gsub(/[+|°C]/, "", $2)
                    if ($2+0 > max && $2 ~ /^[0-9]+\.?[0-9]*$/) max=$2
                }
                END {
                    if (max > 0) printf "%.0f", max
                    else print "N/A"
                }
            ')
        fi

        if command -v zramctl >/dev/null 2>&1; then
            zram_used=$(zramctl --output=DISKSIZE,DATA --bytes 2>/dev/null | awk '
                NR>1 {
                    if ($1 ~ /^[0-9]+$/ && $1 > 0) {
                        disk += $1
                        data += $2
                    }
                }
                END {
                    if (disk > 0) printf "%.0f", (data/disk)*100
                    else print "0"
                }
            ')
        fi

        printf '%s %s' "${temp:-N/A}" "${zram_used:-0}"
    }

    unalias _get_free_kb 2>/dev/null
    function _get_free_kb {
        local avail
        avail=$(df -k / 2>/dev/null | awk 'NR==2 {print $4}')
        [[ "$avail" =~ ^[0-9]+$ ]] && echo "$avail" || echo "0"
    }

    unalias _format_size 2>/dev/null
    function _format_size {
        local kb=$1
        [[ "$kb" =~ ^[0-9]+$ ]] || { echo "0KB"; return; }

        local mb=$((kb / 1024))
        local gb=$((mb / 1024))

        if [[ $kb -lt 1024 ]]; then echo "${kb}KB"
        elif [[ $mb -lt 1024 ]]; then echo "${mb}MB"
        else echo "${gb}GB"
        fi
    }

    unalias _run_task 2>/dev/null
    function _run_task {
        local label=$1
        shift
        echo -ne "   ${GREEN}➤ $label...${NC} "
        if "$@" >/dev/null 2>&1; then
            echo -e "${GREEN}✅${NC}"
            return 0
        else
            echo -e "${RED}❌${NC}"
            return 1
        fi
    }

    # ==============================
    # 🧹 CLEANING FUNCTIONS
    # ==============================
    unalias _os_clean 2>/dev/null
    function _os_clean {
        echo -e "${BLUE}╔════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}        ⚡ OS CLEAN             ${BLUE}║${NC}"
        echo -e "${BLUE}╚════════════════════════════════╝${NC}"

        # ZSH FIX: Separate prompt and read (read -p doesn't work in zsh)
        local confirm=""
        echo -n "Proceed with OS cleanup? [y/N]: "
        read -r confirm
        [[ "$confirm" =~ ^[Yy]$ ]] || return 0

        _sudo_check || return 1

        local space_before
        space_before=$(_get_free_kb)

        echo -e "${CYAN}🗑️  Cleaning package cache...${NC}"
        _pkg_clean

        echo -e "${CYAN}📋 Vacuuming journals...${NC}"
        if command -v journalctl >/dev/null 2>&1; then
            sudo journalctl --vacuum-time=3d --quiet 2>/dev/null || true
        fi

        echo -e "${CYAN}🖼️  Cleaning thumbnails...${NC}"
        if [[ -d "$HOME/.cache/thumbnails" ]]; then
            find "$HOME/.cache/thumbnails" -type f -atime +7 -delete 2>/dev/null || true
        fi

        echo -e "${CYAN}🗑️  Emptying trash...${NC}"
        if command -v gio &>/dev/null; then
            gio trash --empty &>/dev/null || true
        fi
        rm -rf "$HOME/.local/share/Trash/files" "$HOME/.local/share/Trash/info" 2>/dev/null || true
        mkdir -p "$HOME/.local/share/Trash/files" "$HOME/.local/share/Trash/info" 2>/dev/null || true

        # Clean old logs
        sudo find /var/log -type f -name "*.old" -delete 2>/dev/null || true
        sudo find /var/log -type f -name "*.gz" -mtime +30 -delete 2>/dev/null || true

        # Developer cache cleaning (optional)
        local dev_confirm=""
        echo -n "Clean developer caches? (npm/bun/pip) [y/N]: "
        read -r dev_confirm
        if [[ "$dev_confirm" =~ ^[Yy]$ ]]; then
            echo -e "${CYAN}💻 Cleaning developer caches...${NC}"
            command -v npm &>/dev/null && npm cache clean --force 2>/dev/null || true
            command -v bun &>/dev/null && bun pm cache rm 2>/dev/null || true
            command -v pip &>/dev/null && pip cache purge 2>/dev/null || true
            command -v pip3 &>/dev/null && pip3 cache purge 2>/dev/null || true
            command -v pnpm &>/dev/null && pnpm store prune 2>/dev/null || true
            [[ -d "$HOME/.cache/pip" ]] && rm -rf "$HOME/.cache/pip"/* 2>/dev/null || true
            [[ -d "$HOME/.cache/go-build" ]] && rm -rf "$HOME/.cache/go-build"/* 2>/dev/null || true
            [[ -d "$HOME/.cargo/registry/cache" ]] && rm -rf "$HOME/.cargo/registry/cache"/* 2>/dev/null || true
            echo -e "   ${GREEN}✅ Developer caches cleared${NC}"
        fi

        local space_after freed_kb
        space_after=$(_get_free_kb)
        freed_kb=$(( space_after - space_before ))
        local freed_str=""
        [[ $freed_kb -gt 0 ]] && freed_str=" (freed: $(_format_size $freed_kb))"
        echo -e "${GREEN}✅ OS cleanup completed${freed_str}${NC}"
        _log "OS clean executed"
    }


    unalias _container_clean 2>/dev/null
    function _container_clean {
        echo -e "${BLUE}╔════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}      🐳 CONTAINER CLEAN        ${BLUE}║${NC}"
        echo -e "${BLUE}╚════════════════════════════════╝${NC}"

        # ZSH FIX: Separate prompt and read
        local confirm=""
        echo -n "Proceed with container cleanup? [y/N]: "
        read -r confirm
        [[ "$confirm" =~ ^[Yy]$ ]] || return 0

        local total_saved=0

        # Snap cleanup
        if command -v snap >/dev/null 2>&1; then
            echo -e "${CYAN}📦 Cleaning snap packages...${NC}"
            local start_space end_space saved
            start_space=$(_get_free_kb)

            local snap_output
            snap_output=$(snap list --all 2>/dev/null | awk '/disabled/{print $1, $3}')

            if [[ -n "$snap_output" ]]; then
                while read -r name rev; do
                    [[ "$name" =~ ^[a-z0-9-]+$ ]] || continue
                    [[ "$rev" =~ ^[0-9]+$ ]] || continue
                    echo -e "   ${YELLOW}Removing: $name (rev $rev)${NC}"
                    sudo snap remove --revision="$rev" "$name" 2>/dev/null || true
                done <<< "$snap_output"
            fi

            sudo rm -rf /var/lib/snapd/cache/* 2>/dev/null || true

            end_space=$(_get_free_kb)
            saved=$(( (end_space - start_space) * 1024 ))
            [[ $saved -gt 0 ]] && total_saved=$((total_saved + saved))
            echo -e "   ${GREEN}Snap saved: $(_format_size $((saved / 1024)))${NC}"
        fi

        # Flatpak cleanup
        if command -v flatpak >/dev/null 2>&1; then
            echo -e "${CYAN}📦 Cleaning flatpak...${NC}"
            local start_space=$(_get_free_kb)

            flatpak uninstall --unused -y 2>/dev/null || true
            flatpak repair 2>/dev/null || true

            local end_space=$(_get_free_kb)
            local saved=$(( (end_space - start_space) * 1024 ))
            [[ $saved -gt 0 ]] && total_saved=$((total_saved + saved))
            echo -e "   ${GREEN}Flatpak saved: $(_format_size $((saved / 1024)))${NC}"
        fi

        # Docker cleanup
        if command -v docker >/dev/null 2>&1; then
            echo -e "${CYAN}🐳 Cleaning docker...${NC}"
            if sudo docker info >/dev/null 2>&1; then
                local start_space=$(_get_free_kb)

                docker system prune -a --volumes -f 2>/dev/null || true

                local end_space=$(_get_free_kb)
                local saved=$(( (end_space - start_space) * 1024 ))
                [[ $saved -gt 0 ]] && total_saved=$((total_saved + saved))
                echo -e "   ${GREEN}Docker saved: $(_format_size $((saved / 1024)))${NC}"
            else
                echo -e "   ${YELLOW}Docker daemon not running${NC}"
            fi
        fi

        # Podman cleanup
        if command -v podman >/dev/null 2>&1; then
            echo -e "${CYAN}🦭 Cleaning podman...${NC}"
            podman system prune -f 2>/dev/null || true
            echo -e "   ${GREEN}Podman cleaned${NC}"
        fi

        echo -e "${GREEN}✅ Total saved: $(_format_size $((total_saved / 1024)))${NC}"
        _log "Container clean executed"
    }

    unalias _fix_links 2>/dev/null
    function _fix_links {
        echo -e "${BLUE}╔════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}        🔗 FIX LINKS            ${BLUE}║${NC}"
        echo -e "${BLUE}╚════════════════════════════════╝${NC}"

        # ZSH FIX: Separate prompt and read
        local confirm=""
        echo -n "Remove broken symlinks in $HOME? [y/N]: "
        read -r confirm
        [[ "$confirm" =~ ^[Yy]$ ]] || return 0

        local count=0
        while IFS= read -r -d '' link; do
            rm -f "$link" 2>/dev/null && ((count++)) || true
        done < <(find "$HOME" -xdev -maxdepth 3 -xtype l -print0 2>/dev/null)

        echo -e "${GREEN}✅ Removed $count broken symlinks${NC}"
        _log "Fixed $count broken links"
    }

    unalias _orphan_engine 2>/dev/null
    function _orphan_engine {
        echo -e "${BLUE}╔════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}       ⚡ KERNEL CLEAN           ${BLUE}║${NC}"
        echo -e "${BLUE}╚════════════════════════════════╝${NC}"

        local current_kernel
        current_kernel=$(uname -r)
        echo -e "Current kernel: ${GREEN}$current_kernel${NC}"

        _sudo_check || return 1

        case "$DISTRO_ID" in
            ubuntu|debian|linuxmint|pop|elementary)
                local kernels=()
                while IFS= read -r line; do
                    kernels+=("$line")
                done < <(dpkg-query -W -f='${Package}\n' 2>/dev/null | grep -E '^linux-image-[0-9]' | sort -V)

                if [[ ${#kernels[@]} -le 2 ]]; then
                    echo -e "${GREEN}Only ${#kernels[@]} kernels installed, skipping${NC}"
                    return 0
                fi

                local keep1="${kernels[-1]}"
                local keep2="${kernels[-2]}"
                echo -e "Keeping: ${GREEN}$keep1${NC} and ${GREEN}$keep2${NC}"

                local to_remove=()
                for k in "${kernels[@]}"; do
                    if [[ "$k" != "$keep1" && "$k" != "$keep2" && "$k" != *"$current_kernel"* ]]; then
                        to_remove+=("$k")
                    fi
                done

                if [[ ${#to_remove[@]} -gt 0 ]]; then
                    echo -e "${YELLOW}Will remove: ${to_remove[*]}${NC}"
                    # ZSH FIX: Separate prompt and read
                    local confirm2=""
                    echo -n "Confirm? [y/N]: "
                    read -r confirm2
                    if [[ "$confirm2" =~ ^[Yy]$ ]]; then
                        sudo apt-get purge -y "${to_remove[@]}" 2>/dev/null || true
                        sudo apt-get autoremove -y 2>/dev/null || true
                    fi
                fi

                # Remove residual configs
                local residual
                residual=$(dpkg-query -W -f='${Package}\n' 2>/dev/null | grep '^rc' || true)
                if [[ -n "$residual" ]]; then
                    echo "$residual" | xargs -r sudo dpkg --purge 2>/dev/null || true
                fi
                ;;

            fedora|rhel|centos|rocky|almalinux|nobara)
                if command -v dnf >/dev/null 2>&1; then
                    echo -e "${CYAN}Removing old kernels...${NC}"
                    sudo dnf remove --oldinstallonly --setopt installonly_limit=2 -y 2>/dev/null || {
                        # Fallback manual method
                        local old_kernels
                        old_kernels=$(dnf repoquery --installonly --latest-limit=-2 -q 2>/dev/null | grep -v "$current_kernel" || true)
                        if [[ -n "$old_kernels" ]]; then
                            echo "$old_kernels" | xargs -r sudo dnf remove -y 2>/dev/null || true
                        fi
                    }
                fi
                ;;

            arch|manjaro|endeavouros|garuda|cachyos)
                echo -e "${CYAN}Removing orphan packages...${NC}"
                local orphans
                orphans=$(pacman -Qtdq 2>/dev/null || true)
                if [[ -n "$orphans" ]]; then
                    echo "$orphans" | sudo pacman -Rns --noconfirm - 2>/dev/null || true
                fi

                if command -v paccache >/dev/null 2>&1; then
                    echo -e "${CYAN}Cleaning package cache...${NC}"
                    sudo paccache -rk2 2>/dev/null || true
                fi
                ;;

            opensuse*)
                echo -e "${CYAN}Cleaning kernels...${NC}"
                sudo zypper purge-kernels 2>/dev/null || true
                ;;

            *)
                echo -e "${YELLOW}⚠️  Kernel cleanup not implemented for $DISTRO_ID${NC}"
                ;;
        esac

        echo -e "${GREEN}✅ Kernel cleanup completed${NC}"
        _log "Kernel clean executed"
    }

    unalias _ai_mode 2>/dev/null
    function _ai_mode {
        echo -e "${BLUE}╔════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}       🤖 AI DIAGNOSTICS        ${BLUE}║${NC}"
        echo -e "${BLUE}╚════════════════════════════════╝${NC}"

        # Get system stats
        local mem_info
        mem_info=$(free -k 2>/dev/null | awk '/^Mem:/{printf "%.0f %.0f %.0f", $2, $3, ($3/$2)*100}')
        if [[ -z "$mem_info" ]]; then
            mem_info="0 0 0"
        fi
        local mem_total mem_used mem_pct
        read -r mem_total mem_used mem_pct <<< "$mem_info"
        mem_total=${mem_total:-0}
        mem_used=${mem_used:-0}
        mem_pct=${mem_pct:-0}
        # Sanitize: strip anything that isn't a digit (handles floats / empty)
        mem_pct=$(( ${mem_pct//[^0-9]/} + 0 ))

        local disk_info
        disk_info=$(df -k / 2>/dev/null | awk 'NR==2{print $3, $4, $5}')
        if [[ -z "$disk_info" ]]; then
            disk_info="0 0 0%"
        fi
        local disk_used disk_avail disk_pct
        read -r disk_used disk_avail disk_pct <<< "$disk_info"
        disk_used=${disk_used:-0}
        disk_avail=${disk_avail:-0}
        disk_pct=${disk_pct:-0%}
        disk_pct="${disk_pct//%/}"
        # Sanitize: strip non-digits so arithmetic is always safe
        disk_pct=$(( ${disk_pct//[^0-9]/} + 0 ))

        local temp="N/A" zram_used="0"
        read -r temp zram_used < <(_get_temp_zram)
        temp=${temp:-N/A}
        zram_used=${zram_used:-0}

        # Display
        echo -e "📊 ${CYAN}System Status:${NC}"
        echo -e "   Memory: ${mem_pct}% used ($((mem_used/1024))MB / $((mem_total/1024))MB)"
        echo -e "   Disk:   ${disk_pct}% used ($(_format_size $((disk_used/1024))) / $(_format_size $(( (disk_used+disk_avail)/1024 ))))"
        echo -e "   Temp:   ${temp}°C"
        echo -e "   ZRAM:   ${zram_used}% used"

        # AI Recommendations
        local actions=()

        if [[ "$mem_pct" -gt 85 ]]; then
            echo -e "\n${YELLOW}⚠️  HIGH MEMORY USAGE${NC}"
            # ZSH FIX: Separate prompt and read
            local confirm=""
            echo -n "   Drop caches? [y/N]: "
            read -r confirm
            if [[ "$confirm" =~ ^[Yy]$ ]]; then
                _sudo_check && {
                    sudo sync
                    echo 3 | sudo tee /proc/sys/vm/drop_caches >/dev/null 2>&1 || true
                    echo -e "   ${GREEN}✅ Caches dropped${NC}"
                }
            fi
        fi

        if [[ "$disk_pct" -gt 90 ]]; then
            echo -e "\n${RED}🚨 CRITICAL DISK USAGE${NC}"
            _os_clean
        elif [[ "$disk_pct" -gt 80 ]]; then
            echo -e "\n${YELLOW}⚠️  High disk usage${NC}"
            # ZSH FIX: Separate prompt and read
            local confirm=""
            echo -n "   Run OS cleanup? [y/N]: "
            read -r confirm
            [[ "$confirm" =~ ^[Yy]$ ]] && _os_clean
        fi

        # ZSH FIX: Split into two checks — combining != and -gt in one [[...]] can
        # trigger "bad math expression" when temp is "N/A" or a non-integer string.
        if [[ "$temp" != "N/A" ]]; then
            local temp_int=$(( ${temp//[^0-9]/} + 0 ))
            if [[ $temp_int -gt 80 ]]; then
                echo -e "\n${RED}🌡️  HIGH TEMPERATURE${NC}"
                echo -e "   ${YELLOW}Check cooling system!${NC}"
            fi
        fi

        _log "AI mode executed"
    }

    unalias _report 2>/dev/null
    function _report {
        echo -e "${BLUE}╔════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}       📊 SYSTEM REPORT         ${BLUE}║${NC}"
        echo -e "${BLUE}╚════════════════════════════════╝${NC}"

        echo -e "${CYAN}OS Information:${NC}"
        echo -e "   Distribution: $DISTRO_NAME"
        echo -e "   Kernel:       $(uname -r)"
        echo -e "   Architecture: $(uname -m)"
        echo -e "   Hostname:     $(hostname)"

        local uptime_str
        uptime_str=$(uptime -p 2>/dev/null || uptime | sed 's/.*up \([^,]*\),.*/\1/')
        echo -e "   Uptime:       $uptime_str"

        echo -e "\n${CYAN}Resources:${NC}"

        # Memory
        free -h | awk '/Mem:/{printf "   Memory:       %s / %s (%.1f%% used)\n", $3, $2, ($3/$2)*100}'

        # Disk
        df -h / | awk 'NR==2{printf "   Disk (/):     %s / %s (%s used)\n", $3, $2, $5}'

        # Temperature & ZRAM
        local temp zram_used
        read -r temp zram_used < <(_get_temp_zram)
        echo -e "   Temperature:  ${temp}°C"
        echo -e "   ZRAM Usage:   ${zram_used}%"

        # CPU
        echo -e "   CPU:          $(nproc) cores"
        if [[ -r /proc/cpuinfo ]]; then
            local cpu_model
            cpu_model=$(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2 | xargs | cut -c1-40)
            echo -e "   Model:        $cpu_model"
        fi

        # Load average
        echo -e "   Load:         $(cut -d' ' -f1-3 /proc/loadavg 2>/dev/null || echo 'N/A')"

        echo -e "\n${CYAN}Top Processes (by memory):${NC}"
        ps aux --sort=-%mem 2>/dev/null | head -6 | tail -5 | awk '{printf "   %-10s %5s%% %s\n", $1, $4, $11}'

        _log "Report generated"
    }

    # ==============================
    # 🗑️ APPIMAGE ARTIFACT CLEANUP
    # ==============================
    unalias _appimage_cleanup 2>/dev/null
    function _appimage_cleanup {
        echo -e "${BLUE}╔════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}    🗑️  APPIMAGE ARTIFACT CLEAN  ${BLUE}║${NC}"
        echo -e "${BLUE}╚════════════════════════════════╝${NC}"
        echo -e "${CYAN}Scanning for orphaned .desktop & icon files...${NC}\n"

        local desktop_dir="$HOME/.local/share/applications"
        local icon_dir="$HOME/.local/share/icons"
        local autostart_dir="$HOME/.config/autostart"
        local found_count=0
        local removed_count=0
        local -a orphans=()

        # Find .desktop files referencing missing AppImage paths
        if [[ -d "$desktop_dir" ]]; then
            while IFS= read -r -d '' desktop_file; do
                # Extract Exec line
                local exec_line
                exec_line=$(grep -i '^Exec=' "$desktop_file" 2>/dev/null | head -1 | cut -d= -f2- | awk '{print $1}')
                [[ -z "$exec_line" ]] && continue

                # Check if it references an AppImage that no longer exists
                if [[ "$exec_line" == *.AppImage* ]] || [[ "$exec_line" == *.appimage* ]]; then
                    # Strip any args to get the binary path
                    local bin_path
                    bin_path=$(echo "$exec_line" | sed 's/ .*//')
                    if [[ ! -f "$bin_path" ]]; then
                        orphans+=("$desktop_file")
                        (( found_count++ )) || true
                        echo -e "   ${YELLOW}Orphan: $(basename "$desktop_file")${NC}"
                    fi
                fi
            done < <(find "$desktop_dir" -name "*.desktop" -print0 2>/dev/null)
        fi

        # Also check autostart for orphaned AppImage entries
        if [[ -d "$autostart_dir" ]]; then
            while IFS= read -r -d '' desktop_file; do
                local exec_line
                exec_line=$(grep -i '^Exec=' "$desktop_file" 2>/dev/null | head -1 | cut -d= -f2- | awk '{print $1}')
                [[ -z "$exec_line" ]] && continue
                if [[ "$exec_line" == *.AppImage* ]] || [[ "$exec_line" == *.appimage* ]]; then
                    local bin_path
                    bin_path=$(echo "$exec_line" | sed 's/ .*//')
                    if [[ ! -f "$bin_path" ]]; then
                        orphans+=("$desktop_file")
                        (( found_count++ )) || true
                        echo -e "   ${YELLOW}Orphan (autostart): $(basename "$desktop_file")${NC}"
                    fi
                fi
            done < <(find "$autostart_dir" -name "*.desktop" -print0 2>/dev/null)
        fi

        if [[ $found_count -eq 0 ]]; then
            echo -e "${GREEN}✅ No orphaned AppImage artifacts found.${NC}"
            _log "AppImage cleanup: nothing to remove"
            return 0
        fi

        echo ""
        local confirm=""
        echo -n "Remove $found_count orphaned file(s)? [y/N]: "
        read -r confirm
        [[ "$confirm" =~ ^[Yy]$ ]] || return 0

        for f in "${orphans[@]}"; do
            rm -f "$f" 2>/dev/null && (( removed_count++ )) || true
            echo -e "   ${RED}Removed: $(basename "$f")${NC}"
        done

        # Clean up orphaned appimagekit icons
        if [[ -d "$icon_dir" ]]; then
            local icon_count=0
            while IFS= read -r -d '' icon_file; do
                rm -f "$icon_file" 2>/dev/null && (( icon_count++ )) || true
            done < <(find "$icon_dir" -name "appimagekit_*" -print0 2>/dev/null)
            [[ $icon_count -gt 0 ]] && echo -e "   ${RED}Removed $icon_count orphaned icon(s)${NC}"
        fi

        # Refresh desktop database
        command -v update-desktop-database &>/dev/null && \
            update-desktop-database "$desktop_dir" 2>/dev/null || true

        echo -e "${GREEN}✅ Removed $removed_count orphaned AppImage artifact(s).${NC}"
        _log "AppImage cleanup: removed $removed_count files"
    }

    # ==============================
    # 📋 INTERACTIVE MENU
    # ==============================
    unalias _show_menu 2>/dev/null
    function _show_menu {
        # ZSH FIX: Initialize array properly
        local choices
        choices=(
            "🚀  Full System Boost"
            "🤖  AI Smart Cleanup"
            "⚡  OS Clean"
            "🗑️  AppImage Artifact Clean"
            "🐳  Container Clean"
            "🔗  Fix Broken Links"
            "⚡  Kernel Clean"
            "📊  System Report"
            "❌  Exit"
        )

        local choice
        choice=$(printf "%s\n" "${choices[@]}" | \
            fzf --height=70% \
                --layout=reverse \
                --border=rounded \
                --border-label=" System Optimizer v3.0 " \
                --prompt="[$DISTRO_ID] ❯ " \
                --header="Use ↑↓ to navigate, Enter to select, Ctrl+C to quit" \
                --pointer="▶" \
                --marker="✓" \
                --ansi \
                --no-info \
                --cycle)

        [[ -z "$choice" ]] && return 1

        # Extract action (robust emoji & padding removal)
        local action
        action=$(echo "$choice" | sed 's/^[^[:alnum:]]*[[:space:]]*//')

        case "$action" in
            "Full System Boost")
                _os_clean
                _appimage_cleanup
                _container_clean
                _fix_links
                _orphan_engine
                _ai_mode
                _report
                ;;
            "AI Smart Cleanup") _ai_mode ;;
            "OS Clean") _os_clean ;;
            "AppImage Artifact Clean") _appimage_cleanup ;;
            "Container Clean") _container_clean ;;
            "Fix Broken Links") _fix_links ;;
            "Kernel Clean") _orphan_engine ;;
            "System Report") _report ;;
            "Exit") return 0 ;;
            *)
                echo -e "${RED}Unknown option: $action${NC}" >&2
                return 1
                ;;
        esac

        return 0
    }

    # ==============================
    # 🎯 MAIN EXECUTION
    # ==============================
    local menu_result=0

    while true; do
        echo ""
        if ! _show_menu; then
            menu_result=1
            break
        fi

        echo ""
        # ZSH FIX: Separate prompt and read
        echo -n "Press Enter to continue..."
        read -r dummy </dev/tty
        clear
    done

    trap - INT TERM EXIT
    _log "Session ended (result: $menu_result)"
    echo -e "${GREEN}👋 Goodbye!${NC}"

    return $menu_result
}



# ======================================================
#  📦 runtime install
# ======================================================



unalias rt 2>/dev/null
function rt {
    # ১. fzf চেক এবং অটো-ইন্সটলেশন
    if ! command -v fzf &> /dev/null; then
        echo "🔍 fzf খুঁজে পাওয়া যায়নি। ইন্সটল করা হচ্ছে..."

        if [[ "$OSTYPE" == "linux-gnu"* ]]; then
            # Linux (Debian/Ubuntu) এর জন্য
            sudo apt update && sudo apt install fzf -y
        elif [[ "$OSTYPE" == "darwin"* ]]; then
            # macOS এর জন্য (Homebrew প্রয়োজন)
            if command -v brew &> /dev/null; then
                brew install fzf
            else
                echo "❌ Error: Homebrew পাওয়া যায়নি। অনুগ্রহ করে fzf ম্যানুয়ালি ইন্সটল করুন।"
                return 1
            fi
        else
            echo "❌ দুঃখিত, আপনার অপারেটিং সিস্টেমটি অটো-ইন্সটলেশন সাপোর্ট করছে না।"
            return 1
        fi

        echo "✅ fzf ইন্সটলেশন সম্পন্ন হয়েছে!"
    fi

    # ২. মেনু অপশন
    local options=() selected=""
    options=(
        "NVM (Node Version Manager)"
        "Node.js (LTS Version)"
        "Bun (Fast JS Runtime)"
        "Deno (Secure JS Runtime)"
    )

    selected=$(printf "%s\n" "${options[@]}" | fzf \
        --header="🚀 Ultimate Tool Installer (q to Exit)" \
        --reverse --height=40% --border --bind 'q:abort')

    # ৩. সিলেকশন চেক
    if [ $? -ne 0 ] || [ -z "$selected" ]; then
        echo "👋 বিদায়!"
        return 0
    fi

    # ৪. টুল ইন্সটলেশন লজিক
    case "$selected" in
        "NVM (Node Version Manager)")
            if [ -d "$HOME/.nvm" ]; then
                echo "✅ NVM আগে থেকেই আছে।"
            else
                curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
                export NVM_DIR="$HOME/.nvm"
                [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
            fi
            ;;
        "Node.js (LTS Version)")
            export NVM_DIR="$HOME/.nvm"
            [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
            if command -v nvm &> /dev/null; then
                nvm install --lts && nvm use --lts
            else
                echo "❌ আগে NVM ইন্সটল করুন!"
            fi
            ;;
        "Bun (Fast JS Runtime)")
            command -v bun &> /dev/null && echo "✅ Bun আছে: $(bun -v)" || (curl -fsSL https://bun.sh/install | bash && export PATH="$HOME/.bun/bin:$PATH")
            ;;
        "Deno (Secure JS Runtime)")
            command -v deno &> /dev/null && echo "✅ Deno আছে: $(deno -v)" || (curl -fsSL https://deno.land/x/install/install.sh | sh && export PATH="$HOME/.deno/bin:$PATH")
            ;;
    esac
}


# =========================================
# Ultimate Smart PC Optimizer (v5.1 - Clean UI)
# =========================================


unalias ut 2>/dev/null
function ut {
    # ===== 🎨 UI PALETTE =====
    local RED='\033[1;31m' GREEN='\033[1;32m' YELLOW='\033[1;33m'
    local BLUE='\033[1;34m' PURPLE='\033[1;35m' CYAN='\033[1;36m'
    local WHITE='\033[1;37m' BOLD='\033[1m' DIM='\033[2m' NC='\033[0m'
    local LOGFILE="/tmp/pcop_$(whoami)_$$.log"
    : > "$LOGFILE"

    # ===== 🖥️ DISTRO DETECTION =====
    local DISTRO_ID="" PKG_MANAGER="" PKG_INSTALL="" PKG_QUERY=""
    local SERVICE_CMD="systemctl"

    unalias detect_distro 2>/dev/null
    function detect_distro {
        if [[ -f /etc/os-release ]]; then
            source /etc/os-release
            DISTRO_ID=$(echo "$ID" | tr '[:upper:]' '[:lower:]')
        else
            echo -e "${RED}❌ Cannot detect distribution${NC}" && return 1
        fi

        case "$DISTRO_ID" in
            ubuntu|deepin|debian|linuxmint|pop|elementary|zorin|kali|parrot)
                PKG_MANAGER="apt"
                PKG_INSTALL="sudo apt install -y"
                PKG_QUERY="dpkg-query -W -f='${Status}'"
                ;;
            fedora|rhel|centos|rocky|almalinux|nobara)
                PKG_MANAGER="dnf"
                [[ "$DISTRO_ID" == "centos" ]] && [[ -z "$(command -v dnf)" ]] && PKG_MANAGER="yum"
                PKG_INSTALL="sudo $PKG_MANAGER install -y"
                PKG_QUERY="rpm -q"
                ;;
            arch|manjaro|endeavouros|garuda|cachyos|artix)
                PKG_MANAGER="pacman"
                PKG_INSTALL="sudo pacman -S --noconfirm --needed"
                PKG_QUERY="pacman -Q"
                ;;
            opensuse*|suse*)
                PKG_MANAGER="zypper"
                PKG_INSTALL="sudo zypper install -y"
                PKG_QUERY="rpm -q"
                ;;
            alpine)
                PKG_MANAGER="apk"
                PKG_INSTALL="sudo apk add"
                PKG_QUERY="apk info -e"
                SERVICE_CMD="rc-service"
                ;;
            void)
                PKG_MANAGER="xbps"
                PKG_INSTALL="sudo xbps-install -y"
                PKG_QUERY="xbps-query"
                SERVICE_CMD="sv"
                ;;
            *)
                echo -e "${YELLOW}⚠️ Unknown distro. Trying apt...${NC}"
                PKG_MANAGER="apt"
                PKG_INSTALL="sudo apt install -y"
                PKG_QUERY="dpkg-query -W -f='${Status}'"
                ;;
        esac
    }

    detect_distro || return 1
    echo -e "${CYAN} 🖥️  Detected: ${BOLD}${DISTRO_ID}${NC} | Package Manager: ${BOLD}${PKG_MANAGER}${NC}"

    # ===== ⚙️ FZF CHECK =====
    unalias install_fzf_universal 2>/dev/null
    function install_fzf_universal {
        echo -e "${YELLOW}📦 Installing fzf...${NC}"
        case "$PKG_MANAGER" in
            "apt") sudo apt update -y && sudo apt install -y fzf ;;
            "dnf"|"yum") sudo $PKG_MANAGER install -y fzf ;;
            "pacman") sudo pacman -S --noconfirm fzf ;;
            "zypper") sudo zypper install -y fzf ;;
            "apk") sudo apk add fzf ;;
            "xbps") sudo xbps-install -y fzf ;;
            *)
                local FZF_VERSION=$(curl -s https://api.github.com/repos/junegunn/fzf/releases/latest | grep -oP '"tag_name": "\K(.*)(?=")' || echo "0.54.0")
                curl -Lo /tmp/fzf.tar.gz "https://github.com/junegunn/fzf/releases/download/${FZF_VERSION}/fzf-${FZF_VERSION}-linux_amd64.tar.gz"
                tar -xzf /tmp/fzf.tar.gz -C /tmp && sudo mv /tmp/fzf /usr/local/bin/
                rm -f /tmp/fzf.tar.gz
                ;;
        esac
        rehash 2>/dev/null || hash -r 2>/dev/null || true
    }

    local FZF_CMD=""
    FZF_CMD=$(whence -p fzf 2>/dev/null)
    [[ -z "$FZF_CMD" ]] && FZF_CMD=$(command -v fzf 2>/dev/null)
    [[ -z "$FZF_CMD" ]] && FZF_CMD=$(command -v fzf 2>/dev/null)

    if [[ -z "$FZF_CMD" ]]; then
        install_fzf_universal
        FZF_CMD=$(whence -p fzf 2>/dev/null)
        [[ -z "$FZF_CMD" ]] && FZF_CMD=$(command -v fzf 2>/dev/null)
        [[ -z "$FZF_CMD" ]] && FZF_CMD=$(command -v fzf 2>/dev/null)
    fi

    if [[ -z "$FZF_CMD" ]] || [[ ! -x "$FZF_CMD" ]]; then
        echo -e "${RED}❌ fzf installation failed${NC}"
        return 1
    fi

    # ===== 📦 PACKAGE DATABASE =====
    typeset -A PKG_MAP
    PKG_MAP[zram-tools]="zram-tools|zram-generator-defaults|zram-generator|systemd-zram-service|zram-tools|zramctl"
    PKG_MAP[earlyoom]="earlyoom|earlyoom|earlyoom|earlyoom|earlyoom|earlyoom"
    PKG_MAP[htop]="htop|htop|htop|htop|htop|htop"
    PKG_MAP[btop]="btop|btop|btop|btop|btop|btop"
    PKG_MAP[ncdu]="ncdu|ncdu|ncdu|ncdu|ncdu|ncdu"
    PKG_MAP[gdu]="gdu|gdu-disk-usage-analyzer|gdu|gdu|gdu|gdu"
    PKG_MAP[duf]="duf|duf|duf|duf|duf|duf"
    PKG_MAP[dust]="dust|dust|dust|du-dust|dust|dust"
    PKG_MAP[bleachbit]="bleachbit|bleachbit|bleachbit|bleachbit|bleachbit|bleachbit"
    PKG_MAP[ufw]="ufw|ufw|ufw|ufw|ufw|ufw"
    PKG_MAP[fail2ban]="fail2ban|fail2ban|fail2ban|fail2ban|fail2ban|fail2ban"
    PKG_MAP[rkhunter]="rkhunter|rkhunter|rkhunter|rkhunter|rkhunter|rkhunter"
    PKG_MAP[lynis]="lynis|lynis|lynis|lynis|lynis|lynis"
    PKG_MAP[clamav]="clamav|clamav|clamav|clamav|clamav|clamav"
    PKG_MAP[firejail]="firejail|firejail|firejail|firejail|firejail|firejail"
    PKG_MAP[gnupg]="gnupg2|gnupg2|gnupg|gpg2|gnupg|gnupg"
    PKG_MAP[speedtest-cli]="speedtest-cli|speedtest-cli|speedtest-cli|speedtest|speedtest-cli|speedtest-cli"
    PKG_MAP[vnstat]="vnstat|vnstat|vnstat|vnstat|vnstat|vnstat"
    PKG_MAP[nmap]="nmap|nmap|nmap|nmap|nmap|nmap"
    PKG_MAP[iftop]="iftop|iftop|iftop|iftop|iftop|iftop"
    PKG_MAP[nload]="nload|nload|nload|nload|nload|nload"
    PKG_MAP[nethogs]="nethogs|nethogs|nethogs|nethogs|nethogs|nethogs"
    PKG_MAP[curl]="curl|curl|curl|curl|curl|curl"
    PKG_MAP[wget]="wget|wget|wget|wget|wget|wget"
    PKG_MAP[aria2]="aria2|aria2|aria2|aria2|aria2|aria2"
    PKG_MAP[wireguard]="wireguard|wireguard-tools|wireguard-tools|wireguard-tools|wireguard-tools|wireguard"
    PKG_MAP[dog]="dog|dog|dog|dog|dog|dog"
    PKG_MAP[mtr-tiny]="mtr-tiny|mtr|mtr|mtr|mtr|mtr"
    PKG_MAP[tcpdump]="tcpdump|tcpdump|tcpdump|tcpdump|tcpdump|tcpdump"
    PKG_MAP[git]="git|git|git|git|git|git"
    PKG_MAP[docker.io]="docker.io|docker|docker|docker|docker|docker"
    PKG_MAP[docker-compose]="docker-compose|docker-compose|docker-compose|docker-compose|docker-compose|docker-compose"
    PKG_MAP[build-essential]="build-essential|gcc-c++|base-devel|patterns-devel-base-devel|build-base|base-devel"
    PKG_MAP[micro]="micro|micro|micro|micro|micro|micro"
    PKG_MAP[neovim]="neovim|neovim|neovim|neovim|neovim|neovim"
    PKG_MAP[tmux]="tmux|tmux|tmux|tmux|tmux|tmux"
    PKG_MAP[screen]="screen|screen|screen|screen|screen|screen"
    PKG_MAP[python3-pip]="python3-pip|python3-pip|python-pip|python3-pip|py3-pip|python3-pip"
    PKG_MAP[nodejs]="nodejs|nodejs|nodejs|nodejs|nodejs|nodejs"
    PKG_MAP[npm]="npm|npm|npm|npm|npm|npm"
    PKG_MAP[golang-go]="golang-go|golang|go|go|go|go"
    PKG_MAP[rsync]="rsync|rsync|rsync|rsync|rsync|rsync"
    PKG_MAP[jq]="jq|jq|jq|jq|jq|jq"
    PKG_MAP[yq]="yq|yq|yq|yq|yq|yq"
    PKG_MAP[bat]="bat|bat|bat|bat|bat|bat"
    PKG_MAP[eza]="eza|eza|eza|eza|eza|eza"
    PKG_MAP[ripgrep]="ripgrep|ripgrep|ripgrep|ripgrep|ripgrep|ripgrep"
    PKG_MAP[fd-find]="fd-find|fd-find|fd|fd|fd|fd"
    PKG_MAP[zoxide]="zoxide|zoxide|zoxide|zoxide|zoxide|zoxide"
    PKG_MAP[procs]="procs|procs|procs|procs|procs|procs"
    PKG_MAP[tldr]="tldr|tldr|tldr|tldr|tldr|tldr"
    PKG_MAP[chafa]="chafa|chafa|chafa|chafa|chafa|chafa"
    PKG_MAP[fzf]="fzf|fzf|fzf|fzf|fzf|fzf"
    PKG_MAP[fastfetch]="fastfetch|fastfetch|fastfetch|fastfetch|fastfetch|fastfetch"
    PKG_MAP[inxi]="inxi|inxi|inxi|inxi|inxi|inxi"
    PKG_MAP[lm-sensors]="lm-sensors|lm_sensors|lm_sensors|sensors|lm-sensors|lm-sensors"
    PKG_MAP[unzip]="unzip|unzip|unzip|unzip|unzip|unzip"
    PKG_MAP[p7zip-full]="p7zip-full|p7zip|p7zip|p7zip|p7zip|p7zip"
    PKG_MAP[zsh]="zsh|zsh|zsh|zsh|zsh|zsh"
    PKG_MAP[xclip]="xclip|xclip|xclip|xclip|xclip|xclip"
    PKG_MAP[wl-clipboard]="wl-clipboard|wl-clipboard|wl-clipboard|wl-clipboard|wl-clipboard|wl-clipboard"
    PKG_MAP[acpi]="acpi|acpi|acpi|acpi|acpi|acpi"
    PKG_MAP[sysstat]="sysstat|sysstat|sysstat|sysstat|sysstat|sysstat"
    PKG_MAP[stress-ng]="stress-ng|stress-ng|stress-ng|stress-ng|stress-ng|stress-ng"
    PKG_MAP[smem]="smem|smem|smem|smem|smem|smem"
    PKG_MAP[preload]="preload|preloader|preload|preloader|preload|preload"
    PKG_MAP[cpufrequtils]="cpufrequtils|cpufrequtils|cpupower|cpufrequtils|cpufrequtils|cpufrequtils"
    PKG_MAP[gparted]="gparted|gparted|gparted|gparted|gparted|gparted"
    PKG_MAP[smartmontools]="smartmontools|smartmontools|smartmontools|smartmontools|smartmontools|smartmontools"
    PKG_MAP[tree]="tree|tree|tree|tree|tree|tree"
    PKG_MAP[ranger]="ranger|ranger|ranger|ranger|ranger|ranger"
    PKG_MAP[mc]="mc|mc|mc|mc|mc|mc"
    PKG_MAP[glances]="glances|glances|glances|glances|glances|glances"
    PKG_MAP[atop]="atop|atop|atop|atop|atop|atop"
    PKG_MAP[gh]="gh|gh|github-cli|gh|github-cli|github-cli"
    PKG_MAP[lazygit]="lazygit|lazygit|lazygit|lazygit|lazygit|lazygit"
    PKG_MAP[lazydocker]="lazydocker|lazydocker|lazydocker|lazydocker|lazydocker|lazydocker"
    PKG_MAP[httpie]="httpie|httpie|httpie|httpie|httpie|httpie"
    PKG_MAP[stow]="stow|stow|stow|stow|stow|stow"
    PKG_MAP[lsof]="lsof|lsof|lsof|lsof|lsof|lsof"
    PKG_MAP[dnsutils]="dnsutils|bind-utils|bind|bind-utils|bind-tools|bind-tools"
    PKG_MAP[shellcheck]="shellcheck|ShellCheck|shellcheck|ShellCheck|shellcheck|shellcheck"
    PKG_MAP[shfmt]="shfmt|shfmt|shfmt|shfmt|shfmt|shfmt"
    PKG_MAP[socat]="socat|socat|socat|socat|socat|socat"
    PKG_MAP[strace]="strace|strace|strace|strace|strace|strace"
    PKG_MAP[git-delta]="git-delta|git-delta|git-delta|git-delta|git-delta|git-delta"
    PKG_MAP[iputils-ping]="iputils-ping|iputils|iputils|iputils|iputils|iputils"
    PKG_MAP[net-tools]="net-tools|net-tools|net-tools|net-tools|net-tools|net-tools"

    unalias get_pkg_name 2>/dev/null
    function get_pkg_name {
        local generic="$1"
        local mapping="${PKG_MAP[$generic]}"
        [[ -z "$mapping" ]] && echo "$generic" && return
        local idx=1
        case "$PKG_MANAGER" in
            "apt") idx=1 ;;
            "dnf"|"yum") idx=2 ;;
            "pacman") idx=3 ;;
            "zypper") idx=4 ;;
            "apk") idx=5 ;;
            "xbps") idx=6 ;;
        esac
        echo "$mapping" | cut -d'|' -f$idx
    }

    unalias is_installed 2>/dev/null
    function is_installed {
        local pkg="$1"
        case "$PKG_MANAGER" in
            "apt") dpkg-query -W -f='${Status}' "$pkg" 2>/dev/null | grep -q "ok installed" ;;
            "dnf"|"yum"|"zypper") rpm -q "$pkg" &>/dev/null ;;
            "pacman") pacman -Q "$pkg" &>/dev/null ;;
            "apk") apk info -e "$pkg" &>/dev/null ;;
            "xbps") xbps-query "$pkg" &>/dev/null ;;
            *) return 1 ;;
        esac
    }

    # ===== 🎨 RENDER ENGINE WITH INDEX =====
    local menu_items
    menu_items=()

    local tool_list=(
        "PERF|zram-tools|RAM optimization using zRAM"
        "PERF|earlyoom|Prevent system freeze when RAM is low"
        "PERF|htop|Classic interactive process monitor"
        "PERF|btop|Modern & beautiful resource dashboard"
        "PERF|glances|Full system statistics at a glance"
        "PERF|atop|Advanced system & process monitor"
        "PERF|sysstat|System performance tools (sar, iostat)"
        "PERF|stress-ng|Stress test your CPU/RAM/IO"
        "PERF|smem|Report memory usage with PSS/USS"
        "PERF|preload|Adaptive readahead daemon (Speed up apps)"
        "PERF|cpufrequtils|CPU frequency scaling utilities"
        "DISK|ncdu|Disk usage analyzer (NCurses)"
        "DISK|gdu|Fast disk usage analyzer (Go based)"
        "DISK|duf|Visual Disk Usage/Free utility"
        "DISK|dust|A more intuitive version of 'du' in Rust"
        "DISK|bleachbit|Clean system junk and maintain privacy"
        "DISK|stacer|All-in-one system optimizer & GUI"
        "DISK|gparted|GNOME Partition Editor"
        "DISK|smartmontools|Control & monitor SMART storage systems"
        "DISK|tree|List contents of directories in a tree-like format"
        "DISK|ranger|VIM-inspired file manager for terminal"
        "DISK|mc|Midnight Commander (Twin-panel file manager)"
        "SECURE|ufw|Uncomplicated Firewall"
        "SECURE|fail2ban|Protect against brute-force attacks"
        "SECURE|rkhunter|Rootkit and exploit scanner"
        "SECURE|chkrootkit|Locally check for signs of a rootkit"
        "SECURE|lynis|Security auditing tool for Linux"
        "SECURE|clamav|Open source antivirus engine"
        "SECURE|firejail|Sandbox security for applications"
        "SECURE|gnupg|Gnu Privacy Guard for encryption"
        "NET|speedtest-cli|Test internet bandwidth via CLI"
        "NET|vnstat|Console-based network traffic monitor"
        "NET|nmap|Network exploration & security auditing"
        "NET|iftop|Display bandwidth usage on an interface"
        "NET|nload|Real-time network traffic visualization"
        "NET|nethogs|Net usage per process (Top for network)"
        "NET|curl|Command line tool for transferring data"
        "NET|wget|Retrieve files using HTTP, HTTPS, FTP"
        "NET|aria2|High-speed multi-source download utility"
        "NET|wireguard|Fast, modern and secure VPN tunnel"
        "NET|dog|A command-line DNS client (Better dig)"
        "NET|mtr-tiny|Combined ping and traceroute tool"
        "NET|tcpdump|Powerful command-line packet analyzer"
        "DEV|git|Distributed version control system"
        "DEV|docker.io|OS-level virtualization (Docker)"
        "DEV|docker-compose|Define & run multi-container applications"
        "DEV|build-essential|Essential packages for compiling code"
        "DEV|micro|Modern and intuitive terminal-based editor"
        "DEV|neovim|Extensible text editor (Vim 2.0)"
        "DEV|tmux|Terminal multiplexer for managing sessions"
        "DEV|screen|Full-screen window manager/multiplexer"
        "DEV|python3-pip|The Python package installer"
        "DEV|nodejs|JavaScript runtime environment"
        "DEV|npm|The Node.js package manager"
        "DEV|golang-go|The Go programming language"
        "DEV|rsync|Fast, versatile remote/local file-copy"
        "DEV|jq|Command-line JSON processor"
        "DEV|yq|Command-line YAML/XML processor"
        "MODERN|bat|Cat clone with syntax highlighting"
        "MODERN|eza|Modern replacement for 'ls' with icons"
        "MODERN|ripgrep|Extremely fast grep alternative"
        "MODERN|fd-find|Simple, fast alternative to 'find'"
        "MODERN|zoxide|Smarter cd command (Learns your habits)"
        "MODERN|procs|Modern replacement for 'ps' in Rust"
        "MODERN|tldr|Simplified community-driven man pages"
        "MODERN|chafa|Terminal graphics for the 21st century"
        "MODERN|fzf|General-purpose fuzzy finder"
        "SYS|fastfetch|High-performance neofetch alternative"
        "SYS|inxi|Full-featured system information script"
        "SYS|lm-sensors|Read temperature/voltage/fan sensors"
        "SYS|unzip|Decompress zip files"
        "SYS|p7zip-full|7z file archiver with high compression"
        "SYS|zsh|The Z shell (Advanced bash alternative)"
        "SYS|xclip|Command line interface to X selections"
        "SYS|wl-clipboard|Command line copy/paste for Wayland"
        "SYS|acpi|Displays battery and thermal information"
    )

    local idx=0
    for item in "${tool_list[@]}"; do
        idx=$(( idx + 1 ))
        local cat="" generic="" desc=""
        IFS='|' read -r cat generic desc <<< "$item"
        local pkg=$(get_pkg_name "$generic")

        # ZSH FIX: 'status' is read-only in Zsh, use 'pkg_status' instead
        local pkg_status="${DIM}○${NC}"
        is_installed "$pkg" && pkg_status="${GREEN}●${NC}"

        local c_cat=""
        case "$cat" in
            "PERF")   c_cat="${PURPLE}PERF${NC}" ;;
            "DISK")   c_cat="${RED}DISK${NC}" ;;
            "SECURE") c_cat="${GREEN}SECU${NC}" ;;
            "NET")    c_cat="${CYAN}NET ${NC}" ;;
            "DEV")    c_cat="${BLUE}DEV ${NC}" ;;
            "MODERN") c_cat="${YELLOW}MOD ${NC}" ;;
            *)        c_cat="${DIM}SYS ${NC}" ;;
        esac

        local line=""
        line=$(printf "%b ${DIM}[%3d]${NC}  %-12b  ${BOLD}%-18s${NC}  ${DIM}%s${NC}" "$pkg_status" "$idx" "$c_cat" "$generic" "$desc")
        menu_items+=("$line|$generic|$pkg")
    done

    # ===== 🖥️ UI LAUNCHER =====
    local selected_raw=""
    selected_raw=$(printf "%s\n" "${menu_items[@]}" | "$FZF_CMD" \
        --ansi --multi --delimiter='\|' --with-nth=1 \
        --height=90% --layout=reverse --border=rounded \
        --prompt="🔍 Search Arsenal > " \
        --header="  [TAB] Select Multiple  |  [ENTER] Process  |  [Q] Exit  | (${PKG_MANAGER})
  ─────────────────────────────────────────────────────────────────────────
  STAT  [IDX]  CATEGORY       PACKAGE          DESCRIPTION")

    [[ $? -ne 0 || -z "$selected_raw" ]] && { echo -e "\n${YELLOW}👋 Operation cancelled.${NC}"; return 0; }

    # ZSH FIX: Extract selections using temp file
    local selected_tools=()
    local actual_packages=()

    local tmpfile="/tmp/ut_selection_$$.txt"
    printf "%s\n" "$selected_raw" > "$tmpfile"

    while IFS= read -r raw_line; do
        [[ -z "$raw_line" ]] && continue
        local t=$(echo "$raw_line" | awk -F'\|' '{print $2}' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
        local p=$(echo "$raw_line" | awk -F'\|' '{print $3}' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
        [[ -n "$t" ]] && selected_tools+=("$t")
        [[ -n "$p" ]] && actual_packages+=("$p")
    done < "$tmpfile"
    rm -f "$tmpfile"

    [[ ${#selected_tools[@]} -eq 0 ]] && { echo -e "${YELLOW}⚠️ Nothing selected${NC}"; return 0; }

    echo -e "${CYAN}📋 Selected ${#selected_tools[@]} tools:${NC}"
    local _i
    for (( _i = 1; _i <= ${#selected_tools[@]}; _i++ )); do
        echo -e "  ${GREEN}●${NC} ${selected_tools[$_i]} (${actual_packages[$_i]})"
    done

    # ===== ⬇️ INSTALL & CONFIG ENGINE =====
    sudo -v || { echo -e "${RED}❌ Sudo required${NC}"; return 1; }

    # Disable job monitor so zsh doesn't print "[N] + terminated ..." when the
    # keepalive process ends. disown alone is not enough in zsh.
    set +m
    (
        while true; do
            sudo -n true 2>/dev/null || exit
            sleep 60
            kill -0 "$$" 2>/dev/null || exit
        done
    ) &>/dev/null &
    local SUDO_KEEPALIVE=$!
    disown $SUDO_KEEPALIVE 2>/dev/null || true
    set -m

    echo -e "${CYAN}🔧 Processing ${#selected_tools[@]} tools on ${DISTRO_ID}...${NC}"

    case "$PKG_MANAGER" in
        "apt") sudo apt update -y &>>"$LOGFILE" ;;
        "dnf"|"yum") sudo $PKG_MANAGER check-update -y &>>"$LOGFILE" || true ;;
        "pacman") sudo pacman -Sy &>>"$LOGFILE" ;;
        "zypper") sudo zypper refresh &>>"$LOGFILE" ;;
        "apk") sudo apk update &>>"$LOGFILE" ;;
        "xbps") sudo xbps-install -Su &>>"$LOGFILE" || true ;;
    esac

    local RC_FILE="$HOME/.bashrc"
    [[ "$SHELL" == */zsh ]] && RC_FILE="$HOME/.zshrc"
    local SHELL_NAME="${SHELL:t}"

    unalias add_config 2>/dev/null
    function add_config {
        local marker="$1"
        local content="$2"
        if ! grep -qF "$marker" "$RC_FILE" 2>/dev/null; then
            echo -e "\n# $marker" >> "$RC_FILE"
            echo "$content" >> "$RC_FILE"
        fi
    }

    local failed_pkgs=()
    local installed_pkgs=()

    local _j
    for (( _j = 1; _j <= ${#selected_tools[@]}; _j++ )); do
        local t="${selected_tools[$_j]}"
        local pkg="${actual_packages[$_j]}"

        echo -n -e "${WHITE}📦 $t (${pkg})... ${NC}"

        if is_installed "$pkg"; then
            echo -e "${GREEN}✔ Already installed${NC}"
            installed_pkgs+=("$t")
        else
            if eval "$PKG_INSTALL \"$pkg\"" &>>"$LOGFILE"; then
                echo -e "${GREEN}[INSTALLED]${NC}"
                installed_pkgs+=("$t")
            else
                echo -e "${RED}[FAILED]${NC}"
                failed_pkgs+=("$t ($pkg)")
                continue
            fi
        fi

        case "$t" in
            "docker.io")
                sudo usermod -aG docker "$USER" 2>/dev/null || true
                [[ "$SERVICE_CMD" == "systemctl" ]] && sudo systemctl enable --now docker &>>"$LOGFILE" || true
                ;;
            "tmux")
                [[ ! -f ~/.tmux.conf ]] && echo -e "set -g mouse on\nset -g default-terminal \"screen-256color\"" > ~/.tmux.conf
                ;;
            "git")
                git config --global color.ui auto 2>/dev/null || true
                git config --global core.editor "nano" 2>/dev/null || true
                git config --global init.defaultBranch main 2>/dev/null || true
                ;;
            "neovim")
                mkdir -p ~/.config/nvim
                [[ ! -f ~/.config/nvim/init.vim ]] && echo -e "set number\nset relativenumber\nset mouse=a\nset termguicolors" > ~/.config/nvim/init.vim
                add_config "Neovim Alias" "alias nv='nvim'\nalias vim='nvim'"
                ;;
            "zram-tools")
                if [[ "$PKG_MANAGER" == "apt" ]]; then
                    sudo bash -c "echo -e 'PERCENT=60\nALGO=zstd\nPRIORITY=100' > /etc/default/zramswap"
                    sudo systemctl restart zramswap &>>"$LOGFILE" || true
                elif [[ "$PKG_MANAGER" == "pacman" ]]; then
                    sudo pacman -S --noconfirm zram-generator 2>/dev/null || true
                    sudo bash -c "echo -e '[zram0]\nzram-size = ram / 2\ncompression-algorithm = zstd' > /etc/systemd/zram-generator.conf"
                    sudo systemctl daemon-reload &>>"$LOGFILE" || true
                    sudo systemctl start /dev/zram0 &>>"$LOGFILE" || true
                fi
                ;;
            "micro")
                mkdir -p ~/.config/micro
                [[ ! -f ~/.config/micro/settings.json ]] && echo '{"mouse": true, "clipboard": "terminal"}' > ~/.config/micro/settings.json
                ;;
            "ufw")
                command -v ufw &>/dev/null && { sudo ufw allow ssh 2>/dev/null && sudo ufw --force enable &>>"$LOGFILE" || true; }
                ;;
            "htop")
                mkdir -p ~/.config/htop
                [[ ! -f ~/.config/htop/htoprc ]] && echo "highlight_megabytes=1\nshow_program_path=1" > ~/.config/htop/htoprc
                ;;
            "acpi")
                add_config "Battery Status" "alias battery='acpi -V'"
                ;;
            "bat")
                local bat_cmd="bat"
                [[ "$PKG_MANAGER" == "apt" ]] && bat_cmd="batcat"
                add_config "Batcat Alias" "alias cat='$bat_cmd -p'\nalias bat='$bat_cmd'"
                ;;
            "eza")
                add_config "Eza Alias" "alias ls='eza --icons --group-directories-first'"
                ;;
            "zoxide")
                add_config "Zoxide Init" "[ -x \"\$(command -v zoxide)\" ] && eval \"\$(zoxide init $SHELL_NAME)\""
                add_config "Zoxide Alias" "alias cd='z'"
                ;;
            "preload")
                echo -e "${CYAN}🔧 Enabling Preload service...${NC}"
                [[ "$SERVICE_CMD" == "systemctl" ]] && {
                    sudo systemctl enable --now preload &>>"$LOGFILE"
                } || {
                    sudo $SERVICE_CMD preload start &>>"$LOGFILE"
                }
                ;;
            "earlyoom")
                echo -e "${CYAN}🔧 Configuring EarlyOOM...${NC}"
                if [ -f /etc/default/earlyoom ]; then
                    sudo sed -i 's/EARLYOOM_ARGS=.*/EARLYOOM_ARGS="-m 10 -s 5 --prefer '"'^(electron|java|python)'"'"/' /etc/default/earlyoom
                fi
                [[ "$SERVICE_CMD" == "systemctl" ]] && {
                    sudo systemctl enable --now earlyoom &>>"$LOGFILE"
                }
                ;;
            "lm-sensors")
                echo -e "${CYAN}🔍 Detecting Hardware Sensors...${NC}"
                sudo sensors-detect --auto &>>"$LOGFILE"
                [[ "$SERVICE_CMD" == "systemctl" ]] && {
                    sudo systemctl enable --now lm_sensors &>>"$LOGFILE" 2>/dev/null || \
                    sudo systemctl enable --now sensord &>>"$LOGFILE" 2>/dev/null
                }
                add_config "Sensor Alias" "alias temp='sensors'"
                ;;
        esac
    done

    set +m
    kill $SUDO_KEEPALIVE 2>/dev/null || true
    wait $SUDO_KEEPALIVE 2>/dev/null || true
    set -m

    # ===== 🔗 SHELL INTEGRATION =====
    if [[ " ${installed_pkgs[*]} " =~ " fzf " ]]; then
        if [[ "$SHELL_NAME" == "bash" ]] || [[ "$SHELL_NAME" == "zsh" ]]; then
            add_config "FZF Integration" "eval \"\$(fzf --$SHELL_NAME)\""
        fi
    fi

    add_config "HISTORY" "export HISTFILE=\"\$HOME/.zsh_history\"\nexport HISTSIZE=50000\nexport SAVEHIST=50000\nsetopt APPEND_HISTORY\nsetopt INC_APPEND_HISTORY\nsetopt SHARE_HISTORY\nsetopt HIST_EXPIRE_DUPS_FIRST\nsetopt HIST_IGNORE_DUPS\nsetopt HIST_SAVE_NO_DUPS"

    case "$PKG_MANAGER" in
        "apt") sudo apt autoremove -y &>>"$LOGFILE" || true ;;
        "dnf"|"yum") sudo $PKG_MANAGER autoremove -y &>>"$LOGFILE" || true ;;
        "pacman") sudo pacman -Sc --noconfirm &>>"$LOGFILE" || true ;;
    esac

    echo -e "\n${GREEN}✅ Deployment Complete on ${DISTRO_ID}!${NC}"
    echo -e "${GREEN}📦 Installed: ${#installed_pkgs[@]} tools${NC}"

    [[ ${#failed_pkgs[@]} -gt 0 ]] && {
        echo -e "${RED}❌ Failed (${#failed_pkgs[@]}):${NC}"
        printf '  - %s\n' "${failed_pkgs[@]}"
    }

    [[ " ${installed_pkgs[*]} " =~ " docker.io " ]] && echo -e "${YELLOW}⚠️  Log out and back in for Docker group changes${NC}"
    [[ " ${installed_pkgs[*]} " =~ " zoxide " ]] && echo -e "${CYAN}💡 Run 'source $RC_FILE' to enable zoxide${NC}"

    rm -f "$LOGFILE" 2>/dev/null || true
}


# ======================================================
#  📂 all file re name
# ======================================================


unalias rn 2>/dev/null
function rn {
    local target_dir="${1:-.}"
    local dir="" f="" filename="" extension="" clean_name="" clean_ext="" new_name="" filepath=""

    if [ ! -d "$target_dir" ]; then
        echo "Error: Directory $target_dir does not exist."
        return 1
    fi

    echo "Cleaning files in: $target_dir"

    # -print0 ব্যবহার করা হয়েছে যাতে ফাইলের নামে স্পেস বা নিউলাইন থাকলেও সমস্যা না হয়
    find "$target_dir" -maxdepth 1 -type f -print0 | while IFS= read -r -d '' filepath; do

        dir=$(dirname "$filepath")
        f=$(basename "$filepath")

        # ১. নাম এবং এক্সটেনশন আলাদা করা (স্মার্ট চেক)
        if [[ "$f" == *.* ]]; then
            filename="${f%.*}"
            extension=".${f##*.}" # ডটসহ এক্সটেনশন
        else
            filename="$f"
            extension="" # এক্সটেনশন নেই
        fi

        # ২. নাম ক্লিন করা
        clean_name=$(echo "$filename" | tr '[:upper:]' '[:lower:]' | tr ' ' '-' | sed 's/[^a-z0-9_-]//g')
        clean_ext=$(echo "$extension" | tr '[:upper:]' '[:lower:]')

        new_name="${clean_name}${clean_ext}"

        # ৩. রিনেম কন্ডিশন
        if [ "$f" != "$new_name" ]; then
            if [ -e "$dir/$new_name" ]; then
                echo "Skipped: '$new_name' already exists."
            else
                mv "$dir/$f" "$dir/$new_name"
                echo "Renamed: '$f' -> '$new_name'"
            fi
        fi
    done
    echo "Done!"
}



# ======================================================
#  📂 package genarator
# ======================================================

# Smart Universal Package Converter & Manager
unalias pg 2>/dev/null
function pg {
    local file="$1"
    local install_flag="$2"
    local os_type=""
    local pkg_manager=""
    local target_ext=""

    # ১. OS এবং প্যাকেজ ম্যানেজার ডিটেক্ট করা
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        case "$ID" in
            ubuntu|deepin|debian|kali|linuxmint|pop)
                os_type="debian"
                pkg_manager="apt"
                target_ext="deb"
                ;;
            fedora|rhel|centos|amzn)
                os_type="redhat"
                pkg_manager="dnf"
                target_ext="rpm"
                ;;
            arch|manjaro)
                os_type="arch"
                pkg_manager="pacman"
                target_ext="tgz" # Alien Arch এর জন্য tgz ব্যবহার করে
                ;;
            *)
                echo "Sorry Your OS ($PRETTY_NAME) is not supported by this script."
                return 1
                ;;
        esac
    fi

    # ২. ইনপুট চেক
    if [[ -z "$file" ]]; then
        echo "ব্যবহার: superconv [ফাইল_নাম] [-i]"
        echo "OS ডিটেক্ট করা হয়েছে: $PRETTY_NAME"
        return 1
    fi

    # ৩. Alien টুলটি আছে কিনা চেক ও ইনস্টল করা
    if ! command -v alien &> /dev/null; then
        echo "Alien not installed $pkg_manager Package Used To install..."
        if [[ "$pkg_manager" == "apt" ]]; then
            sudo apt update && sudo apt install alien -y
        elif [[ "$pkg_manager" == "dnf" ]]; then
            sudo dnf install alien -y
        elif [[ "$pkg_manager" == "pacman" ]]; then
            sudo pacman -S alien --noconfirm
        fi
    fi

    # ৪. কনভার্ট করা (OS অনুযায়ী target ফরম্যাট সেট করা)
    if [[ -f "$file" ]]; then
        echo "Converter $target_ext format..."

        case "$os_type" in
            "debian") sudo alien --to-deb --scripts "$file" ;;
            "redhat") sudo alien --to-rpm --scripts "$file" ;;
            "arch")   sudo alien --to-tgz --scripts "$file" ;;
        esac

        if [[ $? -eq 0 ]]; then
            echo "--- file Convert successfully ! ---"

            # ৫. কন্ডিশনাল ইনস্টলেশন
            if [[ "$install_flag" == "-i" ]]; then
                local new_pkg=$(ls -t *.$target_ext | head -1)
                echo "This Package intsalling: $new_pkg"

                if [[ "$pkg_manager" == "apt" ]]; then
                    sudo dpkg -i "$new_pkg" && sudo apt install -f
                elif [[ "$pkg_manager" == "dnf" ]]; then
                    sudo dnf install ./"$new_pkg" -y
                elif [[ "$pkg_manager" == "pacman" ]]; then
                    sudo pacman -U "$new_pkg" --noconfirm
                fi
            fi
        else
            echo "convarsion Failed !"
        fi
    else
        echo "file is missing: $file"
    fi
}





# ======================================================
#  📂 ALIASES: Navigation & System
# ======================================================

# --- Basic Navigation ---
alias c='clear'
alias cls='clear'
alias rd='cd /'
alias ..='cd ..'
alias ...='cd ../..'
alias ....='cd ../../..'

# alias drive='cd /media/Rihad/085df205-a554-40c8-b0b1-59a1ad469a94'

unalias drive 2>/dev/null
function drive {
    local drive1_uuid="469a94"
    local drive2_uuid="b2c89f" # Apnar 2nd drive er UUID match kore niben

    local target_path=""

    if [[ "$1" == "1" || -z "$1" ]]; then
        target_path=$(print /media/*/*${drive1_uuid}(-N))
    elif [[ "$1" == "2" ]]; then
        target_path=$(print /media/*/*${drive2_uuid}(-N))
    else
        echo "❌ Invalid option! Use: 'drive' or 'drive 1', 'drive 2'"
        return 1
    fi

    if [ -d "$target_path" ]; then
        cd "$target_path"
        echo "📂 Switched to: $target_path"
    else
        echo "❌ Error: Driveটি খুঁজে পাওয়া যায়নি!"
    fi
}


# --- Quick Folder Jumps (Change paths as needed) ---
alias dev='cd ~/Developer'
alias doc='cd ~/Documents'
alias dow='cd ~/Downloads'
alias des='cd ~/Desktop'
alias pic='cd ~/Pictures'
alias vid='cd ~/Videos'
alias mus='cd ~/Music'

# --- Project Shortcuts ---
alias ar='cd ~/Developer/archive'
alias ba='cd ~/Developer/backend'
alias de='cd ~/Developer/dev'
alias fig='cd ~/Developer/Figma'
alias fr='cd ~/Developer/frontend'
alias fu='cd ~/Developer/fullstack'


# --- System Maintenance ---
unalias update 2>/dev/null
unalias clean 2>/dev/null
unset -f update 2>/dev/null
unset -f clean 2>/dev/null
alias update="fancybash update"
alias clean="fancybash clean"

alias zshrc='code ~/.zshrc'
alias to='code .'
alias rel='source ~/.zshrc && echo "✅ .zshrc reloaded successfully!"'

# --- Network & Server ---
alias serve='python3 -m http.server'
alias ports='ss -tulpn'
alias myip='ip a | grep inet'


# --- PostgreSQL ---
alias pgstart='sudo systemctl start postgresql'
alias pgstop='sudo systemctl stop postgresql'
alias pgrestart='sudo systemctl restart postgresql'
alias pgstatus='sudo systemctl status postgresql'
alias pgenable='sudo systemctl enable postgresql && echo "✅ PostgreSQL auto-start enabled"'
alias pgdisable='sudo systemctl disable postgresql && echo "🚫 PostgreSQL auto-start disabled"'
alias pgl='sudo -u postgres psql'                          # postgres user hisebe login
alias pgdb='psql -U postgres -d'                           # Usage: pgdb mydb
alias pgls='psql -U postgres -c "\\l"'                      # সব database list
alias pgtables='psql -U postgres -c "\\dt"'                 # সব table list
alias pgdump='pg_dump -U postgres'                          # Usage: pgdump mydb > backup.sql
alias pgrestore='psql -U postgres'                          # Usage: pgrestore mydb < backup.sql
unalias pglogs 2>/dev/null
function pglogs {
    if [ -d /var/log/postgresql ] && ls /var/log/postgresql/*.log &>/dev/null; then
        sudo tail -f /var/log/postgresql/*.log
    else
        sudo journalctl -u postgresql -f
    fi
}
alias pgcreate='createdb -U postgres'                      # Usage: pgcreate mydb
alias pgdrop='dropdb -U postgres'                          # Usage: pgdrop mydb
alias pgusers='psql -U postgres -c "\\du"'                  # সব users/roles দেখুন
alias pgsize='psql -U postgres -c "SELECT pg_database.datname, pg_size_pretty(pg_database_size(pg_database.datname)) AS size FROM pg_database ORDER BY pg_database_size(pg_database.datname) DESC;"'  # প্রতিটি DBর সাইজ
alias pgver='psql -U postgres -c "SELECT version();"'     # PostgreSQL version দেখুন
alias pgconn='psql -U postgres -c "SELECT count(*) FROM pg_stat_activity;"'  # active connections

# --- Node/NPX Prisma ORM (np*) — first letter of each word ---
alias np='npx prisma'                                       # npx prisma
alias npi='npx prisma init'                                 # npx prisma init
alias npg='npx prisma generate'                             # npx prisma generate
alias nps='npx prisma studio'                               # npx prisma studio
alias npmd='npx prisma migrate dev'                         # npx prisma migrate dev
alias npmdn='npx prisma migrate dev --name'                 # npx prisma migrate dev --name
alias npmr='npx prisma migrate reset'                       # npx prisma migrate reset
alias npmdp='npx prisma migrate deploy'                     # npx prisma migrate deploy
alias npms='npx prisma migrate status'                      # npx prisma migrate status
alias npdp='npx prisma db push'                             # npx prisma db push
alias npdl='npx prisma db pull'                             # npx prisma db pull
alias npds='npx prisma db seed'                             # npx prisma db seed
alias npf='npx prisma format'                               # npx prisma format
alias npv='npx prisma version'                              # npx prisma version

# --- Bun Prisma ORM (bp*) — first letter of each word ---
alias bp='bunx prisma'                                      # bunx prisma
alias bpi='bunx prisma init'                                # bunx prisma init
alias bpg='bunx prisma generate'                            # bunx prisma generate
alias bps='bunx prisma studio'                              # bunx prisma studio
alias bpmd='bunx prisma migrate dev'                        # bunx prisma migrate dev
alias bpmdn='bunx prisma migrate dev --name'                # bunx prisma migrate dev --name
alias bpmr='bunx prisma migrate reset'                      # bunx prisma migrate reset
alias bpmdp='bunx prisma migrate deploy'                    # bunx prisma migrate deploy
alias bpms='bunx prisma migrate status'                     # bunx prisma migrate status
alias bpdp='bunx prisma db push'                            # bunx prisma db push
alias bpdl='bunx prisma db pull'                            # bunx prisma db pull
alias bpds='bunx prisma db seed'                            # bunx prisma db seed
alias bpf='bunx prisma format'                              # bunx prisma format
alias bpv='bunx prisma version'                             # bunx prisma version






# ======================================================
#  🛠️  FUNCTION TOOLS (Better than Aliases)
# ======================================================


# Create directory and enter it immediately
# Usage: mkd new_folder
unalias mkd 2>/dev/null
function mkd {
    mkdir -p "$1" && cd "$1" && echo "✅ Created & Entered: $1"
}

# Force remove directory
# Usage: rmd folder_name
unalias rmd 2>/dev/null
function rmd {
    rm -rf "$1" && echo "✅ Removed directory: $1"
}

# Remove file with confirmation
# Usage: rmf file.txt
unalias rmf 2>/dev/null
function rmf {
    rm -i "$1" && echo "✅ Removed file: $1"
}

# Auto 'ls' after cd (Lists files automatically when you switch folders)
# cd() {
#     builtin cd "$@" && ls --color=auto -F
# }


# ======================================================
#  📦 DEV STACK ALIASES (NPM, BUN, GIT)
# ======================================================

# --- NPM Shortcuts ---
alias ni='npm install'
alias nid='npm install -D'
alias nr='npm run'
alias nrd='npm run dev'
alias nrb='npm run build'
alias nrs='npm run start'

# --- Bun Shortcuts ---
alias bi='bun install'
alias br='bun run'
alias brd='bun run dev'
alias brb='bun run build'
alias brs='bun run start'
alias html='bun run index.html'
alias w='bun --watch'
alias bhot='bun --hot'

# --- Git Shortcuts ---
alias gi='git init'
alias gs='git status -sb'
alias gl="git log --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit"
alias gd='git diff'
alias gco='git checkout'
alias gcm='git commit -m'
alias gpl='git pull'
alias gps='git push'
alias gpu='git push -u origin $(git branch --show-current)'
alias gb='git branch'
alias gcb='git checkout -b'
alias ga='git add .'
alias gr='git restore'
alias grh='git reset HEAD~1'
alias gc='git clone'
alias gst='git stash'
alias gsta='git stash apply'
alias gpop='git stash pop'
# Fetch and prune deleted branches
alias gfp='git fetch --prune'


alias vlc="flatpak run org.videolan.VLC"
alias brave="flatpak run com.brave.Browser"
alias youtube="brave --app=https://www.youtube.com"

# Handle unknown commands politely
unalias command_not_found_handler 2>/dev/null
function command_not_found_handler {
  echo "❌ Command not found: $1"
  if command -v apt &>/dev/null; then
    echo "🔍 Try searching: apt search $1 | npm i -g $1"
  elif command -v pacman &>/dev/null; then
    echo "🔍 Try searching: pacman -Ss $1 | npm i -g $1"
  elif command -v dnf &>/dev/null; then
    echo "🔍 Try searching: dnf search $1 | npm i -g $1"
  else
    echo "🔍 Try searching via package manager or npm i -g $1"
  fi
}
unalias command_not_found_handle 2>/dev/null
function command_not_found_handle { command_not_found_handler "$@"; }


alias bv='cd ~/Downloads/Brave'
alias ch='cd ~/Downloads/Chrome'
alias gp='cd ~/Downloads/Google\ Photos'
alias pa='cd ~/Downloads/Packet'
alias ss='cd ~/Downloads/Screenshot'
alias vi='cd ~/Downloads/Video'


# Zsh-এর জন্য autocd active করার নিয়ম
setopt autocd

# Remove any Flatpak app paths from LD_LIBRARY_PATH
if [[ -n "$LD_LIBRARY_PATH" ]] && [[ "$LD_LIBRARY_PATH" == *"/var/lib/flatpak/app/"* ]]; then
    # Filter out Flatpak app paths, keep system paths
    new_path=$(echo "$LD_LIBRARY_PATH" | tr ':' '\n' | grep -v "/var/lib/flatpak/app/" | grep -v "$HOME/.local/share/flatpak/app/" | paste -sd ':' -)
    if [[ -n "$new_path" ]]; then
        export LD_LIBRARY_PATH="$new_path"
    else
        unset LD_LIBRARY_PATH
    fi
    unset new_path 2>/dev/null || true
fi

# Note: Zsh Autocompletion system, plugins and compinit are initialized at the top of config.zsh prior to NVM.

# Synchronize and Share history across multiple terminal sessions
export HISTFILE="${HISTFILE:-$HOME/.zsh_history}"
export HISTSIZE=50000
export SAVEHIST=50000

# ZSH History setopts for real-time synchronization and deduplication
setopt SHARE_HISTORY          # Import commands from history file and append immediately
setopt INC_APPEND_HISTORY      # Write to history file immediately upon execution
setopt EXTENDED_HISTORY        # Save command timestamps and duration in history
setopt HIST_EXPIRE_DUPS_FIRST # Expire duplicate entries first when trimming history file
setopt HIST_IGNORE_DUPS        # Do not record entry duplicate of previous line
setopt HIST_IGNORE_ALL_DUPS    # Remove older duplicate entries from history
setopt HIST_FIND_NO_DUPS       # Do not display duplicates when searching
setopt HIST_IGNORE_SPACE       # Do not record lines starting with a space
setopt HIST_SAVE_NO_DUPS       # Do not write duplicate entries to history file
setopt HIST_REDUCE_BLANKS      # Remove extra blanks before saving to history
setopt HIST_VERIFY             # Do not execute immediately upon history expansion

# 1. Auto-LS and FZF Summary Preview when changing directory
# Clear out traditional chpwd to avoid any duplicates from old code
if functions chpwd >/dev/null; then unfunction chpwd; fi

# Accurate and Modern Auto-LS Function
unalias accurate_auto_ls 2>/dev/null
function accurate_auto_ls {
    emulate -L zsh

    # Zsh array parsing flags:
    # (N) empty dir handle, (.) shudhu regular files, (I) ignore traditional exclusions
    local -a total_files=( *(-.N) )
    local -a hidden_files=( .*(.-N) )

    # Safely removing '.' and '..' manually from hidden array
    hidden_files=(${hidden_files:#.(|.)})

    local file_count=${#total_files}
    local hidden_count=${#hidden_files}

    # Clean UI rendering
    echo -e "\n\e[1;35m📂 Directory: ${PWD:t}\e[0m (\e[32m$file_count files\e[0m | \e[33m$hidden_count hidden\e[0m)"
    echo -e "\e[2m───────────────────────────────────────\e[0m"

    # Display using explicit native columns
    ls -FA --color=auto
}

# Standard Zsh hook array registration (Safest approach to avoid duplicates)
typeset -gU chpwd_functions
chpwd_functions=(accurate_auto_ls)

# 2. Advanced FZF Quick CD Function (Optional but highly recommended)
# Terminal-e shudhu 'cf' likhle fzf open hobe pipeline preview shoho

unalias cf 2>/dev/null
function cf {
    # Helper to auto-install missing packages dynamically based on active package manager
    _cf_install_pkg() {
        local tool="$1"
        local apt_pkg="$2"
        local pac_pkg="$3"
        local dnf_pkg="$4"

        echo -e "\033[1;33m⚡ 'cf' requires '$tool'. Dynamic auto-installing for your distro...\033[0m"

        if command -v apt-get &>/dev/null; then
            sudo apt-get update -qq && sudo apt-get install -y "$apt_pkg"
        elif command -v pacman &>/dev/null; then
            sudo pacman -S --noconfirm --needed "$pac_pkg"
        elif command -v dnf &>/dev/null; then
            sudo dnf install -y "$dnf_pkg"
        elif command -v zypper &>/dev/null; then
            sudo zypper install -y "$pac_pkg"
        elif command -v apk &>/dev/null; then
            sudo apk add "$pac_pkg"
        elif command -v brew &>/dev/null; then
            brew install "$pac_pkg"
        else
            echo -e "\033[1;31m❌ Package manager not found. Please install '$tool' manually.\033[0m"
            return 1
        fi
    }

    # 1. Dynamic Dependency Check & Auto-Install for fzf
    if ! command -v fzf &>/dev/null; then
        _cf_install_pkg "fzf" "fzf" "fzf" "fzf" || return 1
    fi

    # 2. Dynamic Dependency Check & Auto-Install for fd
    local fd_cmd=""
    if command -v fd &>/dev/null; then
        fd_cmd="fd"
    elif command -v fdfind &>/dev/null; then
        fd_cmd="fdfind"
    else
        _cf_install_pkg "fd" "fd-find" "fd" "fd-find"
        if command -v fd &>/dev/null; then
            fd_cmd="fd"
        elif command -v fdfind &>/dev/null; then
            fd_cmd="fdfind"
        fi
    fi

    # 3. Dynamic Dependency Check & Auto-Install for zoxide
    if ! command -v zoxide &>/dev/null; then
        _cf_install_pkg "zoxide" "zoxide" "zoxide" "zoxide"
    fi

    # 4. Dynamic Dependency Check & Auto-Install for bat
    if ! command -v bat &>/dev/null && ! command -v batcat &>/dev/null; then
        _cf_install_pkg "bat" "bat" "bat" "bat"
    fi

    # 5. Dynamic Dependency Check & Auto-Install for chafa
    if ! command -v chafa &>/dev/null; then
        _cf_install_pkg "chafa" "chafa" "chafa" "chafa"
    fi

    local dir
    local search_cmd
    local target_dir="${1:-.}"
    if [ "$target_dir" = "." ] && [ "$PWD" = "/" ]; then
        target_dir="$HOME"
    fi

    # Smart hidden filter: allow project hidden folders (.github, .vscode, .env) in projects,
    # while excluding heavy IDE/app data folders at Home root level (.antigravity-ide, .linglong, etc.)
    local hidden_flag="--hidden"
    local exclude_opts="--exclude .git --exclude node_modules --exclude .cache --exclude .antigravity-ide --exclude .linglong --exclude .local --exclude .var --exclude .npm --exclude .nvm --exclude .cargo --exclude .rustup --exclude .electron --exclude .mozilla --exclude /proc --exclude /sys --exclude /dev --exclude /etc --exclude /var --exclude /usr --exclude /tmp --exclude /run/user --exclude /run/systemd"

    local abs_target
    abs_target=$(cd "$target_dir" 2>/dev/null && pwd || echo "$target_dir")
    if [ "$abs_target" = "$HOME" ] || [ "$PWD" = "$HOME" ]; then
        hidden_flag=""
        exclude_opts="--exclude '.*' --exclude node_modules --exclude /proc --exclude /sys --exclude /dev --exclude /etc --exclude /var --exclude /usr --exclude /tmp --exclude /run/user --exclude /run/systemd"
    fi

    if [ -n "$fd_cmd" ]; then
        search_cmd="$fd_cmd $hidden_flag $exclude_opts . \"$target_dir\""
    else
        search_cmd="find \"$target_dir\" \( -path '*/.*' -o -path '*/node_modules*' -o -path '/proc*' -o -path '/sys*' -o -path '/dev*' -o -path '/etc*' -o -path '/var*' -o -path '/usr*' -o -path '/tmp*' -o -path '/run/user*' -o -path '/run/systemd*' \) -prune -o -print 2>/dev/null"
    fi

    # 6. Optimized Multi-Action Workflow (Folders, Videos, Images & PDFs Supported)
    dir=$(eval "$search_cmd" | fzf \
        --height 90% \
        --layout=reverse \
        --border=rounded \
        --prompt="⚡ Dev Walk: " \
        --pointer="❯" \
        --marker="✔" \
        --header="[ENTER] Cd/Open | [CTRL-Z] Recent Dirs (Zoxide) | [CTRL-V] Video | [CTRL-P] PDF | [CTRL-O] Editor | [CTRL-E] Explorer" \
        --header-first \
        --bind "ctrl-y:execute-silent(echo -n {} | (wl-copy 2>/dev/null || xclip -selection clipboard 2>/dev/null || clip.exe 2>/dev/null || pbcopy 2>/dev/null))+change-prompt(📋 Copied! > )" \
        --bind "ctrl-v:execute(v {} 2>/dev/null &)+change-prompt(🎬 Playing Video > )" \
        --bind "ctrl-p:execute((google-chrome {} 2>/dev/null || chromium {} 2>/dev/null || brave {} 2>/dev/null || xdg-open {} 2>/dev/null) &)+change-prompt(📄 PDF Opened > )" \
        --bind "ctrl-o:execute(code {} 2>/dev/null || cursor {} 2>/dev/null || nvim {})+abort" \
        --bind "ctrl-e:execute(nautilus {} 2>/dev/null || dolphin {} 2>/dev/null || explorer.exe {} 2>/dev/null || open {})" \
        --bind "ctrl-z:reload(zoxide query -l 2>/dev/null || echo \$HOME)+change-prompt(⚡ Frecent Dirs > )" \
        --bind "ctrl-h:reload($([ -n "$fd_cmd" ] && echo "$fd_cmd --exclude '.*' --exclude node_modules --exclude /proc --exclude /sys --exclude /dev --exclude /etc --exclude /var --exclude /usr --exclude /tmp --exclude /run/user --exclude /run/systemd . \$(dirname {}) 2>/dev/null" || echo "find \$(dirname {}) \( -path '*/.*' -o -path '*/node_modules*' -o -path '/proc*' -o -path '/sys*' -o -path '/dev*' -o -path '/etc*' -o -path '/var*' -o -path '/usr*' -o -path '/tmp*' -o -path '/run/user*' -o -path '/run/systemd*' \) -prune -o -print 2>/dev/null"))+change-prompt(⚡ Parent: )" \
        --preview '
            if [ -d {} ]; then
                echo -e "\e[1;34m📁 Contents of: {} \e[0m"
                echo -e "\e[2m──────────────────────────────────────────\e[0m"
                if command -v eza &>/dev/null; then
                    eza --tree --level=1 --icons --color=always {} 2>/dev/null | head -20
                elif command -v tree &>/dev/null; then
                    tree -C -L 1 {} 2>/dev/null | head -20
                else
                    ls -FA --color=always {} 2>/dev/null | head -20
                fi
                if git -C {} rev-parse --is-inside-work-tree &>/dev/null; then
                    echo -e "\e[2m──────────────────────────────────────────\e[0m"
                    local branch=$(git -C {} branch --show-current 2>/dev/null || git -C {} rev-parse --short HEAD 2>/dev/null)
                    echo -e "\e[1;32m🌿 Git Repo:\e[0m Branch -> \e[1;36m${branch:-main}\e[0m"
                    echo -e "\e[1;33m📜 Recent Commits:\e[0m"
                    git -C {} log --oneline -n 3 --color=always 2>/dev/null
                fi
            else
                echo -e "\e[1;36m📄 Previewing File: {} \e[0m"
                echo -e "\e[2m──────────────────────────────────────────\e[0m"
                ext=$(echo {} | awk -F. "{print \$NF}" | tr "[:upper:]" "[:lower:]")
                case "$ext" in
                    png|jpg|jpeg|gif|webp|ico|svg)
                        if command -v chafa &>/dev/null; then
                            chafa --size=40x15 {} 2>/dev/null
                        else
                            echo -e "\e[1;35m🖼️ Image File:\e[0m $(basename {})"
                        fi
                        ;;
                    zip|tar|gz|bz2|7z|rar)
                        echo -e "\e[1;33m📦 Archive Contents:\e[0m"
                        if [ "$ext" = "zip" ]; then unzip -l {} 2>/dev/null | head -20
                        elif [ "$ext" = "tar" ] || [ "$ext" = "gz" ]; then tar -tf {} 2>/dev/null | head -20
                        else 7z l {} 2>/dev/null | head -20; fi
                        ;;
                    mp3|wav|flac|m4a|ogg)
                        echo -e "\e[1;35m🎵 Audio File:\e[0m $(basename {})"
                        command -v mediainfo &>/dev/null && mediainfo {} | head -15
                        ;;
                    mp4|mkv|avi|mov|webm|flv|m4v)
                        echo -e "\e[1;35m🎬 Video File:\e[0m $(basename {})"
                        echo -e "\e[1;33m💡 Press ENTER or CTRL-V to play with v()\e[0m"
                        ;;
                    pdf)
                        echo -e "\e[1;36m📄 PDF Document:\e[0m $(basename {})"
                        echo -e "\e[1;33m💡 Press ENTER or CTRL-P to open in Browser\e[0m"
                        ;;
                    *)
                        if command -v bat &>/dev/null; then
                            bat --color=always --style=numbers --line-range :30 {} 2>/dev/null
                        elif command -v batcat &>/dev/null; then
                            batcat --color=always --style=numbers --line-range :30 {} 2>/dev/null
                        else
                            head -n 25 {} 2>/dev/null
                        fi
                        ;;
                esac
            fi
            echo -e "\e[2m──────────────────────────────────────────\e[0m"
            local sz=$(timeout 0.2s du -sh {} 2>/dev/null | cut -f1)
            echo -e "\e[1;33m📊 Size:\e[0m ${sz:-Quick Scan}"
        ' \
        --preview-window=right:50%:wrap)

    if [ -n "$dir" ]; then
        if [ -d "$dir" ]; then
            cd "$dir"
        elif [ -f "$dir" ]; then
            local ext="${dir##*.}"
            ext="${ext:l}"
            case "$ext" in
                mp4|mkv|avi|mov|webm|flv|m4v)
                    echo -e "\033[1;35m🎬 Opening Video with v()...\033[0m"
                    v "$dir"
                    ;;
                pdf)
                    echo -e "\033[1;36m📄 Opening PDF in Chrome...\033[0m"
                    (google-chrome "$dir" 2>/dev/null || google-chrome-stable "$dir" 2>/dev/null || chromium "$dir" 2>/dev/null || chromium-browser "$dir" 2>/dev/null || brave "$dir" 2>/dev/null || xdg-open "$dir" 2>/dev/null) &
                    ;;
                *)
                    code "$dir" 2>/dev/null || cursor "$dir" 2>/dev/null || nvim "$dir"
                    ;;
            esac
        fi
    fi
}


# --- Auto Completion (Handled via unified compinit block) ---

# --- Deepin System Colors ---
if [[ ("$TERM" = *256color || "$TERM" = screen* || "$TERM" = xterm* ) && -f /etc/lscolor-256color ]]; then
    eval $(dircolors -b /etc/lscolor-256color)
else
    eval $(dircolors)
fi
alias ls='ls --color=auto'
alias ll='ls -l'
alias la='ls -A'

# --- Load Bash Aliases if exists ---
if [ -f ~/.bash_aliases ]; then
    source ~/.bash_aliases
fi


# ====================================================================
#              🐳 THE ULTIMATE DOCKER SWISS ARMY KNIFE (ZSH) 🐳
# ====================================================================

# --------------------------------------------------------------------
# ১. স্ট্যাটাস, লিস্ট এবং সাইজ মনিটরিং (Status & Monitoring)
# --------------------------------------------------------------------
alias dps="docker ps --format 'table {{.ID}}\t{{.Names}}\t{{.Status}}\t{{.Ports}}'"
alias dpsa="docker ps -a"
alias di="docker images"
alias dvl="docker volume ls"
alias dnl="docker network ls"
alias dsize="docker system df"
alias dtop="docker stats --format 'table {{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}\t{{.BlockIO}}'"

# --------------------------------------------------------------------
# ১বি. SUDO ডকার শর্টকাট (Sudo Docker Shortcuts)
# --------------------------------------------------------------------
alias sdps="sudo docker ps --format 'table {{.ID}}\t{{.Names}}\t{{.Status}}\t{{.Ports}}'"
alias sdpsa="sudo docker ps -a"
alias sdi="sudo docker images"
alias sdvl="sudo docker volume ls"
alias sdnl="sudo docker network ls"
alias sdsize="sudo docker system df"
alias sdtop="sudo docker stats --format 'table {{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}\t{{.BlockIO}}'"

# --------------------------------------------------------------------
# ১গ. ডকার সার্ভিস কন্ট্রোল (Docker Service Control via systemctl)
# --------------------------------------------------------------------
# ডকার সার্ভিস চালু করতে
alias dstart="sudo systemctl start docker"
# ডকার সার্ভিস বন্ধ করতে
alias doff="sudo systemctl stop docker"
# ডকার সার্ভিস চালু নাকি বন্ধ তা দেখতে
alias dstatus="sudo systemctl status docker"
# পিসি অন হলে ডকার অটো-স্টার্ট চালু করতে (docker + docker.socket উভয়)
alias denable="sudo systemctl enable docker && sudo systemctl enable docker.socket"
# পিসি অন হলে ডকার অটো-স্টার্ট বন্ধ করতে (docker + docker.socket উভয়)
alias ddisable="sudo systemctl disable docker && sudo systemctl disable docker.socket"

# --------------------------------------------------------------------
# ২. কন্টেইনার লাইফসাইকেল (Container Lifecycle & Control)
# --------------------------------------------------------------------
alias dstop="docker stop"
alias drm="docker rm"
alias drmi="docker rmi"
alias drestart="docker restart"
alias dkill="docker rm -f"
alias dstopall='docker stop $(docker ps -q)'
alias drmall='docker rm $(docker ps -a -q)'

# --------------------------------------------------------------------
# ৩. ডিবাগিং, লগ এবং ইমেজ বিল্ড (Debugging & Building)
# --------------------------------------------------------------------
alias dsh="docker exec -it"
alias dlogs="docker logs -f"
alias dbuild="docker build -t"
alias dbuild-nocache="docker build --no-cache -t"
alias dhist="docker history"
alias dports="docker port"

# --------------------------------------------------------------------
# ৪. ডকার কম্পোজ শর্টকাট (Docker Compose)
# --------------------------------------------------------------------
alias dcup="docker compose up -d"
alias dcdn="docker compose down"
alias dclogs="docker compose logs -f"
alias dcupb="docker compose up -d --build"

# --------------------------------------------------------------------
# ৫. কুইক টেস্ট স্যান্ডবক্স (Temporary Test Containers)
# --------------------------------------------------------------------
alias dtest-ubuntu="docker run --rm -it ubuntu:latest bash"
alias dtest-node="docker run --rm -it node:alpine sh"
alias dtest-alpine="docker run --rm -it alpine:latest sh"

# --------------------------------------------------------------------
# ৬. স্মার্ট এবং অ্যাডভান্সড ফাংশন (Advanced Functions)
# --------------------------------------------------------------------

unalias dfind 2>/dev/null
function dfind {
    echo -e "\e[1;34m--> Running Containers:\e[0m"
    docker ps | grep -i "$1"
    echo -e "\n\e[1;32m--> Downloaded Images:\e[0m"
    docker images | grep -i "$1"
}

unalias droot 2>/dev/null
function droot {
    docker exec -it -u root "$1" bash 2>/dev/null || docker exec -it -u root "$1" sh
}

unalias dip 2>/dev/null
function dip {
    docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$1"
}

unalias dwatch 2>/dev/null
function dwatch {
    if [ -z "$1" ]; then echo "Usage: dwatch <container-name>"; return 1; fi
    echo -e "\e[1;35mWatching file changes in '$1' (Press Ctrl+C to stop)...\e[0m"
    watch -n 1 "docker diff $1"
}

unalias dnetstat 2>/dev/null
function dnetstat {
    if [ -z "$1" ]; then echo "Usage: dnetstat <container-name>"; return 1; fi
    echo -e "\e[1;36mActive connections inside '$1':\e[0m"
    docker exec -it "$1" netstat -tulan 2>/dev/null || docker exec -it "$1" ss -tulan 2>/dev/null || echo "Error: Neither netstat nor ss is installed in this container."
}

unalias dtop-proc 2>/dev/null
function dtop-proc {
    if [ -z "$1" ]; then echo "Usage: dtop-proc <container-name>"; return 1; fi
    docker top "$1" aux
}

unalias dbackup 2>/dev/null
function dbackup {
    docker run --rm -v "$1":/volume -v "$(pwd)":/backup alpine tar cvf /backup/"$2" -C /volume .
}

unalias dkill-force 2>/dev/null
function dkill-force {
    echo -e "\e[1;31m⚠️  WARNING: You are about to stop and remove ALL running containers!\e[0m"
    read "confirm?Are you sure? (y/N): "
    if [[ "$confirm" =~ ^[Yy]$ ]]; then
        docker stop $(docker ps -q) 2>/dev/null
        docker rm $(docker ps -a -q) 2>/dev/null
        echo -e "\e[1;32mDone. All containers cleared.\e[0m"
    else
        echo "Operation cancelled."
    fi
}

# --------------------------------------------------------------------
# ০০০. ডকার ড্যাশবোর্ড ও অল-ইন-ওয়ান টার্মিনাল ম্যানেজার (dman & dstats)
# --------------------------------------------------------------------

# dependency check & main launcher
unalias dman 2>/dev/null
dman() {
    # Dependency Check
    if ! command -v fzf >/dev/null 2>&1 || ! command -v gum >/dev/null 2>&1; then
        echo -e "\e[1;31mError: 'fzf' এবং 'gum' ইনস্টল করা নেই! (fzf and gum are required for dman)\e[0m"
        return 1
    fi

    while true; do
        clear
        local active_context
        active_context=$(docker context show 2>/dev/null || echo "default")

        gum style \
            --foreground 212 --border-foreground 212 --border double \
            --align center --width 68 --margin "1 0" --padding "0 2" \
            "🐳 DOCKER DESKTOP - DEVOPS TERMINAL EDITION" \
            "Active Context: $active_context | Use Mouse or Arrow Keys"

        local module
        module=$(gum choose \
            "📦 Container Manager" \
            "🖼️ Image Manager" \
            "💾 Volume Manager" \
            "🌐 Network Manager" \
            "🐙 Docker Compose Controls" \
            "📡 Switch Docker Context (Local/VPS)" \
            "⚡ Live Docker Events Stream" \
            "📊 Live Resource Dashboard" \
            "🧹 System Cleanup & Prune" \
            "❌ Exit")

        case "$module" in
            "📦 Container Manager") _manage_containers ;;
            "🖼️ Image Manager") _manage_images ;;
            "💾 Volume Manager") _manage_volumes ;;
            "🌐 Network Manager") _manage_networks ;;
            "🐙 Docker Compose Controls") _manage_compose ;;
            "📡 Switch Docker Context (Local/VPS)") _switch_docker_context ;;
            "⚡ Live Docker Events Stream") _view_docker_events ;;
            "📊 Live Resource Dashboard") dstats ;;
            "🧹 System Cleanup & Prune") dclean ;;
            "❌ Exit"|*) break ;;
        esac
    done
}

# Container Management TUI
unalias _manage_containers 2>/dev/null
_manage_containers() {
    while true; do
        clear
        if ! docker info >/dev/null 2>&1; then
            gum style --foreground 196 --bold "❌ Docker daemon is not running! Start it first."
            sleep 2
            return 1
        fi

        local cid
        cid=$(docker ps -a --format "table {{.ID}}\t{{.Names}}\t{{.Status}}\t{{.Image}}" 2>/dev/null | \
            fzf --header-lines=1 \
                --prompt="Select Container ❯ " \
                --pointer="▶" \
                --border \
                --height=15 \
                --preview="echo '--- [ LIVE LOGS ] ---' && docker logs --tail 25 {1} 2>/dev/null" \
                --preview-window=right:55%:wrap | awk '{print $1}')

        [ -z "$cid" ] && break

        local cname
        cname=$(docker inspect --format='{{.Name}}' "$cid" 2>/dev/null | sed 's/^\///')
        [ -z "$cname" ] && cname="$cid"

        echo ""
        gum style --foreground 214 --bold "Selected Container: $cname ($cid)"

        local action
        action=$(gum choose \
            "📋 View Logs (Live)" \
            "🐚 Shell Access (Bash/Sh)" \
            "🌐 Open Port in Browser" \
            "📂 File Transfer (Host ⇄ Container)" \
            "⚙️ Update Limits (CPU/RAM)" \
            "💾 Save Container as Image (Commit)" \
            "▶️ Start Container" \
            "⏹️ Stop Container" \
            "⏸️ Pause Container" \
            "▶️ Unpause Container" \
            "🔄 Restart Container" \
            "🔍 Inspect Config" \
            "🗑️ Delete Container" \
            "🔙 Back")

        case "$action" in
            "📋 View Logs (Live)")
                clear
                gum style --foreground 39 "Press Ctrl+C to exit logs..."
                docker logs -f --tail 100 "$cid"
                ;;
            "🐚 Shell Access (Bash/Sh)")
                clear
                docker exec -it "$cid" /bin/sh -c "bash || sh"
                ;;
            "🌐 Open Port in Browser")
                _open_container_port "$cid"
                ;;
            "📂 File Transfer (Host ⇄ Container)")
                _copy_files "$cid"
                ;;
            "⚙️ Update Limits (CPU/RAM)")
                _update_container_resources "$cid"
                ;;
            "💾 Save Container as Image (Commit)")
                _commit_container "$cid"
                ;;
            "▶️ Start Container")
                gum spin --spinner dot --title "Starting $cname..." -- docker start "$cid"
                ;;
            "⏹️ Stop Container")
                gum spin --spinner dot --title "Stopping $cname..." -- docker stop "$cid"
                ;;
            "⏸️ Pause Container")
                gum spin --spinner dot --title "Pausing $cname..." -- docker pause "$cid"
                ;;
            "▶️ Unpause Container")
                gum spin --spinner dot --title "Unpausing $cname..." -- docker unpause "$cid"
                ;;
            "🔄 Restart Container")
                gum spin --spinner dot --title "Restarting $cname..." -- docker restart "$cid"
                ;;
            "🔍 Inspect Config")
                docker inspect "$cid" | fzf --header="Inspect: $cname"
                ;;
            "🗑️ Delete Container")
                if gum confirm "Permanently remove container '$cname'?"; then
                    docker rm -f "$cid"
                fi
                ;;
            "🔙 Back"|*) continue ;;
        esac
    done
}

# Container Sub-functions
unalias _open_container_port 2>/dev/null
_open_container_port() {
    local cid=$1
    local ports
    ports=$(docker port "$cid" 2>/dev/null | awk '{print $3}' | awk -F: '{print $NF}' | sort -u)
    if [ -z "$ports" ]; then
        gum style --foreground 196 "❌ No published ports found for this container!"
        sleep 1.5
        return
    fi
    local selected_port
    selected_port=$(echo "$ports" | gum choose --header="Select Port to Open in Browser:")
    if [ -n "$selected_port" ]; then
        xdg-open "http://localhost:$selected_port" 2>/dev/null || xdg-open "http://127.0.0.1:$selected_port" 2>/dev/null || open "http://localhost:$selected_port" 2>/dev/null
    fi
}

unalias _copy_files 2>/dev/null
_copy_files() {
    local cid=$1
    local direction
    direction=$(gum choose "📥 Copy from Host to Container" "📤 Copy from Container to Host" "🔙 Cancel")

    if [[ "$direction" == "📥 Copy from Host to Container" ]]; then
        local src dest
        src=$(gum input --placeholder "Source Path on Host (e.g., ./app.conf)")
        dest=$(gum input --placeholder "Dest Path in Container (e.g., /etc/app.conf)")
        if [ -n "$src" ] && [ -n "$dest" ]; then
            if docker cp "$src" "$cid:$dest"; then
                gum style --foreground 46 "✔ File copied successfully!"
            else
                gum style --foreground 196 "❌ File copy failed!"
            fi
            sleep 1.5
        fi
    elif [[ "$direction" == "📤 Copy from Container to Host" ]]; then
        local src dest
        src=$(gum input --placeholder "Source Path in Container (e.g., /var/log/app.log)")
        dest=$(gum input --placeholder "Dest Path on Host (e.g., ./app.log)")
        if [ -n "$src" ] && [ -n "$dest" ]; then
            if docker cp "$cid:$src" "$dest"; then
                gum style --foreground 46 "✔ File copied successfully!"
            else
                gum style --foreground 196 "❌ File copy failed!"
            fi
            sleep 1.5
        fi
    fi
}

unalias _update_container_resources 2>/dev/null
_update_container_resources() {
    local cid=$1
    local mem cpus
    mem=$(gum input --placeholder "Memory Limit (e.g., 512m, 2g) - Leave empty to skip")
    cpus=$(gum input --placeholder "CPU Limit (e.g., 1.5, 2) - Leave empty to skip")

    if [ -n "$mem" ] || [ -n "$cpus" ]; then
        local opts=""
        [ -n "$mem" ] && opts="$opts --memory=$mem"
        [ -n "$cpus" ] && opts="$opts --cpus=$cpus"

        if docker update $opts "$cid"; then
            gum style --foreground 46 "✔ Resources updated successfully!"
        else
            gum style --foreground 196 "❌ Failed to update container resources!"
        fi
        sleep 1.5
    fi
}

unalias _commit_container 2>/dev/null
_commit_container() {
    local cid=$1
    local new_image
    new_image=$(gum input --placeholder "New Image Name (e.g., my-custom-app:v2)")
    if [ -n "$new_image" ]; then
        if gum spin --spinner dot --title "Creating image from container..." -- docker commit "$cid" "$new_image"; then
            gum style --foreground 46 "✔ Created Image: $new_image"
        else
            gum style --foreground 196 "❌ Failed to commit container image!"
        fi
        sleep 1.5
    fi
}

# Image Management TUI
unalias _manage_images 2>/dev/null
_manage_images() {
    while true; do
        clear
        if ! docker info >/dev/null 2>&1; then
            gum style --foreground 196 --bold "❌ Docker daemon is not running! Start it first."
            sleep 2
            return 1
        fi

        gum style --foreground 39 --bold "=== DOCKER IMAGES MODULE ==="
        local img_action
        img_action=$(gum choose \
            "📋 List & Inspect Local Images" \
            "📥 Pull Image from Docker Hub" \
            "🔨 Build Image from Local Dockerfile" \
            "🗑️ Remove Selected Image" \
            "🔙 Back")

        case "$img_action" in
            "📋 List & Inspect Local Images")
                local img_id
                img_id=$(docker images --format "table {{.Repository}}\t{{.Tag}}\t{{.ID}}\t{{.Size}}" 2>/dev/null | \
                    fzf --header-lines=1 --prompt="Select Image ❯ " | awk '{print $3}')
                [ -n "$img_id" ] && docker inspect "$img_id" | fzf --header="Image Inspect"
                ;;
            "📥 Pull Image from Docker Hub")
                local image_name
                image_name=$(gum input --placeholder "e.g. nginx:latest, postgres:alpine")
                if [ -n "$image_name" ]; then
                    gum spin --spinner globe --title "Pulling $image_name..." -- docker pull "$image_name"
                    gum style --foreground 46 "✔ Image pulled successfully!"
                    sleep 1.5
                fi
                ;;
            "🔨 Build Image from Local Dockerfile")
                if [ ! -f "Dockerfile" ] && [ ! -f "dockerfile" ]; then
                    gum style --foreground 196 "❌ No Dockerfile found in current directory!"
                    sleep 2
                    continue
                fi
                local tag_name
                tag_name=$(gum input --placeholder "Enter Tag Name (e.g. my-app:v1)")
                if [ -n "$tag_name" ]; then
                    docker build -t "$tag_name" .
                    printf "\nPress Enter to continue..."
                    read -r
                fi
                ;;
            "🗑️ Remove Selected Image")
                local img_id
                img_id=$(docker images --format "table {{.Repository}}\t{{.Tag}}\t{{.ID}}\t{{.Size}}" 2>/dev/null | \
                    fzf --header-lines=1 --prompt="Select Image to Delete ❯ " | awk '{print $3}')
                if [ -n "$img_id" ]; then
                    if gum confirm "Delete image $img_id?"; then
                        docker rmi "$img_id"
                        sleep 1
                    fi
                fi
                ;;
            "🔙 Back"|*) break ;;
        esac
    done
}

# Volume Management TUI
unalias _manage_volumes 2>/dev/null
_manage_volumes() {
    while true; do
        clear
        if ! docker info >/dev/null 2>&1; then
            gum style --foreground 196 --bold "❌ Docker daemon is not running! Start it first."
            sleep 2
            return 1
        fi

        gum style --foreground 214 --bold "=== DOCKER VOLUMES MODULE ==="
        local vol_action
        vol_action=$(gum choose \
            "📋 List Volumes" \
            "➕ Create New Volume" \
            "🔍 Inspect Volume" \
            "🗑️ Remove Volume" \
            "🔙 Back")

        case "$vol_action" in
            "📋 List Volumes")
                docker volume ls | fzf --header-lines=1 --prompt="Volumes ❯ "
                ;;
            "➕ Create New Volume")
                local vol_name
                vol_name=$(gum input --placeholder "Enter Volume Name")
                [ -n "$vol_name" ] && docker volume create "$vol_name" && sleep 1
                ;;
            "🔍 Inspect Volume")
                local vol_id
                vol_id=$(docker volume ls -q | fzf --prompt="Select Volume ❯ ")
                [ -n "$vol_id" ] && docker volume inspect "$vol_id" | fzf
                ;;
            "🗑️ Remove Volume")
                local vol_id
                vol_id=$(docker volume ls -q | fzf --prompt="Select Volume to Remove ❯ ")
                if [ -n "$vol_id" ]; then
                    if gum confirm "Remove Volume $vol_id?"; then
                        docker volume rm "$vol_id"
                        sleep 1
                    fi
                fi
                ;;
            "🔙 Back"|*) break ;;
        esac
    done
}

# Network Management TUI
unalias _manage_networks 2>/dev/null
_manage_networks() {
    while true; do
        clear
        if ! docker info >/dev/null 2>&1; then
            gum style --foreground 196 --bold "❌ Docker daemon is not running! Start it first."
            sleep 2
            return 1
        fi

        gum style --foreground 120 --bold "=== DOCKER NETWORKS MODULE ==="
        local net_action
        net_action=$(gum choose \
            "📋 List Networks" \
            "🔍 Inspect Network" \
            "🗑️ Remove Unused Networks" \
            "🔙 Back")

        case "$net_action" in
            "📋 List Networks")
                docker network ls | fzf --header-lines=1
                ;;
            "🔍 Inspect Network")
                local net_id
                net_id=$(docker network ls --format "table {{.ID}}\t{{.Name}}\t{{.Driver}}" 2>/dev/null | \
                    fzf --header-lines=1 | awk '{print $1}')
                [ -n "$net_id" ] && docker network inspect "$net_id" | fzf
                ;;
            "🗑️ Remove Unused Networks")
                if gum confirm "Prune unused networks?"; then
                    docker network prune -f
                    sleep 1
                fi
                ;;
            "🔙 Back"|*) break ;;
        esac
    done
}

# Docker Compose Management TUI
unalias _manage_compose 2>/dev/null
_manage_compose() {
    clear
    if [ ! -f "docker-compose.yml" ] && [ ! -f "compose.yaml" ] && [ ! -f "docker-compose.yaml" ] && [ ! -f "compose.yml" ]; then
        gum style --foreground 196 "❌ No docker-compose file found in current directory!"
        sleep 2
        return
    fi

    if ! docker info >/dev/null 2>&1; then
        gum style --foreground 196 --bold "❌ Docker daemon is not running! Start it first."
        sleep 2
        return 1
    fi

    gum style --foreground 208 --bold "=== DOCKER COMPOSE MODULE ==="
    local compose_action
    compose_action=$(gum choose \
        "🚀 Compose Up (-d)" \
        "🛑 Compose Down" \
        "🔄 Compose Restart" \
        "📋 Compose Live Logs" \
        "🔙 Back")

    case "$compose_action" in
        "🚀 Compose Up (-d)")
            docker compose up -d 2>/dev/null || docker-compose up -d
            sleep 2
            ;;
        "🛑 Compose Down")
            docker compose down 2>/dev/null || docker-compose down
            sleep 2
            ;;
        "🔄 Compose Restart")
            docker compose restart 2>/dev/null || docker-compose restart
            sleep 2
            ;;
        "📋 Compose Live Logs")
            docker compose logs -f 2>/dev/null || docker-compose logs -f
            ;;
        *) return ;;
    esac
}

# Docker Context Switcher
unalias _switch_docker_context 2>/dev/null
_switch_docker_context() {
    clear
    gum style --foreground 212 --bold "=== DOCKER CONTEXT SWITCHER ==="
    local context
    context=$(docker context ls --format "table {{.Name}}\t{{.Endpoint}}" 2>/dev/null | fzf --header-lines=1 --prompt="Select Context ❯ " | awk '{print $1}')
    if [ -n "$context" ]; then
        if docker context use "$context"; then
            gum style --foreground 46 "✔ Switched to Context: $context"
        else
            gum style --foreground 196 "❌ Failed to switch context to: $context"
        fi
        sleep 1.5
    fi
}

# Docker Events Stream
unalias _view_docker_events 2>/dev/null
_view_docker_events() {
    clear
    if ! docker info >/dev/null 2>&1; then
        gum style --foreground 196 --bold "❌ Docker daemon is not running! Start it first."
        sleep 2
        return 1
    fi
    gum style --foreground 39 "Press Ctrl+C to exit Docker Events Stream..."
    docker events --format 'Type: {{.Type}} | Action: {{.Action}} | Actor: {{.Actor.Attributes.name}} ({{.Time}})'
}

# Realtime Resource Monitor
unalias dstats 2>/dev/null
dstats() {
    clear
    if ! docker info >/dev/null 2>&1; then
        echo -e "\e[1;31m❌ Docker daemon is not running! Start it first with 'dstart'.\e[0m"
        sleep 2
        return 1
    fi
    if command -v gum >/dev/null 2>&1; then
        gum style --foreground 39 "Press Ctrl+C to exit Resource Dashboard..."
    else
        echo -e "\e[1;36mPress Ctrl+C to exit Resource Dashboard...\e[0m"
    fi
    watch -n 1 -c "docker stats --format 'table {{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.MemPerc}}\t{{.NetIO}}\t{{.BlockIO}}\t{{.PIDs}}'"
}

# আলটিমেট সিস্টেম ক্লিনআপ (অব্যবহৃত ক্যাশ, কন্টেইনার, ভলিউম ও ইমেজ ডিলিট করে জিবি জিবি জায়গা খালি করা)
unalias dclean 2>/dev/null
dclean() {
    clear
    if ! docker info >/dev/null 2>&1; then
        echo -e "\e[1;31m❌ Docker daemon is not running! Start it first with 'dstart'.\e[0m"
        sleep 2
        return 1
    fi
    if command -v gum >/dev/null 2>&1; then
        if gum confirm "Warning: This will delete ALL stopped containers, unused images, and dangling volumes!"; then
            gum spin --spinner monkey --title "Pruning Docker System..." -- docker system prune -a --volumes -f
            gum style --foreground 46 "✔ Full Cleanup Complete!"
            sleep 1.5
        fi
    else
        echo -e "\e[1;31m⚠️ Warning: This will delete ALL stopped containers, unused images, and dangling volumes!\e[0m"
        printf "Are you sure? (y/N): "
        read -r confirm
        if [[ "$confirm" =~ ^[Yy]$ ]]; then
            echo -e "\e[1;31m🧹 Performing deep clean of all unused Docker resources...\e[0m"
            docker system prune -a --volumes -f
            echo -e "\e[1;32m✨ Full Cleanup Complete!\e[0m"
            sleep 1.5
        else
            echo "Operation cancelled."
        fi
    fi
}

# --------------------------------------------------------------------
# ৭. ZSH ইন্টেলিজেন্ট ট্যাব কমপ্লিশন (Zsh Smart Tab Completion)
# --------------------------------------------------------------------

# Zsh-এর ডিফল্ট কমপ্লিট করার সিস্টেম (Unified single-pass initialization applied above)

# কন্টেইনারের নামের জন্য কমপ্লিশন ফাংশন
unalias _zsh_docker_containers 2>/dev/null
function _zsh_docker_containers {
    local -a containers
    containers=(${(f)"$(docker ps -a --format '{{.Names}}')"})
    _describe 'containers' containers
}

# ইমেজের নামের জন্য কমপ্লিশন ফাংশন
unalias _zsh_docker_images 2>/dev/null
function _zsh_docker_images {
    local -a images
    images=(${(f)"$(docker images --format '{{.Repository}}' | grep -v '<none>')"})
    _describe 'images' images
}

# নির্দিষ্ট শর্টকাট কমান্ডগুলোর সাথে কমপ্লিশন লিঙ্ক করা
if (( $+functions[compdef] )); then
    compdef _zsh_docker_containers dsh dlogs dstop dkill drestart dports dwatch dnetstat dtop-proc
    compdef _zsh_docker_images drmi dhist
fi

# --------------------------------------------------------------------
# ৮. স্টার্টআপ নোটিফায়ার (Startup Notifier)
# --------------------------------------------------------------------
if command -v docker &> /dev/null && systemctl is-active --quiet docker 2>/dev/null; then
    running_count=$(docker ps -q | wc -l)
    if [ "$running_count" -gt 0 ]; then
        echo -e "\e[1;36m🐳 Docker is active. Running containers: $running_count\e[0m"
    fi
fi



# =====================================================
# Advance C/C++ boilerplate generator
# =====================================================
unalias makecpp 2>/dev/null
function makecpp {
    if [[ -z "$1" ]]; then
        echo "❌ Error: Please provide a project name! (e.g., makecpp my_project)"
        return 1
    fi

    local lang_choice="$2"
    if [[ -z "$lang_choice" ]]; then
        if (( $+commands[fzf] )); then
            echo "🤔 Which language project do you want to create?"
            lang_choice=$(printf "cpp\nc\n" | fzf --prompt="Select Language > " --height=10 --layout=reverse)
            [[ -z "$lang_choice" ]] && lang_choice="cpp"
        else
            printf "🤔 Which language project do you want to create? (c/cpp) [default: cpp]: "
            read lang_choice
        fi
    fi

    local type="cpp"
    local compiler="g++"
    local file_ext="cpp"
    local flags="-std=c++17 -Wall -Wextra -O2"

    if [[ "$lang_choice" == "c" || "$lang_choice" == "C" ]]; then
        type="c"
        compiler="gcc"
        file_ext="c"
        flags="-Wall -Wextra -O2"
    fi

    echo "🚀 Creating Advance $type project: $1..."
    mkdir -p "$1" && cd "$1" || return

    # 1. Generate general boilerplate code
    if [[ "$type" == "c" ]]; then
        cat <<EOF > main.c
#include <stdio.h>

int main() {
    printf("Hello, World! Welcome to C project: %s\n", "$1");
    return 0;
}
EOF
    else
        cat <<EOF > main.cpp
#include <iostream>

int main() {
    std::cout << "Hello, World! Welcome to C++ project: " << "$1" << std::endl;
    return 0;
}
EOF
    fi

    # 2. Create a smart Makefile
    cat <<EOF > Makefile
CC = $compiler
CFLAGS = $flags
TARGET = main

all: \$(TARGET)

\$(TARGET): main.$file_ext
	\$(CC) \$(CFLAGS) main.$file_ext -o \$(TARGET)

run: \$(TARGET)
	./\$(TARGET)

clean:
	rm -f \$(TARGET)
EOF

    # 3. Auto-initialize Git and create .gitignore
    if (( $+commands[git] )); then
        git init -q
        echo -e "main\n*.o\n*.out\n.vscode/" > .gitignore
        echo "✅ Git repository initialized with .gitignore"
    fi

    echo "🎉 Project setup complete!"
    echo "📂 Current directory: $(pwd)"

    # 4. Open in VS Code automatically if available
    if (( $+commands[code] )); then
        echo "💻 Opening in VS Code..."
        code .
    fi
}

unalias t 2>/dev/null
function t {
    if [ $# -eq 0 ]; then
        echo "❌ Provide at least one filename."
        return 1
    fi
    touch "$@"
    for file in "$@"; do
        echo "✅ Created File: $file"
    done
}

# =====================================================
# 🚀 INTERACTIVE GUM & FZF UTILITIES
# =====================================================

# 1. Interactive Git Branch Switcher (FZF / Gum)

# 2. Interactive Git Branch Switcher (FZF / Gum)
gbranch() {
    if ! command -v git &>/dev/null; then
        echo "❌ Git is not installed."
        return 1
    fi

    if ! git rev-parse --is-inside-work-tree &>/dev/null; then
        echo "❌ Not inside a git repository."
        return 1
    fi

    local SCOPE="heads"
    local INCLUDE_ALL=1

    while [ $# -gt 0 ]; do
        case "$1" in
            -l|--local)
                SCOPE="heads"
                INCLUDE_ALL=0
                shift
                ;;
            -r|--remote)
                SCOPE="remotes"
                INCLUDE_ALL=0
                shift
                ;;
            -a|--all)
                INCLUDE_ALL=1
                shift
                ;;
            -h|--help)
                echo "🌿 gbranch — Modern & Interactive Git Branch Manager"
                echo ""
                echo "Usage: gbranch [options]"
                echo "Options:"
                echo "  -l, --local    Show local branches only"
                echo "  -r, --remote   Show remote branches only"
                echo "  -a, --all      Show all branches (default)"
                echo "  -h, --help     Show this help message"
                echo ""
                echo "Shortcuts in FZF:"
                echo "  Enter          Checkout selected branch"
                echo "  Ctrl-D         Delete local branch (git branch -D)"
                echo "  Ctrl-R         Rebase current branch onto selected branch"
                echo "  Ctrl-O         Merge selected branch into current branch"
                return 0
                ;;
            *)
                echo "❌ Unknown option: $1 (use -h for help)"
                return 1
                ;;
        esac
    done

    local REFS_PATTERN=()
    if [ "$INCLUDE_ALL" -eq 1 ]; then
        REFS_PATTERN=(refs/heads refs/remotes)
    elif [ "$SCOPE" = "remotes" ]; then
        REFS_PATTERN=(refs/remotes)
    else
        REFS_PATTERN=(refs/heads)
    fi

    if command -v fzf &>/dev/null; then
        local OUT
        OUT=$(git for-each-ref --sort=-committerdate "${REFS_PATTERN[@]}" \
            --format='%(refname:short) (%(committerdate:relative)) - %(subject)' 2>/dev/null | \
            grep -v -E '(^origin |/HEAD)' | \
            fzf --prompt="🌿 Branch > " \
                --header="[Enter] Checkout | [Ctrl-D] Delete | [Ctrl-R] Rebase | [Ctrl-O] Merge" \
                --expect=ctrl-d,ctrl-r,ctrl-o \
                --preview='git log --graph --color=always --format="%C(auto)%h %C(blue)%ad %C(yellow)%an %C(reset)%s" --date=relative -n 15 {1}' \
                --preview-window=right:55%:wrap)

        [ -z "$OUT" ] && return 0

        local KEY
        KEY=$(echo "$OUT" | head -n 1)
        local RAW_LINE
        RAW_LINE=$(echo "$OUT" | tail -n +2)
        [ -z "$RAW_LINE" ] && return 0

        local BRANCH
        BRANCH=$(echo "$RAW_LINE" | awk '{print $1}')
        local CLEAN_BRANCH
        CLEAN_BRANCH=$(echo "$BRANCH" | sed -e 's#^remotes/origin/##' -e 's#^origin/##')

        case "$KEY" in
            ctrl-d)
                echo -n "⚠️ Delete branch '$CLEAN_BRANCH'? [y/N] "
                read -r ans
                if [[ "$ans" =~ ^[Yy]$ ]]; then
                    git branch -D "$CLEAN_BRANCH"
                else
                    echo "Cancelled."
                fi
                ;;
            ctrl-r)
                echo "🔄 Rebasing onto '$CLEAN_BRANCH'..."
                git rebase "$CLEAN_BRANCH"
                ;;
            ctrl-o)
                echo "🔀 Merging '$CLEAN_BRANCH'..."
                git merge "$CLEAN_BRANCH"
                ;;
            *)
                echo "🚀 Checking out '$CLEAN_BRANCH'..."
                git checkout "$CLEAN_BRANCH"
                ;;
        esac
    elif command -v gum &>/dev/null; then
        local BRANCH
        BRANCH=$(git for-each-ref --sort=-committerdate "${REFS_PATTERN[@]}" --format='%(refname:short)' 2>/dev/null | grep -v -E '(^origin$|/HEAD)' | gum filter --height 10 --placeholder="Select branch...")
        if [ -n "$BRANCH" ]; then
            local CLEAN_BRANCH
            CLEAN_BRANCH=$(echo "$BRANCH" | sed -e 's#^remotes/origin/##' -e 's#^origin/##')
            git checkout "$CLEAN_BRANCH"
        fi
    else
        echo "🌿 Select a Git branch:"
        local BRANCHES=()
        while IFS= read -r line; do
            [ -n "$line" ] && BRANCHES+=("$line")
        done < <(git for-each-ref --sort=-committerdate "${REFS_PATTERN[@]}" --format='%(refname:short)' 2>/dev/null | grep -v -E '(^origin$|/HEAD)')

        if [ ${#BRANCHES[@]} -eq 0 ]; then
            echo "❌ No branches found."
            return 1
        fi

        local i=1
        for b in "${BRANCHES[@]}"; do
            echo "  [$i] $b"
            ((i++))
        done

        echo -n "Select branch number (1-${#BRANCHES[@]}): "
        read -r choice
        if [[ "$choice" =~ ^[0-9]+$ ]] && [ "$choice" -ge 1 ] && [ "$choice" -le "${#BRANCHES[@]}" ]; then
            local SELECTED="${BRANCHES[$((choice-1))]}"
            local CLEAN_BRANCH
            CLEAN_BRANCH=$(echo "$SELECTED" | sed -e 's#^remotes/origin/##' -e 's#^origin/##')
            git checkout "$CLEAN_BRANCH"
        fi
    fi
}

# 3. Advanced Interactive Process Killer (FZF / Gum)
fkill() {
    local QUERY="${1:-}"

    # Port-based search: fkill 3000 -> find & kill process on port 3000
    if [[ "$QUERY" =~ ^[0-9]+$ ]] && [ "$QUERY" -le 65535 ] 2>/dev/null; then
        local PORT_PID
        PORT_PID=$(ss -tlnp 2>/dev/null | awk -v port=":$QUERY" '$0 ~ port {match($0,/pid=([0-9]+)/,a); if(a[1]) print a[1]}' | head -n1)
        [ -z "$PORT_PID" ] && PORT_PID=$(lsof -t -i ":$QUERY" 2>/dev/null | head -n1)
        if [ -n "$PORT_PID" ]; then
            local PROC_NAME
            PROC_NAME=$(ps -p "$PORT_PID" -o comm= 2>/dev/null)
            echo -n "⚡ Kill '$PROC_NAME' (PID $PORT_PID) on port $QUERY? [y/N] "
            read -r ans
            if [[ "$ans" =~ ^[Yy]$ ]]; then
                kill -15 "$PORT_PID" 2>/dev/null && echo "✅ Soft killed PID $PORT_PID ($PROC_NAME)" && return 0
                sleep 1
                kill -9 "$PORT_PID" 2>/dev/null && echo "🔥 Force killed PID $PORT_PID ($PROC_NAME)"
            fi
            return 0
        else
            echo "❌ No process found on port $QUERY"
            return 1
        fi
    fi

    if command -v fzf &>/dev/null; then
        local PS_CMD='ps -eo pid,user,%cpu,%mem,etime,comm 2>/dev/null | sed 1d | sort -k3 -rn | awk '\''{
            user=$2; comm=$6;
            if (user == "root" || user ~ /^systemd/ || user ~ /^daemon/ || user ~ /^dbus/) {
                type="[SYS]";
            } else if (comm ~ /(chrome|code|cursor|firefox|slack|discord|spotify|vlc|zed|electron|idea|webstorm|pycharm|clion|goland|sublime|nautilus|dolphin|thunderbird|obs|gimp|inkscape|steam|telegram|figma|insomnia|postman|tableplus|dbeaver|brave|edge|opera|safari)/) {
                type="[APP]";
            } else {
                type="[USR]";
            }
            printf "%-7s %-6s %-8s %-5s %-5s %-8s %s\n", $1, type, $2, $3, $4, $5, $6;
        }'\'''

        local OUT
        OUT=$(eval "$PS_CMD" | \
            fzf -m --query="$QUERY" \
                --prompt="⚡ Kill > " \
                --header="[Tab] Multi-Select | [Enter] Soft Kill (SIGTERM) | [Ctrl-X] Force Kill (-9) | [Ctrl-R] Reload
Types: [APP] GUI App (Chrome/Code/etc) | [SYS] System Process | [USR] User CLI/Script" \
                --expect=ctrl-x,ctrl-r \
                --bind="ctrl-r:reload($PS_CMD)" \
                --preview='PID={1}; echo "Type:    {2}"; echo "PID:     $PID"; echo "User:    {3}"; echo "CPU:     {4}%"; echo "Mem:     {5}%"; echo "Uptime:  {6}"; echo "Command: {7}"; echo ""; echo "--- Ports ---"; ss -tlnp 2>/dev/null | grep -F "pid=$PID," | awk '\''{print $4}'\'' | sed '\''s/^/  /'\'' || echo "  (none)"' \
                --preview-window=bottom:30%:wrap)

        [ -z "$OUT" ] && return 0

        local KEY
        KEY=$(echo "$OUT" | head -n 1)
        [ "$KEY" = "ctrl-r" ] && return 0

        local PIDS
        PIDS=$(echo "$OUT" | tail -n +2 | awk '{print $1}')
        [ -z "$PIDS" ] && return 0

        local PID_LIST
        PID_LIST=$(echo "$PIDS" | tr '\n' ' ')

        if [ "$KEY" = "ctrl-x" ]; then
            echo "$PIDS" | xargs kill -9 2>/dev/null
            echo "🔥 Force killed (SIGKILL) PID(s): $PID_LIST"
        else
            echo "$PIDS" | xargs kill -15 2>/dev/null
            echo "✅ Soft killed (SIGTERM) PID(s): $PID_LIST"
        fi

    elif command -v gum &>/dev/null; then
        local SELECTED
        SELECTED=$(ps -eo pid,user,%cpu,%mem,comm 2>/dev/null | sed 1d | sort -k3 -rn | \
            gum filter --height 12 --placeholder="Search process...")
        if [ -n "$SELECTED" ]; then
            local PID NAME
            PID=$(echo "$SELECTED" | awk '{print $1}')
            NAME=$(echo "$SELECTED" | awk '{print $5}')
            if gum confirm "Kill '$NAME' (PID $PID)?"; then
                kill -15 "$PID" 2>/dev/null && gum style --foreground 82 "✅ Soft killed '$NAME' (PID $PID)"
            fi
        fi
    else
        echo "⚡ Running processes (sorted by CPU):"
        local PROCS=()
        while IFS= read -r line; do
            [ -n "$line" ] && PROCS+=("$line")
        done < <(ps -eo pid,user,%cpu,%mem,comm 2>/dev/null | sed 1d | sort -k3 -rn | head -n 20)

        local i=1
        for p in "${PROCS[@]}"; do
            printf "  [%2d] %s\n" "$i" "$p"
            ((i++))
        done

        echo -n "Select process to kill (1-${#PROCS[@]}): "
        read -r choice
        if [[ "$choice" =~ ^[0-9]+$ ]] && [ "$choice" -ge 1 ] && [ "$choice" -le "${#PROCS[@]}" ]; then
            local SELECTED="${PROCS[$((choice-1))]}"
            local PID NAME
            PID=$(echo "$SELECTED" | awk '{print $1}')
            NAME=$(echo "$SELECTED" | awk '{print $5}')
            echo -n "Kill '$NAME' (PID $PID)? [y/N] "
            read -r ans
            if [[ "$ans" =~ ^[Yy]$ ]]; then
                kill -15 "$PID" 2>/dev/null && echo "✅ Soft killed '$NAME' (PID $PID)"
            fi
        fi
    fi
}

# 4. Interactive Quick Directory Search & CD
fcd() {
    local DIR=""
    if command -v fzf &>/dev/null; then
        DIR=$(find . -maxdepth 4 -not -path '*/.*' -type d 2>/dev/null | fzf --prompt="Select Directory: ")
    elif command -v gum &>/dev/null; then
        DIR=$(find . -maxdepth 4 -not -path '*/.*' -type d 2>/dev/null | gum filter --height 5 --placeholder="Select Directory...")
    fi

    if [ -n "$DIR" ]; then
        cd "$DIR" || return
    fi
}

# =====================================================
# Zsh Plugins (Already loaded above in unified block)
# =====================================================

# =====================================================
# 📋 TODO & NOTES UTILITIES (Zsh | Linux/macOS/BSD)
# Cross-shell safe: zsh 5+ and bash 4+
# All bugs fixed — see ARCHITECTURE.txt for details
# =====================================================

# ---------------------------------------------------------------------------
# Helper: Portable in-place line deletion.
# GNU sed (Linux) uses `sed -i "Nd"`.
# BSD sed (macOS/FreeBSD) requires `sed -i '' "Nd"`.
# Detection: GNU sed responds to --version; BSD sed does not.
# ---------------------------------------------------------------------------
_fb_sed_delete_line() {
    local n="$1" file="$2"
    if sed --version >/dev/null 2>&1; then
        sed -i "${n}d" "$file"
    else
        sed -i '' "${n}d" "$file"
    fi
}

# ---------------------------------------------------------------------------
# Helper: Clipboard copy — Wayland → X11/xclip → X11/xsel → macOS pbcopy.
# Falls back gracefully with an informative error message.
# ---------------------------------------------------------------------------
_fb_copy_to_clipboard() {
    local content="$1"

    # Try Wayland first if WAYLAND_DISPLAY is active
    if [ -n "${WAYLAND_DISPLAY:-}" ] && command -v wl-copy >/dev/null 2>&1; then
        if printf '%s' "$content" | wl-copy 2>/dev/null; then
            printf '📋 Copied to Wayland clipboard!\n'
            return 0
        fi
    fi

    # Try X11 xclip
    if command -v xclip >/dev/null 2>&1; then
        if printf '%s' "$content" | xclip -selection clipboard 2>/dev/null; then
            printf '📋 Copied to X11 clipboard!\n'
            return 0
        fi
    fi

    # Try X11 xsel
    if command -v xsel >/dev/null 2>&1; then
        if printf '%s' "$content" | xsel --clipboard --input 2>/dev/null; then
            printf '📋 Copied to X11 clipboard!\n'
            return 0
        fi
    fi

    # Try macOS pbcopy
    if command -v pbcopy >/dev/null 2>&1; then
        if printf '%s' "$content" | pbcopy 2>/dev/null; then
            printf '📋 Copied to macOS clipboard!\n'
            return 0
        fi
    fi

    # Fallback attempt wl-copy if WAYLAND_DISPLAY wasn't exported but server exists
    if command -v wl-copy >/dev/null 2>&1; then
        if printf '%s' "$content" | wl-copy 2>/dev/null; then
            printf '📋 Copied to Wayland clipboard!\n'
            return 0
        fi
    fi

    printf '❌ Clipboard copy failed (no working display or clipboard tool found).\n'
    return 1
}

# ---------------------------------------------------------------------------
# Helper: Print numbered task list. Shared across todo sub-commands.
# ---------------------------------------------------------------------------
_fb_todo_show_list() {
    local file="$1"
    if [ -s "$file" ]; then
        printf '\n--- 📋 YOUR TO-DO LIST ---\n'
        nl -w2 -s'. ' "$file"
        printf '\n'
    else
        printf '📋 No tasks pending! Super productive 🎉\n'
    fi
}

# ------------------------------------------------------------------------------
# 1. TODO MANAGER
# Usage: todo | todo add "Task" | todo list | todo done [num] | todo clear
# Deps:  none required — fzf/gum are optional for interactive mode
# Fixes: out-of-range number crash | sed regex injection | echo -e portability
#        local+cmd exit code loss | inconsistent tool detection
# ------------------------------------------------------------------------------
todo() {
    local TODO_FILE="$HOME/.todo_list.txt"
    touch "$TODO_FILE"

    local action="${1:-}"

    case "$action" in

        add)
            shift
            local task="${*:-}"
            # Prompt interactively when task not passed as argument
            if [ -z "$task" ]; then
                if command -v gum >/dev/null 2>&1; then
                    task=$(gum input --placeholder "Type your task here...")
                else
                    printf 'Task: '
                    read -r task
                fi
            fi
            if [ -n "$task" ]; then
                printf '%s\n' "$task" >> "$TODO_FILE"
                printf '✔ Added: "%s"\n' "$task"
            else
                printf '❌ Task cannot be empty!\n'
            fi
            ;;

        done|rm)
            shift
            local num="${1:-}"

            if [ ! -s "$TODO_FILE" ]; then
                printf '📋 No tasks to complete!\n'
                return 0
            fi

            local total
            total=$(wc -l < "$TODO_FILE")

            # ── Branch 1: direct line number provided ──────────────────────
            if [[ "$num" =~ ^[0-9]+$ ]]; then
                # BUG FIX: validate range — prevents sed crash on line 0
                # or on a number larger than the file (both silent data-loss bugs)
                if (( num < 1 || num > total )); then
                    printf '❌ Invalid task number! Valid range: 1–%d\n' "$total"
                    return 1
                fi
                # BUG FIX: separate local declaration from command substitution
                # so that a failed `sed` is not swallowed by `local`'s exit-0
                local task_text
                task_text=$(sed -n "${num}p" "$TODO_FILE")
                _fb_sed_delete_line "$num" "$TODO_FILE"
                printf '🎉 Completed: "%s"\n' "$task_text"
                return 0
            fi

            # ── Branch 2: fzf interactive selection ────────────────────────
            if command -v fzf >/dev/null 2>&1; then
                local selected
                selected=$(cat -n "$TODO_FILE" | fzf \
                    --prompt="Select task to complete ➔ " \
                    --height=40% --reverse --border)
                if [ -n "$selected" ]; then
                    local line_num task_text
                    line_num=$(printf '%s' "$selected" | awk '{print $1}')
                    # awk strips leading whitespace+number cleanly — no sed regex needed
                    task_text=$(printf '%s' "$selected" | awk '{$1=""; sub(/^[[:space:]]+/,""); print}')
                    _fb_sed_delete_line "$line_num" "$TODO_FILE"
                    printf '🎉 Completed: "%s"\n' "$task_text"
                fi

            # ── Branch 3: gum interactive selection ────────────────────────
            elif command -v gum >/dev/null 2>&1; then
                local task_to_remove
                task_to_remove=$(gum choose --header="Select task to mark as Done:" < "$TODO_FILE")
                if [ -n "$task_to_remove" ]; then
                    # BUG FIX: grep -F (fixed-string) — immune to regex injection.
                    # Original code used sed with user data as a regex pattern,
                    # which crashed on tasks containing / & . * ^ $ [ etc.
                    local match_line
                    match_line=$(grep -Fn "$task_to_remove" "$TODO_FILE" 2>/dev/null \
                        | head -1 | cut -d: -f1)
                    if [ -n "$match_line" ]; then
                        _fb_sed_delete_line "$match_line" "$TODO_FILE"
                        printf '🎉 Completed: "%s"\n' "$task_to_remove"
                    else
                        printf '❌ Could not locate the task in file!\n'
                    fi
                fi

            # ── Branch 4: no interactive tool — show numbered list ──────────
            else
                printf '❌ Pass a task number (e.g. todo done 1) or install fzf/gum!\n'
                _fb_todo_show_list "$TODO_FILE"
            fi
            ;;

        clear)
            > "$TODO_FILE"
            printf '🗑️  All tasks cleared!\n'
            ;;

        list|ls)
            _fb_todo_show_list "$TODO_FILE"
            ;;

        -h|--help)
            printf 'Usage:\n'
            printf '  todo              – Open interactive menu / view tasks\n'
            printf '  todo add <task>   – Add a new task\n'
            printf '  todo list         – List all tasks\n'
            printf '  todo done [num]   – Complete a task (interactive if no num)\n'
            printf '  todo clear        – Clear all tasks\n'
            ;;

        "")
            # Full gum menu when available; plain numbered list otherwise
            if command -v gum >/dev/null 2>&1; then
                gum style \
                    --foreground 212 --border normal \
                    --margin "1" --padding "1" \
                    "✨ FANCYBASH TO-DO MANAGER ✨"
                local MENU_CHOICE
                MENU_CHOICE=$(gum choose \
                    "➕ Add Task" "✅ Complete Task" \
                    "📋 View Tasks" "🗑️  Clear All" "❌ Exit")
                case "$MENU_CHOICE" in
                    "➕ Add Task")      todo add ;;
                    "✅ Complete Task") todo done ;;
                    "📋 View Tasks")    _fb_todo_show_list "$TODO_FILE" ;;
                    "🗑️  Clear All")
                        if gum confirm "Are you sure you want to clear all tasks?"; then
                            todo clear
                        fi
                        ;;
                    *) return 0 ;;
                esac
            else
                _fb_todo_show_list "$TODO_FILE"
            fi
            ;;

        *)
            printf '❌ Unknown command: %s\n' "$action"
            printf "Run 'todo --help' for usage.\n"
            return 1
            ;;
    esac
}

# ------------------------------------------------------------------------------
# 2. NOTES MANAGER
# Usage: notes | notes add | notes search | notes --help
# Deps:  none required — fzf/gum/bat/glow are optional
# Fixes: no fallback in notes add | fzf crash when missing | bat preview broken
#        grep regex injection in search | echo -e portability | ls -d error
# ------------------------------------------------------------------------------
_fb_get_note_body() {
    local file="$1"
    if grep -q '^---*' "$file" 2>/dev/null; then
        awk 'flag{print} /^---*$/ && !flag{flag=1}' "$file" | sed '1{/^$/d}'
    else
        cat "$file"
    fi
}

_fb_has_vscode() {
    command -v code >/dev/null 2>&1 || command -v codium >/dev/null 2>&1 || { command -v flatpak >/dev/null 2>&1 && flatpak info com.visualstudio.code >/dev/null 2>&1; }
}

_fb_open_vscode() {
    local target="$1"
    if command -v code >/dev/null 2>&1; then
        code "$target"
    elif command -v codium >/dev/null 2>&1; then
        codium "$target"
    elif command -v flatpak >/dev/null 2>&1 && flatpak info com.visualstudio.code >/dev/null 2>&1; then
        flatpak run com.visualstudio.code "$target"
    fi
}

notes() {
    local NOTE_DIR="$HOME/.my_notes"
    mkdir -p "$NOTE_DIR/General"

    local action="${1:-}"

    # Pick the best available markdown/syntax previewer for fzf --preview
    local PREVIEW_CMD
    if command -v bat >/dev/null 2>&1; then
        PREVIEW_CMD="bat --color=always --style=numbers,changes"
    elif command -v batcat >/dev/null 2>&1; then
        PREVIEW_CMD="batcat --color=always --style=numbers,changes"
    elif command -v glow >/dev/null 2>&1; then
        PREVIEW_CMD="glow -s dark"
    else
        PREVIEW_CMD="cat"
    fi

    case "$action" in

        add)
            local category=""

            # ── Category selection: fzf → gum → plain read ─────────────────
            # BUG FIX: original used `ls -d */` which errors if no dirs exist;
            # replaced with `find -mindepth 1 -maxdepth 1 -type d`.
            # BUG FIX: `notes add` had zero fallback when gum was absent.
            if command -v fzf >/dev/null 2>&1; then
                local cats
                cats=$(find "$NOTE_DIR" -mindepth 1 -maxdepth 1 -type d \
                    -exec basename {} \; 2>/dev/null)
                category=$(printf '➕ Create New Category\n%s\n' "$cats" | \
                    grep -v '^$' | \
                    fzf --prompt="📁 Select Category ➔ " \
                        --height=40% --border)

            elif command -v gum >/dev/null 2>&1; then
                local cats
                cats=$(find "$NOTE_DIR" -mindepth 1 -maxdepth 1 -type d \
                    -exec basename {} \; 2>/dev/null)
                category=$(printf '➕ Create New Category\n%s\n' "$cats" | \
                    grep -v '^$' | \
                    gum choose --header="📁 Select Category:")

            else
                printf 'Available categories:\n'
                find "$NOTE_DIR" -mindepth 1 -maxdepth 1 -type d \
                    -exec basename {} \; 2>/dev/null | nl -w2 -s'. '
                printf 'Category name (Enter = General): '
                read -r category
                [ -z "$category" ] && category="General"
            fi

            [ -z "$category" ] && return 0

            if [ "$category" = "➕ Create New Category" ]; then
                if command -v gum >/dev/null 2>&1; then
                    category=$(gum input --placeholder "New category name...")
                else
                    printf 'New category name: '
                    read -r category
                fi
                [ -z "$category" ] && return 0
                mkdir -p "$NOTE_DIR/$category"
            fi

            # ── Note title ──────────────────────────────────────────────────
            local title=""
            if command -v gum >/dev/null 2>&1; then
                title=$(gum input --placeholder "Note Title (e.g. Docker Commands)...")
            else
                printf 'Note title: '
                read -r title
            fi
            [ -z "$title" ] && return 0

            local file_path="$NOTE_DIR/$category/$title.txt"
            mkdir -p "$NOTE_DIR/$category"

            # ── Overwrite check ─────────────────────────────────────────────
            if [ -f "$file_path" ]; then
                local overwrite="n"
                if command -v gum >/dev/null 2>&1; then
                    gum confirm "Note already exists! Overwrite?" && overwrite="y"
                else
                    printf 'Note already exists! Overwrite? [y/N]: '
                    read -r overwrite
                fi
                [[ "$overwrite" != "y" && "$overwrite" != "Y" ]] && return 0
            fi

            # Write plain-text header (no markdown — anyone can read it)
            printf '%s\n' "$title" > "$file_path"
            printf 'Created: %s\n%s\n\n' "$(date '+%Y-%m-%d %H:%M:%S')" "$(printf -- '-%.0s' {1..40})" >> "$file_path"

            # ── Content input: gum write → $EDITOR / VS Code ─────────────
            if command -v gum >/dev/null 2>&1; then
                local mode
                if _fb_has_vscode; then
                    mode=$(gum choose \
                        "📥 Quick Input via Gum" \
                        "💻 Open in VS Code" \
                        "📝 Use CLI Editor (${EDITOR:-nano})")
                else
                    mode=$(gum choose \
                        "📥 Quick Input via Gum" \
                        "📝 Use CLI Editor (${EDITOR:-nano})")
                fi

                if [[ "$mode" == *"Quick Input"* ]]; then
                    local content
                    content=$(gum write --placeholder "Type your note here... (Ctrl+D to save)")
                    printf '%s\n' "$content" >> "$file_path"
                elif [[ "$mode" == *"VS Code"* ]]; then
                    _fb_open_vscode "$file_path"
                else
                    ${EDITOR:-nano} "$file_path"
                fi
            else
                if _fb_has_vscode; then
                    printf 'Select editor mode:\n'
                    printf '  1. 📥 Quick Input via Terminal\n'
                    printf '  2. 💻 VS Code\n'
                    printf '  3. 📝 CLI Editor (%s)\n' "${EDITOR:-nano}"
                    read -r "?Choose [1-3] (default: 1): " choice </dev/tty
                    case "$choice" in
                        2) _fb_open_vscode "$file_path" ;;
                        3) ${EDITOR:-nano} "$file_path" ;;
                        *)
                            printf 'Enter note content (Ctrl+D to finish):\n'
                            cat >> "$file_path"
                            ;;
                    esac
                else
                    printf 'Select editor mode:\n'
                    printf '  1. 📥 Quick Input via Terminal\n'
                    printf '  2. 📝 CLI Editor (%s)\n' "${EDITOR:-nano}"
                    read -r "?Choose [1-2] (default: 1): " choice </dev/tty
                    case "$choice" in
                        2) ${EDITOR:-nano} "$file_path" ;;
                        *)
                            printf 'Enter note content (Ctrl+D to finish):\n'
                            cat >> "$file_path"
                            ;;
                    esac
                fi
            fi

            printf '✔ Saved to [%s/%s.txt]\n' "$category" "$title"
            ;;

        search|find)
            local query=""
            if command -v gum >/dev/null 2>&1; then
                query=$(gum input --placeholder "Type text to search inside notes...")
            else
                printf 'Search query: '
                read -r query
            fi
            [ -z "$query" ] && return 0

            # ── fzf-less fallback: plain grep ───────────────────────────────
            if ! command -v fzf >/dev/null 2>&1; then
                printf '⚠️  fzf not found — showing raw grep results:\n\n'
                # BUG FIX: grep -F (fixed-string) prevents query being
                # treated as a regex — avoids injection if query has . * + etc.
                grep -rnF "$query" "$NOTE_DIR" 2>/dev/null
                return 0
            fi

            local match_line
            match_line=$(grep -rnF "$query" "$NOTE_DIR" 2>/dev/null | \
                while IFS= read -r line; do
                    local f="$(printf '%s' "$line" | cut -d: -f1)"
                    local rest="${line#$f:}"
                    local rel="${f#$NOTE_DIR/}"
                    printf '%s:%s\t%s\n' "$rel" "$rest" "$line"
                done | fzf \
                --height=60% --border \
                --delimiter='\t' --with-nth=1 \
                --prompt="Matching Lines ➔ " \
                --preview 'f=$(echo {2} | cut -d: -f1); l=$(echo {2} | cut -d: -f2); { command -v bat >/dev/null 2>&1 && bat --color=always --highlight-line "$l" "$f" 2>/dev/null; } || { command -v batcat >/dev/null 2>&1 && batcat --color=always --highlight-line "$l" "$f" 2>/dev/null; } || cat "$f"')

            if [ -n "$match_line" ]; then
                local raw_match file
                raw_match=$(printf '%s' "$match_line" | cut -f2)
                file=$(printf '%s' "$raw_match" | cut -d: -f1)
                ${EDITOR:-nano} "$file"
            fi
            ;;

        -h|--help)
            printf 'Usage:\n'
            printf '  notes          – Browse and preview notes interactively\n'
            printf '  notes add      – Add a new note under a category\n'
            printf '  notes search   – Search text inside all notes\n'
            ;;

        "")
            # ── Plain list fallback when fzf is absent ──────────────────────
            # BUG FIX: original code crashed here when fzf was not installed
            if ! command -v fzf >/dev/null 2>&1; then
                printf '📁 Notes in %s:\n\n' "$NOTE_DIR"
                find "$NOTE_DIR" -type f -name "*.txt" 2>/dev/null | \
                    while IFS= read -r f; do
                        printf '  [%s] %s\n' \
                            "$(basename "$(dirname "$f")")" \
                            "$(basename "$f" .txt)"
                    done
                return 0
            fi

            local selected_line
            selected_line=$(find "$NOTE_DIR" -type f -name "*.txt" 2>/dev/null | \
                while IFS= read -r f; do
                    local rel="${f#$NOTE_DIR/}"
                    local cat_name="$(dirname "$rel")"
                    local title_name="$(basename "$rel" .txt)"
                    printf '📁 %s ➔ %s\t%s\n' "$cat_name" "$title_name" "$f"
                done | fzf \
                --height=70% --reverse --border \
                --delimiter='\t' --with-nth=1 \
                --prompt="🔎 Search Notes ➔ " \
                --preview "$PREVIEW_CMD {2} 2>/dev/null || cat {2}" \
                --preview-window=right:60%)

            [ -z "$selected_line" ] && return 0
            local selected
            selected=$(printf '%s' "$selected_line" | cut -f2)

            local title category note_action
            title=$(basename "$selected" .txt)
            category=$(basename "$(dirname "$selected")")

            # ── Action menu: gum → plain numbered prompt ────────────────────
            if command -v gum >/dev/null 2>&1; then
                gum style --foreground 212 --border normal \
                    "📝 Note: $title | 📁 Category: $category"
                if _fb_has_vscode; then
                    note_action=$(gum choose \
                        "👁️  Full View" "✏️  Edit Note" "💻 Open in VS Code" \
                        "📋 Copy Content" "🗑️  Delete Note" "🔙 Cancel")
                else
                    note_action=$(gum choose \
                        "👁️  Full View" "✏️  Edit Note" \
                        "📋 Copy Content" "🗑️  Delete Note" "🔙 Cancel")
                fi
            else
                printf '\n📝 Note: %s | 📁 Category: %s\n\n' "$title" "$category"
                printf '  1. 👁️  Full View\n'
                printf '  2. ✏️  Edit Note\n'
                printf '  3. 📋 Copy Content\n'
                printf '  4. 🗑️  Delete Note\n'
                printf '  5. Cancel\n\n'
                printf 'Choose [1-5]: '
                local choice
                read -r choice
                case "$choice" in
                    1) note_action="👁️  Full View" ;;
                    2) note_action="✏️  Edit Note" ;;
                    3) note_action="📋 Copy Content" ;;
                    4) note_action="🗑️  Delete Note" ;;
                    *) return 0 ;;
                esac
            fi

            case "$note_action" in
                "👁️  Full View")
                    if command -v glow >/dev/null 2>&1; then
                        glow -p "$selected"
                    elif command -v bat >/dev/null 2>&1; then
                        bat "$selected"
                    elif command -v batcat >/dev/null 2>&1; then
                        batcat "$selected"
                    else
                        less "$selected"
                    fi
                    ;;
                "✏️  Edit Note")
                    ${EDITOR:-nano} "$selected"
                    ;;
                "💻 Open in VS Code")
                    _fb_open_vscode "$selected"
                    ;;
                "📋 Copy Content")
                    _fb_copy_to_clipboard "$(_fb_get_note_body "$selected")"
                    ;;
                "🗑️  Delete Note")
                    local confirm_del="n"
                    if command -v gum >/dev/null 2>&1; then
                        gum confirm "Delete '$title'?" && confirm_del="y"
                    else
                        printf "Delete '%s'? [y/N]: " "$title"
                        read -r confirm_del
                    fi
                    if [[ "$confirm_del" == "y" || "$confirm_del" == "Y" ]]; then
                        rm "$selected"
                        printf '🗑️  Note deleted!\n'
                    fi
                    ;;
            esac
            ;;

        *)
            printf '❌ Unknown command: %s\n' "$action"
            printf "Run 'notes --help' for usage.\n"
            return 1
            ;;
    esac
}

# ==============================================================================
# 🎬 3. FFMPEG MULTIMEDIA SUITE (ffmedia / ffstudio / fftool / fancy_ffmpeg)
# Usage: ffmedia [action]
# Interactive multimedia toolbox leveraging ffmpeg, fzf, and gum.
# ==============================================================================

_fb_media_select_file() {
    local prompt="${1:-Select File:}"
    local pattern="${2:-}"
    local selected=""

    if command -v fzf >/dev/null 2>&1; then
        if [ -n "$pattern" ]; then
            selected=$(find . -maxdepth 3 -type f 2>/dev/null | grep -iE "$pattern" | fzf --prompt="$prompt " --height=40% --reverse)
        else
            selected=$(find . -maxdepth 3 -type f 2>/dev/null | fzf --prompt="$prompt " --height=40% --reverse)
        fi
    elif command -v gum >/dev/null 2>&1; then
        selected=$(gum file .)
    fi

    if [ -z "$selected" ]; then
        printf '%s ' "$prompt"
        read -r -e selected
    fi
    echo "$selected"
}

_fb_media_choose_opt() {
    local title="$1"
    shift
    local options=("$@")
    local choice=""

    if command -v gum >/dev/null 2>&1; then
        local term_rows
        term_rows=$(stty size 2>/dev/null | awk '{print $1}')
        term_rows=${term_rows:-$(tput lines 2>/dev/null)}
        term_rows=${term_rows:-${LINES:-15}}
        local choose_h=$(( term_rows - 2 ))
        (( choose_h > 15 )) && choose_h=15
        (( choose_h < 3 )) && choose_h=3

        gum style --foreground 212 --bold "$title"
        choice=$(printf '%s\n' "${options[@]}" | gum choose --height="$choose_h")
    elif command -v fzf >/dev/null 2>&1; then
        choice=$(printf '%s\n' "${options[@]}" | fzf --prompt="$title ➔ " --height=40% --reverse)
    else
        printf '\n=== %s ===\n' "$title"
        local idx=1
        for opt in "${options[@]}"; do
            printf '  %2d) %s\n' "$idx" "$opt"
            ((idx++))
        done
        printf 'Select option [1-%d]: ' "${#options[@]}"
        local num
        read -r num
        if [[ "$num" =~ ^[0-9]+$ ]] && (( num >= 1 && num <= ${#options[@]} )); then
            choice="${options[$((num-1))]}"
        fi
    fi
    echo "$choice"
}

_fb_media_input() {
    local prompt="$1"
    local default_val="${2:-}"
    local val=""

    if command -v gum >/dev/null 2>&1; then
        val=$(gum input --placeholder "$prompt" --value "$default_val")
    else
        if [ -n "$default_val" ]; then
            printf '%s [%s]: ' "$prompt" "$default_val"
        else
            printf '%s: ' "$prompt"
        fi
        read -r val
        [ -z "$val" ] && val="$default_val"
    fi
    echo "$val"
}

ffmedia() {
    fancybash ensure-dep ffmpeg ffmpeg ffmpeg ffmpeg || return 1

    local action="${1:-}"

    if [ -z "$action" ]; then
        action=$(_fb_media_choose_opt "🎬 FFmedia All-in-One Multimedia Suite" \
            "1. 📦 Compress Video (50%-80% size reduction)" \
            "2. ✂️  Fast Lossless Trim (Instant cut)" \
            "3. 🔗 Concat Videos (Merge clips)" \
            "4. 📐 Resolution & Aspect Ratio (1080p/720p/9:16 Reels)" \
            "5. ⏩ Speed Control (Slow Motion / Time-lapse)" \
            "6. 🔄 Rotate & Flip Video" \
            "7. 🏷️  Watermark & Branding (Logo / Text)" \
            "8. 🖼️  Video Grid & Side-by-Side Comparison" \
            "9. 🎵 Extract Audio (MP3/AAC/WAV/FLAC)" \
            "10. 🔇 Mute Video (Remove audio stream)" \
            "11. 🎧 Replace / Merge Background Audio" \
            "12. 🎚️  Loudness Normalization (-14 / -23 LUFS)" \
            "13. 🌊 Audio Visualizer (Waveform/Spectrum Video)" \
            "14. 🎤 Audio Speed Control (Pitch Preserved)" \
            "15. 📸 Snapshot Capture (Ultra HD JPG/PNG)" \
            "16. 🎞️  Bulk Frame Extraction" \
            "17. 🎨 Pro Quality GIF Creation" \
            "18. 🔲 Video Contact Sheet (Mosaic Grid Thumbnail)" \
            "19. 🎥 Terminal Screen Recorder" \
            "20. ✍️  Subtitle Burn-in (Hardcode SRT/ASS)" \
            "21. 📄 Subtitle Extraction" \
            "22. 🔒 Privacy Clean (Remove EXIF/GPS Metadata)" \
            "23. 🔄 Format Conversion (MP4/MKV/WEBM/MOV/AVI)" \
            "24. ⚡ Batch / Bulk File Processing" \
            "❌ Exit")
    fi

    [ -z "$action" ] || [[ "$action" == *"Exit"* ]] && return 0

    case "$action" in
        "1. "*|*"Compress"*|compress)
            local file=$(_fb_media_select_file "Select video to compress:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$file" ] && return 0
            local preset=$(_fb_media_choose_opt "Select Compression Preset:" \
                "1) Balanced Quality (CRF 23 - Recommended)" \
                "2) High Compression (CRF 28 - ~50-70% size reduction)" \
                "3) Extreme Compression (CRF 32 - ~70-80% size reduction)")
            local crf=23
            [[ "$preset" == *"2)"* ]] && crf=28
            [[ "$preset" == *"3)"* ]] && crf=32
            local ext="${file##*.}"
            local base="${file%.*}"
            local out="${base}_compressed.${ext}"
            printf '\n⚡ Compressing "%s" (CRF %s)...\n' "$file" "$crf"
            ffmpeg -i "$file" -vcodec libx264 -crf "$crf" -preset fast -acodec aac "$out"
            printf '\n✅ Done! Output saved as: %s\n' "$out"
            ;;

        "2. "*|*"Trim"*|trim)
            local file=$(_fb_media_select_file "Select video to trim:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$file" ] && return 0
            local start=$(_fb_media_input "Start timestamp (HH:MM:SS or seconds)" "00:00:00")
            local dur=$(_fb_media_input "Duration (HH:MM:SS or seconds, e.g. 10)" "00:00:10")
            local ext="${file##*.}"
            local base="${file%.*}"
            local out="${base}_trimmed.${ext}"
            printf '\n⚡ Trimming "%s" (Start: %s, Duration: %s)...\n' "$file" "$start" "$dur"
            ffmpeg -ss "$start" -i "$file" -t "$dur" -c copy "$out"
            printf '\n✅ Lossless trim complete: %s\n' "$out"
            ;;

        "3. "*|*"Concat"*|concat)
            printf 'Select files to concat. Enter file paths separated by space (or wildcard like *.mp4):\n'
            local input_pattern=$(_fb_media_input "Files or wildcard (e.g. video1.mp4 video2.mp4 or clip*.mp4)" "")
            [ -z "$input_pattern" ] && return 0
            local list_file=$(mktemp)
            for f in $input_pattern; do
                if [ -f "$f" ]; then
                    local escaped_path=$(realpath "$f" | sed "s/'/'\\\\''/g")
                    printf "file '%s'\n" "$escaped_path" >> "$list_file"
                fi
            done
            if [ ! -s "$list_file" ]; then
                printf '❌ No valid files found!\n'
                rm -f "$list_file"
                return 1
            fi
            local out="merged_$(date +%Y%m%d_%H%M%S).mp4"
            printf '\n⚡ Merging video files...\n'
            ffmpeg -f concat -safe 0 -i "$list_file" -c copy "$out"
            rm -f "$list_file"
            printf '\n✅ Videos merged into: %s\n' "$out"
            ;;

        "4. "*|*"Resolution"*|resolution)
            local file=$(_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$file" ] && return 0
            local mode=$(_fb_media_choose_opt "Select Target Resolution / Aspect Ratio:" \
                "1) 1080p Full HD (1920x1080)" \
                "2) 720p HD (1280x720)" \
                "3) 480p SD (854x480)" \
                "4) Crop 16:9 to 9:16 Portrait (Reels/Shorts)" \
                "5) 9:16 Portrait with Blurred Background")
            local base="${file%.*}"
            local out="${base}_res.${file##*.}"
            case "$mode" in
                *"1)"*) ffmpeg -i "$file" -vf "scale=1920:1080:force_original_aspect_ratio=decrease,pad=1920:1080:(ow-iw)/2:(oh-ih)/2" -c:a copy "$out" ;;
                *"2)"*) ffmpeg -i "$file" -vf "scale=1280:720:force_original_aspect_ratio=decrease,pad=1280:720:(ow-iw)/2:(oh-ih)/2" -c:a copy "$out" ;;
                *"3)"*) ffmpeg -i "$file" -vf "scale=854:480:force_original_aspect_ratio=decrease,pad=854:480:(ow-iw)/2:(oh-ih)/2" -c:a copy "$out" ;;
                *"4)"*) ffmpeg -i "$file" -vf "crop=ih*9/16:ih" -c:a copy "$out" ;;
                *"5)"*) ffmpeg -i "$file" -filter_complex "[0:v]scale=1080:1920:force_original_aspect_ratio=increase,crop=1080:1920,boxblur=20:10[bg];[0:v]scale=1080:1920:force_original_aspect_ratio=decrease[fg];[bg][fg]overlay=(W-w)/2:(H-h)/2" -c:a copy "$out" ;;
            esac
            printf '\n✅ Resolution adjusted: %s\n' "$out"
            ;;

        "5. "*|*"Speed Control"*|speed)
            local file=$(_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$file" ] && return 0
            local spd_opt=$(_fb_media_choose_opt "Select Speed:" \
                "1) 0.25x (Super Slow-Mo)" \
                "2) 0.5x (Slow Motion)" \
                "3) 1.5x (Faster)" \
                "4) 2.0x (Time-lapse 2x)" \
                "5) 4.0x (Time-lapse 4x)")
            local pts="1.0" atempo="1.0"
            [[ "$spd_opt" == *"1)"* ]] && pts="4.0" && atempo="0.5,atempo=0.5"
            [[ "$spd_opt" == *"2)"* ]] && pts="2.0" && atempo="0.5"
            [[ "$spd_opt" == *"3)"* ]] && pts="0.66667" && atempo="1.5"
            [[ "$spd_opt" == *"4)"* ]] && pts="0.5" && atempo="2.0"
            [[ "$spd_opt" == *"5)"* ]] && pts="0.25" && atempo="2.0,atempo=2.0"
            local out="${file%.*}_speed.${file##*.}"
            if ffprobe -i "$file" -show_streams -select_streams a 2>&1 | grep -q "codec_type=audio"; then
                ffmpeg -i "$file" -filter_complex "[0:v]setpts=${pts}*PTS[v];[0:a]atempo=${atempo}[a]" -map "[v]" -map "[a]" "$out"
            else
                ffmpeg -i "$file" -vf "setpts=${pts}*PTS" "$out"
            fi
            printf '\n✅ Speed changed: %s\n' "$out"
            ;;

        "6. "*|*"Rotate"*|rotate)
            local file=$(_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$file" ] && return 0
            local rot_opt=$(_fb_media_choose_opt "Select Rotation / Flip Option:" \
                "1) Rotate 90° Clockwise" \
                "2) Rotate 90° Counter-Clockwise" \
                "3) Rotate 180°" \
                "4) Flip Horizontally (Mirror)" \
                "5) Flip Vertically")
            local vf="transpose=1"
            [[ "$rot_opt" == *"2)"* ]] && vf="transpose=2"
            [[ "$rot_opt" == *"3)"* ]] && vf="transpose=2,transpose=2"
            [[ "$rot_opt" == *"4)"* ]] && vf="hflip"
            [[ "$rot_opt" == *"5)"* ]] && vf="vflip"
            local out="${file%.*}_rotated.${file##*.}"
            ffmpeg -i "$file" -vf "$vf" -c:a copy "$out"
            printf '\n✅ Rotation/Flip complete: %s\n' "$out"
            ;;

        "7. "*|*"Watermark"*|watermark)
            local file=$(_fb_media_select_file "Select main video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$file" ] && return 0
            local type=$(_fb_media_choose_opt "Watermark Type:" "1) Image Logo" "2) Text Banner")
            local out="${file%.*}_watermarked.${file##*.}"
            if [[ "$type" == *"1)"* ]]; then
                local logo=$(_fb_media_select_file "Select watermark image (PNG/JPG):" "\.(png|jpg|jpeg)$")
                [ -z "$logo" ] && return 0
                local pos=$(_fb_media_choose_opt "Watermark Position:" \
                    "1) Top-Right" "2) Top-Left" "3) Bottom-Right" "4) Bottom-Left" "5) Center")
                local overlay="main_w-overlay_w-10:10" # default top-right
                [[ "$pos" == *"2)"* ]] && overlay="10:10"
                [[ "$pos" == *"3)"* ]] && overlay="main_w-overlay_w-10:main_h-overlay_h-10"
                [[ "$pos" == *"4)"* ]] && overlay="10:main_h-overlay_h-10"
                [[ "$pos" == *"5)"* ]] && overlay="(main_w-overlay_w)/2:(main_h-overlay_h)/2"
                ffmpeg -i "$file" -i "$logo" -filter_complex "[1:v]scale=150:-1[logo];[0:v][logo]overlay=${overlay}" -c:a copy "$out"
            else
                local text=$(_fb_media_input "Watermark Text" "FancyBash")
                local safe_text=$(echo "$text" | sed "s/'/\\\\'/g")
                ffmpeg -i "$file" -vf "drawtext=text='${safe_text}':x=w-tw-20:y=h-th-20:fontsize=36:fontcolor=white@0.8:box=1:boxcolor=black@0.4:boxborderw=5" -c:a copy "$out"
            fi
            printf '\n✅ Watermark applied: %s\n' "$out"
            ;;

        "8. "*|*"Video Grid"*|grid)
            local grid_mode=$(_fb_media_choose_opt "Grid Layout:" "1) 2 Videos Side-by-Side" "2) 4 Videos (2x2 Grid)")
            if [[ "$grid_mode" == *"1)"* ]]; then
                local f1=$(_fb_media_select_file "Select Video 1:" "\.(mp4|mkv|mov|avi|webm)$")
                local f2=$(_fb_media_select_file "Select Video 2:" "\.(mp4|mkv|mov|avi|webm)$")
                [ -z "$f1" ] || [ -z "$f2" ] && return 0
                local out="comparison_side_by_side.mp4"
                ffmpeg -i "$f1" -i "$f2" -filter_complex "[0:v]scale=-1:720[v0];[1:v]scale=-1:720[v1];[v0][v1]hstack=inputs=2[v]" -map "[v]" -c:v libx264 "$out"
            else
                local f1=$(_fb_media_select_file "Select Video 1 (Top-Left):" "\.(mp4|mkv|mov|avi|webm)$")
                local f2=$(_fb_media_select_file "Select Video 2 (Top-Right):" "\.(mp4|mkv|mov|avi|webm)$")
                local f3=$(_fb_media_select_file "Select Video 3 (Bottom-Left):" "\.(mp4|mkv|mov|avi|webm)$")
                local f4=$(_fb_media_select_file "Select Video 4 (Bottom-Right):" "\.(mp4|mkv|mov|avi|webm)$")
                [ -z "$f1" ] || [ -z "$f2" ] || [ -z "$f3" ] || [ -z "$f4" ] && return 0
                local out="grid_2x2.mp4"
                ffmpeg -i "$f1" -i "$f2" -i "$f3" -i "$f4" -filter_complex "[0:v]scale=640:360[v0];[1:v]scale=640:360[v1];[2:v]scale=640:360[v2];[3:v]scale=640:360[v3];[v0][v1][v2][v3]xstack=inputs=4:layout=0_0|w0_0|0_h0|w0_h0[v]" -map "[v]" -c:v libx264 "$out"
            fi
            printf '\n✅ Grid video created: %s\n' "$out"
            ;;

        "9. "*|*"Extract Audio"*|audio-extract)
            local file=$(_fb_media_select_file "Select media file:" "\.(mp4|mkv|mov|avi|webm|m4a|flv)$")
            [ -z "$file" ] && return 0
            local fmt=$(_fb_media_choose_opt "Select Output Audio Format:" "1) MP3" "2) AAC" "3) WAV" "4) FLAC" "5) M4A")
            local ext="mp3" acodec="libmp3lame"
            [[ "$fmt" == *"2)"* ]] && ext="aac" && acodec="aac"
            [[ "$fmt" == *"3)"* ]] && ext="wav" && acodec="pcm_s16le"
            [[ "$fmt" == *"4)"* ]] && ext="flac" && acodec="flac"
            [[ "$fmt" == *"5)"* ]] && ext="m4a" && acodec="aac"
            local out="${file%.*}.${ext}"
            ffmpeg -i "$file" -vn -acodec "$acodec" "$out"
            printf '\n✅ Audio extracted to: %s\n' "$out"
            ;;

        "10. "*|*"Mute Video"*|mute)
            local file=$(_fb_media_select_file "Select video to mute:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$file" ] && return 0
            local out="${file%.*}_muted.${file##*.}"
            ffmpeg -i "$file" -an -c:v copy "$out"
            printf '\n✅ Video muted: %s\n' "$out"
            ;;

        "11. "*|*"Replace"*|audio-replace)
            local vid=$(_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$vid" ] && return 0
            local aud=$(_fb_media_select_file "Select audio track file:" "\.(mp3|wav|aac|m4a|flac|ogg)$")
            [ -z "$aud" ] && return 0
            local mode=$(_fb_media_choose_opt "Audio Integration Mode:" \
                "1) Replace original audio completely" \
                "2) Mix new audio with existing video sound")
            local out="${vid%.*}_audio_merged.${vid##*.}"
            if [[ "$mode" == *"1)"* ]] || ! ffprobe -i "$vid" -show_streams -select_streams a 2>&1 | grep -q "codec_type=audio"; then
                ffmpeg -i "$vid" -i "$aud" -c:v copy -c:a aac -map 0:v:0 -map 1:a:0 -shortest "$out"
            else
                ffmpeg -i "$vid" -i "$aud" -filter_complex "[0:a][1:a]amix=inputs=2:duration=first[a]" -map 0:v -map "[a]" -c:v copy "$out"
            fi
            printf '\n✅ Audio track processed: %s\n' "$out"
            ;;

        "12. "*|*"Loudness"*|loudness)
            local file=$(_fb_media_select_file "Select media file:" "\.(mp4|mkv|mov|avi|webm|mp3|wav)$")
            [ -z "$file" ] && return 0
            local std=$(_fb_media_choose_opt "Target Loudness Standard:" \
                "1) YouTube / Podcast (-14 LUFS - Recommended)" \
                "2) EBU R128 Broadcast (-23 LUFS)")
            local lufs="-14"
            [[ "$std" == *"2)"* ]] && lufs="-23"
            local out="${file%.*}_normalized.${file##*.}"
            ffmpeg -i "$file" -af "loudnorm=I=${lufs}:LRA=11:TP=-1.5" "$out"
            printf '\n✅ Loudness normalized (%s LUFS): %s\n' "$lufs" "$out"
            ;;

        "13. "*|*"Visualizer"*|visualizer)
            local aud=$(_fb_media_select_file "Select audio file:" "\.(mp3|wav|aac|m4a|flac|ogg)$")
            [ -z "$aud" ] && return 0
            local viz=$(_fb_media_choose_opt "Select Visualizer Style:" \
                "1) Vector Waveform (Neon Green Line)" \
                "2) Frequency Spectrum (Rainbow Combined)" \
                "3) CQT Color Spectrum" \
                "4) Audio Stereo Histogram")
            local filter="showwaves=s=1280x720:mode=line:colors=0x00FF99[v]"
            [[ "$viz" == *"2)"* ]] && filter="showspectrum=s=1280x720:mode=combined:color=rainbow[v]"
            [[ "$viz" == *"3)"* ]] && filter="showcqt=s=1280x720[v]"
            [[ "$viz" == *"4)"* ]] && filter="ahistogram=s=1280x720[v]"
            local out="${aud%.*}_visualizer.mp4"
            ffmpeg -i "$aud" -filter_complex "$filter" -map "[v]" -map 0:a -c:v libx264 -c:a aac -shortest "$out"
            printf '\n✅ Audio visualizer video created: %s\n' "$out"
            ;;

        "14. "*|*"Audio Speed"*|audio-speed)
            local aud=$(_fb_media_select_file "Select audio file:" "\.(mp3|wav|aac|m4a|flac|ogg)$")
            [ -z "$aud" ] && return 0
            local speed=$(_fb_media_input "Audio speed factor (e.g. 1.25, 1.5, 0.8)" "1.25")
            local out="${aud%.*}_speed_${speed}.${aud##*.}"
            ffmpeg -i "$aud" -af "atempo=${speed}" "$out"
            printf '\n✅ Audio speed changed (pitch preserved): %s\n' "$out"
            ;;

        "15. "*|*"Snapshot"*|snapshot)
            local vid=$(_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$vid" ] && return 0
            local time=$(_fb_media_input "Timestamp (HH:MM:SS or seconds)" "00:00:05")
            local fmt=$(_fb_media_choose_opt "Image Format:" "1) JPEG (.jpg)" "2) PNG (.png)")
            local ext="jpg"
            [[ "$fmt" == *"2)"* ]] && ext="png"
            local out="${vid%.*}_snap_${time//:/}_.${ext}"
            ffmpeg -ss "$time" -i "$vid" -vframes 1 -q:v 2 "$out"
            printf '\n✅ Snapshot extracted: %s\n' "$out"
            ;;

        "16. "*|*"Bulk Frame"*|bulk-frames)
            local vid=$(_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$vid" ] && return 0
            local interval=$(_fb_media_input "Extract frame every N seconds" "1")
            local base=$(basename "${vid%.*}")
            local outdir="${base}_frames"
            mkdir -p "$outdir"
            ffmpeg -i "$vid" -vf "fps=1/${interval}" "${outdir}/frame_%04d.jpg"
            printf '\n✅ Frames extracted into video folder: %s/\n' "$outdir"
            ;;

        "17. "*|*"GIF"*|gif)
            local vid=$(_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$vid" ] && return 0
            local start=$(_fb_media_input "Start timestamp" "00:00:00")
            local dur=$(_fb_media_input "GIF Duration in seconds" "5")
            local res=$(_fb_media_choose_opt "GIF Resolution & Quality:" \
                "1) 480p @ 15fps (Standard)" \
                "2) 720p @ 20fps (HD High Quality)" \
                "3) 360p @ 12fps (Compact)")
            local scale="480" fps="15"
            [[ "$res" == *"2)"* ]] && scale="720" && fps="20"
            [[ "$res" == *"3)"* ]] && scale="360" && fps="12"
            local out="${vid%.*}_pro.gif"
            local tmp_palette="/tmp/ff_palette_$$.png"
            printf '\n⚡ Generating palette and rendering ultra-sharp GIF...\n'
            ffmpeg -ss "$start" -t "$dur" -i "$vid" -vf "fps=${fps},scale=${scale}:-1:flags=lanczos,palettegen" -y "$tmp_palette"
            ffmpeg -ss "$start" -t "$dur" -i "$vid" -i "$tmp_palette" -filter_complex "fps=${fps},scale=${scale}:-1:flags=lanczos[x];[x][1:v]paletteuse" "$out"
            rm -f "$tmp_palette"
            printf '\n✅ Pro-Quality GIF created: %s\n' "$out"
            ;;

        "18. "*|*"Contact Sheet"*|contact-sheet)
            local vid=$(_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$vid" ] && return 0
            local grid=$(_fb_media_choose_opt "Grid Layout:" "1) 3x3 Grid (9 Thumbnails)" "2) 4x4 Grid (16 Thumbnails)")
            local tiles="3x3"
            [[ "$grid" == *"2)"* ]] && tiles="4x4"
            local out="${vid%.*}_contact_sheet.png"
            ffmpeg -i "$vid" -vf "fps=1/10,scale=320:-1,tile=${tiles}" -vsync vfr "$out"
            printf '\n✅ Mosaic Thumbnail Grid created: %s\n' "$out"
            ;;

        "19. "*|*"Screen Recorder"*|screen-record)
            local out="screencast_$(date +%Y%m%d_%H%M%S).mp4"
            local os_type=$(uname -s)
            local audio_opt=$(_fb_media_choose_opt "Select Screen Record Mode:" \
                "1) Full Screen Video + System Audio (Mic/Speaker)" \
                "2) Full Screen Video Only (Silent)")

            printf '\n🎥 Starting Terminal Screen Recorder...\n'
            printf '📌 Press "q" or Ctrl+C in this terminal to stop recording.\n\n'

            if [ "$os_type" = "Linux" ]; then
                local display="${DISPLAY:-:0.0}"
                local res=""
                if command -v xrandr >/dev/null 2>&1; then
                    res=$(xrandr 2>/dev/null | grep '*' | awk '{print $1}' | head -n1)
                elif command -v xdpyinfo >/dev/null 2>&1; then
                    res=$(xdpyinfo 2>/dev/null | grep 'dimensions:' | awk '{print $2}')
                fi
                [ -z "$res" ] && res="1920x1080"

                if [[ "$audio_opt" == *"1)"* ]] && (command -v pactl >/dev/null 2>&1 || command -v pulseaudio >/dev/null 2>&1); then
                    ffmpeg -f x11grab -video_size "$res" -i "$display" -f pulse -i default -c:v libx264 -preset ultrafast -pix_fmt yuv420p -c:a aac "$out"
                else
                    ffmpeg -f x11grab -video_size "$res" -i "$display" -c:v libx264 -preset ultrafast -pix_fmt yuv420p "$out"
                fi
            elif [ "$os_type" = "Darwin" ]; then
                if [[ "$audio_opt" == *"1)"* ]]; then
                    ffmpeg -f avfoundation -i "0:0" -c:v libx264 -preset ultrafast -pix_fmt yuv420p -c:a aac "$out" 2>/dev/null || \
                    ffmpeg -f avfoundation -i "1:0" -c:v libx264 -preset ultrafast -pix_fmt yuv420p -c:a aac "$out"
                else
                    ffmpeg -f avfoundation -i "0" -c:v libx264 -preset ultrafast -pix_fmt yuv420p "$out" 2>/dev/null || \
                    ffmpeg -f avfoundation -i "1" -c:v libx264 -preset ultrafast -pix_fmt yuv420p "$out"
                fi
            else
                if [[ "$audio_opt" == *"1)"* ]]; then
                    ffmpeg -f gdigrab -i desktop -f dshow -i audio="Microphone" -c:v libx264 -preset ultrafast -pix_fmt yuv420p -c:a aac "$out" 2>/dev/null || \
                    ffmpeg -f gdigrab -i desktop -c:v libx264 -preset ultrafast -pix_fmt yuv420p "$out"
                else
                    ffmpeg -f gdigrab -i desktop -c:v libx264 -preset ultrafast -pix_fmt yuv420p "$out"
                fi
            fi
            printf '\n✅ Screen recording saved as: %s\n' "$out"
            ;;

        "20. "*|*"Subtitle Burn"*|subtitle-burn)
            local vid=$(_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$vid" ] && return 0
            local sub=$(_fb_media_select_file "Select subtitle file (.srt/.ass):" "\.(srt|ass)$")
            [ -z "$sub" ] && return 0
            local out="${vid%.*}_subtitled.${vid##*.}"
            local safe_sub=$(echo "$sub" | sed "s/'/\\\\'/g" | sed "s/:/\\\\:/g")
            ffmpeg -i "$vid" -vf "subtitles='${safe_sub}'" -c:a copy "$out"
            printf '\n✅ Subtitle burned into video: %s\n' "$out"
            ;;

        "21. "*|*"Subtitle Extraction"*|subtitle-extract)
            local vid=$(_fb_media_select_file "Select video file:" "\.(mkv|mp4|mov)$")
            [ -z "$vid" ] && return 0
            local out="${vid%.*}.srt"
            ffmpeg -i "$vid" -map 0:s:0 "$out"
            printf '\n✅ Subtitle extracted: %s\n' "$out"
            ;;

        "22. "*|*"Privacy Clean"*|privacy-clean)
            local file=$(_fb_media_select_file "Select media file:" "\.(mp4|mkv|mov|avi|webm|mp3|wav|jpg|png)$")
            [ -z "$file" ] && return 0
            local out="${file%.*}_clean.${file##*.}"
            ffmpeg -i "$file" -map_metadata -1 -c copy "$out"
            printf '\n✅ Privacy clean complete (metadata wiped): %s\n' "$out"
            ;;

        "23. "*|*"Format Conversion"*|convert)
            local file=$(_fb_media_select_file "Select media file to convert:" "\.(mp4|mkv|mov|avi|webm|ts|flv|mp3|wav|aac)$")
            [ -z "$file" ] && return 0
            local fmt=$(_fb_media_choose_opt "Select Target Format:" \
                "1) MP4 (.mp4)" \
                "2) MKV (.mkv)" \
                "3) WEBM (.webm)" \
                "4) MOV (.mov)" \
                "5) AVI (.avi)" \
                "6) MP3 (.mp3)" \
                "7) WAV (.wav)")
            local ext="mp4"
            [[ "$fmt" == *"2)"* ]] && ext="mkv"
            [[ "$fmt" == *"3)"* ]] && ext="webm"
            [[ "$fmt" == *"4)"* ]] && ext="mov"
            [[ "$fmt" == *"5)"* ]] && ext="avi"
            [[ "$fmt" == *"6)"* ]] && ext="mp3"
            [[ "$fmt" == *"7)"* ]] && ext="wav"
            local out="${file%.*}.${ext}"
            ffmpeg -i "$file" "$out"
            printf '\n✅ Format converted to: %s\n' "$out"
            ;;

        "24. "*|*"Batch"*|batch)
            local folder=$(_fb_media_input "Enter folder path (or press enter for current directory)" ".")
            [ ! -d "$folder" ] && printf '❌ Directory not found!\n' && return 1
            local batch_task=$(_fb_media_choose_opt "Select Batch Operation:" \
                "1) Bulk Video Compress" \
                "2) Bulk Format Convert to MP4" \
                "3) Bulk Metadata Wiping (Privacy Clean)" \
                "4) Bulk Audio Extraction (MP3)" \
                "5) Bulk Video Muting")
            printf '\n⚡ Running batch processing on folder: %s...\n' "$folder"
            for f in "$folder"/*.{mp4,mkv,mov,avi,webm}; do
                [ -f "$f" ] || continue
                local base="${f%.*}"
                case "$batch_task" in
                    *"1)"*) ffmpeg -i "$f" -vcodec libx264 -crf 26 -preset fast "${base}_batch_compressed.mp4" -y ;;
                    *"2)"*) ffmpeg -i "$f" "${base}_converted.mp4" -y ;;
                    *"3)"*) ffmpeg -i "$f" -map_metadata -1 -c copy "${base}_clean.${f##*.}" -y ;;
                    *"4)"*) ffmpeg -i "$f" -vn -acodec libmp3lame "${base}_audio.mp3" -y ;;
                    *"5)"*) ffmpeg -i "$f" -an -c:v copy "${base}_muted.${f##*.}" -y ;;
                esac
            done
            printf '\n✅ Batch processing complete!\n'
            ;;

        *)
            printf '❌ Invalid action: %s\n' "$action"
            return 1
            ;;
    esac
}

# Aliases for FFmedia Suite
alias ffstudio='ffmedia'
alias fftool='ffmedia'
alias fancy_ffmpeg='ffmedia'

# ==============================================================================
# 🔐 HARDENED MULTI-VAULT MANAGER (vault / secvault / fvault / fancy_vault)
# ==============================================================================
# Universal Linux & macOS AES-256 Memory-Guarded Directory Vault Manager
# Usage: vault [lock|unlock|list|config|--help]

_fb_vault_store="$HOME/.secret_vaults"
_fb_vault_iterations=500000
_fb_vault_max_attempts=3

_fb_vault_ensure_store() {
    mkdir -p "$_fb_vault_store" 2>/dev/null
    chmod 700 "$_fb_vault_store" 2>/dev/null
    # Clean up broken symlinks in $HOME left behind by unmounted/rebooted RAM vaults
    local sym
    for sym in "$HOME"/*; do
        if [[ -L "$sym" && ! -e "$sym" ]]; then
            local target=$(readlink "$sym" 2>/dev/null)
            if [[ "$target" == *"/dev/shm"* || "$target" == *"secret_ram"* ]]; then
                rm -f "$sym" 2>/dev/null
            fi
        fi
    done
}

_fb_vault_detect_ram_base() {
    local target_ram=""
    if [[ -d "/dev/shm" && -w "/dev/shm" ]]; then
        target_ram="/dev/shm"
    else
        target_ram="${TMPDIR:-/tmp}/.secret_ram_${UID:-$(id -u 2>/dev/null || echo 1000)}"
        mkdir -p "$target_ram" 2>/dev/null
        chmod 700 "$target_ram" 2>/dev/null
    fi
    echo "$target_ram"
}

_fb_vault_banner() {
    if command -v gum &>/dev/null; then
        clear
        gum style \
            --foreground 212 --border-foreground 99 --border double \
            --align center --width 64 --margin "1 0" --padding "0 2" \
            "🔐 HARDENED MULTI-VAULT MANAGER" \
            "Universal Linux & macOS | AES-256 Memory Guard"
    else
        clear
        printf '\033[1;35m════════════════════════════════════════════════════════════════\033[0m\n'
        printf '\033[1;36m  🔐 HARDENED MULTI-VAULT MANAGER (AES-256 Memory Guard)\033[0m\n'
        printf '\033[1;35m════════════════════════════════════════════════════════════════\033[0m\n\n'
    fi
}

_fb_vault_fancy_input() {
    local title="$1"
    local badge="$2"
    local icon="${3:-🔑}"

    if command -v gum &>/dev/null; then
        echo "" >&2
        gum style \
            --border rounded \
            --border-foreground 212 \
            --foreground 255 \
            --padding "0 1" \
            --margin "0 1" \
            --width 58 \
            "$icon  $title  $badge" >&2

        gum input \
            --password \
            --placeholder "••••••••••••••••••••" \
            --prompt "  ❯ " \
            --prompt.foreground 212 \
            --cursor.foreground 99 \
            --width 54
    else
        local val=""
        trap 'stty echo 2>/dev/null' INT TERM EXIT
        printf "%s %s %s: " "$icon" "$title" "$badge" >&2
        stty -echo 2>/dev/null
        read -r val </dev/tty
        stty echo 2>/dev/null
        trap - INT TERM EXIT
        printf "\n" >&2
        echo "$val"
    fi
}

_fb_vault_send_alert() {
    local msg="$1"
    local bot_token="${FB_VAULT_TELEGRAM_BOT_TOKEN:-}"
    local chat_id="${FB_VAULT_TELEGRAM_CHAT_ID:-}"
    if [[ -n "$bot_token" && -n "$chat_id" ]]; then
        curl -s -X POST "https://api.telegram.org/bot${bot_token}/sendMessage" \
             -d chat_id="${chat_id}" \
             -d text="⚠️ VAULT ALERT: $msg" >/dev/null 2>&1 &
    fi
}

_fb_vault_open_explorer() {
    local target_path="$1"
    if [[ "$OSTYPE" == "darwin"* ]]; then
        open "$target_path" &>/dev/null &
    elif command -v xdg-open &>/dev/null; then
        xdg-open "$target_path" &>/dev/null &
    fi
}

_fb_vault_secure_delete() {
    local target="$1"
    [[ -z "$target" ]] && return 0
    if [[ -f "$target" ]]; then
        if command -v shred &>/dev/null; then
            shred -u -n 3 -z "$target" 2>/dev/null || rm -f "$target"
        elif [[ "$OSTYPE" == "darwin"* ]]; then
            rm -P -f "$target" 2>/dev/null || rm -f "$target"
        else
            dd if=/dev/urandom of="$target" bs=1M count=1 2>/dev/null || true
            rm -f "$target"
        fi
    elif [[ -d "$target" && ! -L "$target" ]]; then
        if command -v shred &>/dev/null; then
            find "$target" -type f -exec shred -u -n 3 -z {} + 2>/dev/null
        elif [[ "$OSTYPE" == "darwin"* ]]; then
            find "$target" -type f -exec rm -P {} + 2>/dev/null
        fi
        rm -rf "$target"
    elif [[ -L "$target" ]]; then
        rm -f "$target"
    fi
}

_fb_vault_close_session() {
    local folder_name="$1"
    [[ -z "$folder_name" ]] && return 0
    local ram_base=$(_fb_vault_detect_ram_base)
    local ram_dir="$ram_base/$folder_name"
    local vault_dir="$HOME/$folder_name"

    ([[ -L "$vault_dir" ]] || [[ -d "$vault_dir" ]]) && rm -rf "$vault_dir" 2>/dev/null
    _fb_vault_secure_delete "$ram_dir"
}

_fb_vault_lock_specific() {
    local folder_name="$1"
    _fb_vault_ensure_store
    local encrypted_file="$_fb_vault_store/${folder_name}.enc"
    local panic_hash_file="$_fb_vault_store/${folder_name}.panic"
    local ram_base=$(_fb_vault_detect_ram_base)
    local ram_dir="$ram_base/$folder_name"
    local vault_dir="$HOME/$folder_name"

    _fb_vault_banner
    if command -v gum &>/dev/null; then
        gum style --foreground 220 --bold "[>] Relocking '$folder_name'..."
    else
        printf '\033[1;33m[*] Relocking "%s"...\033[0m\n' "$folder_name"
    fi

    local pass1=$(_fb_vault_fancy_input "Master Password" "[$folder_name]" "🔐")
    local pass2=$(_fb_vault_fancy_input "Confirm Master Password" "[RE-ENTER]" "🔄")

    if [[ "$pass1" != "$pass2" || -z "$pass1" ]]; then
        printf '\033[1;31m  [X] Passwords do not match or empty! Lock cancelled.\033[0m\n'
        read -r "?Press Enter to continue..." _ </dev/tty
        return 1
    fi

    local panic1=$(_fb_vault_fancy_input "Panic Password (Optional)" "[OPTIONAL]" "🚨")
    if [[ -n "$panic1" ]]; then
        printf '%s' "$panic1" | openssl dgst -sha256 | awk '{print $2}' > "$panic_hash_file"
    fi

    local tmp_archive="$ram_base/.tmp_vault_${folder_name}_$$.tar.gz"
    rm -f "$tmp_archive" 2>/dev/null

    echo ""
    local enc_ok=1
    if command -v gum &>/dev/null; then
        gum spin --spinner dot --title "Archiving & Encrypting AES-256..." -- \
            tar -czf "$tmp_archive" -C "$ram_base" "$folder_name"
    else
        tar -czf "$tmp_archive" -C "$ram_base" "$folder_name" 2>/dev/null
    fi

    if [[ $? -eq 0 && -s "$tmp_archive" ]]; then
        openssl enc -aes-256-cbc -pbkdf2 -iter $_fb_vault_iterations -pass pass:"$pass1" -in "$tmp_archive" -out "$encrypted_file" 2>/dev/null
        if [[ $? -eq 0 && -s "$encrypted_file" ]]; then
            enc_ok=0
        fi
    fi
    _fb_vault_secure_delete "$tmp_archive"

    if [[ $enc_ok -eq 0 ]]; then
        chmod 600 "$encrypted_file" 2>/dev/null
        [[ -L "$vault_dir" ]] && rm -f "$vault_dir"
        _fb_vault_secure_delete "$ram_dir"

        printf '\033[1;32m  [✓] "%s" locked & encrypted successfully!\033[0m\n' "$folder_name"
    else
        _fb_vault_secure_delete "$encrypted_file"
        printf '\033[1;31m  [X] Encryption failed! Memory vault preserved.\033[0m\n'
    fi
    read -r "?Press Enter to continue..." _ </dev/tty
}

_fb_vault_lock() {
    _fb_vault_ensure_store
    _fb_vault_banner
    local target_dir="${1:-}"

    if [[ -z "$target_dir" ]]; then
        if command -v fzf &>/dev/null; then
            target_dir=$(find "$HOME" -maxdepth 2 -type d ! -path '*/.*' ! -path '*/node_modules*' ! -path '*/Library*' 2>/dev/null | \
                fzf --height=12 --layout=reverse --border=rounded \
                    --prompt="📁 Select folder to lock > " \
                    --color="border:99,header:220,pointer:212,fg:255")
        else
            read -r "?Enter directory path to lock: " target_dir </dev/tty
        fi
    fi

    if [[ -z "$target_dir" || ! -d "$target_dir" ]]; then
        printf '\033[1;31m[!] No valid directory selected.\033[0m\n'
        read -r "?Press Enter to continue..." _ </dev/tty
        return 1
    fi

    local folder_name=$(basename "$target_dir")
    local encrypted_file="$_fb_vault_store/${folder_name}.enc"
    local panic_hash_file="$_fb_vault_store/${folder_name}.panic"
    local ram_base=$(_fb_vault_detect_ram_base)
    local ram_dir="$ram_base/$folder_name"

    local pass1=$(_fb_vault_fancy_input "New Master Password" "[$folder_name]" "🛡️")
    if [[ -z "$pass1" ]]; then
        printf '\033[1;31m  [X] Password cannot be empty!\033[0m\n'
        read -r "?Press Enter to continue..." _ </dev/tty
        return 1
    fi

    local pass2=$(_fb_vault_fancy_input "Confirm Password" "[RE-ENTER]" "🔄")
    if [[ "$pass1" != "$pass2" ]]; then
        printf '\033[1;31m  [X] Passwords do not match! Aborting.\033[0m\n'
        read -r "?Press Enter to continue..." _ </dev/tty
        return 1
    fi

    local panic1=$(_fb_vault_fancy_input "Panic Password (Optional)" "[OPTIONAL]" "🚨")
    if [[ -n "$panic1" ]]; then
        printf '%s' "$panic1" | openssl dgst -sha256 | awk '{print $2}' > "$panic_hash_file"
    fi

    local parent_dir=$(dirname "$target_dir")
    local tmp_archive="$ram_base/.tmp_vault_${folder_name}_$$.tar.gz"
    rm -f "$tmp_archive" 2>/dev/null

    echo ""
    local enc_ok=1
    if command -v gum &>/dev/null; then
        gum spin --spinner dot --title "Encrypting '$folder_name' with AES-256..." -- \
            tar -czf "$tmp_archive" -C "$parent_dir" "$folder_name"
    else
        tar -czf "$tmp_archive" -C "$parent_dir" "$folder_name" 2>/dev/null
    fi

    if [[ $? -eq 0 && -s "$tmp_archive" ]]; then
        openssl enc -aes-256-cbc -pbkdf2 -iter $_fb_vault_iterations -pass pass:"$pass1" -in "$tmp_archive" -out "$encrypted_file" 2>/dev/null
        if [[ $? -eq 0 && -s "$encrypted_file" ]]; then
            enc_ok=0
        fi
    fi
    _fb_vault_secure_delete "$tmp_archive"

    if [[ $enc_ok -eq 0 ]]; then
        chmod 600 "$encrypted_file"
        [[ -L "$target_dir" ]] && rm -f "$target_dir"

        if command -v gum &>/dev/null; then
            gum spin --spinner monkey --title "Securely wiping original directory..." -- sleep 1
        fi
        _fb_vault_secure_delete "$target_dir"
        _fb_vault_secure_delete "$ram_dir"

        printf '\033[1;32m  [✓] "%s" locked successfully!\033[0m\n' "$folder_name"
    else
        _fb_vault_secure_delete "$encrypted_file"
        printf '\033[1;31m  [X] Encryption failed! Original directory preserved.\033[0m\n'
    fi

    read -r "?Press Enter to continue..." _ </dev/tty
}

_fb_vault_unlock() {
    _fb_vault_ensure_store
    _fb_vault_banner

    local available_vaults=(${(f)"$(find "$_fb_vault_store" -maxdepth 1 -name "*.enc" -exec basename {} .enc \; 2>/dev/null)"})

    if [[ ${#available_vaults[@]} -eq 0 || -z "${available_vaults[1]}" ]]; then
        printf '\033[1;31m  [!] No locked vaults found!\033[0m\n'
        read -r "?Press Enter to continue..." _ </dev/tty
        return 0
    fi

    local selected_folder="${1:-}"
    if [[ -z "$selected_folder" ]]; then
        if command -v fzf &>/dev/null; then
            selected_folder=$(printf "%s\n" "${available_vaults[@]}" | \
                fzf --height=10 --layout=reverse --border=rounded \
                    --prompt="🔓 Select vault to unlock > " \
                    --color="border:99,header:220,pointer:212,fg:255")
        else
            printf 'Available Vaults:\n'
            printf ' - %s\n' "${available_vaults[@]}"
            read -r "?Enter vault folder name to unlock: " selected_folder </dev/tty
        fi
    fi

    if [[ -z "$selected_folder" ]]; then
        return 0
    fi

    local encrypted_file="$_fb_vault_store/${selected_folder}.enc"
    local panic_hash_file="$_fb_vault_store/${selected_folder}.panic"
    local ram_base=$(_fb_vault_detect_ram_base)
    local ram_dir="$ram_base/$selected_folder"
    local vault_dir="$HOME/$selected_folder"

    local attempts=0

    while [[ $attempts -lt $_fb_vault_max_attempts ]]; do
        local remaining=$((_fb_vault_max_attempts - attempts))
        local pass=$(_fb_vault_fancy_input "Enter Secret Password" "[$selected_folder | Attempts remaining: $remaining]" "🔑")

        [[ -z "$pass" ]] && continue

        # Panic Password Check
        if [[ -f "$panic_hash_file" ]]; then
            local input_hash=$(printf '%s' "$pass" | openssl dgst -sha256 | awk '{print $2}')
            local saved_panic_hash=$(cat "$panic_hash_file")

            if [[ "$input_hash" == "$saved_panic_hash" ]]; then
                _fb_vault_send_alert "PANIC PASSWORD USED FOR $selected_folder! Data wiped."
                _fb_vault_secure_delete "$encrypted_file"
                _fb_vault_secure_delete "$panic_hash_file"

                mkdir -p "$vault_dir"
                echo "Confidential Project Notes $(date +%Y)..." > "$vault_dir/notes.txt"

                _fb_vault_open_explorer "$vault_dir"

                printf '\033[1;32m  [✓] Access Granted!\033[0m\n'
                read -r "?Press Enter to continue..." _ </dev/tty
                return 0
            fi
        fi

        rm -rf "$ram_dir" 2>/dev/null

        local tmp_dec_archive="$ram_base/.tmp_dec_${selected_folder}_$$.tar.gz"
        rm -f "$tmp_dec_archive" 2>/dev/null

        echo ""
        if command -v gum &>/dev/null; then
            gum spin --spinner line --title "Decrypting & mounting in memory (RAM)..." -- \
                openssl enc -d -aes-256-cbc -pbkdf2 -iter $_fb_vault_iterations -pass pass:"$pass" -in "$encrypted_file" -out "$tmp_dec_archive"
        else
            openssl enc -d -aes-256-cbc -pbkdf2 -iter $_fb_vault_iterations -pass pass:"$pass" -in "$encrypted_file" -out "$tmp_dec_archive" 2>/dev/null
        fi

        local dec_ok=1
        if [[ -s "$tmp_dec_archive" ]]; then
            tar -xzf "$tmp_dec_archive" -C "$ram_base" 2>/dev/null
            [[ $? -eq 0 && -d "$ram_dir" ]] && dec_ok=0
        fi
        _fb_vault_secure_delete "$tmp_dec_archive"

        if [[ $dec_ok -eq 0 ]]; then
            printf '\033[1;32m  [✓] "%s" unlocked successfully! (Loaded in RAM)\033[0m\n' "$selected_folder"

            ([[ -L "$vault_dir" ]] || [[ -d "$vault_dir" ]]) && rm -rf "$vault_dir" 2>/dev/null
            ln -s "$ram_dir" "$vault_dir"

            _fb_vault_open_explorer "$vault_dir"

            printf '\033[1;36m  [!] Directory opened in file explorer. (Encrypted backup preserved in ~/.secret_vaults/)\033[0m\n'

            echo ""
            local relock_choice=""
            if command -v gum &>/dev/null; then
                relock_choice=$(gum choose "Interactive Vault Session (Auto-relock on exit)" "10-Minute Auto-Relock Timer" "Relock Now")
                if [[ $? -ne 0 || -z "$relock_choice" ]]; then
                    printf '\033[1;33m  [!] Selection cancelled (ESC). Relocking vault...\033[0m\n'
                    _fb_vault_close_session "$selected_folder"
                    return 0
                fi
            else
                printf 'Select Relock Mode:\n'
                printf '  1) Interactive Vault Session (Auto-relock on exit/terminal close)\n'
                printf '  2) 10-Minute Auto-Relock Timer\n'
                printf '  3) Relock Now\n'
                read -r "?Enter choice [1-3] (default: 1): " relock_choice </dev/tty
                [[ -z "$relock_choice" ]] && relock_choice="1"
            fi

            if [[ "$relock_choice" == *"Session"* ]] || [[ "$relock_choice" == "1" ]]; then
                printf '\033[1;32m  [🔓] Entering Vault Shell Session. Type "exit" or close terminal to auto-relock.\033[0m\n'
                (
                    trap "_fb_vault_close_session '$selected_folder'" EXIT HUP INT TERM
                    ${SHELL:-/bin/zsh}
                )
                _fb_vault_close_session "$selected_folder"
                return 0
            elif [[ "$relock_choice" == *"10-Minute"* ]] || [[ "$relock_choice" == "2" ]]; then
                printf '\033[1;33m  [*] 10-minute auto-relock background timer started...\033[0m\n'
                (
                    sleep 600
                    _fb_vault_close_session "$selected_folder"
                ) &>/dev/null &
                return 0
            else
                _fb_vault_close_session "$selected_folder"
                printf '\033[1;32m  [✓] "%s" relocked & RAM purged.\033[0m\n' "$selected_folder"
                return 0
            fi
        else
            rm -rf "$ram_dir" 2>/dev/null
            attempts=$((attempts + 1))
            remaining=$((_fb_vault_max_attempts - attempts))
            local delay=$((attempts * 3))

            printf '\033[1;31m  [X] Invalid Password!\033[0m\n'

            if [[ $remaining -gt 0 ]]; then
                printf '\033[1;33m  [!] Anti-brute-force delay: Waiting %d seconds...\033[0m\n' "$delay"
                sleep $delay
            fi
        fi
    done

    # Self-Destruct Sequence
    clear
    printf '\033[1;31m🚨 [ALERT] Failed password attempts limit reached (%d/3)!\033[0m\n' "$_fb_vault_max_attempts"
    printf '\033[1;31mSELF-DESTRUCT: Permanently wiping "%s"...\033[0m\n' "$selected_folder"

    _fb_vault_send_alert "3 Failed attempts on $selected_folder! Self-destruct activated."

    _fb_vault_secure_delete "$encrypted_file"
    _fb_vault_secure_delete "$panic_hash_file"
    [[ -L "$vault_dir" ]] && rm -f "$vault_dir"
    _fb_vault_secure_delete "$vault_dir"
    _fb_vault_secure_delete "$ram_dir"

    printf '\033[1;32m  [✓] All vault data securely destroyed.\033[0m\n'
    return 1
}

_fb_vault_list() {
    _fb_vault_ensure_store
    _fb_vault_banner
    printf '\033[1;36m📋 Encrypted Vaults List:\033[0m\n\n'
    local count=0
    local ram_base=$(_fb_vault_detect_ram_base)
    for f in "$_fb_vault_store"/*.enc(N); do
        if [[ -f "$f" ]]; then
            local name=$(basename "$f" .enc)
            local size=$(du -sh "$f" 2>/dev/null | cut -f1)
            local ram_dir="$ram_base/$name"
            if [[ -d "$ram_dir" ]]; then
                printf '  • 🔓 \033[1;32m%-20s\033[0m (Size: %-8s) \033[1;36m[UNLOCKED in RAM]\033[0m\n' "$name" "$size"
            else
                printf '  • 🔒 \033[1;33m%-20s\033[0m (Size: %-8s) \033[1;30m[LOCKED]\033[0m\n' "$name" "$size"
            fi
            count=$((count + 1))
        fi
    done
    if [[ $count -eq 0 ]]; then
        printf '  \033[1;30m(No encrypted vaults found)\033[0m\n'
    fi
    printf '\n'
    read -r "?Press Enter to continue..." _ </dev/tty
}

_fb_vault_config() {
    _fb_vault_banner
    printf '\033[1;36m⚙️  Telegram Alert Configuration:\033[0m\n\n'
    read -r "?Telegram Bot Token: " token </dev/tty
    read -r "?Telegram Chat ID: " chat_id </dev/tty

    if [[ -n "$token" && -n "$chat_id" ]]; then
        export FB_VAULT_TELEGRAM_BOT_TOKEN="$token"
        export FB_VAULT_TELEGRAM_CHAT_ID="$chat_id"
        printf '\033[1;32m[✓] Telegram alert configuration saved successfully!\033[0m\n'
    else
        printf '\033[1;33m[!] Telegram configuration left empty.\033[0m\n'
    fi
    read -r "?Press Enter to continue..." _ </dev/tty
}

_fb_vault_delete() {
    _fb_vault_ensure_store
    _fb_vault_banner

    # Collect available locked vaults
    local available_vaults=()
    while IFS= read -r v; do
        [[ -n "$v" ]] && available_vaults+=("$v")
    done < <(find "$_fb_vault_store" -maxdepth 1 -name "*.enc" -exec basename {} .enc \; 2>/dev/null)

    if [[ ${#available_vaults[@]} -eq 0 ]]; then
        printf '\033[1;31m  [!] No locked vaults found to delete!\033[0m\n'
        read -r "?Press Enter to continue..." _ </dev/tty
        return 0
    fi

    # Let user select vault
    local selected_folder=""
    if command -v fzf &>/dev/null; then
        selected_folder=$(printf '%s\n' "${available_vaults[@]}" | \
            fzf --height=10 --layout=reverse --border=rounded \
                --prompt="🗑️  Select vault to DELETE > " \
                --color="border:196,header:220,pointer:196,fg:255")
    elif command -v gum &>/dev/null; then
        selected_folder=$(gum choose --header="🗑️  Select vault to DELETE:" "${available_vaults[@]}")
    else
        printf 'Available Vaults:\n'
        printf '  - %s\n' "${available_vaults[@]}"
        read -r "?Enter vault folder name to delete: " selected_folder </dev/tty
    fi

    [[ -z "$selected_folder" ]] && return 0

    local encrypted_file="$_fb_vault_store/${selected_folder}.enc"
    local panic_hash_file="$_fb_vault_store/${selected_folder}.panic"
    local ram_base=$(_fb_vault_detect_ram_base)
    local tmp_dec="$ram_base/.tmp_verify_${selected_folder}_$$.tar.gz"

    printf '\n'
    if command -v gum &>/dev/null; then
        gum style --foreground 196 --bold --border rounded --border-foreground 196 \
            --padding "0 2" --width 58 \
            "⚠️  WARNING: Permanently delete vault '${selected_folder}'?" \
            "This action CANNOT be undone!"
    else
        printf '\033[1;31m  ⚠️  WARNING: This will PERMANENTLY delete vault "%s"!\033[0m\n' "$selected_folder"
        printf '\033[1;31m  This action CANNOT be undone!\033[0m\n'
    fi
    printf '\n'

    # Verify master password before deleting
    local pass=$(_fb_vault_fancy_input "Enter Master Password to CONFIRM Delete" "[$selected_folder]" "🗑️")
    if [[ -z "$pass" ]]; then
        printf '\033[1;33m  [!] Password empty. Delete cancelled.\033[0m\n'
        read -r "?Press Enter to continue..." _ </dev/tty
        return 1
    fi

    # Verify password by attempting decryption and testing tar archive integrity
    rm -f "$tmp_dec" 2>/dev/null
    openssl enc -d -aes-256-cbc -pbkdf2 -iter $_fb_vault_iterations \
        -pass pass:"$pass" -in "$encrypted_file" -out "$tmp_dec" 2>/dev/null

    local verify_ok=1
    if [[ -s "$tmp_dec" ]] && tar -tzf "$tmp_dec" &>/dev/null; then
        verify_ok=0
    fi
    rm -f "$tmp_dec" 2>/dev/null

    if [[ $verify_ok -ne 0 ]]; then
        printf '\033[1;31m  [X] Wrong password! Delete aborted. Vault is safe.\033[0m\n'
        read -r "?Press Enter to continue..." _ </dev/tty
        return 1
    fi

    # Password verified — permanently destroy
    _fb_vault_secure_delete "$encrypted_file"
    [[ -f "$panic_hash_file" ]] && _fb_vault_secure_delete "$panic_hash_file"

    printf '\033[1;32m  [✓] Vault "%s" permanently deleted and securely wiped.\033[0m\n' "$selected_folder"
    _fb_vault_send_alert "Vault $selected_folder permanently deleted by user."
    read -r "?Press Enter to continue..." _ </dev/tty
}

vault() {
    fancybash ensure-dep openssl openssl openssl openssl || return 1
    fancybash ensure-dep tar tar tar tar || return 1

    local action="${1:-}"

    case "$action" in
        lock)
            _fb_vault_lock "${2:-}"
            ;;
        unlock)
            _fb_vault_unlock "${2:-}"
            ;;
        list)
            _fb_vault_list
            ;;
        delete|remove)
            _fb_vault_delete
            ;;
        config)
            _fb_vault_config
            ;;
        --help|-h|help)
            _fb_vault_banner
            printf 'Usage: vault [command] [options]\n\n'
            printf 'Commands:\n'
            printf '  vault lock [path]     – Encrypt & lock folder\n'
            printf '  vault unlock [name]   – Unlock & decrypt vault to RAM\n'
            printf '  vault list            – List all encrypted vaults\n'
            printf '  vault delete          – Delete a vault (password verified)\n'
            printf '  vault config          – Configure Telegram security alert\n'
            printf '  vault --help          – Display this help message\n'
            printf '\nAliases: secvault, fvault, fancy_vault\n'
            ;;
        "")
            while true; do
                _fb_vault_banner
                local choice=""
                if command -v fzf &>/dev/null; then
                    choice=$(printf "🔒 Lock Directory\n🔓 Unlock Directory\n📋 List Vaults\n🗑️ Delete Vault\n⚙️ Telegram Config\n❌ Exit" | \
                        fzf --height=12 \
                            --layout=reverse \
                            --border=rounded \
                            --margin=1,2 \
                            --prompt="⚡ Select Option > " \
                            --color="border:99,header:220,pointer:212,fg:255")
                elif command -v gum &>/dev/null; then
                    choice=$(gum choose "🔒 Lock Directory" "🔓 Unlock Directory" "📋 List Vaults" "🗑️ Delete Vault" "⚙️ Telegram Config" "❌ Exit")
                else
                    printf '1) 🔒 Lock Directory\n2) 🔓 Unlock Directory\n3) 📋 List Vaults\n4) 🗑️ Delete Vault\n5) ⚙️ Telegram Config\n6) ❌ Exit\n'
                    read -r "?Select option: " choice </dev/tty
                fi

                case "$choice" in
                    *"Lock Directory"*|1)
                        _fb_vault_lock
                        ;;
                    *"Unlock Directory"*|2)
                        _fb_vault_unlock
                        ;;
                    *"List Vaults"*|3)
                        _fb_vault_list
                        ;;
                    *"Delete Vault"*|4)
                        _fb_vault_delete
                        ;;
                    *"Telegram Config"*|5)
                        _fb_vault_config
                        ;;
                    *"Exit"*|6)
                        break
                        ;;
                    *)
                        break
                        ;;
                esac
            done
            ;;
        *)
            printf '❌ Invalid command: %s. Run "vault --help" for details.\n' "$action"
            return 1
            ;;
    esac
}

# Aliases for Hardened Multi-Vault Manager
alias secvault='vault'
alias fvault='vault'
alias fancy_vault='vault'

# --- FancyBash Developer & Performance Helpers ---
fb-recompile() {
    local target="${FB_DIR:-$HOME/.fancybash}/config.zsh"
    [[ ! -f "$target" ]] && target="$HOME/.zshrc"
    if zcompile -R "${target}.zwc" "$target" 2>/dev/null; then
        echo "✔ FancyBash Zsh bytecode recompiled (${target}.zwc)."
    else
        echo "✘ Recompilation failed for ${target}."
        return 1
    fi
}
alias fb-zsh-bench='if command -v hyperfine >/dev/null 2>&1; then hyperfine "zsh -i -c exit"; else time zsh -i -c exit; fi'

# =====================================================
# End of .zshrc
# =====================================================

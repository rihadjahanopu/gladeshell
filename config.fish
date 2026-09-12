# ==============================================================================
#   ULTRA-THIN COMPACT PRO FISH ENVIRONMENT
#   Author: [Rihad Jahan Opu]
#   Version: 2.0.0 Complete Multi-Distro Edition
#   Purpose: A fast, beautiful, and productive terminal for Web Development
#   Supports: Ubuntu/Debian, Fedora/RHEL/CentOS, Arch, macOS, Alpine, openSUSE
#   Verified: 2026 - Cross-platform compatibility
# ==============================================================================

# Disable default Fish greeting
set -g fish_greeting ""

# ======================================================
# 🎨 RAINBOW COLOR & EMOJI SETUP
# ======================================================

set -g rainbow_colors 31 32 33 34 35 36 91 92 93 94 95 96

functions -e rand_color 2>/dev/null
function rand_color
    set -l idx (random 1 (count $rainbow_colors))
    echo $rainbow_colors[$idx]
end

functions -e rand_emoji 2>/dev/null
function rand_emoji
    set -l folder (basename (pwd))
    switch $folder
        case "*web*"
            echo "🌐"
        case "*node*"
            echo "🟢"
        case "*bun*"
            echo "🥐"
        case "*py*"
            echo "🐍"
        case "*proj*"
            echo "💻"
        case "*"
            set -l emojis 🔥 ⚡️ 🚀 💫 🌈 🌀 ✨ 🧠
            set -l idx (random 1 (count $emojis))
            echo $emojis[$idx]
    end
end

# ======================================================
# HELPERS
# ======================================================

function _fb_is_intel_or_amd
    if test -f /proc/cpuinfo; and grep -qiE 'intel|amd' /proc/cpuinfo >/dev/null 2>&1
        return 0
    end
    if command -v lspci >/dev/null 2>&1; and lspci 2>/dev/null | grep -qiE 'intel|amd|radeon' >/dev/null 2>&1
        return 0
    end
    return 1
end

function _fb_ensure_vulkan
    if not _fb_is_intel_or_amd
        return 0
    end
    if command -v vulkaninfo >/dev/null 2>&1
        return 0
    end
    echo -e "\033[1;33m⚡ Intel/AMD hardware detected. Auto-installing Vulkan drivers & tools...\033[0m"
    if command -v apt-get >/dev/null 2>&1; or command -v apt >/dev/null 2>&1
        sudo apt update -qq; and sudo apt install -y mesa-vulkan-drivers vulkan-tools
    else if command -v pacman >/dev/null 2>&1
        sudo pacman -S --noconfirm --needed vulkan-intel vulkan-radeon vulkan-tools
    else if command -v dnf >/dev/null 2>&1
        sudo dnf install -y mesa-vulkan-drivers vulkan-tools
    else if command -v zypper >/dev/null 2>&1
        sudo zypper install -y mesa-vulkan-drivers vulkan-tools
    else if command -v apk >/dev/null 2>&1
        sudo apk add vulkan-loader vulkan-tools
    end
end

# Central Dynamic Dependency Auto-Installer
function _fb_ensure_dep
    set -l cmd "$argv[1]"
    set -l apt_pkg "(if test -n "$argv[2]"; echo "$argv[2]"; else; echo "$cmd"; end)"
    set -l pac_pkg "(if test -n "$argv[3]"; echo "$argv[3]"; else; echo "$cmd"; end)"
    set -l dnf_pkg "(if test -n "$argv[4]"; echo "$argv[4]"; else; echo "$cmd"; end)"

    if command -v "$cmd" >/dev/null 2>&1
        return 0
    end

    if [ "$cmd" = "fd" ] && command -v fdfind >/dev/null 2>&1
        return 0
    end
    if [ "$cmd" = "bat" ] && command -v batcat >/dev/null 2>&1
        return 0
    end
    if [ "$cmd" = "exa" ] && command -v eza >/dev/null 2>&1
        return 0
    end
    if test "$cmd" = "vulkan"; or test "$cmd" = "vulkaninfo"
        _fb_ensure_vulkan
        return $status
    end

    echo -e "\033[1;33m⚡ Missing tool '$cmd'. Auto-installing for your Linux distro...\033[0m"

    if command -v apt-get >/dev/null 2>&1
        sudo apt-get update -qq && sudo apt-get install -y "$apt_pkg"
    else if command -v pacman >/dev/null 2>&1
        sudo pacman -S --noconfirm --needed "$pac_pkg"
    else if command -v dnf >/dev/null 2>&1
        sudo dnf install -y "$dnf_pkg"
    else if command -v zypper >/dev/null 2>&1
        sudo zypper install -y "$pac_pkg"
    else if command -v apk >/dev/null 2>&1
        sudo apk add "$pac_pkg"
    else if command -v brew >/dev/null 2>&1
        brew install "$pac_pkg"
    else
        echo -e "\033[1;31m❌ Package manager not found. Please install '$cmd' manually.\033[0m"
        return 1
    end
end

set -g _fb_git_cache_file "/tmp/.fb_git_cache_fish_${USER}_%self"

function _fb_update_git_async
    fish -c "
        set -l branch (git branch --show-current 2>/dev/null)
        if test -z \"\$branch\"
            set branch (git rev-parse --short HEAD 2>/dev/null)
            test -n \"\$branch\"; and set branch \"➦ \$branch\"
        end
        if test -n \"\$branch\"
            set -l dirty \"\"
            test -n (git status --porcelain --untracked-files=no 2>/dev/null); and set dirty \" ❗\"
            echo \"\$branch\$dirty\" > \"$_fb_git_cache_file\"
        else
            rm -f \"$_fb_git_cache_file\" 2>/dev/null
        end
    " >/dev/null 2>&1 &
    disown 2>/dev/null
end

functions -e parse_git_branch 2>/dev/null
function parse_git_branch
    if not test -d .git; and not git rev-parse --is-inside-work-tree >/dev/null 2>&1
        rm -f "$_fb_git_cache_file" 2>/dev/null
        return
    end
    _fb_update_git_async
    if test -f "$_fb_git_cache_file"
        cat "$_fb_git_cache_file" 2>/dev/null
    end
end

set -g _fb_versions_cache_file "/tmp/.fb_versions_cache_fish_$USER"
set -g _fb_last_path "$PATH"

function _fb_update_tool_versions_async
    fish -c "
        set -l n_ver ''
        set -l p_ver ''
        set -l b_ver ''
        command -v node >/dev/null 2>&1; and set n_ver \"🟢 (node -v)\"
        command -v npm  >/dev/null 2>&1; and set p_ver \"📦 v(npm -v)\"
        command -v bun  >/dev/null 2>&1; and set b_ver \"🥐 v(bun -v)\"

        echo \"set -g _fb_cached_node '\$n_ver'\" > \"$_fb_versions_cache_file\"
        echo \"set -g _fb_cached_npm '\$p_ver'\" >> \"$_fb_versions_cache_file\"
        echo \"set -g _fb_cached_bun '\$b_ver'\" >> \"$_fb_versions_cache_file\"
    " >/dev/null 2>&1 &
    disown 2>/dev/null
end

function _fb_check_and_refresh_versions_cache
    if not test -f "$_fb_versions_cache_file"; or test "$PATH" != "$_fb_last_path"
        set -g _fb_last_path "$PATH"
        _fb_update_tool_versions_async
        return
    end
end

_fb_check_and_refresh_versions_cache
test -f "$_fb_versions_cache_file"; and source "$_fb_versions_cache_file" 2>/dev/null

functions -e node_version 2>/dev/null
function node_version
    _fb_check_and_refresh_versions_cache
    test -f "$_fb_versions_cache_file"; and source "$_fb_versions_cache_file" 2>/dev/null
    echo "$_fb_cached_node"
end
functions -e npm_version 2>/dev/null
function npm_version
    _fb_check_and_refresh_versions_cache
    test -f "$_fb_versions_cache_file"; and source "$_fb_versions_cache_file" 2>/dev/null
    echo "$_fb_cached_npm"
end
functions -e bun_version 2>/dev/null
function bun_version
    _fb_check_and_refresh_versions_cache
    test -f "$_fb_versions_cache_file"; and source "$_fb_versions_cache_file" 2>/dev/null
    echo "$_fb_cached_bun"
end
functions -e time_date 2>/dev/null
function time_date echo "📅 (date +'%b %d')"; }

functions -e sys_info 2>/dev/null
function sys_info
    if test -f /proc/meminfo
    set -l mem_total (awk '/MemTotal/ {print $argv[2]}' /proc/meminfo 2>/dev/null)
    set -l mem_avail (awk '/MemAvailable/ {print $argv[2]}' /proc/meminfo 2>/dev/null)
        if test -n "$mem_total" && -n "$mem_avail"
    set -l mem_used (((mem_total - mem_avail) / 1024))
    set -l mem_total_mb ((mem_total / 1024))
            echo "🧠 ${mem_used}M/${mem_total_mb}M"
            return
        end
    end
    if command -v free >/dev/null 2>&1
    set -l RAM (free -h 2>/dev/null | awk '/^Mem/ {print $argv[3] "/" $argv[2]}')
        test -n "$RAM" && echo "🧠 ${RAM}"
    end
end

functions -e battery_info 2>/dev/null
function battery_info
    test -f /sys/class/power_supply/BAT0/capacity && echo "🔋(cat /sys/class/power_supply/BAT0/capacity)%"
end

functions -e kernel_version 2>/dev/null
function kernel_version
    echo "🐧 (uname -r | cut -d'-' -f1)"
end

functions -e cpu_temp 2>/dev/null
function cpu_temp
    set -l temp ""
    if test -f /sys/class/thermal/thermal_zone0/temp
    set -l raw (cat /sys/class/thermal/thermal_zone0/temp 2>/dev/null)
        test -n "$raw" && "$raw" -gt 0 && temp=((raw / 1000))
    end
    if test -z "$temp" && command -v sensors >/dev/null 2>&1
        temp=(sensors 2>/dev/null | grep -iE 'Package id 0|Core 0|temp1' | head -n1 | grep -oP '\+\K[0-9.]+' | head -n1 | cut -d. -f1)
    end
    if test -n "$temp"
        if test "$temp" -gt 70
            echo -e " \e[91m🌡️ ${temp}°C\e[0m"
        else if test "$temp" -gt 55
            echo -e " \e[93m🌡️ ${temp}°C\e[0m"
        else
            echo -e " \e[92m🌡️ ${temp}°C\e[0m"
        end
    end
end

# 📁 FOLDER SIZE
functions -e folder_size 2>/dev/null
function folder_size
    set -l size ""
    if command -v timeout >/dev/null 2>&1
        size=(timeout 0.2s du -sh . 2>/dev/null | cut -f1)
    else
        size=(du -sh . 2>/dev/null | cut -f1)
    end
    test -n "$size" && echo "📂 ${size}" || echo "📂 ~"
end

functions -e disk_usage 2>/dev/null
function disk_usage
    echo " 💽 (df -h / | awk 'NR==2 {print $argv[4]}') free"
end

functions -e load_avg 2>/dev/null
function load_avg
    echo " ⚖️ (uptime | awk -F'load average:' '{ print $argv[2] }' | cut -d',' -f1 | sed 's/ //g')"
end

# এটি কাজ করার জন্য আপনার config.fish এর একদম শুরুতে এই ২ লাইন থাকতে হবে:
function timer_start timer=(if test -n "$timer"; echo "$timer"; else; echo "$SECONDS"; end); }
trap 'timer_start' DEBUG

functions -e get_duration 2>/dev/null
function get_duration
    set -l delta ((SECONDS - timer))
    if test $delta -ge 1
        echo " ⏱️ ${delta}s"
    end
    unset timer
end

functions -e check_readonly 2>/dev/null
function check_readonly
    [ ! -w . ] && echo " 🔒"
end

functions -e pending_updates 2>/dev/null
function pending_updates
    set -l updates 0

    # ১. আর্চ লিনাক্স (Arch Linux) এর জন্য চেক
    if command -v checkupdates >/dev/null 2>&1
        updates=(checkupdates 2>/dev/null | wc -l)

    # ২. ফেডোরা (Fedora / RHEL) এর জন্য চেক
    else if command -v dnf >/dev/null 2>&1
        updates=(dnf check-update -q 2>/dev/null | grep -c '^[a-zA-Z0-9]')

    # ৩. ডেবিয়ান/উবুন্টু/ডিপিন (Deepin) এর জন্য চেক
    else if test -f /var/lib/update-notifier/updates-available
        set updates (cat /var/lib/update-notifier/updates-available | grep -Po '^[0-9]+(?= updates? can be installed)' | head -n1)

    # ৪. অল্টারনেটিভ ডেবিয়ান পদ্ধতি (যদি ফাইল না থাকে)
    else if command -v apt-get >/dev/null 2>&1
        # এটি কিছুটা স্লো হতে পারে, তাই সাইলেন্টলি চেক করবে
        updates=(apt-get -s upgrade 2>/dev/null | grep -iP '^[0-9]+ upgraded' | cut -d' ' -f1)
    end

    # যদি আপডেট ১ এর বেশি হয় তবেই দেখাবে
    if test -n "$updates" && "$updates" -gt 0
        echo " 🆙 $updates"
    end
end

# ======================================================
# 🎨 PROMPT THEME ENGINE & CUSTOMIZATION (fancy_theme)
# ======================================================

# ⚡ Pre-cmd Telemetry Cache (Smart: only compute what the active theme needs)
function _fb_precmd
    set -g _fb_git ""
    set -g _fb_emoji ""
    set -g _fb_color ""
    set -g _fb_size ""
    set -g _fb_node ""
    set -g _fb_npm ""
    set -g _fb_bun ""
    set -g _fb_temp ""
    set -g _fb_date ""

    set -l theme_name "minimal"
    if test -n "$argv[1]"
        set theme_name "$argv[1]"
    else if test -f ~/.fancybash_theme
        set -l saved (cat ~/.fancybash_theme 2>/dev/null | string trim)
        test -n "$saved"; and set theme_name "$saved"
    end

    set -l theme_code ""
    if functions -q "fb_theme_$theme_name"
        set theme_code (functions "fb_theme_$theme_name")
    end

    string match -q "*_fb_git*" -- "$theme_code"; and set -g _fb_git (parse_git_branch)
    string match -q "*_fb_emoji*" -- "$theme_code"; and set -g _fb_emoji (rand_emoji)
    string match -q "*_fb_color*" -- "$theme_code"; and set -g _fb_color (rand_color)
    string match -q "*_fb_size*" -- "$theme_code"; and set -g _fb_size (folder_size)
    string match -q "*_fb_node*" -- "$theme_code"; and set -g _fb_node (node_version)
    string match -q "*_fb_npm*" -- "$theme_code"; and set -g _fb_npm (npm_version)
    string match -q "*_fb_bun*" -- "$theme_code"; and set -g _fb_bun (bun_version)
    string match -q "*_fb_temp*" -- "$theme_code"; and set -g _fb_temp (cpu_temp)
    string match -q "*_fb_date*" -- "$theme_code"; and set -g _fb_date (time_date)
end

# --- Theme 1: Minimal (Default) ---
function fb_theme_minimal
    set -l last_status $argv[1]
    set -l color_code $_fb_color
    test -z "$color_code"; and set color_code (rand_color)
    set -l emoji_symbol $_fb_emoji
    test -z "$emoji_symbol"; and set emoji_symbol (rand_emoji)
    set -l dir_name (basename (pwd))
    test "$dir_name" = "$USER"; and set dir_name "~"
    test -z "$dir_name"; and set dir_name "/"

    set -l prompt_color "\033[1;{$color_code}m"
    set -l reset "\033[0m"

    echo -n -e "{$emoji_symbol} {$prompt_color}{$dir_name}{$reset}\n"

    if test $last_status -eq 0
        echo -n -e "\033[1;36m❯❯❯ \033[0m"
    else
        echo -n -e "\033[1;31m❯❯❯ \033[0m"
    end
end

# --- Theme 2: Full (Detailed Two-Line) ---
function fb_theme_full
    set -l last_status $argv[1]
    set -l color_code $_fb_color
    test -z "$color_code"; and set color_code (rand_color)
    set -l emoji_symbol $_fb_emoji
    test -z "$emoji_symbol"; and set emoji_symbol (rand_emoji)
    set -l dir_name (basename (pwd))
    test "$dir_name" = "$USER"; and set dir_name "~"
    test -z "$dir_name"; and set dir_name "/"

    set -l prompt_color "\033[1;{$color_code}m"
    set -l reset "\033[0m"

    echo -n -e "{$emoji_symbol} {$prompt_color}{$dir_name}{$reset} "
    if test -n "$_fb_git"
        echo -n -e "\033[1;33m($_fb_git)$reset"
    end
    echo -n -e "$_fb_size$_fb_temp$(disk_usage)\n"

    if test $last_status -eq 0
        echo -n -e "\033[1;36m❯❯❯ \033[0m"
    else
        echo -n -e "\033[1;31m❯❯❯ \033[0m"
    end
end

# --- Theme 3: RobbyRussell ---
function fb_theme_robbyrussell
    echo -n -e "\033[1;32m➜  \033[1;36m"(basename (pwd))"\033[0m"
    if test -n "$_fb_git"
        echo -n -e " \033[1;35mgit:($_fb_git)\033[0m"
    end
    echo -n -e " \033[1;32m❯ \033[0m"
end

# --- Theme 4: Powerlevel10k ---
function fb_theme_p10k
    set -l last_status $argv[1]
    echo -n -e "\033[1;34m╭─\033[0m 🐧 \033[1;32m$USER@fancybash\033[0m in \033[1;36m"(basename (pwd))"\033[0m $_fb_size$_fb_git\n"
    if test $last_status -eq 0
        echo -n -e "\033[1;34m╰─\033[1;32m❯ \033[0m"
    else
        echo -n -e "\033[1;34m╰─\033[1;31m❯ \033[0m"
    end
end

# --- Theme 5: Agnoster ---
function fb_theme_agnoster
    echo -n -e "\033[44;30m 💻 $USER \033[42;34m \033[30m"(basename (pwd))" \033[43;32m \033[30m$_fb_git \033[0;33m\033[0m\n\033[1;36m❯ \033[0m"
end

# --- Theme 6: Catppuccin ---
function fb_theme_catppuccin
    echo -n -e "\033[38;2;203;166;247m\033[48;2;203;166;247;38;2;30;30;46m 🐱 catppuccin \033[48;2;137;180;250;38;2;203;166;247m\033[48;2;137;180;250;38;2;30;30;46m 📂 "(basename (pwd))" \033[0;38;2;137;180;250m\033[0m$_fb_git\n\033[38;2;245;194;231m❯❯❯ \033[0m"
end

# --- Theme 7: TokyoNight ---
function fb_theme_tokyonight
    echo -n -e "\033[1;35m🌌 \033[1;36m"(basename (pwd))"\033[0m \033[38;2;125;207;255m$_fb_git\033[0m\n\033[1;35m⚡ \033[0m"
end

# --- Theme 8: Dracula ---
function fb_theme_dracula
    echo -n -e "\033[38;2;189;147;249m🧛 $USER \033[38;2;255;121;198min \033[38;2;139;233;253m"(basename (pwd))" $_fb_git\n\033[38;2;80;250;123m❯ \033[0m"
end

# --- Theme 9: Nord ---
function fb_theme_nord
    echo -n -e "\033[38;2;136;192;208m❄️  "(basename (pwd))" \033[38;2;129;161;193m$_fb_git\n\033[38;2;143;188;187m❯ \033[0m"
end

# --- Theme 10: Bira ---
function fb_theme_bira
    echo -n -e "\033[1;33m╭─\033[1;32m$USER@fancybash \033[1;34m"(basename (pwd))" \033[1;35m$_fb_git\n\033[1;33m╰─\033[1;32m\$ \033[0m"
end

# --- Theme 11: Pure ---
function fb_theme_pure
    echo -n -e "\n\033[1;34m"(basename (pwd))"\033[0m $_fb_git\n\033[1;32m❯ \033[0m"
end

# --- Theme 12: Starship ---
function fb_theme_starship
    echo -n -e "\033[1;35m🚀 $USER \033[1;32min \033[1;36m"(basename (pwd))"\033[0m $_fb_git\n\033[1;33m❯ \033[0m"
end

# --- Theme 13: Cyberpunk ---
function fb_theme_cyberpunk
    echo -n -e "\033[48;2;255;238;0;38;2;0;0;0m ⚡ CYBER \033[48;2;0;240;255;38;2;0;0;0m "(basename (pwd))" \033[0m$_fb_git\n\033[38;2;255;0;85m▶▶ \033[0m"
end

# --- Theme 14: Synthwave ---
function fb_theme_synthwave
    echo -n -e "\033[38;2;255;0;127m🌅 \033[38;2;0;255;255m"(basename (pwd))" \033[38;2;157;0;255m$_fb_git\n\033[38;2;255;0;127m❯❯ \033[0m"
end

# --- Theme 15: Gruvbox ---
function fb_theme_gruvbox
    echo -n -e "\033[38;2;250;189;47m🌴 $USER \033[38;2;142;192;124min "(basename (pwd))" \033[38;2;254;128;25m$_fb_git\n\033[38;2;250;189;47m❯ \033[0m"
end

# --- Theme 16: OneDark ---
function fb_theme_onedark
    echo -n -e "\033[38;2;97;175;239m🌐 $USER \033[38;2;198;120;221m"(basename (pwd))" \033[38;2;152;195;121m$_fb_git\n\033[38;2;97;175;239m❯ \033[0m"
end

# --- Theme 17: Sorin ---
function fb_theme_sorin
    echo -n -e "\033[1;34m$USER@fancybash \033[1;36m"(basename (pwd))"\033[0m $_fb_git\n\033[1;35m❯ \033[0m"
end

# --- Theme 18: Spaceship ---
function fb_theme_spaceship
    echo -n -e "\n\033[1;34m🚀 $USER\033[0m in \033[1;36m"(basename (pwd))"\033[0m $_fb_git\n\033[1;32m➜ \033[0m"
end

# --- Theme 19: HalfLife ---
function fb_theme_halflife
    echo -n -e "\033[38;2;255;152;0mλ \033[38;2;255;235;59m"(basename (pwd))" \033[38;2;255;152;0m$_fb_git\n\033[38;2;255;152;0m▶ \033[0m"
end

# --- Theme 20: Paradox ---
function fb_theme_paradox
    echo -n -e "\033[44;37m ⚡ "(basename (pwd))" \033[45;34m \033[37m$_fb_git \033[0;35m\033[0m\n❯ "
end

# --- Theme 21: Bureau ---
function fb_theme_bureau
    echo -n -e "\033[1;30m[\033[1;33m$USER@fancybash\033[1;30m] \033[1;36m"(basename (pwd))"\033[0m $_fb_git\n\033[1;32m❯ \033[0m"
end

# --- Theme 22: Gallifrey ---
function fb_theme_gallifrey
    echo -n -e "\033[38;2;255;215;0m⏳ \033[38;2;0;191;255m"(basename (pwd))" \033[38;2;255;215;0m$_fb_git\n\033[38;2;0;191;255m❯ \033[0m"
end

# --- Theme 23: Material ---
function fb_theme_material
    echo -n -e "\033[38;2;0;230;118m💎 $USER \033[38;2;0;229;255m"(basename (pwd))" \033[38;2;213;0;249m$_fb_git\n\033[38;2;0;230;118m❯ \033[0m"
end

# --- Theme 24: Monokai ---
function fb_theme_monokai
    echo -n -e "\033[38;2;166;226;46m🔥 $USER \033[38;2;249;38;114min "(basename (pwd))" \033[38;2;230;219;116m$_fb_git\n\033[38;2;166;226;46m❯ \033[0m"
end

# --- Theme 25: Palenight ---
function fb_theme_palenight
    echo -n -e "\033[38;2;199;146;234m🍇 "(basename (pwd))" \033[38;2;137;221;255m$_fb_git\n\033[38;2;255;83;112m❯ \033[0m"
end

# --- Theme 26: PowerlineClassic ---
function fb_theme_powerlineclassic
    echo -n -e "\033[44;37m $USER@fancybash \033[42;34m \033[30m"(basename (pwd))" \033[0;32m\033[0m $_fb_git\n❯ "
end

# --- Theme 27: Lambda ---
function fb_theme_lambda
    echo -n -e "\033[1;32mλ \033[1;34m"(basename (pwd))"\033[0m $_fb_git \033[1;32m❯\033[0m "
end

# --- Theme 28: Hyper ---
function fb_theme_hyper
    echo -n -e "\033[38;2;255;0;255m⚡ \033[38;2;0;255;255m"(basename (pwd))" \033[38;2;255;255;0m$_fb_git\n\033[38;2;255;0;255m❯ \033[0m"
end

# --- Theme 29: Slick ---
function fb_theme_slick
    echo -n -e "\033[1;32m● \033[1;36m"(basename (pwd))"\033[0m $_fb_git \033[1;32m❯\033[0m "
end

# --- Theme 30: Matrix ---
function fb_theme_matrix
    echo -n -e "\033[38;2;0;255;0m📟 $USER@fancybash:"(basename (pwd))" $_fb_git\n\033[38;2;0;255;0m[matrix]❯ \033[0m"
end

# --- Theme 31: Sunset ---
function fb_theme_sunset
    echo -n -e "\033[38;2;255;107;107m🌇 "(basename (pwd))" \033[38;2;254;202;87m$_fb_git\n\033[38;2;95;39;205m❯ \033[0m"
end

# --- Theme 32: Solarized ---
function fb_theme_solarized
    echo -n -e "\033[38;2;181;137;0m☀️ $USER \033[38;2;42;161;152min "(basename (pwd))" \033[38;2;203;75;22m$_fb_git\n\033[38;2;38;139;210m❯ \033[0m"
end

# --- Theme 33: RosePine ---
function fb_theme_rosepine
    echo -n -e "\033[38;2;235;111;146m🌹 "(basename (pwd))" \033[38;2;194;157;224m$_fb_git\n\033[38;2;246;193;119m❯ \033[0m"
end

# --- Theme 34: Everforest ---
function fb_theme_everforest
    echo -n -e "\033[38;2;167;192;128m🌲 $USER \033[38;2;226;183;110m"(basename (pwd))" \033[38;2;127;187;179m$_fb_git\n\033[38;2;167;192;128m❯ \033[0m"
end

# --- Theme 35: Kanagawa ---
function fb_theme_kanagawa
    echo -n -e "\033[38;2;126;156;216m🌊 $USER \033[38;2;224;109;118min "(basename (pwd))" \033[38;2;224;158;114m$_fb_git\n\033[38;2;152;171;117m❯ \033[0m"
end

# --- Theme 36: NightOwl ---
function fb_theme_nightowl
    echo -n -e "\033[38;2;130;170;255m🦉 $USER \033[38;2;127;219;202m"(basename (pwd))" \033[38;2;199;146;234m$_fb_git\n\033[38;2;255;203;107m❯ \033[0m"
end

# --- Theme 37: Cobalt2 ---
function fb_theme_cobalt2
    echo -n -e "\033[38;2;255;196;0m⚡ $USER \033[38;2;0;136;255min "(basename (pwd))" \033[38;2;0;229;255m$_fb_git\n\033[38;2;255;196;0m❯ \033[0m"
end

# --- Theme 38: ShadesOfPurple ---
function fb_theme_shadesofpurple
    echo -n -e "\033[38;2;189;147;249m🍇 "(basename (pwd))" \033[38;2;255;214;0m$_fb_git\n\033[38;2;255;128;0m❯❯ \033[0m"
end

# --- Theme 39: Ayu ---
function fb_theme_ayu
    echo -n -e "\033[38;2;255;143;64m🎨 $USER \033[38;2;149;230;203m"(basename (pwd))" \033[38;2;255;215;0m$_fb_git\n\033[38;2;255;143;64m❯ \033[0m"
end

# --- Theme 40: Snazzy ---
function fb_theme_snazzy
    echo -n -e "\033[38;2;255;92;87m✨ "(basename (pwd))" \033[38;2;95;250;224m$_fb_git\n\033[38;2;255;106;193m❯ \033[0m"
end

# --- Theme 41: Outrun ---
function fb_theme_outrun
    echo -n -e "\033[38;2;255;0;127m🌆 $USER \033[38;2;0;240;255min "(basename (pwd))" \033[38;2;255;200;0m$_fb_git\n\033[38;2;255;0;127m▶▶ \033[0m"
end

# --- Theme 42: Oceanic ---
function fb_theme_oceanic
    echo -n -e "\033[38;2;102;153;204m🌊 "(basename (pwd))" \033[38;2;236;95;103m$_fb_git\n\033[38;2;95;179;179m❯ \033[0m"
end

# --- Theme 43: Moonlight ---
function fb_theme_moonlight
    echo -n -e "\033[38;2;130;170;255m🌙 $USER \033[38;2;180;160;255m"(basename (pwd))" \033[38;2;137;221;255m$_fb_git\n\033[38;2;197;134;192m❯ \033[0m"
end

# --- Theme 44: PaperColor ---
function fb_theme_papercolor
    echo -n -e "\033[38;2;95;175;135m📜 $USER@fancybash \033[38;2;215;135;0m"(basename (pwd))" \033[38;2;175;0;95m$_fb_git\n\033[38;2;0;95;135m❯ \033[0m"
end

# --- Theme 45: Horizon ---
function fb_theme_horizon
    echo -n -e "\033[38;2;233;92;114m🌄 "(basename (pwd))" \033[38;2;240;144;122m$_fb_git\n\033[38;2;250;197;145m❯ \033[0m"
end

# --- Theme 46: CatppuccinFrappe ---
function fb_theme_catppuccin_frappe
    echo -n -e "\033[38;2;202;158;230m☕ $USER \033[38;2;186;194;222min "(basename (pwd))" \033[38;2;153;209;219m$_fb_git\n\033[38;2;244;184;228m❯❯ \033[0m"
end

# --- Theme 47: DraculaPro ---
function fb_theme_dracula_pro
    echo -n -e "\033[38;2;162;122;255m🗡️  $USER \033[38;2;255;128;191m"(basename (pwd))" \033[38;2;128;255;234m$_fb_git\n\033[38;2;80;250;123m▶ \033[0m"
end

# --- Theme 48: CyberSamurai ---
function fb_theme_cyber_samurai
    echo -n -e "\033[38;2;255;42;109m🥷 $USER \033[38;2;5;217;232m"(basename (pwd))" \033[38;2;255;242;0m$_fb_git\n\033[38;2;255;42;109m❯❯❯ \033[0m"
end

# --- Theme 49: Evergreen ---
function fb_theme_evergreen
    echo -n -e "\033[38;2;46;204;113m🍃 "(basename (pwd))" \033[38;2;26;188;156m$_fb_git\n\033[38;2;52;152;219m❯ \033[0m"
end

# --- Theme 50: Ghost ---
function fb_theme_ghost
    echo -n -e "\033[38;2;189;195;199m👻 $USER \033[38;2;149;165;166min "(basename (pwd))" \033[38;2;236;240;241m$_fb_git\n\033[38;2;127;140;141m❯ \033[0m"
end

# --- Theme 51: Oxide ---
function fb_theme_oxide
    echo -n -e "\033[38;2;211;84;0m⚙️  "(basename (pwd))" \033[38;2;230;126;34m$_fb_git\n\033[38;2;241;196;15m❯ \033[0m"
end

# --- Theme 52: NeonPulse ---
function fb_theme_neon_pulse
    echo -n -e "\033[38;2;57;255;20m🔮 $USER \033[38;2;255;20;147m"(basename (pwd))" \033[38;2;0;255;255m$_fb_git\n\033[38;2;57;255;20m⚡ \033[0m"
end

# --- Theme 53: Volcano ---
function fb_theme_volcano
    echo -n -e "\033[38;2;231;76;60m🌋 "(basename (pwd))" \033[38;2;192;57;43m$_fb_git\n\033[38;2;243;156;18m▶ \033[0m"
end

# --- Theme 54: Sakura ---
function fb_theme_sakura
    echo -n -e "\033[38;2;255;183;178m🌸 $USER \033[38;2;255;218;193min "(basename (pwd))" \033[38;2;226;240;203m$_fb_git\n\033[38;2;255;154;162m❯ \033[0m"
end

# --- Theme 55: Galaxy ---
function fb_theme_galaxy
    echo -n -e "\033[38;2;155;89;182m🌌 $USER \033[38;2;142;68;173m"(basename (pwd))" \033[38;2;52;152;219m$_fb_git\n\033[38;2;241;196;15m✨ ❯ \033[0m"
end




# --- Main Fish Prompt Hook ---
function fish_prompt
    _fb_precmd
    set -l last_status $status
    set -l current_theme "minimal"

    if test -f ~/.fancybash_theme
        set -l saved_theme (cat ~/.fancybash_theme 2>/dev/null | string trim)
        if functions -q "fb_theme_$saved_theme"
            set current_theme "$saved_theme"
        end
    end

    if functions -q "fb_theme_$current_theme"
        eval "fb_theme_$current_theme $last_status"
    else
        fb_theme_minimal $last_status
    end
end

# --- Theme Gallery Preview Function ---
function _fb_preview_all_themes
    _fb_precmd 2>/dev/null
    set -l available_themes (functions -a | string match -r '^fb_theme_.*' | string replace 'fb_theme_' '' | sort -u)
    set -l count (count $available_themes)
    set -l idx 1

    begin
        echo -e "\033[1;35m🎨 FANCYBASH PROMPT THEME GALLERY ({$count} Themes)\033[0m"
        echo -e "\033[36m──────────────────────────────────────────────────────────\033[0m"

        for t in $available_themes
            echo -e "\033[1;33m📌 [{$idx}/{$count}] $t\033[0m"
            "fb_theme_$t" 0 2>/dev/null
            echo -e "\033[38;2;60;60;80m──────────────────────────────────────────────────────────\033[0m"
            set idx (math "$idx + 1")
        end
    end | less -RFX
end

# --- Fancybash Auto-Upgrade & Shell Reload ---
# Internal marker refs (split to avoid grep false-positives in installed block)
set -g _FB_FISH_ID 'fancy-fishrc'
set -g _FB_FISH_START "# >>> $_FB_FISH_ID >>>"
set -g _FB_FISH_END   "# <<< $_FB_FISH_ID <<<"

function _fb_ensure_bottom --description "Ensure fancybash block is at the bottom of config.fish"
    set -l rc_file (test -n "$argv[1]"; and echo $argv[1]; or echo "$HOME/.config/fish/config.fish")
    set -l sm (test -n "$argv[2]"; and echo $argv[2]; or echo "$_FB_FISH_START")
    set -l em (test -n "$argv[3]"; and echo $argv[3]; or echo "$_FB_FISH_END")

    test -f "$rc_file"; or return 0
    grep -qF "$sm" "$rc_file" 2>/dev/null; or return 0
    grep -qF "$em" "$rc_file" 2>/dev/null; or return 0

    set -l end_ln (grep -n "$em" "$rc_file" | tail -1 | cut -d: -f1)
    set -l total_ln (wc -l < "$rc_file")

    if test -n "$end_ln"; and test "$end_ln" -lt "$total_ln"
        set -l start_ln (grep -n "$sm" "$rc_file" | head -1 | cut -d: -f1)
        test -n "$start_ln"; or return 0
        set -l tmp_file (mktemp 2>/dev/null; or echo "/tmp/fb_fish_reorder_$$")
        awk -v s="$start_ln" -v e="$end_ln" 'NR < s || NR > e' "$rc_file" > "$tmp_file"
        awk -v s="$start_ln" -v e="$end_ln" 'NR >= s && NR <= e' "$rc_file" >> "$tmp_file"
        mv "$tmp_file" "$rc_file" 2>/dev/null; or begin; cat "$tmp_file" > "$rc_file"; rm -f "$tmp_file"; end
    end
end

# Auto-reorder in background on interactive startup
if status is-interactive
    _fb_ensure_bottom "$HOME/.config/fish/config.fish" "$_FB_FISH_START" "$_FB_FISH_END" &>/dev/null &
end

functions -e fancy_upgrade 2>/dev/null
functions -e fancy_update 2>/dev/null
function fancy_upgrade --description "Upgrade fancybash to latest version and auto reload shell"
    echo -e "\033[1;35m⚡ Upgrading fancybash to latest version...\033[0m"
    if curl -fsSL https://fancybash.netlify.app/i.sh | bash; or curl -fsSL https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main/i.sh | bash
        _fb_ensure_bottom "$HOME/.config/fish/config.fish" "$_FB_FISH_START" "$_FB_FISH_END" 2>/dev/null
        echo -e "\033[1;32m✨ fancybash upgraded successfully! Auto-reloading shell...\033[0m"
        set -l current_shell (status fish-path 2>/dev/null; or echo "fish")
        exec $current_shell
    else
        echo -e "\033[1;31m❌ Upgrade failed. Please check your network connection.\033[0m"
        return 1
    end
end
function fancy_update --description "Alias for fancy_upgrade"
    fancy_upgrade $argv
end

# --- Interactive Theme Switcher Function ---
function fancy_theme --description "Interactive prompt theme switcher"
    set -l chosen_theme $argv[1]

    if test "$chosen_theme" = "preview" -o "$chosen_theme" = "list"
        _fb_preview_all_themes
        return 0
    end

    if test "$chosen_theme" = "upgrade" -o "$chosen_theme" = "update"
        fancy_upgrade $argv[2..-1]
        return $status
    end

    if test "$chosen_theme" = "sync" -o "$chosen_theme" = "bottom"
        _fb_ensure_bottom "$HOME/.config/fish/config.fish" "$_FB_FISH_START" "$_FB_FISH_END"
        echo -e "\033[1;32m✨ Fancybash position synced to absolute bottom of config.fish!\033[0m"
        return 0
    end

    set -l available_themes (functions -a | string match -r '^fb_theme_.*' | string replace 'fb_theme_' '')

    if test -z "$available_themes"
        echo "❌ No themes found!"
        return 1
    end

    if test -z "$chosen_theme"
        if command -v gum >/dev/null 2>&1
            set chosen_theme (string join \n $available_themes | gum choose --header="🎨 Select Fancybash Prompt Theme:")
        else if command -v fzf >/dev/null 2>&1
            set chosen_theme (string join \n $available_themes | fzf --prompt="🎨 Select Theme: " --height=50% --layout=reverse --border)
        else
            echo "🎨 Available Themes:"
            for t in $available_themes
                echo "   • $t"
            end
            read -P "👉 Select theme name: " chosen_theme
        end
    end

    test -z "$chosen_theme"; and return 0

    if functions -q "fb_theme_$chosen_theme"
        echo "$chosen_theme" > ~/.fancybash_theme
        echo "✅ Theme switched to: $chosen_theme"
    else
        echo "❌ Invalid theme '$chosen_theme'."
        echo "Available themes: $available_themes"
        return 1
    end
end

# Shortcut function: typing 'fancy' opens theme selector menu
function fancy --description "Shortcut for fancy_theme"
    fancy_theme $argv
end



# ======================================================
#  ⚡ INTERACTIVE SETUP SCRIPTS
# ======================================================

# --- Initialize a Project (Bun or NPM) ---
functions -e ii 2>/dev/null
function ii
    set -l has_bun 0 has_npm=0 has_pnpm=0 has_yarn=0
    command -v bun >/dev/null 2>&1 && has_bun=1
    command -v npm >/dev/null 2>&1 && has_npm=1
    command -v pnpm >/dev/null 2>&1 && has_pnpm=1
    command -v yarn >/dev/null 2>&1 && has_yarn=1

    echo "🚀 Select Package Manager:"
    test $has_bun -eq 1 && echo "1) 🥐 Bun (Fast)" || echo "1) 🥐 Bun (Not installed)"
    test $has_npm -eq 1 && echo "2) 📦 NPM (Standard)" || echo "2) 📦 NPM (Not installed)"
    test $has_pnpm -eq 1 && echo "3) 🟡 PNPM (Strict)" || echo "3) 🟡 PNPM (Not installed)"
    test $has_yarn -eq 1 && echo "4) 🧶 Yarn (Classic)" || echo "4) 🧶 Yarn (Not installed)"
    read -P "Enter choice [1-4]: " choice

    switch $choice
        case 1) test $has_bun -eq 1 && bun init -y || {
            echo "❌ Bun not found!"
            return 1
        } ;;
        case 2) test $has_npm -eq 1 && npm init -y || {
            echo "❌ NPM not found!"
            return 1
        } ;;
        case 3) test $has_pnpm -eq 1 && pnpm init || {
            echo "❌ PNPM not found!"
            return 1
        } ;;
        case 4) test $has_yarn -eq 1 && yarn init -y || {
            echo "❌ Yarn not found!"
            return 1
        } ;;
        case *
            echo "❌ Cancelled."
            return 1
    end

    # Create .gitignore if missing
    if test ! -f .gitignore
        echo "node_modules/" >.gitignore
        echo "✅ .gitignore created."
    end
    echo "✅ Project initialized!"
end

# --- Setup Next.js Project ---
functions -e next 2>/dev/null
function next
    echo "⚡ Setup Next.js with:"
    echo "1) Bun"
    echo "2) NPM"
    read -P "Choice: " c
    switch $c
        case 1) bunx create-next-app@latest . ;;
        case 2) npx create-next-app@latest . ;;
        case *) echo "Invalid choice" ;;
    end
end

# --- Setup Vite shadcn ui ---

# Auto-patch tsconfig/jsconfig with baseUrl and @/* paths
# Priority: tsconfig.app.json (Vite TS) -> tsconfig.json (Next.js TS) -> jsconfig.json (JS)
functions -e _ui_patch_tsconfig 2>/dev/null
function _ui_patch_tsconfig
    set -l tsconfig

    if test -f "tsconfig.app.json"
        tsconfig="tsconfig.app.json"
        echo "  info: Vite (TS) detected -> patching tsconfig.app.json"

    else if test -f "tsconfig.json"
        tsconfig="tsconfig.json"
        echo "  info: TypeScript project -> patching tsconfig.json"

    else if test -f "jsconfig.json"
        tsconfig="jsconfig.json"
        echo "  info: JavaScript project -> patching jsconfig.json"

    else
        # No config file found — check if JS project and create jsconfig.json
        if test -f "package.json" && ! grep -q '"typescript"' package.json 2>/dev/null
            echo "  note: JavaScript project detected — creating jsconfig.json with @/* alias..."
            node -e "
        const fs = require('fs');
        const jsconfig = {
          compilerOptions: {
            baseUrl: '.',
            paths: { '@/*': ['./src/*'] }
          end
        };
        fs.writeFileSync('jsconfig.json', JSON.stringify(jsconfig, null, 2));
        console.log('  -> jsconfig.json created with @/* alias');
      "
            return
        else
            echo "warning: No tsconfig/jsconfig found, skipping..."
            return
        end
    end

    # Check if paths already set
    if grep -q '"@/\*"' "$tsconfig"
        echo "  ok: $tsconfig paths already configured."
        return
    end

    echo "patching: $tsconfig with baseUrl & @/* paths..."

    # Use node to safely patch JSON
    node -e "
    const fs = require('fs');
    const raw = fs.readFileSync('$tsconfig', 'utf8');
    const json = JSON.parse(raw);
    if (!json.compilerOptions) json.compilerOptions = {};
    json.compilerOptions.baseUrl = '.';
    json.compilerOptions.paths = { '@/*': ['./src/*'] };
    fs.writeFileSync('$tsconfig', JSON.stringify(json, null, 2));
    console.log('  -> baseUrl & paths written to ' + '$tsconfig');
  "
end

# Auto-patch vite.config.ts with path alias and tailwind import
functions -e _ui_patch_viteconfig 2>/dev/null
function _ui_patch_viteconfig
    set -l viteconfig
    viteconfig=(ls vite.config.ts vite.config.js 2>/dev/null | head -n1)

    if test -z "$viteconfig"
        echo "warning: vite.config.ts/js not found, skipping..."
        return
    end

    echo "patching: $viteconfig with path alias & tailwind..."

    set -l content
    content=(cat "$viteconfig")

    # Add: import path from "path"
    if ! echo "$content" | grep -q 'import path from'
        sed -i '1s|^|import path from "path"
|' "$viteconfig"
        echo "  -> Added: import path from path"
    else
        echo "  ok: path import already exists."
    end

    # Add: import tailwindcss from "@tailwindcss/vite"
    if ! grep -q '@tailwindcss/vite' "$viteconfig"
        sed -i '1s|^|import tailwindcss from "@tailwindcss/vite"
|' "$viteconfig"
        echo "  -> Added: import tailwindcss from @tailwindcss/vite"
    else
        echo "  ok: tailwindcss import already exists."
    end

    # Add tailwindcss() to plugins array if missing
    if ! grep -q 'tailwindcss()' "$viteconfig"
        sed -i 's/plugins: \[/plugins: [tailwindcss(), /' "$viteconfig"
        echo "  -> Added: tailwindcss() to plugins"
    else
        echo "  ok: tailwindcss() plugin already exists."
    end

    # Add resolve.alias if missing (no leading comma)
    if ! grep -q '"@"' "$viteconfig" && ! grep -q "@:" "$viteconfig"
        sed -i '/^})/i\  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  }' "$viteconfig"
        echo "  -> Added: resolve.alias @/* -> ./src"
    else
        echo "  ok: resolve.alias already exists."
    end
end

functions -e ui 2>/dev/null
function ui
    echo "Setup Shadcn UI"
    echo ""

    # Auto-detect project type
    set -l project_type
    if test -f "tsconfig.app.json"
        project_type="vite"
        echo "  Detected: Vite project"
    else if test -f "next.config.js" || -f "next.config.ts" || -f "next.config.mjs"
        project_type="nextjs"
        echo "  Detected: Next.js project"
    else if grep -q '"next"' package.json 2>/dev/null
        project_type="nextjs"
        echo "  Detected: Next.js project (via package.json)"
    else
        echo "  Could not auto-detect project type."
        echo "  1) Vite (React)"
        echo "  2) Next.js"
        read -P "Choose manually: " pt
        switch $pt
            case 1) project_type="vite" ;;
            case 2) project_type="nextjs" ;;
            case *
                echo "Invalid choice"
                return
        end
    end

    echo ""
    echo "Package manager:"
    echo "1) Bun"
    echo "2) NPM"
    read -P "Choice: " pm

    read -P "Add specific components? (e.g. button card input): " components

    # STEP 1: Patch tsconfig FIRST — shadcn init requires @/* path alias
    echo ""
    echo "Pre-configuring path aliases before shadcn init..."
    _ui_patch_tsconfig

    if test "$project_type" == "vite"
        switch $pm
            case 1
                echo ""
                echo "Initializing Shadcn UI with Bun (Vite)..."
                bunx --bun shadcn@latest init -t vite
                if test -n "$components"
                    echo "Adding components: $components..."
                    bunx --bun shadcn@latest add $components
                else
                    echo "Adding default Button component..."
                    bunx --bun shadcn@latest add button
                end
            case 2
                echo ""
                echo "Initializing Shadcn UI with NPM (Vite)..."
                npx shadcn@latest init -t vite
                if test -n "$components"
                    echo "Adding components: $components..."
                    npx shadcn@latest add $components
                else
                    echo "Adding default Button component..."
                    npx shadcn@latest add button
                end
            case *
                echo "Invalid package manager choice"
                return
        end

        # STEP 2: Patch vite.config only for Vite projects
        echo ""
        echo "Patching vite.config with alias & tailwind..."
        _ui_patch_viteconfig

    else if test "$project_type" == "nextjs"
        switch $pm
            case 1
                echo ""
                echo "Initializing Shadcn UI with Bun (Next.js)..."
                bunx --bun shadcn@latest init
                if test -n "$components"
                    echo "Adding components: $components..."
                    bunx --bun shadcn@latest add $components
                else
                    echo "Adding default Button component..."
                    bunx --bun shadcn@latest add button
                end
            case 2
                echo ""
                echo "Initializing Shadcn UI with NPM (Next.js)..."
                npx shadcn@latest init
                if test -n "$components"
                    echo "Adding components: $components..."
                    npx shadcn@latest add $components
                else
                    echo "Adding default Button component..."
                    npx shadcn@latest add button
                end
            case *
                echo "Invalid package manager choice"
                return
        end
        echo "  Next.js detected — vite.config patch skipped."
    end

    echo ""
    echo "---------------------------------------------------"
    echo "Shadcn UI setup complete!"
    echo "Happy coding with Shadcn!"
    echo "---------------------------------------------------"
end

# --- Setup Vite (React/Vue) Project ---
functions -e vite 2>/dev/null
function vite
    echo "⚡ Setup Vite with:"
    echo "1) Bun"
    echo "2) NPM"
    read -P "Choice: " c

    read -P "Add Tailwind CSS v4? (y/n): " tw

    switch $c
        case 1
            bunx create-vite@latest .
            if test "$tw" == "y"
                if ! bun add tailwindcss @tailwindcss/vite
                    echo "❌ Install failed with Bun."
                    read -P "Try with --force? (y/n): " force
                    test "$force" = "y" && bun add tailwindcss @tailwindcss/vite --force
                end
            end
        case 2
            npx create-vite@latest .
            if test "$tw" == "y"
                if ! npm install tailwindcss @tailwindcss/vite
                    echo "❌ Install failed with NPM (Peer Dependency Conflict likely)."
                    read -P "Try with --legacy-peer-deps? (y/n): " legacy
                    test "$legacy" = "y" && npm install tailwindcss @tailwindcss/vite --legacy-peer-deps
                end
            end
        case *
            echo "Invalid choice"
            return
    end

    if test "$tw" == "y"
        # Create src folder and CSS file setup
        mkdir -p src
        CSS_FILE="src/index.css"
        [ -f "src/style.css" ] && CSS_FILE="src/style.css"

        # Add the Tailwind v4 import
        echo '@import "tailwindcss";' >"$CSS_FILE"

        echo "---------------------------------------------------"
        echo "✅ Tailwind CSS v4 packages installed!"
        echo "✅ Added '@import \"tailwindcss\";' to $CSS_FILE"
        echo ""
        echo "⚠️  ACTION REQUIRED: You must update your Vite config manually."
        echo ""
        echo "Open your vite.config.ts (or .js) and add these two lines:"
        echo "  1. import tailwindcss from '@tailwindcss/vite'"
        echo "  2. Add tailwindcss() to the plugins array."
        echo "---------------------------------------------------"
    end
end

# ======================================================
# 🚀 Install Tailwind CSS + Helpers
# ======================================================

functions -e css 2>/dev/null
function css
    if test ! -f package.json
        echo "❌ Error: package.json not found!"
        return 1
    end

    # Auto-detect package manager
    set -l pm "npm"
    test -f bun.lockb && pm="bun"

    echo "📦 Installing Tailwind via $pm..."
    if test "$pm" == "bun"
        bun add -D tailwindcss clsx tailwind-merge
        bunx tailwindcss init -p
    else
        npm install -D tailwindcss clsx tailwind-merge
        npx tailwindcss init -p
    end
    echo "✅ Tailwind CSS Ready!"
end

#  Kill Port (Usage: kp 3000)
functions -e kp 2>/dev/null
function kp
    if test -z "$argv[1]"
        echo "❌ Port number required!"
        return
    end
    lsof -ti:$argv[1] | xargs kill -9 >/dev/null 2>&1 && echo "✅ Port $argv[1] killed." || echo "❌ Port $argv[1] not in use."
end

# ======================================================
# 🚀 Universal Extractor (Usage: ex file.zip)
# ======================================================

functions -e ex 2>/dev/null
function ex
    if test -f "$argv[1]"
        switch $argv[1]
            *.tar.bz2 | *.tbz2) tar xjf "$argv[1]" ;;
            *.tar.gz | *.tgz) tar xzf "$argv[1]" ;;
            *.tar.xz) tar xJf "$argv[1]" ;;
            *.tar.zst | *.zst) unzstd "$argv[1]" 2>/dev/null || tar --zstd -xf "$argv[1]" ;;
            *.bz2) bunzip2 "$argv[1]" ;;
            *.rar) unrar x "$argv[1]" 2>/dev/null || 7z x "$argv[1]" ;;
            *.gz) gunzip "$argv[1]" ;;
            *.tar) tar xf "$argv[1]" ;;
            *.zip) unzip "$argv[1]" ;;
            *.7z) 7z x "$argv[1]" ;;
            case *) echo "❌ Unknown archive format" ;;
        end
    else
        echo "❌ '$argv[1]' is not a valid file"
    end
end

# Usage: ff filename (Dynamic fd auto-installer)
functions -e ff 2>/dev/null
function ff
    _fb_ensure_dep fd fd-find fd fd-find || return 1
    set -l fd_cmd "fd"
    command -v fdfind >/dev/null 2>&1 && fd_cmd="fdfind"
    "$fd_cmd" -H -E "node_modules" -E ".git" "$argv[1]"
end

# Secret Key Generator (Usage: gen 32)
functions -e gen 2>/dev/null
function gen
    set -l len "(if test -n "$argv[1]"; echo "$argv[1]"; else; echo "24"; end)"
    echo -e "🔑 Base64: \033[1;32m(openssl rand -base64 "$len" 2>/dev/null | cut -c1-"$len")\033[0m"
    echo -e "🔑 Hex:    \033[1;36m(openssl rand -hex "$len" 2>/dev/null | cut -c1-"$len")\033[0m"
end

# Backup File (Usage: bak .env)
functions -e bak 2>/dev/null
function bak
    cp "$argv[1]" "$argv[1].bak" && echo "✅ Created: $argv[1].bak"
end

# Global IP & Location Details
alias iploc='curl -s ipinfo.io/json | grep -E "ip|city|region|org"'

# Search Command History
# Usage: h git
alias h='history | grep'

# FZF History Search (Usage: fh - Dynamic fzf auto-installer)
functions -e fh 2>/dev/null
function fh
    _fb_ensure_dep fzf fzf fzf fzf || return 1
    set -l cmd (history | awk '{$argv[1]=""; print $0}' | fzf --reverse +s)
    test -n "$cmd" && eval "$cmd"
end

# Dynamic Auto-installing Wrappers for Modern CLI Tools
functions -e bat 2>/dev/null
function bat
    _fb_ensure_dep bat bat bat bat || return 1
    if command -v batcat >/dev/null 2>&1
        command batcat "$@"
    else
        command bat "$@"
    end
end

functions -e eza 2>/dev/null
function eza
    _fb_ensure_dep eza eza eza eza || return 1
    command eza "$@"
end

functions -e z 2>/dev/null
function z
    if ! command -v zoxide >/dev/null 2>&1
        _fb_ensure_dep zoxide zoxide zoxide zoxide || return 1
        eval "(zoxide init (if test -n "$SHELL_NAME"; echo "$SHELL_NAME"; else; echo "bash"; end))"
    end
    zoxide "$@"
end

functions -e tree 2>/dev/null
function tree
    if ! command -v tree >/dev/null 2>&1 && ! command -v eza >/dev/null 2>&1
        _fb_ensure_dep tree tree tree tree || return 1
    end
    if command -v eza >/dev/null 2>&1
        eza --tree "$@"
    else
        command tree "$@"
    end
end

functions -e tldr 2>/dev/null
function tldr
    _fb_ensure_dep tldr tldr tldr tldr || return 1
    command tldr "$@"
end

# Safe Delete - moves to system trash
functions -e trash 2>/dev/null
function trash
    if command -v gio >/dev/null 2>&1
        gio trash "$@" && echo "🗑 Moved to Trash via GIO."
    else
        mkdir -p ~/.local/share/Trash/files/ 2>/dev/null
        mv "$@" ~/.local/share/Trash/files/ 2>/dev/null || mv "$@" ~/.Trash/ 2>/dev/null && echo "🗑 Moved to Trash."
    end
end

# ======================================================
# 🚀 INTERACTIVE GIT WIP & PUSH
# ======================================================

functions -e gwip 2>/dev/null
functions -e gcommit 2>/dev/null

function gwip
    if ! command -v git >/dev/null 2>&1
        echo "❌ Git is not installed."
        return 1
    end

    if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1
        echo "❌ Not a git repository."
        return 1
    end

    # 1. Auto stage files
    git add .

    # Direct CLI argument mode
    if test (count $argv) -gt 0
        set -l first_arg $argv[1]
        set -l prefix "🚧 WIP"
        set -l rest_msg ""

        switch $first_arg
            case "feat" "✨"
                set prefix "✨ feat"
                set rest_msg (string join " " $argv[2..-1])
            case "fix" "🐛"
                set prefix "🐛 fix"
                set rest_msg (string join " " $argv[2..-1])
            case "docs" "📝"
                set prefix "📝 docs"
                set rest_msg (string join " " $argv[2..-1])
            case "style" "💄"
                set prefix "💄 style"
                set rest_msg (string join " " $argv[2..-1])
            case "refactor" "♻️"
                set prefix "♻️ refactor"
                set rest_msg (string join " " $argv[2..-1])
            case "test" "🧪"
                set prefix "🧪 test"
                set rest_msg (string join " " $argv[2..-1])
            case "chore" "🔧"
                set prefix "🔧 chore"
                set rest_msg (string join " " $argv[2..-1])
            case "wip" "🚧"
                set prefix "🚧 WIP"
                set rest_msg (string join " " $argv[2..-1])
            case "-m"
                set rest_msg (string join " " $argv[2..-1])
            case "*"
                set rest_msg (string join " " $argv)
        end

        if test -z "$rest_msg"
            set rest_msg "Save point ("(date +'%Y-%m-%d %H:%M')")"
        end
        set -l full_msg "$prefix: $rest_msg"
        git commit -m "$full_msg" || return 1

        set -l cur_branch (git branch --show-current 2>/dev/null)
        set -l push_cmd
        if test -n "$cur_branch"
            set push_cmd "git push -u origin \"$cur_branch\""
        else
            set push_cmd "git push -u"
        end

        set -l push_log (mktemp 2>/dev/null; or echo "/tmp/gwip_push.log")

        if command -v gum >/dev/null 2>&1
            if gum spin --spinner dot --title "Pushing to remote..." -- sh -c "$push_cmd >\"$push_log\" 2>&1"
                printf '\033[1;32m✅ Everything committed and pushed successfully!\033[0m\n'
                rm -f "$push_log" 2>/dev/null
            else
                if grep -qE "(non-fast-forward|fetch first|behind)" "$push_log" 2>/dev/null
                    echo -e "\033[1;33m🔄 Remote has new commits. Auto-syncing (git pull --rebase)...\033[0m"
                    rm -f "$push_log" 2>/dev/null
                    set -l sync_branch "$cur_branch"
                    if test -z "$sync_branch"
                        set sync_branch "HEAD"
                    end
                    if git pull --rebase origin "$sync_branch"
                        echo -e "\033[1;36m🚀 Retrying push...\033[0m"
                        if git push -u origin "$sync_branch"
                            printf '\033[1;32m✅ Synced and pushed successfully!\033[0m\n'
                            return 0
                        end
                    else
                        echo -e "\033[0;31m⚠️ Merge Conflict detected!\033[0m"
                        echo -e "\033[1;33mPlease resolve conflicts in VS Code, then run:\033[0m"
                        echo -e "  1) \033[1;36mgit add .\033[0m"
                        echo -e "  2) \033[1;36mgit rebase --continue\033[0m"
                        echo -e "  3) \033[1;36mgwip\033[0m"
                        return 1
                    end
                end

                echo -e "\033[0;31m❌ Push failed!\033[0m"
                if test -s "$push_log"
                    echo -e "\033[1;33mGit Error Details:\033[0m"
                    cat "$push_log"
                end
                rm -f "$push_log" 2>/dev/null
                echo -e "\033[1;33m💡 Note: Your local commit was created successfully.\033[0m"
                return 1
            end
        else
            if sh -c "$push_cmd"
                echo -e "\033[1;32m✅ Everything committed and pushed successfully!\033[0m"
            else
                echo -e "\033[0;31m❌ Push failed!\033[0m"
                echo -e "\033[1;33m💡 Note: Your local commit was created successfully.\033[0m"
                return 1
            end
        end
        return 0
    end

    if command -v gum >/dev/null 2>&1
        # 2. Select Commit Type
        set -l term_rows (stty size 2>/dev/null | awk '{print $1}')
        if test -z "$term_rows"
            set term_rows (tput lines 2>/dev/null)
        end
        if test -z "$term_rows"
            set term_rows 15
        end

        set -l choose_h (math "$term_rows - 2")
        if test $choose_h -gt 10
            set choose_h 10
        end
        if test $choose_h -lt 3
            set choose_h 3
        end

        set -l TYPE (gum choose --height "$choose_h" \
            "✏️  Custom..." \
            "🚧 WIP: Work in progress" \
            "✨ feat: New feature" \
            "🐛 fix: Bug fix" \
            "📝 docs: Documentation" \
            "💄 style: Styling" \
            "♻️ refactor: Refactoring" \
            "🧪 test: Adding tests" \
            "🔧 chore: Maintenance")

        if test -z "$TYPE"
            echo "⚠️ Commit cancelled."
            return 0
        end

        set -l TYPE_PREFIX
        set -l CUSTOM_NAME

        if string match -q "*Custom*" -- "$TYPE"
            set CUSTOM_NAME (gum input --placeholder "Type your custom commit prefix (e.g. 🚧 WIP:: login-ui)...")
            set -l custom_status $status
            if test $custom_status -ne 0 -o -z "$CUSTOM_NAME"
                echo "⚠️ Commit cancelled."
                while read -t 0.05 -n 1 -l _ 2>/dev/null; end
                return 0
            end
            set TYPE_PREFIX "$CUSTOM_NAME"
        else
            set TYPE_PREFIX (echo "$TYPE" | awk '{print $1 " " $2}' | string replace -r ':$' '')
        end

        # 3. Input Commit Message
        set -l MSG (gum input --placeholder "Enter commit message (Leave empty for default)...")
        set -l msg_status $status

        # Flush leftover stdin response bytes before committing
        while read -t 0.05 -n 1 -l _ 2>/dev/null; end

        if test $msg_status -ne 0
            echo "⚠️ Commit cancelled."
            return 0
        end

        set -l FULL_MSG
        if test -z "$MSG"
            set FULL_MSG "$TYPE_PREFIX: Save point ("(date +'%Y-%m-%d %H:%M')")"
        else
            set FULL_MSG "$TYPE_PREFIX: $MSG"
        end

        # 4. Commit
        git commit -m "$FULL_MSG" || return 1

        # 5. Push with Gum Spinner
        set -l cur_branch (git branch --show-current 2>/dev/null)
        set -l push_cmd
        if test -n "$cur_branch"
            set push_cmd "git push -u origin \"$cur_branch\""
        else
            set push_cmd "git push -u"
        end

        set -l push_log (mktemp 2>/dev/null; or echo "/tmp/gwip_push.log")

        if gum spin --spinner dot --title "Pushing to remote..." -- sh -c "$push_cmd >\"$push_log\" 2>&1"
            printf '\033[1;32m✅ Everything committed and pushed successfully!\033[0m\n'
            rm -f "$push_log" 2>/dev/null
        else
            if grep -qE "(non-fast-forward|fetch first|behind)" "$push_log" 2>/dev/null
                echo -e "\033[1;33m🔄 Remote has new commits. Auto-syncing (git pull --rebase)...\033[0m"
                rm -f "$push_log" 2>/dev/null
                set -l sync_branch "$cur_branch"
                if test -z "$sync_branch"
                    set sync_branch "HEAD"
                end
                if git pull --rebase origin "$sync_branch"
                    echo -e "\033[1;36m🚀 Retrying push...\033[0m"
                    if git push -u origin "$sync_branch"
                        printf '\033[1;32m✅ Synced and pushed successfully!\033[0m\n'
                        return 0
                    end
                else
                    echo -e "\033[0;31m⚠️ Merge Conflict detected!\033[0m"
                    echo -e "\033[1;33mPlease resolve conflicts in VS Code, then run:\033[0m"
                    echo -e "  1) \033[1;36mgit add .\033[0m"
                    echo -e "  2) \033[1;36mgit rebase --continue\033[0m"
                    echo -e "  3) \033[1;36mgwip\033[0m"
                    return 1
                end
            end

            echo -e "\033[0;31m❌ Push failed!\033[0m"
            if test -s "$push_log"
                echo -e "\033[1;33mGit Error Details:\033[0m"
                cat "$push_log"
            end
            rm -f "$push_log" 2>/dev/null
            echo -e "\033[1;33m💡 Note: Your local commit was created successfully.\033[0m"
            while read -t 0.05 -n 1 -l _ 2>/dev/null; end
            return 1
        end

        # Final stdin flush to prevent escape sequence leakage into shell prompt
        while read -t 0.05 -n 1 -l _ 2>/dev/null; end
    else
        # Fallback if gum is not installed
        echo -e "\033[1;36m🚀 Git Quick Push Mode\033[0m"
        read -P "📝 Enter commit message [Enter for default]: " msg
        set -l final_msg "$msg"
        if test -z "$final_msg"
            set final_msg "Work in progress (Save Point)"
        end
        git commit -m "🚧 WIP: $final_msg" || return 1

        set -l cur_branch (git branch --show-current 2>/dev/null)
        if test -n "$cur_branch"
            git push -u origin "$cur_branch"
        else
            git push -u
        end
    end
end

alias gcommit=gwip

# ======================================================
#  📦 universal remove
# ======================================================

functions -e uu 2>/dev/null
function uu
    set -l RED '\033[1;31m' GRN='\033[1;32m' YLW='\033[1;33m' CYN='\033[1;36m' BOLD='\033[1m' NC='\033[0m'

    # --- OS & Package Manager Detection ---
    set -l PKG_MGR ""
    if command -v apt-get >/dev/null 2>&1
        PKG_MGR="apt"
    else if command -v pacman >/dev/null 2>&1
        PKG_MGR="pacman"
    else if command -v dnf >/dev/null 2>&1
        PKG_MGR="dnf"
    else
        echo -e "${RED}Unsupported package manager! Cannot proceed.${NC}"
        return 1
    end

    # --- Install fzf dynamically if missing ---
    if ! command -v fzf >/dev/null 2>&1
        echo -e "${YLW}fzf is missing. Installing...${NC}"
        switch $PKG_MGR
            case apt) sudo apt update && sudo apt install -y fzf ;;
            case pacman) sudo pacman -Sy --noconfirm fzf ;;
            case dnf) sudo dnf install -y fzf ;;
        end
    end

    sudo -v || {
        echo -e "${RED}Sudo authentication failed.${NC}"
        return 1
    end

    sync
    set -l START_KB (df -k / | awk 'NR==2 {print $argv[4]}')
    set -l APPS_RAW ""
    set -l idx 1

    echo -e "${CYN}🔍 Harvesting System Assets...${NC}"

    functions -e shred_animation 2>/dev/null
function shred_animation
    set -l PID $argv[1]
    set -l pkg $argv[2]
    set -l sp '/-\|'
    set -l i 0
    set -l exit_status 0
        tput civis 2>/dev/null || true

        while kill -0 "$PID" 2>/dev/null; do
    set -l filled ((i % 21))
    set -l empty ((20 - filled))
    set -l bar ""
    set -l j
            for j in (seq 1 $filled); set bar "$bar"█""; end
    set -l e_bar ""
            for j in (seq 1 $empty); set e_bar "$e_bar"▒""; end
            printf "\r${CYN}⚡ Processing ${BOLD}%s${NC}: ${RED}[${GRN}%s${RED}%s${RED}]${NC} %s${NC}" "$pkg" "$bar" "$e_bar" "${sp:i%4:1}"
            ((i++))
            sleep 0.1
        end

        # CRITICAL FIX: Wait for process and capture exit code
        wait "$PID" 2>/dev/null
        exit_status=$?

    set -l cols (tput cols 2>/dev/null || echo 80)
        printf "\r%-${cols}s\r" " "
        tput cnorm 2>/dev/null || true

        return $exit_status
    end

    functions -e format_name 2>/dev/null
function format_name
        echo "$argv[1]" | sed -E 's/(google-chrome-stable|google-chrome)/chrome/g; s/(brave-browser)/brave/g; s/code/vscode/g; s/(-stable|-bin|-desktop)//g; s/\.[a-zA-Z0-9]+$//' | cut -c1-18
    end

    # --- Data Collection ---
    if command -v snap >/dev/null 2>&1
        while read -r pkg ver rev dev notes; do
            test "$pkg" =~ ^(Name|core|snapd|bare|gtk|gnome|kf5|qt) && continue
    set -l name (format_name "$pkg")
    set -l size (du -sh /var/lib/snapd/snaps/"${pkg}"_*.snap 2>/dev/null | tail -1 | awk '{print $argv[1]}')
    set -l inst_date (snap info "$pkg" 2>/dev/null | grep "installed:" | awk '{print $argv[2]}')
            APPS_RAW+="(printf "%-4s | %-18s | %-10s | %-12s | %-8s | %-10s | %s\n" "$idx" "$name" "snap" "$ver" "(if test -n "$size"; echo "$size"; else; echo "N/A"; end)" "(if test -n "$inst_date"; echo "$inst_date"; else; echo "N/A"; end)" "$pkg")"$'\n'
            ((idx++))
        end
    end

    if command -v flatpak >/dev/null 2>&1
        while IFS=$'\t' read -r id name ver; do
    set -l clean_n (format_name "$name")
    set -l fp_path "/var/lib/flatpak/app/$id"
            test ! -d "$fp_path" && fp_path="$HOME/.local/share/flatpak/app/$id"
    set -l size (du -sh "$fp_path" 2>/dev/null | awk '{print $argv[1]}')
    set -l inst_date (stat -c %y "$fp_path" 2>/dev/null | awk '{print $argv[1]}')
            APPS_RAW+="(printf "%-4s | %-18s | %-10s | %-12s | %-8s | %-10s | %s\n" "$idx" "$clean_n" "flatpak" "$ver" "(if test -n "$size"; echo "$size"; else; echo "~MB"; end)" "$inst_date" "$id")"$'\n'
            ((idx++))
        end
    end

    while IFS= read -r -d '' path; do
    set -l name (format_name "(basename "$path")")
    set -l size (du -sh "$path" 2>/dev/null | awk '{print $argv[1]}')
    set -l inst_date (stat -c %y "$path" 2>/dev/null | awk '{print $argv[1]}')
        APPS_RAW+="(printf "%-4s | %-18s | %-10s | %-12s | %-8s | %-10s | %s\n" "$idx" "$name" "appimage" "Local" "(if test -n "$size"; echo "$size"; else; echo "N/A"; end)" "(if test -n "$inst_date"; echo "$inst_date"; else; echo "N/A"; end)" "$path")"$'\n'
        ((idx++))
    end

    switch $PKG_MGR
        case apt
            while IFS=' ' read -r pkg ver; do
                test "$pkg" =~ ^(linux-|grub|systemd|lib|python|gir1) && continue
    set -l name (format_name "$pkg")
    set -l size_kb (dpkg-query -W -f='${Installed-Size}\n' "$pkg" 2>/dev/null)
    set -l size "N/A"
                if test -n "$size_kb" && "$size_kb" =~ ^[0-9]+$
                    if ((size_kb >= 1048576))
                        size=(awk "BEGIN {printf \"%.1fGB\", $size_kb/1048576}")
                    else
                        size=(awk "BEGIN {printf \"%.1fMB\", $size_kb/1024}")
                    end
                end
    set -l inst_date (stat -c %y "/var/lib/dpkg/info/${pkg}.list" 2>/dev/null | awk '{print $argv[1]}' || echo "N/A")
                APPS_RAW+="(printf "%-4s | %-18s | %-10s | %-12s | %-8s | %-10s | %s\n" "$idx" "$name" "apt" "${ver:0:10}" "$size" "$inst_date" "$pkg")"$'\n'
                ((idx++))
            end
        case pacman
            while IFS=' ' read -r pkg ver; do
                test "$pkg" =~ ^(linux|grub|systemd|lib) && continue
    set -l name (format_name "$pkg")
                APPS_RAW+="(printf "%-4s | %-18s | %-10s | %-12s | %-8s | %-10s | %s\n" "$idx" "$name" "pacman" "${ver:0:10}" "N/A" "N/A" "$pkg")"$'\n'
                ((idx++))
            end
        case dnf
            while IFS=' ' read -r pkg ver; do
                test "$pkg" =~ ^(kernel|grub|systemd|lib) && continue
    set -l name (format_name "$pkg")
                APPS_RAW+="(printf "%-4s | %-18s | %-10s | %-12s | %-8s | %-10s | %s\n" "$idx" "$name" "dnf" "${ver:0:10}" "N/A" "N/A" "$pkg")"$'\n'
                ((idx++))
            end
    end

    APPS_RAW="${APPS_RAW%$'\n'}"
    test -z "$APPS_RAW" && {
        echo -e "${YLW}No applications found.${NC}"
        return
    end

    set -l SELECTED
    set SELECTED (echo "$APPS_RAW" | fzf \
        --ansi --multi --layout=reverse --border=rounded \
        --prompt="🎯 Asset Target: " \
        --delimiter=' \| ' --with-nth=1,2,3 \
        --header="(printf "%-5s %-20s %-11s " "IDX" "NAME" "SOURCE")" \
        --preview-window='right,45%,border-rounded,wrap' \
        --preview='
            RED="\033[1;31m"; GRN="\033[1;32m"; YLW="\033[1;33m"; CYN="\033[1;36m"; BOLD="\033[1m"; NC="\033[0m"
            name=(echo {2}); src=(echo {3}); ver=(echo {4}); size=(echo {5}); idate=(echo {6})
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

    test -z "$SELECTED" && {
        echo -e "${YLW}No selection made.${NC}"
        return
    end

    set -l count
    count=(echo "$SELECTED" | wc -l)
    echo -e "\n${YLW}⚠️ You have selected ${BOLD}$count${NC} ${YLW}apps to uninstall:${NC}"
    echo -e "${CYN}┌──────────────────────────────────────────┐${NC}"
    echo "$SELECTED" | awk -F ' \\| ' '{printf "│ • %-38s │\n", $argv[2]}'
    echo -e "${CYN}└──────────────────────────────────────────┘${NC}"

    read -r -p "Are you sure you want to proceed? (y/N): " confirm
    test ! "$confirm" =~ ^[Yy]$ && {
        echo -e "${RED}Aborted.${NC}"
        return
    end
    sudo -v || {
        echo -e "${RED}Sudo authentication failed.${NC}"
        return
    end

    set -l OLD_SET "+m"
    test $- = *m* && OLD_SET="-m"
    set +m

    set -l failed_apps ""

    while IFS= read -r line; do
        test -z "$line" && continue
    set -l pkg_display (echo "$line" | awk -F ' \\| ' '{print $argv[2]}' | xargs)
    set -l src_type (echo "$line" | awk -F ' \\| ' '{print $argv[3]}' | xargs)
    set -l orig_id (echo "$line" | awk -F ' \\| ' '{print $argv[7]}' | xargs)

        test -z "$src_type" || -z "$orig_id" && continue

        # CRITICAL FIX: Run in background and capture PID properly
        (
    set -l exit_code 0
            switch $src_type
                case snap
                    sudo snap remove "$orig_id" >/dev/null 2>&1 || exit_code=1
    set -l snap_name (basename "$orig_id")
                    rm -rf ~/snap/"$snap_name" 2>/dev/null
                case flatpak
                    flatpak uninstall -y --delete-data "$orig_id" >/dev/null 2>&1 || exit_code=1
                    rm -rf ~/.var/app/"$orig_id" 2>/dev/null
                    rm -rf ~/.local/share/flatpak/app/"$orig_id" 2>/dev/null
                case appimage
                    if test -f "$orig_id"
                        rm -f "$orig_id" >/dev/null 2>&1 || exit_code=1
                    else
                        exit_code=1
                    end
    set -l appimage_name (basename "$orig_id" .AppImage 2>/dev/null)
                    test -n "$appimage_name" && {
                        rm -f ~/.local/share/applications/appimagekit_*"${appimage_name}"*.desktop 2>/dev/null
                        rm -f ~/.local/share/applications/"${appimage_name}".desktop 2>/dev/null
                        rm -rf ~/.local/share/icons/hicolor/*/apps/appimagekit_*"${appimage_name}"* 2>/dev/null
                        rm -f ~/.config/AppImageLauncher/entries/"${appimage_name}"* 2>/dev/null
                    end
                case apt
                    sudo apt purge -y "$orig_id" >/dev/null 2>&1 || exit_code=1
                    if test "$orig_id" =~ ^[a-z0-9-]+$ ]] && [[ ! "$orig_id" =~ ^(python|lib|systemd|xorg|gtk|gnome|kde|qt)
                        test -d ~/.config/"$orig_id" && rm -rf ~/.config/"$orig_id" 2>/dev/null
                        test -d ~/.cache/"$orig_id" && rm -rf ~/.cache/"$orig_id" 2>/dev/null
                        test -d ~/.local/share/"$orig_id" && rm -rf ~/.local/share/"$orig_id" 2>/dev/null
    set -l base_name (echo "$orig_id" | sed 's/-desktop//g; s/-stable//g; s/-git//g; s/-bin//g')
                        if test "$base_name" != "$orig_id"
                            test -d ~/.config/"$base_name" && rm -rf ~/.config/"$base_name" 2>/dev/null
                            test -d ~/.cache/"$base_name" && rm -rf ~/.cache/"$base_name" 2>/dev/null
                            test -d ~/.local/share/"$base_name" && rm -rf ~/.local/share/"$base_name" 2>/dev/null
                        end
                        test -d ~/."${base_name}" && rm -rf ~/."${base_name}" 2>/dev/null
                    end
                case pacman
                    sudo pacman -Rns --noconfirm "$orig_id" >/dev/null 2>&1 || exit_code=1
                    if test "$orig_id" =~ ^[a-z0-9-]+$ ]] && [[ ! "$orig_id" =~ ^(python|lib|systemd|xorg|gtk|gnome|kde|qt)
                        test -d ~/.config/"$orig_id" && rm -rf ~/.config/"$orig_id" 2>/dev/null
                        test -d ~/.cache/"$orig_id" && rm -rf ~/.cache/"$orig_id" 2>/dev/null
                        test -d ~/.local/share/"$orig_id" && rm -rf ~/.local/share/"$orig_id" 2>/dev/null
    set -l base_name (echo "$orig_id" | sed 's/-desktop//g; s/-stable//g; s/-git//g; s/-bin//g')
                        if test "$base_name" != "$orig_id"
                            test -d ~/.config/"$base_name" && rm -rf ~/.config/"$base_name" 2>/dev/null
                            test -d ~/.cache/"$base_name" && rm -rf ~/.cache/"$base_name" 2>/dev/null
                            test -d ~/.local/share/"$base_name" && rm -rf ~/.local/share/"$base_name" 2>/dev/null
                        end
                        test -d ~/."${base_name}" && rm -rf ~/."${base_name}" 2>/dev/null
                    end
                case dnf
                    sudo dnf remove -y "$orig_id" >/dev/null 2>&1 || exit_code=1
                    if test "$orig_id" =~ ^[a-z0-9-]+$ ]] && [[ ! "$orig_id" =~ ^(python|lib|systemd|xorg|gtk|gnome|kde|qt)
                        test -d ~/.config/"$orig_id" && rm -rf ~/.config/"$orig_id" 2>/dev/null
                        test -d ~/.cache/"$orig_id" && rm -rf ~/.cache/"$orig_id" 2>/dev/null
                        test -d ~/.local/share/"$orig_id" && rm -rf ~/.local/share/"$orig_id" 2>/dev/null
    set -l base_name (echo "$orig_id" | sed 's/-desktop//g; s/-stable//g; s/-git//g; s/-bin//g')
                        if test "$base_name" != "$orig_id"
                            test -d ~/.config/"$base_name" && rm -rf ~/.config/"$base_name" 2>/dev/null
                            test -d ~/.cache/"$base_name" && rm -rf ~/.cache/"$base_name" 2>/dev/null
                            test -d ~/.local/share/"$base_name" && rm -rf ~/.local/share/"$base_name" 2>/dev/null
                        end
                        test -d ~/."${base_name}" && rm -rf ~/."${base_name}" 2>/dev/null
                    end
            end
            exit $exit_code
        ) &
    set -l PID $!

        # CRITICAL FIX: Properly check exit status
        if shred_animation "$PID" "$pkg_display"
            echo -e "${GRN}✔ $pkg_display has been shredded.${NC}"
        else
            echo -e "${RED}✘ $pkg_display failed to uninstall.${NC}"
            failed_apps+="$pkg_display ($src_type), "
        end
    end

    set "$OLD_SET"

    test -n "$failed_apps" && echo -e "\n${RED}Failed: ${failed_apps%, }${NC}"

    # --- Turbo Clean ---
    echo -e "\n${CYN}➜ Initializing Turbo Clean Protocol...${NC}\n"

    if command -v snap >/dev/null 2>&1
        echo -ne "${YLW}➜ Purging old Snap revisions...${NC} "
        LANG=en_US.UTF-8 snap list --all 2>/dev/null | awk '/disabled/{print $argv[1], $argv[3]}' | while read -r snapname revision; do
            test -n "$snapname" && -n "$revision" && sudo snap remove "$snapname" --revision="$revision" >/dev/null 2>&1
        end
        echo -e "${GRN}OK${NC}"
    end

    echo -ne "${YLW}➜ Cleaning AppImage artifacts...${NC} "
    find ~/.local/share/applications -name "*appimage*" -type f 2>/dev/null | while IFS= read -r file; do
    set -l exec_path
        exec_path=(grep "^Exec=" "$file" 2>/dev/null | head -1 | cut -d'=' -f2 | cut -d' ' -f1)
        if test -n "$exec_path" && ! -f "$exec_path"
            rm -f "$file"
        end
    end
    echo -e "${GRN}OK${NC}"

    if command -v flatpak >/dev/null 2>&1
        echo -ne "${YLW}➜ Removing unused Flatpak data...${NC} "
        flatpak uninstall --unused -y >/dev/null 2>&1
        echo -e "${GRN}OK${NC}"
    end

    echo -ne "${YLW}➜ Purging unused system configs & cache ($PKG_MGR)...${NC} "
    switch $PKG_MGR
        case apt
            sudo apt autoremove -y >/dev/null 2>&1
            sudo apt autoclean -y >/dev/null 2>&1
            sudo apt clean >/dev/null 2>&1
        case pacman
    set -l orphans
            orphans=(pacman -Qtdq 2>/dev/null)
            test -n "$orphans" && sudo pacman -Rns --noconfirm $orphans >/dev/null 2>&1
            sudo pacman -Sc --noconfirm >/dev/null 2>&1
        case dnf
            sudo dnf autoremove -y >/dev/null 2>&1
            sudo dnf clean all >/dev/null 2>&1
    end
    echo -e "${GRN}OK${NC}"

    sync
    sleep 1
    set -l END_KB (df -k / | awk 'NR==2 {print $argv[4]}')
    set -l SAVED_MB (((START_KB - END_KB) / 1024))

    echo -e "\n${GRN}✅ Cleanup Successful!${NC}"
    if ((SAVED_MB > 0))
        echo -e "${CYN}🚀 Total Space Recovered: ${BOLD}${SAVED_MB} MB${NC}\n"
    else if ((SAVED_MB == 0))
        echo -e "${CYN}📊 No significant space change${NC}\n"
    else
        echo -e "${YLW}⚠️  Space calculation shows negative value (disk activity during cleanup)${NC}\n"
    end
end

# ======================================================
#  📦 Universal Update pack
# ======================================================

functions -e uup 2>/dev/null
function uup
    # --- UI Colors & Styles ---
    set -l RED '\033[1;31m' GRN='\033[1;32m' YLW='\033[1;33m' BLU='\033[1;34m'
    set -l PUR '\033[1;35m' CYN='\033[1;36m' BOLD='\033[1m' NC='\033[0m'

    # --- OS & Package Manager Detection ---
    set -l OS_TYPE (uname -s)
    set -l PKG_MGR ""
    if test "$OS_TYPE" = "Linux"
        if command -v apt >/dev/null 2>&1
            PKG_MGR="apt"
        else if command -v pacman >/dev/null 2>&1
            PKG_MGR="pacman"
        else if command -v dnf >/dev/null 2>&1
            PKG_MGR="dnf"
        end
    else if test "$OS_TYPE" = "Darwin" PKG_MGR="brew"; end

    # --- Dependency Check (fzf) ---
    if ! command -v fzf >/dev/null 2>&1
        echo -e "${YLW}🔍 fzf not found. Installing...${NC}"
        if [ "$OS_TYPE" = "Darwin" ] || command -v brew >/dev/null 2>&1
            brew install fzf
        else if test "$PKG_MGR" = "apt"
            sudo apt update && sudo apt install fzf -y
        else if test "$PKG_MGR" = "pacman"
            sudo pacman -S fzf --noconfirm
        else if test "$PKG_MGR" = "dnf"
            sudo dnf install fzf -y
        end
    end

    clear
    echo ""
    echo -e "  ${BOLD}Manager:${NC} $PKG_MGR | ${BOLD}User:${NC} (whoami) | ${BOLD}OS:${NC} $OS_TYPE"
    echo ""
    # --- Step 0: Smart Selection via FZF ---
    set -l tasks (
        "0. ALL_MAINTENANCE_TASKS"
        "1. Core_System_Update"
        "2. Snap_Package_Refresh"
        "3. Flatpak_Cleanup_Update"
        "4. Bun_Runtime_Upgrade"
        "5. Node.js_LTS_Sync"
        "6. Global_NPM_Update"
        "7. Full_System_Deep_Clean"
    )

    # Fixed FZF Color Typo (#9ece6a)
    set -l SELECTED_TASKS (printf "%s\n" "$tasks" | fzf \
        --ansi --multi --height=18 --layout=reverse --border=rounded \
        --prompt="⚡ Action: " --header="[TAB] Select | [ENTER] Execute" \
        --color='bg+:#292e42,hl:#bb9af7,prompt:#7dcfff,pointer:#f7768e,marker:#9ece6a' \
        --preview 'if test {1} == "0." echo "Execute all updates and cleanup."; else echo "Action: {1}" | sed "s/_/ /g"; end' \
        --preview-window='up:1:wrap')

    if test -z "$SELECTED_TASKS"
        echo -e "${RED}❌ No tasks selected. Aborting...${NC}"
        return
    end

    # --- Sudo Keep-alive ---
    echo -e "${YLW}🔑 Requesting sudo permission...${NC}"
    sudo -v || return
    (while true; do
        sudo -n true
        sleep 60
    end) 2>/dev/null &
    set -l SUDO_PID $!
    trap "kill $SUDO_PID 2>/dev/null" RETURN INT TERM

    # --- Execute All Logic ---
    if test "$SELECTED_TASKS" == *"0. ALL_MAINTENANCE_TASKS"*
        SELECTED_TASKS=(printf "%s\n" "$tasks")
    end

    # 1. OS Core
    if test "$SELECTED_TASKS" == *"1. Core_System_Update"*
        echo -e "\n${BOLD}${YLW}🔍 [1/7] Updating OS Core ($PKG_MGR)...${NC}"
        echo ""
        switch $PKG_MGR
            case apt) sudo apt update && sudo apt upgrade -y ;;
            case pacman) sudo pacman -Syu --noconfirm ;;
            case dnf) sudo dnf upgrade --refresh -y ;;
            case brew) brew update && brew upgrade ;;
        end
    end

    # 2. Snap
    if test "$SELECTED_TASKS" == *"2. Snap_Package_Refresh"*
        echo -e "\n${BOLD}${GRN}📦 [2/7] Checking Snap Environment...${NC}"
        echo ""
        if ! command -v snap >/dev/null 2>&1
            echo -e "  ${YLW}⚠ Snap is not installed on this system. Skipping...${NC}"
        else
    set -l sc (snap refresh --list 2>/dev/null)
            test -n "$sc" && "$sc" != *"up to date"* && sudo snap refresh || echo -e "  ${BLU}ℹ Snaps are up-to-date.${NC}"
        end
    end

    # 3. Flatpak
    if test "$SELECTED_TASKS" == *"3. Flatpak_Cleanup_Update"*
        echo -e "\n${BOLD}${CYN}💎 [3/7] Checking Flatpak Environment...${NC}"
        echo ""
        if ! command -v flatpak >/dev/null 2>&1
            echo -e "  ${YLW}⚠ Flatpak is not installed on this system. Skipping...${NC}"
        else
    set -l f_updates (flatpak remote-ls --updates 2>/dev/null)
            if test -z "$f_updates"
                echo -e "  ${BLU}No Flatpak updates available. Skipping...${NC}"
            else
                flatpak update -y
                echo -e "  ${GRN}✅ Flatpak updated!${NC}"
            end
            flatpak uninstall --unused -y >/dev/null 2>&1
        end
    end

    # 4. Bun
    if test "$SELECTED_TASKS" == *"4. Bun_Runtime_Upgrade"*
        echo -e "\n${BOLD}${CYN}🥬 [4/7] Upgrading Bun Runtime...${NC}"
        echo ""
        if command -v bun >/dev/null 2>&1
            bun upgrade
        else
            echo -e "  ${YLW}⚠ Bun is not installed. Skipping...${NC}"
        end
    end

    # 5. Node.js
    if test "$SELECTED_TASKS" == *"5. Node.js_LTS_Sync"*
        echo -e "\n${BOLD}${PUR}🟢 [5/7] Syncing Node.js (LTS Version)...${NC}"
        echo ""
    set -l NVM_PATH "(if test -n "$NVM_DIR"; echo "$NVM_DIR"; else; echo "$HOME/.nvm"; end)/nvm.sh"
        if test -f "$NVM_PATH"
            source "$NVM_PATH"
            nvm install --lts --reinstall-packages-from=node
            nvm use --lts
            nvm alias default 'lts/*'
        else
            echo -e "  ${YLW}⚠ NVM/Node not found. Skipping...${NC}"
        end
    end

    # 6. Global NPM
    if test "$SELECTED_TASKS" == *"6. Global_NPM_Update"*
        echo -e "\n${BOLD}${YLW}✨ [6/7] Finalizing NPM Update...${NC}"
        echo ""
        if command -v npm >/dev/null 2>&1
            npm install -g npm@latest
        else
            echo -e "  ${YLW}⚠ NPM is not installed. Skipping...${NC}"
        end
    end

    # --- 8. Full Deep Clean (Now including Snap & Flatpak) ---
    if test "$SELECTED_TASKS" == *"7. Full_System_Deep_Clean"*
        echo -e "\n${BOLD}${RED}📦 [7/7] Full System Deep Cleaning...${NC}"
        echo ""
        # OS Native Clean
        switch $PKG_MGR
            case apt) sudo apt autoremove -y && sudo apt autoclean ;;
            case pacman) sudo pacman -Rns (pacman -Qtdq) --noconfirm 2>/dev/null || echo -e "  ${BLU}ℹ No orphans.${NC}" ;;
            case dnf) sudo dnf autoremove -y ;;
            case brew) brew cleanup ;;
        end

        # Snap Clean
        if command -v snap >/dev/null 2>&1
            echo -e "  ${CYN}📦 Cleaning old Snap revisions...${NC}"
            LANG=C snap list --all | awk '/disabled/{print $argv[1], $argv[3]}' | while read sn rv; do sudo snap remove "$sn" --revision="$rv"; done
        end

        # Flatpak Deep Clean (NEW)
        if command -v flatpak >/dev/null 2>&1
            echo ""
            echo -e "${CYN}💎 Cleaning Flatpak unused runtimes & cache...${NC}"
            flatpak uninstall --unused -y >/dev/null 2>&1
            flatpak repair --user >/dev/null 2>&1
            flatpak repair >/dev/null 2>&1
            # Cleaning flatpak cache
            rm -rf ~/.var/app/*/cache/* >/dev/null 2>&1
            echo -e "  ${GRN}✅ Flatpak cleaned.${NC}"
        end
    end

    echo -e "\n${PUR}─────────────────────────────────────────────────────────────${NC}"
    echo -e "  ${BOLD}${GRN}✅ MISSION ACCOMPLISHED! YOUR PC IS NOW AT MAX POWER.${NC}"
    echo -e "${PUR}─────────────────────────────────────────────────────────────${NC}"

    # Notification (Fixed Multi-OS)
    if test "$OS_TYPE" = "Darwin"
        osascript -e 'display notification "System optimized successfully" with title "uup Tool"' 2>/dev/null
    else if command -v notify-send >/dev/null 2>&1
        notify-send "uup Tool" "All selected updates completed successfully."
    end
end

# ======================================================
#  🆘 HELP MENU — Modern UI/UX Edition
# ======================================================
functions -e keep 2>/dev/null
function keep
    # Modern Color Palette
    set -l RESET '\033[0m'
    set -l BOLD '\033[1m'
    set -l DIM '\033[2m'

    # Primary Colors
    set -l CYAN '\033[38;5;51m'    # Electric Cyan
    set -l PINK '\033[38;5;213m'   # Hot Pink
    set -l PURPLE '\033[38;5;141m' # Soft Purple
    set -l GREEN '\033[38;5;82m'   # Neon Green
    set -l YELLOW '\033[38;5;220m' # Gold Yellow
    set -l ORANGE '\033[38;5;208m' # Orange
    set -l BLUE '\033[38;5;75m'    # Sky Blue
    set -l RED '\033[38;5;203m'    # Soft Red
    set -l WHITE '\033[38;5;255m'  # Pure White
    set -l GRAY '\033[38;5;245m'   # Gray

    # Background Colors
    set -l BG_DARK '\033[48;5;234m' # Dark background
    set -l BG_CARD '\033[48;5;236m' # Card background

    # Icons
    set -l ICON_ROCKET '🚀'
    set -l ICON_FOLDER '📂'
    set -l ICON_FILE '📄'
    set -l ICON_GEAR '⚙️'
    set -l ICON_PACKAGE '📦'
    set -l ICON_BUN '🥐'
    set -l ICON_GIT '🌿'
    set -l ICON_LIGHTNING '⚡'
    set -l ICON_TERMINAL '💻'
    set -l ICON_WARNING '⚠️'
    set -l ICON_STAR '✨'
    set -l ICON_SEARCH '🔍'
    set -l ICON_PRISMA '💎'

    # Clear screen for clean look
    clear

    # Header with gradient effect

    echo -e "${CYAN} ╔══════════════════════════════════════════════════════════════════════════╗${RESET}"
    echo -e "${CYAN} ║${RESET}  ${BOLD}${PINK}${ICON_ROCKET}  MASTER COMMAND CENTER ${RESET}${CYAN}│${RESET} ${GRAY}Developer Rihad's Ultimate Fish Environment${RESET}      ${CYAN} ${RESET}"
    echo -e "${CYAN} ╚══════════════════════════════════════════════════════════════════════════╝${RESET}"
    echo -e "${GRAY}  v2.0 • Modern Terminal UX • "(date +'%B %d, %Y')"${RESET}\n"

    # Function to print category headers
    functions -e print_category 2>/dev/null
    function print_category
        set -l icon $argv[1]
        set -l title $argv[2]
        set -l color $argv[3]
        echo -e "\n  ${color}┌─────────────────────────────────────────────────────────────────────┐${RESET}"
        echo -e "  ${color}│${RESET} ${BOLD}${icon}  ${title}${RESET}${color}                                    ${RESET}"
        echo -e "  ${color}└─────────────────────────────────────────────────────────────────────┘${RESET}"
    end

    # Function to print command row
    functions -e print_cmd 2>/dev/null
    function print_cmd
        set -l cmd $argv[1]
        set -l desc $argv[2]
        set -l example $argv[3]
        set -l cmd_color $argv[4]

        if test -z "$example"
            printf "     ${BOLD}${cmd_color}%-12s${RESET} ${GRAY}│${RESET} %s\n" "$cmd" "$desc"
        else
            printf "     ${BOLD}${cmd_color}%-12s${RESET} ${GRAY}│${RESET} %-35s ${DIM}%s${RESET}\n" "$cmd" "$desc" "$example"
        end
    end

    # Function to print alias row
    functions -e print_alias 2>/dev/null
    function print_alias
        set -l alias $argv[1]
        set -l equals $argv[2]
        set -l full $argv[3]
        set -l color $argv[4]
        printf "     ${BOLD}${color}%-6s${RESET} ${GRAY}%s${RESET} ${DIM}%s${RESET}\n" "$alias" "$equals" "$full"
    end

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
    echo -e "\n  ${DIM}(date +'%H:%M:%S') • Bash v${BASH_VERSION:0:3} • (whoami)@(hostname) • $PWD${RESET}\n"
end

# ======================================================
#  📦 Run ts / js file on terminal
# ======================================================

functions -e run 2>/dev/null
function run
    # Color Codes
    CYAN='\033[0;36m'
    YELLOW='\033[1;33m'
    BLUE='\033[1;34m'
    GREEN='\033[1;32m'
    RED='\033[0;31m'
    BOLD='\033[1m'
    NC='\033[0m'

    # File list (.js and .ts)
    files=((ls *.js *.ts 2>/dev/null))

    if test (count $files) -eq 0
        echo -e "${RED}󱓇 No .js or .ts files found!${NC}"
        return 1
    end

    # Modern Header
    echo -e "\n${CYAN}╭──────────────────────────────────────────╮${NC}"
    echo -e "${CYAN}│${NC}  ${BOLD}⚡ BUN INTERACTIVE RUNNER${NC}               ${CYAN}│${NC}"
    echo -e "${CYAN}╰──────────────────────────────────────────╯${NC}"

    # List Display with Icons
    for i in "(seq (count $files))"; do
        ext="${files[$i]##*.}"

        # Icon selection based on extension
        if test "$ext" == "ts"
            icon="${BLUE}📘${NC}" # Blue Book for TS
        else
            icon="${YELLOW}📒${NC}" # Yellow Book for JS
        end

        # Beautifully aligned row
        printf "${CYAN}  [%2d]${NC}  %b  %-30s\n" "((i + 1))" "$icon" "${files[$i]}"
    end

    echo -e "${CYAN}────────────────────────────────────────────${NC}"

    # Smart Input Prompt
    echo -e "${YELLOW}👉 Enter file number (or Ctrl+C):${NC}"
    read -P "❯ " choice

    # File selection validation
    if test $choice -gt 0 && $choice -le (count $files)
        selected_file=${files[((choice - 1))]}

        echo -e "\n${GREEN}✔ Selected:${NC} ${BOLD}$selected_file${NC}"
        echo -e "${CYAN}────────────────────────────────────────────${NC}"

        # Mode Selection Menu
        echo -e "\n${YELLOW}👉 Choose run mode:${NC}"
        echo -e "${CYAN}  [1]${NC}  🚀  ${BOLD}bun run${NC}     (default)"
        echo -e "${CYAN}  [2]${NC}  🔥  ${BOLD}bun --hot${NC}   (hot reload)"
        echo -e "${CYAN}  [3]${NC}  👁  ${BOLD}bun --watch${NC} (watch mode)"
        echo -e "${CYAN}────────────────────────────────────────────${NC}"
        read -P "❯ " mode

        # Determine command based on mode
        switch $mode
            case 2
                bun_action="--hot"
                mode_label="HOT RELOAD"
                mode_color="${RED}"
            case 3
                bun_action="--watch"
                mode_label="WATCH MODE"
                mode_color="${YELLOW}"
            case *
                bun_action="run"
                mode_label="RUN"
                mode_color="${GREEN}"
        end

        echo -e "\n${mode_color}⚙ $mode_label:${NC} ${BOLD}$selected_file${NC}\n"

        # Execute
        bun $bun_action "$selected_file"
    else
        echo -e "\n${RED}✘ Error: Invalid selection!${NC}"
    end
end

# ======================================================
#  📦 VIDEO FILLTER AND OPEN
# ======================================================

functions -e v 2>/dev/null
function v
    set -l TARGET "(if test -n "$argv[1]"; echo "$argv[1]"; else; echo "$PWD"; end)"

    # 🎥 Smart Player Detection (fastest first)
    set -l PLAYER_CMD
    if command -v flatpak >/dev/null 2>&1 && flatpak list 2>/dev/null | grep -q 'org.videolan.VLC'
        PLAYER_CMD="flatpak run org.videolan.VLC"
    else if command -v vlc >/dev/null 2>&1
        PLAYER_CMD="vlc"
    else if command -v mpv >/dev/null 2>&1
        PLAYER_CMD="mpv --fs --no-terminal"
    else if command -v totem >/dev/null 2>&1
        PLAYER_CMD="totem"
    else if command -v celluloid >/dev/null 2>&1
        PLAYER_CMD="celluloid"
    else if command -v xdg-open >/dev/null 2>&1
        PLAYER_CMD="xdg-open"
    else
        echo -e "\e[1;31m❌ কোনো ভিডিও প্লেয়ার পাওয়া যায়নি। VLC বা mpv ইন্সটল করুন।\e[0m"
        return 1
    end

    # Direct file play support if target is a single video file
    if test -f "$TARGET"
        echo -e "\e[1;35m🎬 Playing video:\e[0m (basename "$TARGET")"
        # Use read array to handle multi-word player commands (e.g., "flatpak run org.videolan.VLC")
        local -a player_args
        read -ra player_args <<< "$PLAYER_CMD"
        "$player_args" "$TARGET" >/dev/null 2>&1 &
        disown $! 2>/dev/null || true
        return 0
    end

    # 🔍 Find Videos inside directory
    set -l DIR "$TARGET"
    set -l RAW_LIST
    set RAW_LIST (find "$DIR" -type f \( -iname "*.mp4" -o -iname "*.mkv" -o -iname "*.avi" -o -iname "*.mov" -o -iname "*.webm" -o -iname "*.flv" -o -iname "*.m4v" \) 2>/dev/null | sort)

    [ -z "$RAW_LIST" ] && echo "❌ No videos found" && return 1

    # 🎨 UI Header setup (Idx, Folder, Name)
    set -l HEADER_STR (printf "\e[1;34m%-5s \e[1;33m%-20s \e[1;35m%-s\e[0m" "IDX" "FOLDER" "VIDEO NAME")

    set -l SELECTED_LINE
    set SELECTED_LINE (echo "$RAW_LIST" | awk -F/ '{
        idx = NR;
        folder = (NF-1);
        filename = $NF;

        # 📂 ফোল্ডার আইকন যোগ করা
        folder_with_icon = "📁 " folder;

        # ✂️ Filename truncation (Limit: 55)
        if (length(filename) > 55) filename = substr(filename, 1, 52) "...";
        # ✂️ Folder truncation (Limit: 17 for icon + name)
        if (length(folder_with_icon) > 17) folder_with_icon = substr(folder_with_icon, 1, 14) "...";

        printf "\033[34m%-5s \033[33m%-20s \033[0m%s\n", idx, folder_with_icon, filename
    }' | fzf \
        --ansi \
        --reverse \
        --height=60% \
        --border=rounded \
        --header="$HEADER_STR" \
        --header-first \
        --prompt="🔍 Search: " \
        --pointer="▶" \
        --color="bg+:-1,fg+:white,hl:yellow,hl+:cyan,header:blue,prompt:cyan,pointer:green")

    # ▶ Play Logic
    if test -n "$SELECTED_LINE"
        # প্রথম কলাম থেকে ইনডেক্স বের করা
    set -l INDEX (echo "$SELECTED_LINE" | sed 's/\x1b\[[0-9;]*m//g' | awk '{print $argv[1]}')

        if ! test "$INDEX" =~ ^[0-9]+$
            echo "❌ Invalid selection"
            return 1
        end

    set -l FULL_PATH (echo "$RAW_LIST" | sed -n "${INDEX}p")

        if test -z "$FULL_PATH" ] || [ ! -f "$FULL_PATH"
            echo "❌ Video file not found"
            return 1
        end

        echo -e "\e[1;92m▶ Playing:\e[0m (basename "$FULL_PATH")"
        local -a player_args
        read -ra player_args <<< "$PLAYER_CMD"
        "$player_args" "$FULL_PATH" >/dev/null 2>&1 &
        disown $! 2>/dev/null || true
    else
        echo "👋 Exit"
    end
end

# ======================================================
#  📦universal clean
# ======================================================

functions -e uc 2>/dev/null
function uc
    # ==============================
    # 🎨 COLORS & SAFETY
    # ==============================
    set -l RED "\033[0;31m" GREEN="\033[0;32m" YELLOW="\033[1;33m"
    set -l BLUE "\033[0;34m" CYAN="\033[0;36m" MAGENTA="\033[0;35m" NC="\033[0m"

    set -o pipefail
    # NOTE: errexit and nounset are intentionally disabled inside uc() — they
    # cause early exits and terminal crashes when running interactively.
    # set -o errexit  # DISABLED: fires EXIT trap mid-function, killing the terminal
    # set -o nounset  # DISABLED: any unset var in subshell kills the session

    # ==============================
    # 🔐 TRAP & LOG SETUP
    # ==============================
    set -l LOG_FILE "/tmp/uc-optimizer-(id -u).log"

    # Try to use HOME first, fallback to /tmp
    if test -d "${HOME:-}"
    set -l cache_dir "${HOME}/.cache/uc-optimizer"
        if mkdir -p "$cache_dir" 2>/dev/null
            LOG_FILE="${cache_dir}/optimizer.log"
        end
    end

    # Security: Remove symlink if exists
    if test -L "$LOG_FILE"
        rm -f "$LOG_FILE" 2>/dev/null || LOG_FILE="/dev/null"
    end

    # Create log file
    if ! touch "$LOG_FILE" 2>/dev/null
        LOG_FILE="/dev/null"
    end

    functions -e _log 2>/dev/null
function _log
        echo "(date '+%Y-%m-%d %H:%M:%S') - $argv[1]" >>"$LOG_FILE" 2>/dev/null || true
    end

    functions -e _trap_exit 2>/dev/null
function _trap_exit
    set -l exit_code $?
        trap - INT TERM EXIT
        echo -e "\n${RED}⚠️  Interrupted (Exit code: $exit_code)${NC}" >&2
        _log "Session interrupted with code $exit_code"
        # Use 'return' not 'exit' — 'exit' kills the entire interactive terminal session
        return 130
    end
    trap _trap_exit INT TERM EXIT

    _log "Session started [UID: (id -u), PID: $$]"

    # ==============================
    # 🐧 DISTRO DETECTION
    # ==============================
    functions -e _detect_distro 2>/dev/null
function _detect_distro
    set -l d_id "unknown" d_name="Unknown Linux"

        if test -r /etc/os-release
            # Source safely with restricted scope
            while IFS='=' read -r key value; do
                switch $key
                    case ID) d_id="${value//\"/}" ;;
                    case NAME) d_name="${value//\"/}" ;;
                end
            done </etc/os-release
            d_id=(echo "$d_id" | tr '[:upper:]' '[:lower:]')
        else if test -r /etc/redhat-release
            d_id="rhel"
            d_name="Red Hat Enterprise Linux"
        else if test -r /etc/arch-release
            d_id="arch"
            d_name="Arch Linux"
        else if test -r /etc/alpine-release
            d_id="alpine"
            d_name="Alpine Linux"
        end

        # Validate ID format
        if test ! "$d_id" =~ ^[a-z0-9._-]+$
            d_id="unknown"
        end

        printf '%s|%s' "$d_id" "$d_name"
    end

    set -l distro_info
    distro_info=(_detect_distro)
    set -l DISTRO_ID "${distro_info%%|*}"
    set -l DISTRO_NAME "${distro_info##*|}"

    # Fallback if parsing failed
    test -z "$DISTRO_ID" && DISTRO_ID="unknown"
    test -z "$DISTRO_NAME" && DISTRO_NAME="Unknown"

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
    functions -e _sudo_check 2>/dev/null
function _sudo_check
        if test $EUID -eq 0
            return 0 # Already root
        end
        if ! sudo -n true 2>/dev/null
            echo -e "${YELLOW}🔐 Sudo authentication required...${NC}"
            if ! sudo -v
                echo -e "${RED}❌ Sudo authentication failed${NC}" >&2
                return 1
            end
        end
        return 0
    end

    functions -e _pkg_install 2>/dev/null
function _pkg_install
    set -l pkgs ("$@")
    set -l pkg_manager ""

        _sudo_check || return 1

        # Detect package manager
        switch $DISTRO_ID
            ubuntu | debian | linuxmint | pop | elementary | zorin)
                pkg_manager="apt"
            fedora | rhel | centos | rocky | almalinux | nobara)
                if command -v dnf >/dev/null 2>&1
                    pkg_manager="dnf"
                else
                    pkg_manager="yum"
                end
            arch | manjaro | endeavouros | garuda | cachyos)
                pkg_manager="pacman"
            opensuse* | suse* | tumbleweed | leap)
                pkg_manager="zypper"
            case alpine
                pkg_manager="apk"
            case void
                pkg_manager="xbps"
            case gentoo
                pkg_manager="emerge"
            case nixos
                pkg_manager="nix"
            case *
                echo -e "${RED}❌ Unsupported distro: $DISTRO_ID${NC}" >&2
                return 1
        end

        echo -e "${BLUE}📦 Installing: ${pkgs[*]}${NC}"

    set -l exit_code 0
        switch $pkg_manager
            case apt
                sudo apt-get update -qq &&
                    sudo DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends "$pkgs"
                exit_code=$?
            case dnf
                sudo dnf install -y --setopt=install_weak_deps=False "$pkgs"
                exit_code=$?
            case yum
                sudo yum install -y "$pkgs"
                exit_code=$?
            case pacman
                sudo pacman -Sy --noconfirm --needed "$pkgs"
                exit_code=$?
            case zypper
                sudo zypper --non-interactive install --no-recommends "$pkgs"
                exit_code=$?
            case apk
                sudo apk add --no-cache "$pkgs"
                exit_code=$?
            case xbps
                sudo xbps-install -y "$pkgs"
                exit_code=$?
            case emerge
                sudo emerge -av "$pkgs"
                exit_code=$?
            case nix
                sudo nix-env -iA $pkgs
                exit_code=$?
        end

        return $exit_code
    end

    # ==============================
    # 🔄 PATH & ENV REFRESH
    # ==============================
    functions -e _refresh_env 2>/dev/null
function _refresh_env
        # Reload PATH
    set -l paths (
            "/usr/local/sbin"
            "/usr/local/bin"
            "/usr/sbin"
            "/usr/bin"
            "/sbin"
            "/bin"
            "${HOME}/.local/bin"
            "${HOME}/bin"
            "${HOME}/.fzf/bin"
        )

    set -l new_path ""
        for p in "$paths"; do
            test -d "$p" && new_path="${new_path:+$new_path:}$p"
        end

        # Preserve existing PATH entries not in our list
    set -l IFS ':'
        for p in $PATH; do
            if test ":$new_path:" != *":$p:"* ]] && [[ -d "$p"
                new_path="$new_path:$p"
            end
        end
        unset IFS

set -gx PATH "$new_path"

        # Refresh hash table
        hash -r 2>/dev/null || true

        # Source bashrc if exists (for new completions)
        test -f "$HOME/.bashrc" && source "$HOME/.bashrc" 2>/dev/null || true
    end

    # ==============================
    # 🛠️ TOOL INSTALLERS
    # ==============================
    functions -e _install_fzf 2>/dev/null
function _install_fzf
        echo -e "${YELLOW}⚠️  fzf not found. Installing...${NC}"

        # Try package manager first
    set -l pkg_name "fzf"

        # Some distros have different names
        switch $DISTRO_ID
            case alpine) pkg_name="fzf" ;;
        end

        if _pkg_install "$pkg_name"
            _configure_fzf
            return 0
        end

        # Fallback: Git installation
        echo -e "${YELLOW}📥 Package manager failed. Trying git install...${NC}"

        if ! command -v git >/dev/null 2>&1
            _pkg_install "git" || {
                echo -e "${RED}❌ git not available${NC}" >&2
                return 1
            end
        end

    set -l fzf_dir "${HOME}/.fzf"
        rm -rf "$fzf_dir" 2>/dev/null || true

        if git clone --depth 1 https://github.com/junegunn/fzf.git "$fzf_dir" 2>/dev/null
            "$fzf_dir/install" --all --no-bash --no-fish --no-zsh 2>/dev/null || true
            _configure_fzf
            return 0
        end

        echo -e "${RED}❌ fzf installation failed${NC}" >&2
        return 1
    end

    functions -e _configure_fzf 2>/dev/null
function _configure_fzf
        echo -e "${BLUE}🔧 Configuring fzf...${NC}"

        _refresh_env

        # Verify fzf is available
        if ! command -v fzf >/dev/null 2>&1
            # Manual PATH addition
            test -f "${HOME}/.fzf/bin/fzf" ; and set -gx PATH "${HOME}/.fzf/bin:$PATH"
            test -f "/usr/bin/fzf" ; and set -gx PATH "/usr/bin:$PATH"
        end

        # Setup shell integration
    set -l bashrc "${HOME}/.bashrc"
    set -l fzf_shell ""

        # Find fzf shell files
        for d in "/usr/share/doc/fzf/examples" "/usr/share/fzf" "/usr/share/fzf/shell" "${HOME}/.fzf/shell"; do
            test -f "$d/completion.bash" && fzf_shell="$d" && break
        end

        if test -n "$fzf_shell" ]] && [[ -f "$bashrc"
            # Add to bashrc if not present
            if ! grep -q "fzf" "$bashrc" 2>/dev/null
                {
                    echo ""
                    echo "# fzf configuration added by uc-optimizer"
                    echo "source $fzf_shell/completion.bash 2>/dev/null || true"
                    echo "source $fzf_shell/key-bindings.bash 2>/dev/null || true"
                } >>"$bashrc"
            end
        end

        # Load immediately
        test -n "$fzf_shell" && source "$fzf_shell/completion.bash" 2>/dev/null || true
        test -n "$fzf_shell" && source "$fzf_shell/key-bindings.bash" 2>/dev/null || true

        # Final verification
        if command -v fzf >/dev/null 2>&1
            echo -e "${GREEN}✅ fzf (fzf --version | head -1) installed${NC}"
            return 0
        else
            echo -e "${RED}❌ fzf configuration incomplete${NC}" >&2
            return 1
        end
    end

    functions -e _install_sensors 2>/dev/null
function _install_sensors
        echo -e "${YELLOW}⚠️  sensors not found. Installing...${NC}"

    set -l pkg_name "lm-sensors"
        test "$DISTRO_ID" = "arch" || "$DISTRO_ID" = "manjaro" && pkg_name="lm_sensors"
        test "$DISTRO_ID" = "alpine" && pkg_name="lm-sensors"

        if ! _pkg_install "$pkg_name"
            echo -e "${RED}❌ lm-sensors installation failed${NC}" >&2
            return 1
        end

        echo -e "${BLUE}🔧 Configuring sensors...${NC}"

        if ! command -v sensors-detect >/dev/null 2>&1
            echo -e "${YELLOW}⚠️  sensors-detect not found${NC}"
            return 0 # Partial success
        end

        # Non-interactive configuration
        echo -e "${CYAN}🌡️  Detecting hardware sensors (this may take a moment)...${NC}"

        # Create answers file for sensors-detect
    set -l answers ""
        for _ in {1..10}; do
            answers="${answers}YES\n"
        end

        echo -e "$answers" | sudo sensors-detect --no-interactive 2>/dev/null ||
            echo -e "$answers" | sudo sensors-detect 2>/dev/null || true

        # Load common modules
    set -l modules (coretemp nct6775 k10temp acpi_cpufreq it87)
        for mod in "$modules"; do
            sudo modprobe "$mod" 2>/dev/null || true
        end

        # Enable service
        if command -v systemctl >/dev/null 2>&1
            sudo systemctl enable --now lm-sensors 2>/dev/null ||
                sudo systemctl enable --now sensord 2>/dev/null || true
        end

        # Generate sensors.conf if missing
        if test ! -f /etc/sensors3.conf ]] && [[ ! -f /etc/sensors.conf
            sudo sensors -s 2>/dev/null || true
        end

        # Test
        if sensors >/dev/null 2>&1
            echo -e "${GREEN}✅ sensors configured successfully${NC}"
        else
            echo -e "${YELLOW}⚠️  sensors configured but no sensors detected${NC}"
        end

        return 0
    end

    functions -e _install_zram 2>/dev/null
function _install_zram
        echo -e "${YELLOW}⚠️  zram tools not found. Installing...${NC}"

    set -l pkg_name "util-linux"
    set -l extra_pkgs ()

        switch $DISTRO_ID
            ubuntu | debian | linuxmint | pop)
                pkg_name="zram-tools"
            fedora | rhel | centos | rocky | almalinux | nobara)
                pkg_name="zram-generator"
            arch | manjaro | endeavouros)
                pkg_name="zram-generator"
                extra_pkgs=("util-linux")
            case alpine
                pkg_name="zram-init"
            case opensuse*
                pkg_name="systemd-zram-service"
        end

        if ! _pkg_install "$pkg_name" "$extra_pkgs"
            echo -e "${RED}❌ zram package installation failed${NC}" >&2
            return 1
        end

        echo -e "${BLUE}🔧 Configuring zram...${NC}"

        # Check if zram already configured
        if test -e /dev/zram0 && swapon -s 2>/dev/null | grep -q zram
            echo -e "${GREEN}✅ zram already active${NC}"
            return 0
        end

        # Load module
        if ! lsmod 2>/dev/null | grep -q "^zram"
            sudo modprobe zram num_devices=1 2>/dev/null || {
                echo -e "${RED}❌ Cannot load zram module${NC}" >&2
                return 1
            end
        end

        # Calculate size (50% of RAM)
        set -l mem_total zram_size
        mem_total=(awk '/MemTotal/{print $argv[2]}' /proc/meminfo 2>/dev/null || echo "0")
        set zram_size ((mem_total * 512)) # KB to bytes / 2

        test "$zram_size" -lt 104857600 && zram_size=536870912 # Minimum 512MB

        # Configure
        echo "$zram_size" | sudo tee /sys/block/zram0/disksize >/dev/null 2>/dev/null || {
            echo -e "${RED}❌ Cannot set zram size${NC}" >&2
            return 1
        end

        sudo mkswap /dev/zram0 >/dev/null 2>&1 || true
        sudo swapon /dev/zram0 -p 100 >/dev/null 2>&1 || {
            echo -e "${RED}❌ Cannot enable zram swap${NC}" >&2
            return 1
        end

        # Persistent config for systemd-based systems
        if test "$DISTRO_ID" == "arch" || "$DISTRO_ID" == "fedora" ]] && [[ -d /etc/systemd
    set -l zram_conf "/etc/systemd/zram-generator.conf"
            if test ! -f "$zram_conf"
                sudo tee "$zram_conf" >/dev/null <<'EOF'
[zram0]
zram-size = ram / 2
compression-algorithm = zstd
swap-priority = 100
EOF
            end
        end

        echo -e "${GREEN}✅ zram configured: ((zram_size / 1024 / 1024))MB${NC}"
        return 0
    end

    # ==============================
    # 🔍 DEPENDENCY CHECK
    # ==============================
    echo -e "${BLUE}🔍 Checking dependencies...${NC}"

    local -a missing_tools=()
    set -l install_failed 0

    command -v fzf >/dev/null 2>&1 || missing_tools+=("fzf")
    command -v sensors >/dev/null 2>&1 || missing_tools+=("sensors")
    command -v zramctl >/dev/null 2>&1 || missing_tools+=("zram")

    if test (count $missing_tools) -gt 0
        echo -e "${YELLOW}📋 Missing: ${missing_tools[*]}${NC}"
        echo -e "${CYAN}🚀 Installing...${NC}"
        echo ""

        for tool in "$missing_tools"; do
            switch $tool
                case fzf
                    if ! _install_fzf
                        set install_failed ((install_failed + 1))
                        echo -e "${RED}CRITICAL: fzf is required${NC}" >&2
                    end
                case sensors
                    _install_sensors || install_failed=((install_failed + 1))
                case zram
                    _install_zram || install_failed=((install_failed + 1))
            end
            echo ""
        end

        _refresh_env
    end

    # Final verification
    set -l critical_fail 0
    if ! command -v fzf >/dev/null 2>&1
        echo -e "${RED}❌ CRITICAL: fzf not available${NC}" >&2
        critical_fail=1
    end

    if ! command -v sensors >/dev/null 2>&1
        echo -e "${YELLOW}⚠️  sensors not available (temp monitoring disabled)${NC}"
    end

    if ! command -v zramctl >/dev/null 2>&1
        echo -e "${YELLOW}⚠️  zramctl not available (zram monitoring disabled)${NC}"
    end

    if test $critical_fail -eq 1
        echo -e "${RED}❌ Cannot continue without fzf${NC}" >&2
        _log "FAILED: Missing critical dependency fzf"
        return 1
    end

    echo -e "${GREEN}✅ Dependencies ready!${NC}"
    _log "Dependencies satisfied"
    sleep 1
    clear

    # ==============================
    # 🔧 CORE FUNCTIONS
    # ==============================
    functions -e _pkg_clean 2>/dev/null
function _pkg_clean
        _sudo_check || return 1
        switch $DISTRO_ID
            ubuntu | debian | linuxmint | pop | elementary)
                sudo apt-get autoremove --purge -y && sudo apt-get autoclean
            fedora | rhel | centos | rocky | almalinux | nobara)
                if command -v dnf >/dev/null 2>&1
                    sudo dnf autoremove -y && sudo dnf clean all
                else
                    sudo yum autoremove -y && sudo yum clean all
                end
            arch | manjaro | endeavouros | garuda)
                sudo pacman -Sc --noconfirm
                command -v paccache >/dev/null 2>&1 && sudo paccache -r
            opensuse* | suse*)
                sudo zypper clean
            case alpine
                sudo apk cache clean
            case void
                sudo xbps-remove -yo 2>/dev/null || true
        end
    end

    functions -e _get_temp_zram 2>/dev/null
function _get_temp_zram
    set -l temp "N/A" zram_used="0"

        if command -v sensors >/dev/null 2>&1
            set temp (sensors 2>/dev/null | awk '
                /°C/ {
                    gsub(/[+|°C]/, "", $argv[2])
                    if ($argv[2]+0 > max && $argv[2] ~ /^[0-9]+\.?[0-9]*$/) max=$argv[2]
                end
                END {
                    if (max > 0) printf "%.0f", max
                    else print "N/A"
                end
            ')
        end

        if command -v zramctl >/dev/null 2>&1
            set zram_used (zramctl --output=DISKSIZE,DATA --bytes 2>/dev/null | awk '
                NR>1 {
                    if ($argv[1] ~ /^[0-9]+$/ && $argv[1] > 0) {
                        disk += $argv[1]
                        data += $argv[2]
                    end
                end
                END {
                    if (disk > 0) printf "%.0f", (data/disk)*100
                    else print "0"
                end
            ')
        end

        printf '%s %s' "(if test -n "$temp"; echo "$temp"; else; echo "N/A"; end)" "(if test -n "$zram_used"; echo "$zram_used"; else; echo "0"; end)"
    end

    functions -e _get_free_kb 2>/dev/null
function _get_free_kb
    set -l avail
        avail=(df -k / 2>/dev/null | awk 'NR==2 {print $argv[4]}')
        test "$avail" =~ ^[0-9]+$ && echo "$avail" || echo "0"
    end

    functions -e _format_size 2>/dev/null
function _format_size
    set -l kb $argv[1]
        test "$kb" =~ ^[0-9]+$ || {
            echo "0KB"
            return
        end

    set -l mb ((kb / 1024))
    set -l gb ((mb / 1024))

        if test $kb -lt 1024
            echo "${kb}KB"
        else if test $mb -lt 1024
            echo "${mb}MB"
        else
            echo "${gb}GB"
        end
    end

    functions -e _run_task 2>/dev/null
function _run_task
    set -l label $argv[1]
        shift
        echo -ne "   ${GREEN}➤ $label...${NC} "
        if "$@" >/dev/null 2>&1
            echo -e "${GREEN}✅${NC}"
            return 0
        else
            echo -e "${RED}❌${NC}"
            return 1
        end
    end

    # ==============================
    # 🧹 CLEANING FUNCTIONS
    # ==============================
    functions -e _os_clean 2>/dev/null
function _os_clean
        echo -e "${BLUE}╔════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}        ⚡ OS CLEAN             ${BLUE}║${NC}"
        echo -e "${BLUE}╚════════════════════════════════╝${NC}"

        read -r -p "Proceed with OS cleanup? [y/N]: " confirm
        test "$confirm" =~ ^[Yy]$ || return 0

        _sudo_check || return 1

    set -l space_before
        space_before=(_get_free_kb)

        echo -e "${CYAN}🗑️  Cleaning package cache...${NC}"
        _pkg_clean

        echo -e "${CYAN}📋 Vacuuming journals...${NC}"
        if command -v journalctl >/dev/null 2>&1
            sudo journalctl --vacuum-time=3d --quiet 2>/dev/null || true
        end

        echo -e "${CYAN}🖼️  Cleaning thumbnails...${NC}"
        if test -d "$HOME/.cache/thumbnails"
            find "$HOME/.cache/thumbnails" -type f -atime +7 -delete 2>/dev/null || true
        end

        echo -e "${CYAN}🗑️  Emptying trash...${NC}"
        if command -v gio >/dev/null 2>&1
            gio trash --empty >/dev/null 2>&1 || true
        end
        rm -rf "$HOME/.local/share/Trash/files" "$HOME/.local/share/Trash/info" 2>/dev/null || true
        mkdir -p "$HOME/.local/share/Trash/files" "$HOME/.local/share/Trash/info" 2>/dev/null || true

        # Clean old logs
        sudo find /var/log -type f -name "*.old" -delete 2>/dev/null || true
        sudo find /var/log -type f -name "*.gz" -mtime +30 -delete 2>/dev/null || true

        # Developer cache cleaning (optional)
    set -l dev_confirm ""
        echo -n "Clean developer caches? (npm/bun/pip) [y/N]: "
        read -r dev_confirm
        if test "$dev_confirm" =~ ^[Yy]$
            echo -e "${CYAN}💻 Cleaning developer caches...${NC}"
            command -v npm >/dev/null 2>&1 && npm cache clean --force 2>/dev/null || true
            command -v bun >/dev/null 2>&1 && bun pm cache rm 2>/dev/null || true
            command -v pip >/dev/null 2>&1 && pip cache purge 2>/dev/null || true
            command -v pip3 >/dev/null 2>&1 && pip3 cache purge 2>/dev/null || true
            command -v pnpm >/dev/null 2>&1 && pnpm store prune 2>/dev/null || true
            test -d "$HOME/.cache/pip" && rm -rf "$HOME/.cache/pip"/* 2>/dev/null || true
            test -d "$HOME/.cache/go-build" && rm -rf "$HOME/.cache/go-build"/* 2>/dev/null || true
            test -d "$HOME/.cargo/registry/cache" && rm -rf "$HOME/.cargo/registry/cache"/* 2>/dev/null || true
            echo -e "   ${GREEN}✅ Developer caches cleared${NC}"
        end

        set -l space_after freed_kb freed_str
        space_after=(_get_free_kb)
        set freed_kb ((space_after - space_before))
        freed_str=""
        test $freed_kb -gt 0 && freed_str=" (freed: (_format_size $freed_kb))"
        echo -e "${GREEN}✅ OS cleanup completed${freed_str}${NC}"
        _log "OS clean executed"
    end

    functions -e _container_clean 2>/dev/null
function _container_clean
        echo -e "${BLUE}╔════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}      🐳 CONTAINER CLEAN        ${BLUE}║${NC}"
        echo -e "${BLUE}╚════════════════════════════════╝${NC}"

        read -r -p "Proceed with container cleanup? [y/N]: " confirm
        test "$confirm" =~ ^[Yy]$ || return 0

    set -l total_saved 0

        # Snap cleanup
        if command -v snap >/dev/null 2>&1
            echo -e "${CYAN}📦 Cleaning snap packages...${NC}"
            set -l start_space end_space saved
            start_space=(_get_free_kb)

    set -l snap_output
            snap_output=(snap list --all 2>/dev/null | awk '/disabled/{print $argv[1], $argv[3]}')

            if test -n "$snap_output"
                while read -r name rev; do
                    test "$name" =~ ^[a-z0-9-]+$ || continue
                    test "$rev" =~ ^[0-9]+$ || continue
                    echo -e "   ${YELLOW}Removing: $name (rev $rev)${NC}"
                    sudo snap remove --revision="$rev" "$name" 2>/dev/null || true
                end
            end

            sudo rm -rf /var/lib/snapd/cache/* 2>/dev/null || true

            end_space=(_get_free_kb)
            set saved (((end_space - start_space) * 1024))
            test $saved -gt 0 && total_saved=((total_saved + saved))
            echo -e "   ${GREEN}Snap saved: (_format_size ((saved / 1024)))${NC}"
        end

        # Flatpak cleanup
        if command -v flatpak >/dev/null 2>&1
            echo -e "${CYAN}📦 Cleaning flatpak...${NC}"
    set -l start_space (_get_free_kb)

            flatpak uninstall --unused -y 2>/dev/null || true
            flatpak repair 2>/dev/null || true

            # Clean app caches (DISABLED: Causes browsers to lose cookies/sessions)
            # if [[ -d "$HOME/.var/app" ]]; then
            #     for app in "$HOME/.var/app/"*; do
            #         [[ -d "$app/cache" ]] && rm -rf "$app/cache/"* 2>/dev/null || true
            #     done
            # fi

    set -l end_space (_get_free_kb)
    set -l saved (((end_space - start_space) * 1024))
            test $saved -gt 0 && total_saved=((total_saved + saved))
            echo -e "   ${GREEN}Flatpak saved: (_format_size ((saved / 1024)))${NC}"
        end

        # Docker cleanup
        if command -v docker >/dev/null 2>&1
            echo -e "${CYAN}🐳 Cleaning docker...${NC}"
            if sudo docker info >/dev/null 2>&1
    set -l start_space (_get_free_kb)

                docker system prune -a --volumes -f 2>/dev/null || true

    set -l end_space (_get_free_kb)
    set -l saved (((end_space - start_space) * 1024))
                test $saved -gt 0 && total_saved=((total_saved + saved))
                echo -e "   ${GREEN}Docker saved: (_format_size ((saved / 1024)))${NC}"
            else
                echo -e "   ${YELLOW}Docker daemon not running${NC}"
            end
        end

        # Podman cleanup
        if command -v podman >/dev/null 2>&1
            echo -e "${CYAN}🦭 Cleaning podman...${NC}"
            podman system prune -f 2>/dev/null || true
            echo -e "   ${GREEN}Podman cleaned${NC}"
        end

        echo -e "${GREEN}✅ Total saved: (_format_size ((total_saved / 1024)))${NC}"
        _log "Container clean executed"
    end

    functions -e _fix_links 2>/dev/null
function _fix_links
        echo -e "${BLUE}╔════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}        🔗 FIX LINKS            ${BLUE}║${NC}"
        echo -e "${BLUE}╚════════════════════════════════╝${NC}"

        read -r -p "Remove broken symlinks in $HOME? [y/N]: " confirm
        test "$confirm" =~ ^[Yy]$ || return 0

    set -l count 0
        while IFS= read -r -d '' link; do
            rm -f "$link" 2>/dev/null && ((count++)) || true
        end

        echo -e "${GREEN}✅ Removed $count broken symlinks${NC}"
        _log "Fixed $count broken links"
    end

    functions -e _orphan_engine 2>/dev/null
function _orphan_engine
        echo -e "${BLUE}╔════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}       ⚡ KERNEL CLEAN           ${BLUE}║${NC}"
        echo -e "${BLUE}╚════════════════════════════════╝${NC}"

    set -l current_kernel
        current_kernel=(uname -r)
        echo -e "Current kernel: ${GREEN}$current_kernel${NC}"

        _sudo_check || return 1

        switch $DISTRO_ID
            ubuntu | debian | linuxmint | pop | elementary)
    set -l kernels ()
                while IFS= read -r line; do
                    kernels+=("$line")
                end

                if test (count $kernels) -le 2
                    echo -e "${GREEN}Only (count $kernels) kernels installed, skipping${NC}"
                    return 0
                end

    set -l keep1 "${kernels[-1]}"
    set -l keep2 "${kernels[-2]}"
                echo -e "Keeping: ${GREEN}$keep1${NC} and ${GREEN}$keep2${NC}"

    set -l to_remove ()
                for k in "$kernels"; do
                    if test "$k" != "$keep1" && "$k" != "$keep2" && "$k" != *"$current_kernel"*
                        to_remove+=("$k")
                    end
                end

                if test (count $to_remove) -gt 0
                    echo -e "${YELLOW}Will remove: ${to_remove[*]}${NC}"
                    read -r -p "Confirm? [y/N]: " confirm2
                    if test "$confirm2" =~ ^[Yy]$
                        sudo apt-get purge -y "$to_remove" 2>/dev/null || true
                        sudo apt-get autoremove -y 2>/dev/null || true
                    end
                end

                # Remove residual configs
    set -l residual
                residual=(dpkg-query -W -f='${Package}\n' 2>/dev/null | grep '^rc' || true)
                if test -n "$residual"
                    echo "$residual" | xargs -r sudo dpkg --purge 2>/dev/null || true
                end

            fedora | rhel | centos | rocky | almalinux | nobara)
                if command -v dnf >/dev/null 2>&1
                    echo -e "${CYAN}Removing old kernels...${NC}"
                    sudo dnf remove --oldinstallonly --setopt installonly_limit=2 -y 2>/dev/null || {
                        # Fallback manual method
    set -l old_kernels
                        old_kernels=(dnf repoquery --installonly --latest-limit=-2 -q 2>/dev/null | grep -v "$current_kernel" || true)
                        if test -n "$old_kernels"
                            echo "$old_kernels" | xargs -r sudo dnf remove -y 2>/dev/null || true
                        end
                    end
                end

            arch | manjaro | endeavouros | garuda | cachyos)
                echo -e "${CYAN}Removing orphan packages...${NC}"
    set -l orphans
                orphans=(pacman -Qtdq 2>/dev/null || true)
                if test -n "$orphans"
                    echo "$orphans" | sudo pacman -Rns --noconfirm - 2>/dev/null || true
                end

                if command -v paccache >/dev/null 2>&1
                    echo -e "${CYAN}Cleaning package cache...${NC}"
                    sudo paccache -rk2 2>/dev/null || true
                end

            case opensuse*
                echo -e "${CYAN}Cleaning kernels...${NC}"
                sudo zypper purge-kernels 2>/dev/null || true

            case *
                echo -e "${YELLOW}⚠️  Kernel cleanup not implemented for $DISTRO_ID${NC}"
        end

        echo -e "${GREEN}✅ Kernel cleanup completed${NC}"
        _log "Kernel clean executed"
    end

    functions -e _ai_mode 2>/dev/null
function _ai_mode
        echo -e "${BLUE}╔════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}       🤖 AI DIAGNOSTICS        ${BLUE}║${NC}"
        echo -e "${BLUE}╚════════════════════════════════╝${NC}"

        # Get system stats
    set -l mem_info
        set mem_info (free | awk '/Mem:/{printf "%.0f %.0f %.0f", $argv[2], $argv[3], ($argv[3]/$argv[2])*100}')
    set -l mem_total "${mem_info%% *}"
    set -l mem_used "${mem_info#* }"
        mem_used="${mem_used%% *}"
    set -l mem_pct "${mem_info##* }"

    set -l disk_info
        disk_info=(df -k / | awk 'NR==2{print $argv[3], $argv[4], $argv[5]}')
    set -l disk_used "${disk_info%% *}"
    set -l disk_avail "${disk_info#* }"
        disk_avail="${disk_avail%% *}"
    set -l disk_pct "${disk_info##* }"
        disk_pct="${disk_pct//%/}"

        set -l temp zram_used
        read -r temp zram_used < <(_get_temp_zram)

        # Display
        echo -e "📊 ${CYAN}System Status:${NC}"
        echo -e "   Memory: ${mem_pct}% used (((mem_used / 1024))MB / ((mem_total / 1024))MB)"
        echo -e "   Disk:   ${disk_pct}% used ((_format_size ((disk_used / 1024))) / (_format_size (((disk_used + disk_avail) / 1024))))"
        echo -e "   Temp:   ${temp}°C"
        echo -e "   ZRAM:   ${zram_used}% used"

        # AI Recommendations
    set -l actions ()

        if test "$mem_pct" -gt 85
            echo -e "\n${YELLOW}⚠️  HIGH MEMORY USAGE${NC}"
            read -r -p "   Drop caches? [y/N]: " confirm
            if test "$confirm" =~ ^[Yy]$
                _sudo_check && {
                    sudo sync
                    echo 3 | sudo tee /proc/sys/vm/drop_caches >/dev/null 2>&1 || true
                    echo -e "   ${GREEN}✅ Caches dropped${NC}"
                end
            end
        end

        if test "$disk_pct" -gt 90
            echo -e "\n${RED}🚨 CRITICAL DISK USAGE${NC}"
            _os_clean
        else if test "$disk_pct" -gt 80
            echo -e "\n${YELLOW}⚠️  High disk usage${NC}"
            read -r -p "   Run OS cleanup? [y/N]: " confirm
            test "$confirm" =~ ^[Yy]$ && _os_clean
        end

        if test "$temp" != "N/A" && "$temp" -gt 80
            echo -e "\n${RED}🌡️  HIGH TEMPERATURE${NC}"
            echo -e "   ${YELLOW}Check cooling system!${NC}"
        end

        _log "AI mode executed"
    end

    functions -e _report 2>/dev/null
function _report
        echo -e "${BLUE}╔════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}       📊 SYSTEM REPORT         ${BLUE}║${NC}"
        echo -e "${BLUE}╚════════════════════════════════╝${NC}"

        echo -e "${CYAN}OS Information:${NC}"
        echo -e "   Distribution: $DISTRO_NAME"
        echo -e "   Kernel:       (uname -r)"
        echo -e "   Architecture: (uname -m)"
        echo -e "   Hostname:     (hostname)"

    set -l uptime_str
        set uptime_str (uptime -p 2>/dev/null || uptime | sed 's/.*up \([^,]*\),.*/\1/')
        echo -e "   Uptime:       $uptime_str"

        echo -e "\n${CYAN}Resources:${NC}"

        # Memory
        free -h | awk '/Mem:/{printf "   Memory:       %s / %s (%.1f%% used)\n", $argv[3], $argv[2], ($argv[3]/$argv[2])*100}'

        # Disk
        df -h / | awk 'NR==2{printf "   Disk (/):     %s / %s (%s used)\n", $argv[3], $argv[2], $argv[5]}'

        # Temperature & ZRAM
        set -l temp zram_used
        read -r temp zram_used < <(_get_temp_zram)
        echo -e "   Temperature:  ${temp}°C"
        echo -e "   ZRAM Usage:   ${zram_used}%"

        # CPU
        echo -e "   CPU:          (nproc) cores"
        if test -r /proc/cpuinfo
    set -l cpu_model
            cpu_model=(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2 | xargs | cut -c1-40)
            echo -e "   Model:        $cpu_model"
        end

        # Load average
        echo -e "   Load:         (cut -d' ' -f1-3 /proc/loadavg 2>/dev/null || echo 'N/A')"

        echo -e "\n${CYAN}Top Processes (by memory):${NC}"
        ps aux --sort=-%mem 2>/dev/null | head -6 | tail -5 | awk '{printf "   %-10s %5s%% %s\n", $argv[1], $argv[4], $11}'

        _log "Report generated"
    end

    # ==============================
    # 🗑️ APPIMAGE ARTIFACT CLEANUP
    # ==============================
    functions -e _appimage_cleanup 2>/dev/null
function _appimage_cleanup
        echo -e "${BLUE}╔════════════════════════════════╗${NC}"
        echo -e "${BLUE}║${NC}    🗑️  APPIMAGE ARTIFACT CLEAN  ${BLUE}║${NC}"
        echo -e "${BLUE}╚════════════════════════════════╝${NC}"
        echo -e "${CYAN}Scanning for orphaned .desktop & icon files...${NC}\n"

    set -l desktop_dir "$HOME/.local/share/applications"
    set -l icon_dir "$HOME/.local/share/icons"
    set -l autostart_dir "$HOME/.config/autostart"
    set -l found_count 0 removed_count=0
        local -a orphans=()

        # Find .desktop files pointing to missing AppImage binaries
        if test -d "$desktop_dir"
            while IFS= read -r -d '' desktop_file; do
                set -l exec_line bin_path
                exec_line=(grep -i '^Exec=' "$desktop_file" 2>/dev/null | head -1 | cut -d= -f2- | awk '{print $argv[1]}')
                test -z "$exec_line" && continue
                if test "$exec_line" == *.AppImage* ]] || [[ "$exec_line" == *.appimage*
                    bin_path=(echo "$exec_line" | sed 's/ .*//')
                    if test ! -f "$bin_path"
                        orphans+=("$desktop_file")
                        ((found_count++)) || true
                        echo -e "   ${YELLOW}Orphan: (basename "$desktop_file")${NC}"
                    end
                end
            end
        end

        # Also check autostart directory
        if test -d "$autostart_dir"
            while IFS= read -r -d '' desktop_file; do
                set -l exec_line bin_path
                exec_line=(grep -i '^Exec=' "$desktop_file" 2>/dev/null | head -1 | cut -d= -f2- | awk '{print $argv[1]}')
                test -z "$exec_line" && continue
                if test "$exec_line" == *.AppImage* ]] || [[ "$exec_line" == *.appimage*
                    bin_path=(echo "$exec_line" | sed 's/ .*//')
                    if test ! -f "$bin_path"
                        orphans+=("$desktop_file")
                        ((found_count++)) || true
                        echo -e "   ${YELLOW}Orphan (autostart): (basename "$desktop_file")${NC}"
                    end
                end
            end
        end

        if test $found_count -eq 0
            echo -e "${GREEN}✅ No orphaned AppImage artifacts found.${NC}"
            _log "AppImage cleanup: nothing to remove"
            return 0
        end

        echo ""
        read -r -p "Remove $found_count orphaned file(s)? [y/N]: " confirm
        test "$confirm" =~ ^[Yy]$ || return 0

        for f in "$orphans"; do
            rm -f "$f" 2>/dev/null && ((removed_count++)) || true
            echo -e "   ${RED}Removed: (basename "$f")${NC}"
        end

        # Remove orphaned appimagekit icon files
        if test -d "$icon_dir"
    set -l icon_count 0
            while IFS= read -r -d '' icon_file; do
                rm -f "$icon_file" 2>/dev/null && ((icon_count++)) || true
            end
            test $icon_count -gt 0 && echo -e "   ${RED}Removed $icon_count orphaned icon(s)${NC}"
        end

        # Refresh the desktop application database
        command -v update-desktop-database >/dev/null 2>&1 &&
            update-desktop-database "$desktop_dir" 2>/dev/null || true

        echo -e "${GREEN}✅ Removed $removed_count orphaned AppImage artifact(s).${NC}"
        _log "AppImage cleanup: removed $removed_count files"
    end

    # ==============================
    # 📋 INTERACTIVE MENU
    # ==============================
    functions -e _show_menu 2>/dev/null
function _show_menu
        local -a choices=(
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

    set -l choice
        set choice (printf "%s\n" "$choices" |
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

        test -z "$choice" && return 1

        # Extract action (robust emoji & padding removal)
    set -l action
        action=(echo "$choice" | sed 's/^[^[:alnum:]]*[[:space:]]*//')

        switch $action
            "Full System Boost")
                _os_clean
                _appimage_cleanup
                _container_clean
                _fix_links
                _orphan_engine
                _ai_mode
                _report
            "AI Smart Cleanup") _ai_mode ;;
            "OS Clean") _os_clean ;;
            "AppImage Artifact Clean") _appimage_cleanup ;;
            "Container Clean") _container_clean ;;
            "Fix Broken Links") _fix_links ;;
            "Kernel Clean") _orphan_engine ;;
            "System Report") _report ;;
            case "Exit") return 0 ;;
            case *
                echo -e "${RED}Unknown option: $action${NC}" >&2
                return 1
        end

        return 0
    end

    # ==============================
    # 🎯 MAIN EXECUTION
    # ==============================
    set -l menu_result 0

    while true; do
        echo ""
        if ! _show_menu
            menu_result=1
            break
        end

        echo ""
        read -r -p "Press Enter to continue..." dummy </dev/tty
        clear
    end

    trap - INT TERM EXIT
    _log "Session ended (result: $menu_result)"
    echo -e "${GREEN}👋 Goodbye!${NC}"

    return $menu_result
end

# ======================================================
#  📦 runtime install
# ======================================================

functions -e rt 2>/dev/null
function rt
    # ১. fzf চেক এবং অটো-ইন্সটলেশন
    if ! command -v fzf >/dev/null 2>&1
        echo "🔍 fzf খুঁজে পাওয়া যায়নি। ইন্সটল করা হচ্ছে..."

        if test "$OSTYPE" == "linux-gnu"*
            # Linux (Debian/Ubuntu) এর জন্য
            sudo apt update && sudo apt install fzf -y
        else if test "$OSTYPE" == "darwin"*
            # macOS এর জন্য (Homebrew প্রয়োজন)
            if command -v brew >/dev/null 2>&1
                brew install fzf
            else
                echo "❌ Error: Homebrew পাওয়া যায়নি। অনুগ্রহ করে fzf ম্যানুয়ালি ইন্সটল করুন।"
                return 1
            end
        else
            echo "❌ দুঃখিত, আপনার অপারেটিং সিস্টেমটি অটো-ইন্সটলেশন সাপোর্ট করছে না।"
            return 1
        end

        echo "✅ fzf ইন্সটলেশন সম্পন্ন হয়েছে!"
    end

    # ২. মেনু অপশন
    options=(
        "NVM (Node Version Manager)"
        "Node.js (LTS Version)"
        "Bun (Fast JS Runtime)"
        "Deno (Secure JS Runtime)"
    )

    set selected (printf "%s\n" "$options" | fzf \
        --header="🚀 Ultimate Tool Installer (q to Exit)" \
        --reverse --height=40% --border --bind 'q:abort')

    # ৩. সিলেকশন চেক
    if test $? -ne 0 ] || [ -z "$selected"
        echo "👋 বিদায়!"
        return 0
    end

    # ৪. টুল ইন্সটলেশন লজিক
    switch $selected
        "NVM (Node Version Manager)")
            if test -d "$HOME/.nvm"
                echo "✅ NVM আগে থেকেই আছে।"
            else
                curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
set -gx NVM_DIR "$HOME/.nvm"
                [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
            end
        "Node.js (LTS Version)")
set -gx NVM_DIR "$HOME/.nvm"
            [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
            if command -v nvm >/dev/null 2>&1
                nvm install --lts && nvm use --lts
            else
                echo "❌ আগে NVM ইন্সটল করুন!"
            end
        "Bun (Fast JS Runtime)")
            command -v bun >/dev/null 2>&1 && echo "✅ Bun আছে: (bun -v)" || (curl -fsSL https://bun.sh/install | bash ; and set -gx PATH "$HOME/.bun/bin:$PATH")
        "Deno (Secure JS Runtime)")
            command -v deno >/dev/null 2>&1 && echo "✅ Deno আছে: (deno -v)" || (curl -fsSL https://deno.land/x/install/install.sh | sh ; and set -gx PATH "$HOME/.deno/bin:$PATH")
    end
end

# =========================================
# Ultimate Smart PC Optimizer (v5.1 - Clean UI)
# =========================================

functions -e ut 2>/dev/null
function ut
    # ===== 🎨 UI PALETTE =====
    set -l RED '\033[1;31m' GREEN='\033[1;32m' YELLOW='\033[1;33m'
    set -l BLUE '\033[1;34m' PURPLE='\033[1;35m' CYAN='\033[1;36m'
    set -l WHITE '\033[1;37m' BOLD='\033[1m' DIM='\033[2m' NC='\033[0m'
    set -l LOGFILE "/tmp/pcop_(whoami)_$$.log"
    : >"$LOGFILE"

    # ===== 🖥️ DISTRO DETECTION =====
    set -l DISTRO_ID "" PKG_MANAGER="" PKG_INSTALL="" PKG_QUERY=""
    set -l SERVICE_CMD "systemctl"

    functions -e detect_distro 2>/dev/null
function detect_distro
        if test -f /etc/os-release
            source /etc/os-release
            DISTRO_ID="${ID,,}"
        else
            echo -e "${RED}❌ Cannot detect distribution${NC}" && return 1
        end

        switch $DISTRO_ID
            ubuntu | deepin | debian | linuxmint | pop | elementary | zorin | kali | parrot)
                PKG_MANAGER="apt"
                PKG_INSTALL="sudo apt install -y"
                PKG_QUERY="dpkg-query -W -f='${Status}'"
            fedora | rhel | centos | rocky | almalinux | nobara)
                PKG_MANAGER="dnf"
                test "$DISTRO_ID" = "centos" && test -z "(command -v dnf)" && PKG_MANAGER="yum"
                PKG_INSTALL="sudo $PKG_MANAGER install -y"
                PKG_QUERY="rpm -q"
            arch | manjaro | endeavouros | garuda | cachyos | artix)
                PKG_MANAGER="pacman"
                PKG_INSTALL="sudo pacman -S --noconfirm --needed"
                PKG_QUERY="pacman -Q"
            opensuse* | suse*)
                PKG_MANAGER="zypper"
                PKG_INSTALL="sudo zypper install -y"
                PKG_QUERY="rpm -q"
            case alpine
                PKG_MANAGER="apk"
                PKG_INSTALL="sudo apk add"
                PKG_QUERY="apk info -e"
                SERVICE_CMD="rc-service"
            case void
                PKG_MANAGER="xbps"
                PKG_INSTALL="sudo xbps-install -y"
                PKG_QUERY="xbps-query"
                SERVICE_CMD="sv"
            case *
                echo -e "${YELLOW}⚠️ Unknown distro. Trying apt...${NC}"
                PKG_MANAGER="apt"
                PKG_INSTALL="sudo apt install -y"
                PKG_QUERY="dpkg-query -W -f='${Status}'"
        end
    end

    detect_distro || return 1
    echo -e "${CYAN} 🖥️  Detected: ${BOLD}${DISTRO_ID}${NC} | Package Manager: ${BOLD}${PKG_MANAGER}${NC}"

    # ===== ⚙️ FZF CHECK =====
    functions -e install_fzf_universal 2>/dev/null
function install_fzf_universal
        echo -e "${YELLOW}📦 Installing fzf...${NC}"
        switch $PKG_MANAGER
            case "apt") sudo apt update -y && sudo apt install -y fzf ;;
            "dnf" | "yum") sudo $PKG_MANAGER install -y fzf ;;
            case "pacman") sudo pacman -S --noconfirm fzf ;;
            case "zypper") sudo zypper install -y fzf ;;
            case "apk") sudo apk add fzf ;;
            case "xbps") sudo xbps-install -y fzf ;;
            case *
    set -l FZF_VERSION (curl -s https://api.github.com/repos/junegunn/fzf/releases/latest | grep -oP '"tag_name": "\K(.*)(?=")' || echo "0.54.0")
                curl -Lo /tmp/fzf.tar.gz "https://github.com/junegunn/fzf/releases/download/${FZF_VERSION}/fzf-${FZF_VERSION}-linux_amd64.tar.gz"
                tar -xzf /tmp/fzf.tar.gz -C /tmp && sudo mv /tmp/fzf /usr/local/bin/
                rm /tmp/fzf.tar.gz
        end
    end

    if ! command -v fzf >/dev/null 2>&1
        install_fzf_universal || return 1
    end

    # ===== 📦 PACKAGE DATABASE =====
    declare -A PKG_MAP=(
        ["zram-tools"]="zram-tools|zram-generator-defaults|zram-generator|systemd-zram-service|zram-tools|zramctl"
        ["earlyoom"]="earlyoom|earlyoom|earlyoom|earlyoom|earlyoom|earlyoom"
        ["htop"]="htop|htop|htop|htop|htop|htop"
        ["btop"]="btop|btop|btop|btop|btop|btop"
        ["ncdu"]="ncdu|ncdu|ncdu|ncdu|ncdu|ncdu"
        ["gdu"]="gdu|gdu-disk-usage-analyzer|gdu|gdu|gdu|gdu"
        ["duf"]="duf|duf|duf|duf|duf|duf"
        ["dust"]="dust|dust|dust|du-dust|dust|dust"
        ["bleachbit"]="bleachbit|bleachbit|bleachbit|bleachbit|bleachbit|bleachbit"
        ["ufw"]="ufw|ufw|ufw|ufw|ufw|ufw"
        ["fail2ban"]="fail2ban|fail2ban|fail2ban|fail2ban|fail2ban|fail2ban"
        ["rkhunter"]="rkhunter|rkhunter|rkhunter|rkhunter|rkhunter|rkhunter"
        ["lynis"]="lynis|lynis|lynis|lynis|lynis|lynis"
        ["clamav"]="clamav|clamav|clamav|clamav|clamav|clamav"
        ["firejail"]="firejail|firejail|firejail|firejail|firejail|firejail"
        ["gnupg"]="gnupg2|gnupg2|gnupg|gpg2|gnupg|gnupg"
        ["speedtest-cli"]="speedtest-cli|speedtest-cli|speedtest-cli|speedtest|speedtest-cli|speedtest-cli"
        ["vnstat"]="vnstat|vnstat|vnstat|vnstat|vnstat|vnstat"
        ["nmap"]="nmap|nmap|nmap|nmap|nmap|nmap"
        ["iftop"]="iftop|iftop|iftop|iftop|iftop|iftop"
        ["nload"]="nload|nload|nload|nload|nload|nload"
        ["nethogs"]="nethogs|nethogs|nethogs|nethogs|nethogs|nethogs"
        ["curl"]="curl|curl|curl|curl|curl|curl"
        ["wget"]="wget|wget|wget|wget|wget|wget"
        ["aria2"]="aria2|aria2|aria2|aria2|aria2|aria2"
        ["wireguard"]="wireguard|wireguard-tools|wireguard-tools|wireguard-tools|wireguard-tools|wireguard"
        ["dog"]="dog|dog|dog|dog|dog|dog"
        ["mtr-tiny"]="mtr-tiny|mtr|mtr|mtr|mtr|mtr"
        ["tcpdump"]="tcpdump|tcpdump|tcpdump|tcpdump|tcpdump|tcpdump"
        ["git"]="git|git|git|git|git|git"
        ["docker.io"]="docker.io|docker|docker|docker|docker|docker"
        ["docker-compose"]="docker-compose|docker-compose|docker-compose|docker-compose|docker-compose|docker-compose"
        ["build-essential"]="build-essential|gcc-c++|base-devel|patterns-devel-base-devel|build-base|base-devel"
        ["micro"]="micro|micro|micro|micro|micro|micro"
        ["neovim"]="neovim|neovim|neovim|neovim|neovim|neovim"
        ["tmux"]="tmux|tmux|tmux|tmux|tmux|tmux"
        ["screen"]="screen|screen|screen|screen|screen|screen"
        ["python3-pip"]="python3-pip|python3-pip|python-pip|python3-pip|py3-pip|python3-pip"
        ["nodejs"]="nodejs|nodejs|nodejs|nodejs|nodejs|nodejs"
        ["npm"]="npm|npm|npm|npm|npm|npm"
        ["golang-go"]="golang-go|golang|go|go|go|go"
        ["rsync"]="rsync|rsync|rsync|rsync|rsync|rsync"
        ["jq"]="jq|jq|jq|jq|jq|jq"
        ["yq"]="yq|yq|yq|yq|yq|yq"
        ["bat"]="bat|bat|bat|bat|bat|bat"
        ["eza"]="eza|eza|eza|eza|eza|eza"
        ["ripgrep"]="ripgrep|ripgrep|ripgrep|ripgrep|ripgrep|ripgrep"
        ["fd-find"]="fd-find|fd-find|fd|fd|fd|fd"
        ["zoxide"]="zoxide|zoxide|zoxide|zoxide|zoxide|zoxide"
        ["procs"]="procs|procs|procs|procs|procs|procs"
        ["tldr"]="tldr|tldr|tldr|tldr|tldr|tldr"
        ["chafa"]="chafa|chafa|chafa|chafa|chafa|chafa"
        ["fzf"]="fzf|fzf|fzf|fzf|fzf|fzf"
        ["fastfetch"]="fastfetch|fastfetch|fastfetch|fastfetch|fastfetch|fastfetch"
        ["inxi"]="inxi|inxi|inxi|inxi|inxi|inxi"
        ["lm-sensors"]="lm-sensors|lm_sensors|lm_sensors|sensors|lm-sensors|lm-sensors"
        ["unzip"]="unzip|unzip|unzip|unzip|unzip|unzip"
        ["p7zip-full"]="p7zip-full|p7zip|p7zip|p7zip|p7zip|p7zip"
        ["zsh"]="zsh|zsh|zsh|zsh|zsh|zsh"
        ["xclip"]="xclip|xclip|xclip|xclip|xclip|xclip"
        ["wl-clipboard"]="wl-clipboard|wl-clipboard|wl-clipboard|wl-clipboard|wl-clipboard|wl-clipboard"
        ["acpi"]="acpi|acpi|acpi|acpi|acpi|acpi"
        ["sysstat"]="sysstat|sysstat|sysstat|sysstat|sysstat|sysstat"
        ["stress-ng"]="stress-ng|stress-ng|stress-ng|stress-ng|stress-ng|stress-ng"
        ["smem"]="smem|smem|smem|smem|smem|smem"
        ["preload"]="preload|preloader|preload|preloader|preload|preload"
        ["cpufrequtils"]="cpufrequtils|cpufrequtils|cpupower|cpufrequtils|cpufrequtils|cpufrequtils"
        ["gparted"]="gparted|gparted|gparted|gparted|gparted|gparted"
        ["smartmontools"]="smartmontools|smartmontools|smartmontools|smartmontools|smartmontools|smartmontools"
        ["tree"]="tree|tree|tree|tree|tree|tree"
        ["ranger"]="ranger|ranger|ranger|ranger|ranger|ranger"
        ["mc"]="mc|mc|mc|mc|mc|mc"
        ["glances"]="glances|glances|glances|glances|glances|glances"
        ["atop"]="atop|atop|atop|atop|atop|atop"
        ["gh"]="gh|gh|github-cli|gh|github-cli|github-cli"
        ["lazygit"]="lazygit|lazygit|lazygit|lazygit|lazygit|lazygit"
        ["lazydocker"]="lazydocker|lazydocker|lazydocker|lazydocker|lazydocker|lazydocker"
        ["httpie"]="httpie|httpie|httpie|httpie|httpie|httpie"
        ["stow"]="stow|stow|stow|stow|stow|stow"
        ["lsof"]="lsof|lsof|lsof|lsof|lsof|lsof"
        ["dnsutils"]="dnsutils|bind-utils|bind|bind-utils|bind-tools|bind-tools"
        ["shellcheck"]="shellcheck|ShellCheck|shellcheck|ShellCheck|shellcheck|shellcheck"
        ["shfmt"]="shfmt|shfmt|shfmt|shfmt|shfmt|shfmt"
        ["socat"]="socat|socat|socat|socat|socat|socat"
        ["strace"]="strace|strace|strace|strace|strace|strace"
        ["git-delta"]="git-delta|git-delta|git-delta|git-delta|git-delta|git-delta"
        ["iputils-ping"]="iputils-ping|iputils|iputils|iputils|iputils|iputils"
        ["net-tools"]="net-tools|net-tools|net-tools|net-tools|net-tools|net-tools"
    )

    functions -e get_pkg_name 2>/dev/null
function get_pkg_name
    set -l generic "$argv[1]"
    set -l mapping "${PKG_MAP[$generic]}"
        test -z "$mapping" && echo "$generic" && return
    set -l idx 1
        switch $PKG_MANAGER
            case "apt") idx=1 ;;
            "dnf" | "yum") idx=2 ;;
            case "pacman") idx=3 ;;
            case "zypper") idx=4 ;;
            case "apk") idx=5 ;;
            case "xbps") idx=6 ;;
        end
        echo "$mapping" | cut -d'|' -f$idx
    end

    functions -e is_installed 2>/dev/null
function is_installed
    set -l pkg "$argv[1]"
        switch $PKG_MANAGER
            case "apt") dpkg-query -W -f='${Status}' "$pkg" 2>/dev/null | grep -q "ok installed" ;;
            "dnf" | "yum" | "zypper") rpm -q "$pkg" >/dev/null 2>&1 ;;
            case "pacman") pacman -Q "$pkg" >/dev/null 2>&1 ;;
            case "apk") apk info -e "$pkg" >/dev/null 2>&1 ;;
            case "xbps") xbps-query "$pkg" >/dev/null 2>&1 ;;
            case *) return 1 ;;
        end
    end

    # ===== 🎨 RENDER ENGINE WITH INDEX =====
    set -l menu_items ()
    set -l tool_list (
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
        "DEV|rsync|Fast, versatile remote/set -l file-copy"
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

    # INDEX COUNTER ADD KORA HOYECHE
    set -l idx 0
    for item in "$tool_list"; do
        ((idx++))
        IFS='|' read -r cat generic desc <<<"$item"
    set -l pkg (get_pkg_name "$generic")

    set -l status "${DIM}○${NC}"
        is_installed "$pkg" && status="${GREEN}●${NC}"

        switch $cat
            case "PERF") c_cat="${PURPLE}PERF${NC}" ;;
            case "DISK") c_cat="${RED}DISK${NC}" ;;
            case "SECURE") c_cat="${GREEN}SECU${NC}" ;;
            case "NET") c_cat="${CYAN}NET ${NC}" ;;
            case "DEV") c_cat="${BLUE}DEV ${NC}" ;;
            case "MODERN") c_cat="${YELLOW}MOD ${NC}" ;;
            case *) c_cat="${DIM}SYS ${NC}" ;;
        end

        # INDEX NUMBER ADD KORA HOYECHE - FORMAT: [idx]
    set -l line (printf "%b ${DIM}[%3d]${NC}  %-12b  ${BOLD}%-18s${NC}  ${DIM}%s${NC}" "$status" "$idx" "$c_cat" "$generic" "$desc")
        menu_items+=("$line|$generic|$pkg")
    end

    # ===== 🖥️ UI LAUNCHER WITH INDEX =====
    set -l selected_raw (printf "%s\n" "$menu_items" | fzf \
        --ansi --multi --delimiter='\|' --with-nth=1 \
        --height=90% --layout=reverse --border=rounded \
        --prompt="🔍 Search Arsenal > " \
        --header="  [TAB] Select Multiple  |  [ENTER] Process  |  [Q] Exit  | (${PKG_MANAGER})
  ─────────────────────────────────────────────────────────────────────────
  STAT  [IDX]  CATEGORY       PACKAGE          DESCRIPTION")

    test $? -ne 0 || -z "$selected_raw" && {
        echo -e "\n${YELLOW}👋 Operation cancelled.${NC}"
        return 0
    end

    # CRITICAL FIX: Proper extraction (index 2 and 3, skip 1)
    mapfile -t selected_tools <<<"(echo "$selected_raw" | awk -F'|' '{print $argv[2]}')"
    mapfile -t actual_packages <<<"(echo "$selected_raw" | awk -F'|' '{print $argv[3]}')"

    # Trim whitespace
    for i in "(seq (count $selected_tools))"; do
        selected_tools[$i]=(echo "${selected_tools[$i]}" | xargs)
        actual_packages[$i]=(echo "${actual_packages[$i]}" | xargs)
    end

    test (count $selected_tools) -eq 0 && {
        echo -e "${YELLOW}⚠️ Nothing selected${NC}"
        return 0
    end

    # ===== ⬇️ INSTALL & CONFIG ENGINE =====
    sudo -v || {
        echo -e "${RED}❌ Sudo required${NC}"
        return 1
    end

    # Keep sudo alive - SILENT VERSION
    (
        while true; do
            sudo -n true 2>/dev/null || exit
            sleep 60
            kill -0 "$$" 2>/dev/null || exit
        end
    ) >/dev/null 2>&1 &
    set -l SUDO_KEEPALIVE $!

    # Disown to prevent job messages
    disown $SUDO_KEEPALIVE 2>/dev/null || true

    echo -e "${CYAN}🔧 Processing (count $selected_tools) tools on ${DISTRO_ID}...${NC}"

    switch $PKG_MANAGER
        case "apt") sudo apt update -y &>>"$LOGFILE" ;;
        "dnf" | "yum") sudo $PKG_MANAGER check-update -y &>>"$LOGFILE" || true ;;
        case "pacman") sudo pacman -Sy &>>"$LOGFILE" ;;
        case "zypper") sudo zypper refresh &>>"$LOGFILE" ;;
        case "apk") sudo apk update &>>"$LOGFILE" ;;
        case "xbps") sudo xbps-install -Su &>>"$LOGFILE" || true ;;
    end

    set -l RC_FILE "$HOME/.bashrc"
    test "$SHELL" = */zsh && RC_FILE="$HOME/.zshrc"
    set -l SHELL_NAME (basename "$SHELL")

    functions -e add_config 2>/dev/null
function add_config
    set -l marker "$argv[1]"
    set -l content "$argv[2]"
        if ! grep -qF "$marker" "$RC_FILE" 2>/dev/null
            echo -e "\n# $marker" >>"$RC_FILE"
            echo "$content" >>"$RC_FILE"
        end
    end

    set -l failed_pkgs ()
    set -l installed_pkgs ()

    for i in "(seq (count $selected_tools))"; do
    set -l t "${selected_tools[$i]}"
    set -l pkg "${actual_packages[$i]}"

        echo -n -e "${WHITE}📦 $t (${pkg})... ${NC}"

        if is_installed "$pkg"
            echo -e "${GREEN}✔ Already installed${NC}"
            installed_pkgs+=("$t")
        else
            # SAFER: No eval
            if $PKG_INSTALL "$pkg" &>>"$LOGFILE"
                echo -e "${GREEN}[INSTALLED]${NC}"
                installed_pkgs+=("$t")
            else
                echo -e "${RED}[FAILED]${NC}"
                failed_pkgs+=("$t ($pkg)")
                continue
            end
        end

        # Auto-Config (only for successfully installed)
        switch $t
            "docker.io")
                sudo usermod -aG docker "$USER" 2>/dev/null || true
                test "$SERVICE_CMD" = "systemctl" && sudo systemctl enable --now docker &>>"$LOGFILE" || true
            case "tmux"
                test ! -f ~/.tmux.conf && echo -e "set -g mouse on\nset -g default-terminal \"screen-256color\"" >~/.tmux.conf
            case "git"
                git config --global color.ui auto 2>/dev/null || true
                git config --global core.editor "nano" 2>/dev/null || true
                git config --global init.defaultBranch main 2>/dev/null || true
            case "neovim"
                mkdir -p ~/.config/nvim
                test ! -f ~/.config/nvim/init.vim && echo -e "set number\nset relativenumber\nset mouse=a\nset termguicolors" >~/.config/nvim/init.vim
                add_config "Neovim Alias" "alias v='nvim'\nalias vim='nvim'"
            case "zram-tools"
                if test "$PKG_MANAGER" == "apt"
                    sudo bash -c "echo -e 'PERCENT=60\nALGO=zstd\nPRIORITY=100' > /etc/default/zramswap"
                    sudo systemctl restart zramswap &>>"$LOGFILE" || true
                else if test "$PKG_MANAGER" == "pacman"
                    # Install generator first
                    sudo pacman -S --noconfirm zram-generator 2>/dev/null || true
                    sudo bash -c "echo -e '[zram0]\nzram-size = ram / 2\ncompression-algorithm = zstd' > /etc/systemd/zram-generator.conf"
                    sudo systemctl daemon-reload &>>"$LOGFILE" || true
                    sudo systemctl start /dev/zram0 &>>"$LOGFILE" || true
                end
            case "micro"
                mkdir -p ~/.config/micro
                test ! -f ~/.config/micro/settings.json && echo '{"mouse": true, "clipboard": "terminal"}' >~/.config/micro/settings.json
            case "ufw"
                command -v ufw >/dev/null 2>&1 && { sudo ufw allow ssh 2>/dev/null && sudo ufw --force enable &>>"$LOGFILE" || true; }
            case "htop"
                mkdir -p ~/.config/htop
                test ! -f ~/.config/htop/htoprc && echo "highlight_megabytes=1\nshow_program_path=1" >~/.config/htop/htoprc
            case "acpi"
                add_config "Battery Status" "alias battery='acpi -V'"
            case "bat"
    set -l bat_cmd "bat"
                test "$PKG_MANAGER" = "apt" && bat_cmd="batcat"
                add_config "Batcat Alias" "alias cat='$bat_cmd -p'\nalias bat='$bat_cmd'"
            case "eza"
                add_config "Eza Alias" "alias ls='eza --icons --group-directories-first'"
            case "zoxide"
                add_config "Zoxide Init" "[ -x \"\(command -v zoxide)\" ] && eval \"\(zoxide init $SHELL_NAME)\""
                add_config "Zoxide Alias" "alias cd='z'"
            case "preload"
                echo -e "${CYAN}🔧 Enabling Preload service...${NC}"
                test "$SERVICE_CMD" = "systemctl" && {
                    sudo systemctl enable --now preload &>>"$LOGFILE"
                } || {
                    sudo $SERVICE_CMD preload start &>>"$LOGFILE"
                end

            case "earlyoom"
                echo -e "${CYAN}🔧 Configuring EarlyOOM (Memory Protection)...${NC}"
                # Default settings: 10% memory and 5% swap threshold
                if test -f /etc/default/earlyoom
                    sudo sed -i 's/EARLYOOM_ARGS=.*/EARLYOOM_ARGS="-m 10 -s 5 --prefer '"'^(electron|java|python)'"'"/' /etc/default/earlyoom
                end
                test "$SERVICE_CMD" = "systemctl" && {
                    sudo systemctl enable --now earlyoom &>>"$LOGFILE"
                end

            case "lm-sensors"
                echo -e "${CYAN}🔍 Detecting Hardware Sensors (Automatic)...${NC}"
                # --yes flag gives auto-confirmation to all sensor detection prompts
                sudo sensors-detect --auto &>>"$LOGFILE"
                test "$SERVICE_CMD" = "systemctl" && {
                    sudo systemctl enable --now lm_sensors &>>"$LOGFILE" 2>/dev/null ||
                        sudo systemctl enable --now sensord &>>"$LOGFILE" 2>/dev/null
                end
                add_config "Sensor Alias" "alias temp='sensors'"
        end
    end

    # Kill sudo keepalive - SILENT
    kill $SUDO_KEEPALIVE 2>/dev/null || true
    wait $SUDO_KEEPALIVE 2>/dev/null || true

    # ===== 🔗 SHELL INTEGRATION =====
    if test " ${installed_pkgs[*]} " =~ " fzf "
        if test "$SHELL_NAME" == "bash" ]] || [[ "$SHELL_NAME" == "zsh"
            add_config "FZF Integration" "eval \"\(fzf --$SHELL_NAME)\""
        end
    end

    add_config "HISTORY" "export HISTSIZE=10000\nexport HISTFILESIZE=20000"

    # Cleanup
    switch $PKG_MANAGER
        case "apt") sudo apt autoremove -y &>>"$LOGFILE" || true ;;
        "dnf" | "yum") sudo $PKG_MANAGER autoremove -y &>>"$LOGFILE" || true ;;
        case "pacman") sudo pacman -Sc --noconfirm &>>"$LOGFILE" || true ;;
    end

    # Report
    echo -e "\n${GREEN}✅ Deployment Complete on ${DISTRO_ID}!${NC}"
    echo -e "${GREEN}📦 Installed: (count $installed_pkgs) tools${NC}"

    test (count $failed_pkgs) -gt 0 && {
        echo -e "${RED}❌ Failed ((count $failed_pkgs)):${NC}"
        printf '  - %s\n' "$failed_pkgs"
    end

    test " ${installed_pkgs[*]} " =~ " docker.io " && echo -e "${YELLOW}⚠️  Log out and back in for Docker group changes${NC}"
    test " ${installed_pkgs[*]} " =~ " zoxide " && echo -e "${CYAN}💡 Run 'source $RC_FILE' to enable zoxide${NC}"

    # Cleanup log
    rm -f "$LOGFILE" 2>/dev/null || true
end

# ======================================================
#  📂 all file re name
# ======================================================

functions -e rn 2>/dev/null
function rn
    set -l target_dir "(if test -n "$argv[1]"; echo "$argv[1]"; else; echo "."; end)"

    if test ! -d "$target_dir"
        echo "Error: Directory $target_dir does not exist."
        return 1
    end

    echo "Cleaning files in: $target_dir"

    # -print0 ব্যবহার করা হয়েছে যাতে ফাইলের নামে স্পেস বা নিউলাইন থাকলেও সমস্যা না হয়
    find "$target_dir" -maxdepth 1 -type f -print0 | while IFS= read -r -d '' filepath; do

        dir=(dirname "$filepath")
        f=(basename "$filepath")

        # ১. নাম এবং এক্সটেনশন আলাদা করা (স্মার্ট চেক)
        if test "$f" == *.*
            filename="${f%.*}"
            extension=".${f##*.}" # ডটসহ এক্সটেনশন
        else
            filename="$f"
            extension="" # এক্সটেনশন নেই
        end

        # ২. নাম ক্লিন করা
        clean_name=(echo "$filename" | tr '[:upper:]' '[:lower:]' | tr ' ' '-' | sed 's/[^a-z0-9_-]//g')
        clean_ext=(echo "$extension" | tr '[:upper:]' '[:lower:]')

        new_name="${clean_name}${clean_ext}"

        # ৩. রিনেম কন্ডিশন
        if test "$f" != "$new_name"
            if test -e "$dir/$new_name"
                echo "Skipped: '$new_name' already exists."
            else
                mv "$dir/$f" "$dir/$new_name"
                echo "Renamed: '$f' -> '$new_name'"
            end
        end
    end
    echo "Done!"
end

# ======================================================
#  📂 package genarator
# ======================================================

# Smart Universal Package Converter & Manager
functions -e pg 2>/dev/null
function pg
    set -l file "$argv[1]"
    set -l install_flag "$argv[2]"
    set -l os_type ""
    set -l pkg_manager ""
    set -l target_ext ""

    # ১. OS এবং প্যাকেজ ম্যানেজার ডিটেক্ট করা
    if test -f /etc/os-release
        . /etc/os-release
        switch $ID
            ubuntu | deepin | debian | kali | linuxmint | pop)
                os_type="debian"
                pkg_manager="apt"
                target_ext="deb"
            fedora | rhel | centos | amzn)
                os_type="redhat"
                pkg_manager="dnf"
                target_ext="rpm"
            arch | manjaro)
                os_type="arch"
                pkg_manager="pacman"
                target_ext="tgz" # Alien Arch এর জন্য tgz ব্যবহার করে
            case *
                echo "Sorry Your OS ($PRETTY_NAME) is not supported by this script."
                return 1
        end
    end

    # ২. ইনপুট চেক
    if test -z "$file"
        echo "ব্যবহার: superconv [ফাইল_নাম] [-i]"
        echo "OS ডিটেক্ট করা হয়েছে: $PRETTY_NAME"
        return 1
    end

    # ৩. Alien টুলটি আছে কিনা চেক ও ইনস্টল করা
    if ! command -v alien >/dev/null 2>&1
        echo "Alien not installed $pkg_manager Package Used To install..."
        if test "$pkg_manager" == "apt"
            sudo apt update && sudo apt install alien -y
        else if test "$pkg_manager" == "dnf"
            sudo dnf install alien -y
        else if test "$pkg_manager" == "pacman"
            sudo pacman -S alien --noconfirm
        end
    end

    # ৪. কনভার্ট করা (OS অনুযায়ী target ফরম্যাট সেট করা)
    if test -f "$file"
        echo "Converter $target_ext format..."

        switch $os_type
            case "debian") sudo alien --to-deb --scripts "$file" ;;
            case "redhat") sudo alien --to-rpm --scripts "$file" ;;
            case "arch") sudo alien --to-tgz --scripts "$file" ;;
        end

        if test $? -eq 0
            echo "--- file Convert successfully ! ---"

            # ৫. কন্ডিশনাল ইনস্টলেশন
            if test "$install_flag" == "-i"
    set -l new_pkg (ls -t *.$target_ext | head -1)
                echo "This Package intsalling: $new_pkg"

                if test "$pkg_manager" == "apt"
                    sudo dpkg -i "$new_pkg" && sudo apt install -f
                else if test "$pkg_manager" == "dnf"
                    sudo dnf install ./"$new_pkg" -y
                else if test "$pkg_manager" == "pacman"
                    sudo pacman -U "$new_pkg" --noconfirm
                end
            end
        else
            echo "convarsion Failed !"
        end
    else
        echo "file is missing: $file"
    end
end

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

functions -e drive 2>/dev/null
function drive
    # 1st and 2nd drive er unique sesh ongsho (UUID) ekhane bosiye din
    set -l drive1_uuid "469a94"
    set -l drive2_uuid "b2c89f" # <--- Apnar 2nd drive er UUID match kore niben

    set -l selected_uuid ""
    set -l target_path ""

    # Kon drive e jabe seta select korbe
    if test "$argv[1]" == "1" || -z "$argv[1]"
        selected_uuid="$drive1_uuid"
    else if test "$argv[1]" == "2"
        selected_uuid="$drive2_uuid"
    else
        echo "❌ Invalid option! Use: 'drive' or 'drive 1', 'drive 2'"
        return 1
    end

    # Fish-e dynamically path khunje ber korar upay
    # /media/ er bhitore thaka path gulo loop korbe
    for path in /media/*/*"$selected_uuid"*/ /media/*/*"$selected_uuid"; do
        if test -d "$path"
            target_path="$path"
            break
        end
    end

    # Path jodi thikthak thake tahole cd korbe
    if test -d "$target_path"
        cd "$target_path"
        echo "📂 Switched to: $PWD"
    else
        echo "❌ Error: Driveটি খুঁজে পাওয়া যায়নি! এটি কি মাউন্ট করা আছে?"
    end
end

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
functions -e update 2>/dev/null
functions -e clean 2>/dev/null
function update
    echo -e "\033[1;36m🔄 Updating system packages...\033[0m"
    if command -v apt-get >/dev/null 2>&1
        sudo apt-get update && sudo apt-get upgrade -y && sudo apt-get dist-upgrade -y && sudo apt-get install -f
    else if command -v pacman >/dev/null 2>&1
        sudo pacman -Syu --noconfirm
    else if command -v dnf >/dev/null 2>&1
        sudo dnf upgrade --refresh -y
    else if command -v brew >/dev/null 2>&1
        brew update && brew upgrade
    end
    if command -v flatpak >/dev/null 2>&1
        echo -e "\033[1;34m📦 Updating Flatpaks...\033[0m"
        flatpak update -y
    end
    if command -v snap >/dev/null 2>&1
        echo -e "\033[1;35m⚡ Refreshing Snaps...\033[0m"
        sudo snap refresh 2>/dev/null || true
    end
end

function clean
    echo -e "\033[1;33m🧹 Cleaning system caches...\033[0m"
    if command -v apt-get >/dev/null 2>&1
        sudo apt-get autoremove --purge -y && sudo apt-get autoclean && sudo apt-get clean -y
    else if command -v pacman >/dev/null 2>&1
    set -l orphans
        orphans=(pacman -Qtdq 2>/dev/null)
        test -n "$orphans" && sudo pacman -Rns --noconfirm $orphans 2>/dev/null || true
        sudo pacman -Sc --noconfirm
    else if command -v dnf >/dev/null 2>&1
        sudo dnf autoremove -y && sudo dnf clean all
    else if command -v brew >/dev/null 2>&1
        brew cleanup
    end
    if command -v flatpak >/dev/null 2>&1
        echo -e "\033[1;34m💎 Cleaning Flatpak unused data...\033[0m"
        flatpak uninstall --unused -y && flatpak repair
    end
end

alias bashrc='code ~/.bashrc'
alias to='code .'
alias rel='source ~/.bashrc && echo "✅ .bashrc reloaded successfully!"'

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
alias pgl='sudo -u postgres psql'           # postgres user hisebe login
alias pgdb='psql -U postgres -d'            # Usage: pgdb mydb
alias pgls='psql -U postgres -c "\\l"'      # সব database list
alias pgtables='psql -U postgres -c "\\dt"' # সব table list
alias pgdump='pg_dump -U postgres'          # Usage: pgdump mydb > backup.sql
alias pgrestore='psql -U postgres'          # Usage: pgrestore mydb < backup.sql
functions -e pglogs 2>/dev/null
function pglogs
    if [ -d /var/log/postgresql ] && ls /var/log/postgresql/*.log >/dev/null 2>&1
        sudo tail -f /var/log/postgresql/*.log
    else
        sudo journalctl -u postgresql -f
    end
end
alias pgcreate='createdb -U postgres'                                                                                                                                                                # Usage: pgcreate mydb
alias pgdrop='dropdb -U postgres'                                                                                                                                                                    # Usage: pgdrop mydb
alias pgusers='psql -U postgres -c "\\du"'                                                                                                                                                           # সব users/roles দেখুন
alias pgsize='psql -U postgres -c "SELECT pg_database.datname, pg_size_pretty(pg_database_size(pg_database.datname)) AS size FROM pg_database ORDER BY pg_database_size(pg_database.datname) DESC;"' # প্রতিটি DBর সাইজ
alias pgver='psql -U postgres -c "SELECT version();"'                                                                                                                                                # PostgreSQL version দেখুন
alias pgconn='psql -U postgres -c "SELECT count(*) FROM pg_stat_activity;"'                                                                                                                          # active connections

# --- Node/NPX Prisma ORM (np*) — first letter of each word ---
alias np='npx prisma'                       # npx prisma
alias npi='npx prisma init'                 # npx prisma init
alias npg='npx prisma generate'             # npx prisma generate
alias nps='npx prisma studio'               # npx prisma studio
alias npmd='npx prisma migrate dev'         # npx prisma migrate dev
alias npmdn='npx prisma migrate dev --name' # npx prisma migrate dev --name
alias npmr='npx prisma migrate reset'       # npx prisma migrate reset
alias npmdp='npx prisma migrate deploy'     # npx prisma migrate deploy
alias npms='npx prisma migrate status'      # npx prisma migrate status
alias npdp='npx prisma db push'             # npx prisma db push
alias npdl='npx prisma db pull'             # npx prisma db pull
alias npds='npx prisma db seed'             # npx prisma db seed
alias npf='npx prisma format'               # npx prisma format
alias npv='npx prisma version'              # npx prisma version

# --- Bun Prisma ORM (bp*) — first letter of each word ---
alias bp='bunx prisma'                       # bunx prisma
alias bpi='bunx prisma init'                 # bunx prisma init
alias bpg='bunx prisma generate'             # bunx prisma generate
alias bps='bunx prisma studio'               # bunx prisma studio
alias bpmd='bunx prisma migrate dev'         # bunx prisma migrate dev
alias bpmdn='bunx prisma migrate dev --name' # bunx prisma migrate dev --name
alias bpmr='bunx prisma migrate reset'       # bunx prisma migrate reset
alias bpmdp='bunx prisma migrate deploy'     # bunx prisma migrate deploy
alias bpms='bunx prisma migrate status'      # bunx prisma migrate status
alias bpdp='bunx prisma db push'             # bunx prisma db push
alias bpdl='bunx prisma db pull'             # bunx prisma db pull
alias bpds='bunx prisma db seed'             # bunx prisma db seed
alias bpf='bunx prisma format'               # bunx prisma format
alias bpv='bunx prisma version'              # bunx prisma version

# ======================================================
#  🛠️  FUNCTION TOOLS (Better than Aliases)
# ======================================================

# Create directory and enter it immediately
# Usage: mkd new_folder
functions -e mkd 2>/dev/null
function mkd
    mkdir -p "$argv[1]" && cd "$argv[1]" && echo "✅ Created & Entered: $argv[1]"
end

# Force remove directory
# Usage: rmd folder_name
functions -e rmd 2>/dev/null
function rmd
    rm -rf "$argv[1]" && echo "✅ Removed directory: $argv[1]"
end

# Remove file with confirmation
# Usage: rmf file.txt
functions -e rmf 2>/dev/null
function rmf
    rm -i "$argv[1]" && echo "✅ Removed file: $argv[1]"
end

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
alias gpu='git push -u origin (git branch --show-current)'
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
functions -e command_not_found_handle 2>/dev/null
function command_not_found_handle
    echo "❌ Command not found: $argv[1]"
    if command -v apt >/dev/null 2>&1
        echo "🔍 Try searching: apt search $argv[1] | npm i -g $argv[1]"
    else if command -v pacman >/dev/null 2>&1
        echo "🔍 Try searching: pacman -Ss $argv[1] | npm i -g $argv[1]"
    else if command -v dnf >/dev/null 2>&1
        echo "🔍 Try searching: dnf search $argv[1] | npm i -g $argv[1]"
    else
        echo "🔍 Try searching via package manager or npm i -g $argv[1]"
    end
end

alias bv='cd ~/Downloads/Brave'
alias ch='cd ~/Downloads/Chrome'
alias gp='cd ~/Downloads/Google\ Photos'
alias pa='cd ~/Downloads/Packet'
alias ss='cd ~/Downloads/Screenshot'
alias vi='cd ~/Downloads/Video'


# Remove any Flatpak app paths from LD_LIBRARY_PATH
if test -n "$LD_LIBRARY_PATH" ]] && [[ "$LD_LIBRARY_PATH" == *"/var/lib/flatpak/app/"*
    # Filter out Flatpak app paths, keep system paths
    new_path=(echo "$LD_LIBRARY_PATH" | tr ':' '\n' | grep -v "/var/lib/flatpak/app/" | grep -v "$HOME/.local/share/flatpak/app/" | paste -sd ':' -)
    if test -n "$new_path"
set -gx LD_LIBRARY_PATH "$new_path"
    else
        unset LD_LIBRARY_PATH
    end
end

# ==============================================================================
# 1. Auto-LS and FZF Summary Preview when changing directory (Fish Version)
# ==============================================================================

functions -e accurate_auto_ls 2>/dev/null
function accurate_auto_ls
    # Shudhu jokhon directory change hobe (kew cd korbe) tokhon e run hobe
    if test "$PWD" = "$LAST_PWD"
        return
    end
    LAST_PWD="$PWD"

    # Fish-er native alternative array parse logic
    # (Dotglob on kore hidden file accurately count korar jonne)
    local -a total_items=(*)

    set -l file_count 0
    set -l hidden_count 0

    for item in "$total_items"; do
        # Shudhu regular files count hobe
        if test -f "$item" ] && [ ! -L "$item"
            ((file_count++))
            # Jodi name '.' diye shuru hoy
            if test "$item" == .*
                ((hidden_count++))
            end
        end
    end

    # ${PWD:t} er Fish equivalent hocche $(basename "$PWD")
    set -l dir_name
    dir_name=(basename "$PWD")

    # Clean UI rendering
    echo -e "\n\e[1;35m📂 Directory: $dir_name\e[0m (\e[32m$file_count files\e[0m | \e[33m$hidden_count hidden\e[0m)"
    echo -e "\e[2m───────────────────────────────────────\e[0m"

    # Display using explicit native columns
    ls -FA --color=auto
end

# Fish-er chpwd hook alternative: PROMPT_COMMAND array pipeline registration
LAST_PWD="$PWD"
if test ! " ${PROMPT_COMMAND[*]} " == *"accurate_auto_ls"*
    if test -n "$PROMPT_COMMAND"
        PROMPT_COMMAND="accurate_auto_ls;$PROMPT_COMMAND"
    else
        PROMPT_COMMAND="accurate_auto_ls"
    end
end

# ==============================================================================
# 2. Advanced FZF Quick CD Function
# ==============================================================================
# Terminal-e shudhu 'cf' likhle fzf open hobe pipeline preview shoho

functions -e cf 2>/dev/null
function cf
    # Helper to auto-install missing packages dynamically based on active package manager
function _cf_install_pkg
    set -l tool "$argv[1]"
    set -l apt_pkg "$argv[2]"
    set -l pac_pkg "$argv[3]"
    set -l dnf_pkg "$argv[4]"

        echo -e "\033[1;33m⚡ 'cf' requires '$tool'. Dynamic auto-installing for your distro...\033[0m"

        if command -v apt-get >/dev/null 2>&1
            sudo apt-get update -qq && sudo apt-get install -y "$apt_pkg"
        else if command -v pacman >/dev/null 2>&1
            sudo pacman -S --noconfirm --needed "$pac_pkg"
        else if command -v dnf >/dev/null 2>&1
            sudo dnf install -y "$dnf_pkg"
        else if command -v zypper >/dev/null 2>&1
            sudo zypper install -y "$pac_pkg"
        else if command -v apk >/dev/null 2>&1
            sudo apk add "$pac_pkg"
        else if command -v brew >/dev/null 2>&1
            brew install "$pac_pkg"
        else
            echo -e "\033[1;31m❌ Package manager not found. Please install '$tool' manually.\033[0m"
            return 1
        end
    end

    # 1. Dynamic Dependency Check & Auto-Install for fzf
    if ! command -v fzf >/dev/null 2>&1
        _cf_install_pkg "fzf" "fzf" "fzf" "fzf" || return 1
    end

    # 2. Dynamic Dependency Check & Auto-Install for fd
    set -l fd_cmd ""
    if command -v fd >/dev/null 2>&1
        fd_cmd="fd"
    else if command -v fdfind >/dev/null 2>&1
        fd_cmd="fdfind"
    else
        _cf_install_pkg "fd" "fd-find" "fd" "fd-find"
        if command -v fd >/dev/null 2>&1
            fd_cmd="fd"
        else if command -v fdfind >/dev/null 2>&1
            fd_cmd="fdfind"
        end
    end

    # 3. Dynamic Dependency Check & Auto-Install for zoxide
    if ! command -v zoxide >/dev/null 2>&1
        _cf_install_pkg "zoxide" "zoxide" "zoxide" "zoxide"
    end

    # 4. Dynamic Dependency Check & Auto-Install for bat
    if ! command -v bat >/dev/null 2>&1; and ! command -v batcat >/dev/null 2>&1
        _cf_install_pkg "bat" "bat" "bat" "bat"
    end

    # 5. Dynamic Dependency Check & Auto-Install for chafa
    if ! command -v chafa >/dev/null 2>&1
        _cf_install_pkg "chafa" "chafa" "chafa" "chafa"
    end

    set -l dir
    set -l search_cmd
    set -l target_dir "(if test -n "$argv[1]"; echo "$argv[1]"; else; echo "."; end)"
    if test "$target_dir" = "." ] && [ "$PWD" = "/"
        target_dir="$HOME"
    end

    # Smart hidden filter: allow project hidden folders (.github, .vscode, .env) in projects,
    # while excluding heavy IDE/app data folders at Home root level (.antigravity-ide, .linglong, etc.)
    set -l hidden_flag "--hidden"
    set -l exclude_opts "--exclude .git --exclude node_modules --exclude .cache --exclude .antigravity-ide --exclude .linglong --exclude .local --exclude .var --exclude .npm --exclude .nvm --exclude .cargo --exclude .rustup --exclude .electron --exclude .mozilla --exclude /proc --exclude /sys --exclude /dev --exclude /etc --exclude /var --exclude /usr --exclude /tmp --exclude /run/user --exclude /run/systemd"

    set -l abs_target
    abs_target=(cd "$target_dir" 2>/dev/null && pwd || echo "$target_dir")
    if test "$abs_target" = "$HOME" ] || [ "$PWD" = "$HOME"
        hidden_flag=""
        exclude_opts="--exclude '.*' --exclude node_modules --exclude /proc --exclude /sys --exclude /dev --exclude /etc --exclude /var --exclude /usr --exclude /tmp --exclude /run/user --exclude /run/systemd"
    end

    if test -n "$fd_cmd"
        search_cmd="$fd_cmd $hidden_flag $exclude_opts . \"$target_dir\""
    else
        search_cmd="find \"$target_dir\" \( -path '*/.*' -o -path '*/node_modules*' -o -path '/proc*' -o -path '/sys*' -o -path '/dev*' -o -path '/etc*' -o -path '/var*' -o -path '/usr*' -o -path '/tmp*' -o -path '/run/user*' -o -path '/run/systemd*' \) -prune -o -print 2>/dev/null"
    end

    # 6. Optimized Multi-Action Workflow (Folders, Videos, Images & PDFs Supported)
    set dir (eval "$search_cmd" | fzf \
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
        --bind "ctrl-h:reload(([ -n "$fd_cmd" ] && echo "$fd_cmd --exclude '.*' --exclude node_modules --exclude /proc --exclude /sys --exclude /dev --exclude /etc --exclude /var --exclude /usr --exclude /tmp --exclude /run/user --exclude /run/systemd . \(dirname {}) 2>/dev/null" || echo "find \(dirname {}) \( -path '*/.*' -o -path '*/node_modules*' -o -path '/proc*' -o -path '/sys*' -o -path '/dev*' -o -path '/etc*' -o -path '/var*' -o -path '/usr*' -o -path '/tmp*' -o -path '/run/user*' -o -path '/run/systemd*' \) -prune -o -print 2>/dev/null"))+change-prompt(⚡ Parent: )" \
        --preview '
            if [ -d {} ]; then
                echo -e "\e[1;34m📁 Contents of: {} \e[0m"
                echo -e "\e[2m──────────────────────────────────────────\e[0m"
                if command -v eza >/dev/null 2>&1; then
                    eza --tree --level=1 --icons --color=always {} 2>/dev/null | head -20
                elif command -v tree >/dev/null 2>&1; then
                    tree -C -L 1 {} 2>/dev/null | head -20
                else
                    ls -FA --color=always {} 2>/dev/null | head -20
                fi
                if git -C {} rev-parse --is-inside-work-tree >/dev/null 2>&1; then
                    echo -e "\e[2m──────────────────────────────────────────\e[0m"
                    branch=$(git -C {} branch --show-current 2>/dev/null || git -C {} rev-parse --short HEAD 2>/dev/null)
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
                        if command -v chafa >/dev/null 2>&1; then
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
                        command -v mediainfo >/dev/null 2>&1 && mediainfo {} | head -15
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
                        if command -v bat >/dev/null 2>&1; then
                            bat --color=always --style=numbers --line-range :30 {} 2>/dev/null
                        elif command -v batcat >/dev/null 2>&1; then
                            batcat --color=always --style=numbers --line-range :30 {} 2>/dev/null
                        else
                            head -n 25 {} 2>/dev/null
                        fi
                        ;;
                esac
            fi
            echo -e "\e[2m──────────────────────────────────────────\e[0m"
            sz=$(timeout 0.2s du -sh {} 2>/dev/null | cut -f1)
            echo -e "\e[1;33m📊 Size:\e[0m ${sz:-Quick Scan}"
        ' \
        --preview-window=right:50%:wrap)

    if test -n "$dir"
        if test -d "$dir"
            cd "$dir" || return
        else if test -f "$dir"
            set -l ext "${dir##*.}"
            ext="${ext,,}"
            switch $ext
                case mp4|mkv|avi|mov|webm|flv|m4v
                    echo -e "\033[1;35m🎬 Opening Video with v()...\033[0m"
                    v "$dir"
                case pdf
                    echo -e "\033[1;36m📄 Opening PDF in Chrome...\033[0m"
                    (google-chrome "$dir" 2>/dev/null || google-chrome-stable "$dir" 2>/dev/null || chromium "$dir" 2>/dev/null || chromium-browser "$dir" 2>/dev/null || brave "$dir" 2>/dev/null || xdg-open "$dir" 2>/dev/null) &
                case *
                    code "$dir" 2>/dev/null || cursor "$dir" 2>/dev/null || nvim "$dir"
            end
        end
    end
end

# ====================================================================
#              🐳 THE ULTIMATE DOCKER SWISS ARMY KNIFE 🐳
# ====================================================================

# --------------------------------------------------------------------
# ১. স্ট্যাটাস, লিস্ট এবং সাইজ মনিটরিং (Status & Monitoring)
# --------------------------------------------------------------------
# চলমান কন্টেইনারের সুন্দর ক্লিন লিস্ট দেখতে
alias dps="docker ps --format 'table {{.ID}}\t{{.Names}}\t{{.Status}}\t{{.Ports}}'"
# সব কন্টেইনারের লিস্ট দেখতে (স্টপ হওয়াগুলোসহ)
alias dpsa="docker ps -a"
# পিসিতে ডাউনলোড করা সব ইমেজের লিস্ট
alias di="docker images"
# সব ভলিউম (Storage) এর লিস্ট
alias dvl="docker volume ls"
# সব ডকার নেটওয়ার্কের লিস্ট
alias dnl="docker network ls"
# ডকার টোটাল কতটুকু ডিস্ক স্পেস বা সাইজ খাচ্ছে তা দেখা
alias dsize="docker system df"
# লাইভ রিসোর্স মনিটরিং (কোন কন্টেইনার কত RAM/CPU খাচ্ছে তা দেখতে)
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
# এক কমান্ডে চলমান কন্টেইনার ফোর্স স্টপ ও ডিলিট করা
alias dkill="docker rm -f"
# রানিং সব কন্টেইনার একসাথে স্টপ করতে
alias dstopall="docker stop \(docker ps -q)"
# স্টপ হওয়া সব কন্টেইনার একসাথে ডিলিট করতে
alias drmall="docker rm \(docker ps -a -q)"

# --------------------------------------------------------------------
# ৩. ডিবাগিং, লগ এবং ইমেজ বিল্ড (Debugging & Building)
# --------------------------------------------------------------------
# কন্টেইনারের ভেতরে ঢুকে টার্মিনাল চালাতে
alias dsh="docker exec -it"
# কন্টেইনারের লাইভ লগ দেখতে ফলো মোডসহ
alias dlogs="docker logs -f"
# ডকার ইমেজ বিল্ড করার শর্টকাট
alias dbuild="docker build -t"
# একদম স্ক্র্যাচ থেকে নতুন বিল্ড করা (ক্যাশ ইমেজ বাদ দিয়ে)
alias dbuild-nocache="docker build --no-cache -t"
# ইমেজের লেয়ার এবং হিস্ট্রি দেখা
alias dhist="docker history"
# কন্টেইনারের ওপেন পোর্টগুলো দ্রুত চেক করা
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
# এই কন্টেইনারগুলো এক্সিট করার সাথে সাথে পিসি থেকে অটো ডিলিট হয়ে যাবে
alias dtest-ubuntu="docker run --rm -it ubuntu:latest bash"
alias dtest-node="docker run --rm -it node:alpine sh"
alias dtest-alpine="docker run --rm -it alpine:latest sh"

# --------------------------------------------------------------------
# ৬. স্মার্ট এবং অ্যাডভান্সড ফাংশন (Advanced Functions)
# --------------------------------------------------------------------

# ইমেজ বা কন্টেইনারের নাম দিয়ে সার্চ করা
functions -e dfind 2>/dev/null
function dfind
    echo -e "\e[1;34m--> Running Containers:\e[0m"
    docker ps | grep -i "$argv[1]"
    echo -e "\n\e[1;32m--> Downloaded Images:\e[0m"
    docker images | grep -i "$argv[1]"
end

# কন্টেইনারের ভেতর সরাসরি Root ইউজার হিসেবে ঢোকা (পারমিশন এরর এড়াতে)
functions -e droot 2>/dev/null
function droot
    docker exec -it -u root "$argv[1]" bash 2>/dev/null || docker exec -it -u root "$argv[1]" sh
end

# নির্দিষ্ট কন্টেইনারের শুধু লোকাল IP এড্রেসটি দেখতে
functions -e dip 2>/dev/null
function dip
    docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$argv[1]"
end

# কন্টেইনারের রিয়েল-টাইম ফাইল সিস্টেম পরিবর্তন লাইভ ট্র্যাকিং করা
functions -e dwatch 2>/dev/null
function dwatch
    if test -z "$argv[1]"
        echo "Usage: dwatch <container-name>"
        return 1
    end
    echo -e "\e[1;35mWatching file changes in '$argv[1]' (Press Ctrl+C to stop)...\e[0m"
    watch -n 1 "docker diff $argv[1]"
end

# কন্টেইনারের ট্রাফিক এবং লাইভ পোর্ট বাইন্ডিং ডিবাগ করা
functions -e dnetstat 2>/dev/null
function dnetstat
    if test -z "$argv[1]"
        echo "Usage: dnetstat <container-name>"
        return 1
    end
    echo -e "\e[1;36mActive connections inside '$argv[1]':\e[0m"
    docker exec -it "$argv[1]" netstat -tulan 2>/dev/null || docker exec -it "$argv[1]" ss -tulan 2>/dev/null || echo "Error: Neither netstat nor ss is installed in this container."
end

# কন্টেইনারের ভেতরের প্রসেস ট্রি (Process Tree) দেখা
unalias dtop-proc 2>/dev/null
function dtop-proc {
    if test -z "$argv[1]"
        echo "Usage: dtop-proc <container-name>"
        return 1
    end
    docker top "$argv[1]" aux
end

# কোনো ডকার কন্টেইনারের ভলিউম ডিরেক্টলি ব্যাকআপ নেওয়া (Tar ফাইল হিসেবে)
functions -e dbackup 2>/dev/null
function dbackup
    docker run --rm -v "$argv[1]":/volume -v "(pwd)":/backup alpine tar cvf /backup/"$argv[2]" -C /volume .
end

# ইন্টারেক্টিভ সব চলমান কন্টেইনার ফোর্স কিল করা (কনফার্মেশনসহ)
unalias dkill-force 2>/dev/null
function dkill-force {
    echo -e "\e[1;31m⚠️  WARNING: You are about to stop and remove ALL running containers!\e[0m"
    read -P "Are you sure? (y/N): " confirm
    if test "$confirm" =~ ^[Yy]$
        docker stop (docker ps -q) 2>/dev/null
        docker rm (docker ps -a -q) 2>/dev/null
        echo -e "\e[1;32mDone. All containers cleared.\e[0m"
    else
        echo "Operation cancelled."
    end
end

# --------------------------------------------------------------------
# ০০০. ডকার ড্যাশবোর্ড ও অল-ইন-ওয়ান টার্মিনাল ম্যানেজার (dman & dstats)
# --------------------------------------------------------------------

# dependency check & main launcher
functions -e dman 2>/dev/null
function dman
    # Dependency Check
    if ! command -v fzf >/dev/null 2>&1 || ! command -v gum >/dev/null 2>&1
        echo -e "\e[1;31mError: 'fzf' এবং 'gum' ইনস্টল করা নেই! (fzf and gum are required for dman)\e[0m"
        return 1
    end

    while true; do
        clear
    set -l active_context
        active_context=(docker context show 2>/dev/null || echo "default")

        gum style \
            --foreground 212 --border-foreground 212 --border double \
            --align center --width 68 --margin "1 0" --padding "0 2" \
            "🐳 DOCKER DESKTOP - DEVOPS TERMINAL EDITION" \
            "Active Context: $active_context | Use Mouse or Arrow Keys"

    set -l module
        set module (gum choose \
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

        switch $module
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
        end
    end
end

# Container Management TUI
functions -e _manage_containers 2>/dev/null
function _manage_containers
    while true; do
        clear
        if ! docker info >/dev/null 2>&1
            gum style --foreground 196 --bold "❌ Docker daemon is not running! Start it first."
            sleep 2
            return 1
        end

    set -l cid
        set cid (docker ps -a --format "table {{.ID}}\t{{.Names}}\t{{.Status}}\t{{.Image}}" 2>/dev/null | \
            fzf --header-lines=1 \
                --prompt="Select Container ❯ " \
                --pointer="▶" \
                --border \
                --height=15 \
                --preview="echo '--- [ LIVE LOGS ] ---' && docker logs --tail 25 {1} 2>/dev/null" \
                --preview-window=right:55%:wrap | awk '{print $argv[1]}')

        [ -z "$cid" ] && break

    set -l cname
        cname=(docker inspect --format='{{.Name}}' "$cid" 2>/dev/null | sed 's/^\///')
        [ -z "$cname" ] && cname="$cid"

        echo ""
        gum style --foreground 214 --bold "Selected Container: $cname ($cid)"

    set -l action
        set action (gum choose \
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

        switch $action
            "📋 View Logs (Live)")
                clear
                gum style --foreground 39 "Press Ctrl+C to exit logs..."
                docker logs -f --tail 100 "$cid"
            "🐚 Shell Access (Bash/Sh)")
                clear
                docker exec -it "$cid" /bin/sh -c "bash || sh"
            "🌐 Open Port in Browser")
                _open_container_port "$cid"
            "📂 File Transfer (Host ⇄ Container)")
                _copy_files "$cid"
            "⚙️ Update Limits (CPU/RAM)")
                _update_container_resources "$cid"
            "💾 Save Container as Image (Commit)")
                _commit_container "$cid"
            "▶️ Start Container")
                gum spin --spinner dot --title "Starting $cname..." -- docker start "$cid"
            "⏹️ Stop Container")
                gum spin --spinner dot --title "Stopping $cname..." -- docker stop "$cid"
            "⏸️ Pause Container")
                gum spin --spinner dot --title "Pausing $cname..." -- docker pause "$cid"
            "▶️ Unpause Container")
                gum spin --spinner dot --title "Unpausing $cname..." -- docker unpause "$cid"
            "🔄 Restart Container")
                gum spin --spinner dot --title "Restarting $cname..." -- docker restart "$cid"
            "🔍 Inspect Config")
                docker inspect "$cid" | fzf --header="Inspect: $cname"
            "🗑️ Delete Container")
                if gum confirm "Permanently remove container '$cname'?"
                    docker rm -f "$cid"
                end
            "🔙 Back"|*) continue ;;
        end
    end
end

# Container Sub-functions
functions -e _open_container_port 2>/dev/null
function _open_container_port
    set -l cid $argv[1]
    set -l ports
    ports=(docker port "$cid" 2>/dev/null | awk '{print $argv[3]}' | awk -F: '{print $NF}' | sort -u)
    if test -z "$ports"
        gum style --foreground 196 "❌ No published ports found for this container!"
        sleep 1.5
        return
    end
    set -l selected_port
    selected_port=(echo "$ports" | gum choose --header="Select Port to Open in Browser:")
    if test -n "$selected_port"
        xdg-open "http://localhost:$selected_port" 2>/dev/null || xdg-open "http://127.0.0.1:$selected_port" 2>/dev/null || open "http://localhost:$selected_port" 2>/dev/null
    end
end

functions -e _copy_files 2>/dev/null
function _copy_files
    set -l cid $argv[1]
    set -l direction
    direction=(gum choose "📥 Copy from Host to Container" "📤 Copy from Container to Host" "🔙 Cancel")

    if test "$direction" == "📥 Copy from Host to Container"
        set -l src dest
        set src (gum input --placeholder "Source Path on Host (e.g., ./app.conf)")
        set dest (gum input --placeholder "Dest Path in Container (e.g., /etc/app.conf)")
        if test -n "$src" ] && [ -n "$dest"
            if docker cp "$src" "$cid:$dest"
                gum style --foreground 46 "✔ File copied successfully!"
            else
                gum style --foreground 196 "❌ File copy failed!"
            end
            sleep 1.5
        end
    else if test "$direction" == "📤 Copy from Container to Host"
        set -l src dest
        set src (gum input --placeholder "Source Path in Container (e.g., /var/log/app.log)")
        set dest (gum input --placeholder "Dest Path on Host (e.g., ./app.log)")
        if test -n "$src" ] && [ -n "$dest"
            if docker cp "$cid:$src" "$dest"
                gum style --foreground 46 "✔ File copied successfully!"
            else
                gum style --foreground 196 "❌ File copy failed!"
            end
            sleep 1.5
        end
    end
end

functions -e _update_container_resources 2>/dev/null
function _update_container_resources
    set -l cid $argv[1]
    set -l mem cpus
    set mem (gum input --placeholder "Memory Limit (e.g., 512m, 2g) - Leave empty to skip")
    set cpus (gum input --placeholder "CPU Limit (e.g., 1.5, 2) - Leave empty to skip")

    if test -n "$mem" ] || [ -n "$cpus"
    set -l opts ""
        [ -n "$mem" ] && opts="$opts --memory=$mem"
        [ -n "$cpus" ] && opts="$opts --cpus=$cpus"

        if docker update $opts "$cid"
            gum style --foreground 46 "✔ Resources updated successfully!"
        else
            gum style --foreground 196 "❌ Failed to update container resources!"
        end
        sleep 1.5
    end
end

functions -e _commit_container 2>/dev/null
function _commit_container
    set -l cid $argv[1]
    set -l new_image
    set new_image (gum input --placeholder "New Image Name (e.g., my-custom-app:v2)")
    if test -n "$new_image"
        if gum spin --spinner dot --title "Creating image from container..." -- docker commit "$cid" "$new_image"
            gum style --foreground 46 "✔ Created Image: $new_image"
        else
            gum style --foreground 196 "❌ Failed to commit container image!"
        end
        sleep 1.5
    end
end

# Image Management TUI
functions -e _manage_images 2>/dev/null
function _manage_images
    while true; do
        clear
        if ! docker info >/dev/null 2>&1
            gum style --foreground 196 --bold "❌ Docker daemon is not running! Start it first."
            sleep 2
            return 1
        end

        gum style --foreground 39 --bold "=== DOCKER IMAGES MODULE ==="
    set -l img_action
        set img_action (gum choose \
            "📋 List & Inspect Local Images" \
            "📥 Pull Image from Docker Hub" \
            "🔨 Build Image from Local Dockerfile" \
            "🗑️ Remove Selected Image" \
            "🔙 Back")

        switch $img_action
            "📋 List & Inspect Local Images")
    set -l img_id
                set img_id (docker images --format "table {{.Repository}}\t{{.Tag}}\t{{.ID}}\t{{.Size}}" 2>/dev/null | \
                    fzf --header-lines=1 --prompt="Select Image ❯ " | awk '{print $argv[3]}')
                [ -n "$img_id" ] && docker inspect "$img_id" | fzf --header="Image Inspect"
            "📥 Pull Image from Docker Hub")
    set -l image_name
                image_name=(gum input --placeholder "e.g. nginx:latest, postgres:alpine")
                if test -n "$image_name"
                    gum spin --spinner globe --title "Pulling $image_name..." -- docker pull "$image_name"
                    gum style --foreground 46 "✔ Image pulled successfully!"
                    sleep 1.5
                end
            "🔨 Build Image from Local Dockerfile")
                if test ! -f "Dockerfile" ] && [ ! -f "dockerfile"
                    gum style --foreground 196 "❌ No Dockerfile found in current directory!"
                    sleep 2
                    continue
                end
    set -l tag_name
                set tag_name (gum input --placeholder "Enter Tag Name (e.g. my-app:v1)")
                if test -n "$tag_name"
                    docker build -t "$tag_name" .
                    printf "\nPress Enter to continue..."
                    read -r
                end
            "🗑️ Remove Selected Image")
    set -l img_id
                set img_id (docker images --format "table {{.Repository}}\t{{.Tag}}\t{{.ID}}\t{{.Size}}" 2>/dev/null | \
                    fzf --header-lines=1 --prompt="Select Image to Delete ❯ " | awk '{print $argv[3]}')
                if test -n "$img_id"
                    if gum confirm "Delete image $img_id?"
                        docker rmi "$img_id"
                        sleep 1
                    end
                end
            "🔙 Back"|*) break ;;
        end
    end
end

# Volume Management TUI
functions -e _manage_volumes 2>/dev/null
function _manage_volumes
    while true; do
        clear
        if ! docker info >/dev/null 2>&1
            gum style --foreground 196 --bold "❌ Docker daemon is not running! Start it first."
            sleep 2
            return 1
        end

        gum style --foreground 214 --bold "=== DOCKER VOLUMES MODULE ==="
    set -l vol_action
        set vol_action (gum choose \
            "📋 List Volumes" \
            "➕ Create New Volume" \
            "🔍 Inspect Volume" \
            "🗑️ Remove Volume" \
            "🔙 Back")

        switch $vol_action
            "📋 List Volumes")
                docker volume ls | fzf --header-lines=1 --prompt="Volumes ❯ "
            "➕ Create New Volume")
    set -l vol_name
                vol_name=(gum input --placeholder "Enter Volume Name")
                [ -n "$vol_name" ] && docker volume create "$vol_name" && sleep 1
            "🔍 Inspect Volume")
    set -l vol_id
                vol_id=(docker volume ls -q | fzf --prompt="Select Volume ❯ ")
                [ -n "$vol_id" ] && docker volume inspect "$vol_id" | fzf
            "🗑️ Remove Volume")
    set -l vol_id
                vol_id=(docker volume ls -q | fzf --prompt="Select Volume to Remove ❯ ")
                if test -n "$vol_id"
                    if gum confirm "Remove Volume $vol_id?"
                        docker volume rm "$vol_id"
                        sleep 1
                    end
                end
            "🔙 Back"|*) break ;;
        end
    end
end

# Network Management TUI
functions -e _manage_networks 2>/dev/null
function _manage_networks
    while true; do
        clear
        if ! docker info >/dev/null 2>&1
            gum style --foreground 196 --bold "❌ Docker daemon is not running! Start it first."
            sleep 2
            return 1
        end

        gum style --foreground 120 --bold "=== DOCKER NETWORKS MODULE ==="
    set -l net_action
        set net_action (gum choose \
            "📋 List Networks" \
            "🔍 Inspect Network" \
            "🗑️ Remove Unused Networks" \
            "🔙 Back")

        switch $net_action
            "📋 List Networks")
                docker network ls | fzf --header-lines=1
            "🔍 Inspect Network")
    set -l net_id
                set net_id (docker network ls --format "table {{.ID}}\t{{.Name}}\t{{.Driver}}" 2>/dev/null | \
                    fzf --header-lines=1 | awk '{print $argv[1]}')
                [ -n "$net_id" ] && docker network inspect "$net_id" | fzf
            "🗑️ Remove Unused Networks")
                if gum confirm "Prune unused networks?"
                    docker network prune -f
                    sleep 1
                end
            "🔙 Back"|*) break ;;
        end
    end
end

# Docker Compose Management TUI
functions -e _manage_compose 2>/dev/null
function _manage_compose
    clear
    if test ! -f "docker-compose.yml" ] && [ ! -f "compose.yaml" ] && [ ! -f "docker-compose.yaml" ] && [ ! -f "compose.yml"
        gum style --foreground 196 "❌ No docker-compose file found in current directory!"
        sleep 2
        return
    end

    if ! docker info >/dev/null 2>&1
        gum style --foreground 196 --bold "❌ Docker daemon is not running! Start it first."
        sleep 2
        return 1
    end

    gum style --foreground 208 --bold "=== DOCKER COMPOSE MODULE ==="
    set -l compose_action
    set compose_action (gum choose \
        "🚀 Compose Up (-d)" \
        "🛑 Compose Down" \
        "🔄 Compose Restart" \
        "📋 Compose Live Logs" \
        "🔙 Back")

    switch $compose_action
        "🚀 Compose Up (-d)")
            docker compose up -d 2>/dev/null || docker-compose up -d
            sleep 2
        "🛑 Compose Down")
            docker compose down 2>/dev/null || docker-compose down
            sleep 2
        "🔄 Compose Restart")
            docker compose restart 2>/dev/null || docker-compose restart
            sleep 2
        "📋 Compose Live Logs")
            docker compose logs -f 2>/dev/null || docker-compose logs -f
        case *) return ;;
    end
end

# Docker Context Switcher
functions -e _switch_docker_context 2>/dev/null
function _switch_docker_context
    clear
    gum style --foreground 212 --bold "=== DOCKER CONTEXT SWITCHER ==="
    set -l context
    context=(docker context ls --format "table {{.Name}}\t{{.Endpoint}}" 2>/dev/null | fzf --header-lines=1 --prompt="Select Context ❯ " | awk '{print $argv[1]}')
    if test -n "$context"
        if docker context use "$context"
            gum style --foreground 46 "✔ Switched to Context: $context"
        else
            gum style --foreground 196 "❌ Failed to switch context to: $context"
        end
        sleep 1.5
    end
end

# Docker Events Stream
functions -e _view_docker_events 2>/dev/null
function _view_docker_events
    clear
    if ! docker info >/dev/null 2>&1
        gum style --foreground 196 --bold "❌ Docker daemon is not running! Start it first."
        sleep 2
        return 1
    end
    gum style --foreground 39 "Press Ctrl+C to exit Docker Events Stream..."
    docker events --format 'Type: {{.Type}} | Action: {{.Action}} | Actor: {{.Actor.Attributes.name}} ({{.Time}})'
end

# Realtime Resource Monitor
functions -e dstats 2>/dev/null
function dstats
    clear
    if ! docker info >/dev/null 2>&1
        echo -e "\e[1;31m❌ Docker daemon is not running! Start it first with 'dstart'.\e[0m"
        sleep 2
        return 1
    end
    if command -v gum >/dev/null 2>&1
        gum style --foreground 39 "Press Ctrl+C to exit Resource Dashboard..."
    else
        echo -e "\e[1;36mPress Ctrl+C to exit Resource Dashboard...\e[0m"
    end
    watch -n 1 -c "docker stats --format 'table {{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.MemPerc}}\t{{.NetIO}}\t{{.BlockIO}}\t{{.PIDs}}'"
end

# আলটিমেট সিস্টেম ক্লিনআপ (অব্যবহৃত ক্যাশ, কন্টেইনার, ভলিউম ও ইমেজ ডিলিট করে জিবি জিবি জায়গা খালি করা)
functions -e dclean 2>/dev/null
function dclean
    clear
    if ! docker info >/dev/null 2>&1
        echo -e "\e[1;31m❌ Docker daemon is not running! Start it first with 'dstart'.\e[0m"
        sleep 2
        return 1
    end
    if command -v gum >/dev/null 2>&1
        if gum confirm "Warning: This will delete ALL stopped containers, unused images, and dangling volumes!"
            gum spin --spinner monkey --title "Pruning Docker System..." -- docker system prune -a --volumes -f
            gum style --foreground 46 "✔ Full Cleanup Complete!"
            sleep 1.5
        end
    else
        echo -e "\e[1;31m⚠️ Warning: This will delete ALL stopped containers, unused images, and dangling volumes!\e[0m"
        printf "Are you sure? (y/N): "
        read -r confirm
        if test "$confirm" =~ ^[Yy]$
            echo -e "\e[1;31m🧹 Performing deep clean of all unused Docker resources...\e[0m"
            docker system prune -a --volumes -f
            echo -e "\e[1;32m✨ Full Cleanup Complete!\e[0m"
            sleep 1.5
        else
            echo "Operation cancelled."
        end
    end
end

# --------------------------------------------------------------------
# ৭. স্মার্ট ট্যাব কমপ্লিশন এবং নোটিফায়ার (Tab Completion & Notifier)
# --------------------------------------------------------------------

# শর্টকাট কমান্ডগুলোর জন্য কন্টেইনারের নাম অটো-কম্প্লিট (Tab) করা
functions -e _docker_containers_completion 2>/dev/null
function _docker_containers_completion
    set -l curr_arg ${COMP_WORDS[COMP_CWORD]}
    set -l actions (docker ps -a --format "{{.Names}}")
    COMPREPLY=((compgen -W "$actions" -- "$curr_arg"))
end
complete -F _docker_containers_completion dsh dlogs dstop dkill drestart dports dwatch dnetstat dtop-proc

# শর্টকাট কমান্ডগুলোর জন্য ইমেজের নাম অটো-কম্প্লিট করা
functions -e _docker_images_completion 2>/dev/null
function _docker_images_completion
    set -l curr_arg ${COMP_WORDS[COMP_CWORD]}
    set -l images (docker images --format "{{.Repository}}")
    COMPREPLY=((compgen -W "$images" -- "$curr_arg"))
end
complete -F _docker_images_completion drmi dhist

# টার্মিনাল ওপেন করলেই ব্যাকগ্রাউন্ডে কয়টি কন্টেইনার চলছে তা মনে করিয়ে দেওয়া
if command -v docker >/dev/null 2>&1 && systemctl is-active --quiet docker
    running_count=(docker ps -q | wc -l)
    if test "$running_count" -gt 0
        echo -e "\e[1;36m🐳 Docker is active. Running containers: $running_count\e[0m"
    end
end

# ======================================================
# Advance C/C++ boilerplate generator
# ======================================================
functions -e makecpp 2>/dev/null
function makecpp
    if test -z "$argv[1]"
        echo "❌ Error: Please provide a project name! (e.g., makecpp my_project)"
        return 1
    end

    set -l lang_choice "$argv[2]"
    if test -z "$lang_choice"
        if command -v fzf >/dev/null 2>&1
            echo "🤔 Which language project do you want to create?"
            lang_choice=(printf "cpp\nc\n" | fzf --prompt="Select Language > " --height=10 --layout=reverse)
            [ -z "$lang_choice" ] && lang_choice="cpp"
        else
            printf "🤔 Which language project do you want to create? (c/cpp) [default: cpp]: "
            read lang_choice
        end
    end

    set -l type "cpp"
    set -l compiler "g++"
    set -l file_ext "cpp"
    set -l flags "-std=c++17 -Wall -Wextra -O2"

    if test "$lang_choice" = "c" ] || [ "$lang_choice" = "C"
        type="c"
        compiler="gcc"
        file_ext="c"
        flags="-Wall -Wextra -O2"
    end

    echo "🚀 Creating Advance $type project: $argv[1]..."
    mkdir -p "$argv[1]" && cd "$argv[1]" || return

    # 1. Generate general boilerplate code
    if test "$type" = "c"
        cat <<EOF >main.c
#include <stdio.h>

int main() {
    printf("Hello, World! Welcome to C project: %s\n", "$argv[1]");
    return 0;
end
EOF
    else
        cat <<EOF >main.cpp
#include <iostream>

int main() {
    std::cout << "Hello, World! Welcome to C++ project: " << "$argv[1]" << std::endl;
    return 0;
end
EOF
    end

    # 2. Create a smart Makefile
    cat <<EOF >Makefile
CC = $compiler
CFLAGS = $flags
TARGET = main

all: \(TARGET)

\(TARGET): main.$file_ext
	\(CC) \(CFLAGS) main.$file_ext -o \(TARGET)

run: \(TARGET)
	./\(TARGET)

clean:
	rm -f \(TARGET)
EOF

    # 3. Auto-initialize Git and create .gitignore
    if command -v git >/dev/null 2>&1
        git init -q
        echo -e "main\n*.o\n*.out\n.vscode/" >.gitignore
        echo "✅ Git repository initialized with .gitignore"
    end

    echo "🎉 Project setup complete!"
    echo "📂 Current directory: (pwd)"

    # 4. Open in VS Code automatically if available
    if command -v code >/dev/null 2>&1
        echo "💻 Opening in VS Code..."
        code .
    end
end

functions -e t 2>/dev/null
function t
    if test $# -eq 0
        echo "❌ Provide at least one filename."
        return 1
    end
    touch "$@"
    for file in "$@"; do
        echo "✅ Created File: $file"
    end
end

# =====================================================
# 🚀 INTERACTIVE GUM & FZF UTILITIES
# =====================================================

# 1. Interactive Git Branch Switcher (FZF / Gum)

# 2. Interactive Git Branch Switcher (FZF / Gum)
function gbranch
    if ! command -v git >/dev/null 2>&1
        echo "❌ Git is not installed."
        return 1
    end

    if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1
        echo "❌ Not inside a git repository."
        return 1
    end

    set -l REFS_PATTERN "refs/heads" "refs/remotes"
    set -l INCLUDE_ALL 1

    for arg in $argv
        switch $arg
            case -l --local
                set REFS_PATTERN "refs/heads"
                set INCLUDE_ALL 0
            case -r --remote
                set REFS_PATTERN "refs/remotes"
                set INCLUDE_ALL 0
            case -a --all
                set REFS_PATTERN "refs/heads" "refs/remotes"
                set INCLUDE_ALL 1
            case -h --help
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
            case '*'
                echo "❌ Unknown option: $arg (use -h for help)"
                return 1
        end
    end

    if command -v fzf >/dev/null 2>&1
        set -l OUT (git for-each-ref --sort=-committerdate $REFS_PATTERN --format='%(refname:short) (%(committerdate:relative)) - %(subject)' 2>/dev/null | \
            grep -v -E '(^origin |/HEAD)' | \
            fzf --prompt="🌿 Branch > " \
                --header="[Enter] Checkout | [Ctrl-D] Delete | [Ctrl-R] Rebase | [Ctrl-O] Merge" \
                --expect=ctrl-d,ctrl-r,ctrl-o \
                --preview='git log --graph --color=always --format="%C(auto)%h %C(blue)%ad %C(yellow)%an %C(reset)%s" --date=relative -n 15 {1}' \
                --preview-window=right:55%:wrap)

        if test (count $OUT) -lt 2
            return 0
        end

        set -l KEY $OUT[1]
        set -l RAW_LINE $OUT[2..-1]
        if test -z "$RAW_LINE"
            return 0
        end

        set -l BRANCH (echo "$RAW_LINE" | awk '{print $1}')
        set -l CLEAN_BRANCH (echo "$BRANCH" | sed -e 's#^remotes/origin/##' -e 's#^origin/##')

        switch $KEY
            case ctrl-d
                read -l -P "⚠️ Delete branch '$CLEAN_BRANCH'? [y/N] " confirm
                if string match -ri '^[yY]$' -- "$confirm"
                    git branch -D "$CLEAN_BRANCH"
                else
                    echo "Cancelled."
                end
            case ctrl-r
                echo "🔄 Rebasing onto '$CLEAN_BRANCH'..."
                git rebase "$CLEAN_BRANCH"
            case ctrl-o
                echo "🔀 Merging '$CLEAN_BRANCH'..."
                git merge "$CLEAN_BRANCH"
            case '*'
                echo "🚀 Checking out '$CLEAN_BRANCH'..."
                git checkout "$CLEAN_BRANCH"
        end
    else if command -v gum >/dev/null 2>&1
        set -l BRANCH (git for-each-ref --sort=-committerdate $REFS_PATTERN --format='%(refname:short)' 2>/dev/null | grep -v -E '(^origin |/HEAD)' | gum filter --height 10 --placeholder="Select branch...")
        if test -n "$BRANCH"
            set -l CLEAN_BRANCH (echo "$BRANCH" | sed -e 's#^remotes/origin/##' -e 's#^origin/##')
            git checkout "$CLEAN_BRANCH"
        end
    else
        echo "🌿 Select a Git branch:"
        set -l BRANCHES (git for-each-ref --sort=-committerdate $REFS_PATTERN --format='%(refname:short)' 2>/dev/null | grep -v -E '(^origin |/HEAD)')
        if test (count $BRANCHES) -eq 0
            echo "❌ No branches found."
            return 1
        end

        for i in (seq (count $BRANCHES))
            echo "  [$i] $BRANCHES[$i]"
        end

        read -l -P "Select branch number (1-"(count $BRANCHES)"): " choice
        if string match -r '^\d+$' -- "$choice"; and test "$choice" -ge 1; and test "$choice" -le (count $BRANCHES)
            set -l SELECTED $BRANCHES[$choice]
            set -l CLEAN_BRANCH (echo "$SELECTED" | sed -e 's#^remotes/origin/##' -e 's#^origin/##')
            git checkout "$CLEAN_BRANCH"
        end
    end
end

# 3. Advanced Interactive Process Killer (FZF / Gum)
function fkill
    set -l QUERY (string join '' $argv)

    # Port-based search: fkill 3000 -> find & kill process on port 3000
    if string match -rq '^[0-9]+$' -- "$QUERY"; and test "$QUERY" -le 65535 2>/dev/null
        set -l PORT_PID (ss -tlnp 2>/dev/null | awk -v p=":$QUERY" '$0~p{match($0,/pid=([0-9]+)/,a);if(a[1])print a[1]}' | head -n1)
        if test -z "$PORT_PID"
            set PORT_PID (lsof -t -i ":$QUERY" 2>/dev/null | head -n1)
        end
        if test -n "$PORT_PID"
            set -l PROC_NAME (ps -p "$PORT_PID" -o comm= 2>/dev/null)
            read -l -P "⚡ Kill '$PROC_NAME' (PID $PORT_PID) on port $QUERY? [y/N] " confirm
            if string match -riq '^[yY]$' -- "$confirm"
                kill -15 "$PORT_PID" 2>/dev/null; and echo "✅ Soft killed PID $PORT_PID ($PROC_NAME)"; and return 0
                sleep 1
                kill -9 "$PORT_PID" 2>/dev/null; and echo "🔥 Force killed PID $PORT_PID ($PROC_NAME)"
            end
            return 0
        else
            echo "❌ No process found on port $QUERY"
            return 1
        end
    end

    if command -v fzf >/dev/null 2>&1
        set -l PS_CMD 'ps -eo pid,user,%cpu,%mem,etime,comm 2>/dev/null | sed 1d | sort -k3 -rn | awk '\''{
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
        set -l OUT (eval $PS_CMD | \
            fzf -m --query="$QUERY" \
                --prompt="⚡ Kill > " \
                --header="[Tab] Multi-Select | [Enter] Soft Kill (SIGTERM) | [Ctrl-X] Force Kill (-9) | [Ctrl-R] Reload
Types: [APP] GUI App (Chrome/Code/etc) | [SYS] System Process | [USR] User CLI/Script" \
                --expect=ctrl-x,ctrl-r \
                --bind="ctrl-r:reload($PS_CMD)" \
                --preview='echo "Type:    {2}"; echo "PID:     {1}"; echo "User:    {3}"; echo "CPU:     {4}%"; echo "Mem:     {5}%"; echo "Uptime:  {6}"; echo "Command: {7}"' \
                --preview-window=bottom:30%:wrap)

        if test (count $OUT) -lt 2
            return 0
        end

        set -l KEY $OUT[1]
        if test "$KEY" = ctrl-r
            return 0
        end

        set -l PIDS (printf '%s\n' $OUT[2..-1] | awk '{print $1}')
        if test (count $PIDS) -eq 0
            return 0
        end

        set -l PID_LIST (string join ' ' $PIDS)

        if test "$KEY" = ctrl-x
            echo $PIDS | xargs kill -9 2>/dev/null
            echo "🔥 Force killed (SIGKILL) PID(s): $PID_LIST"
        else
            echo $PIDS | xargs kill -15 2>/dev/null
            echo "✅ Soft killed (SIGTERM) PID(s): $PID_LIST"
        end

    else if command -v gum >/dev/null 2>&1
        set -l SELECTED (ps -eo pid,user,%cpu,%mem,comm 2>/dev/null | sed 1d | sort -k3 -rn | \
            gum filter --height 12 --placeholder="Search process...")
        if test -n "$SELECTED"
            set -l PID (echo "$SELECTED" | awk '{print $1}')
            set -l NAME (echo "$SELECTED" | awk '{print $5}')
            if gum confirm "Kill '$NAME' (PID $PID)?"
                kill -15 "$PID" 2>/dev/null; and gum style --foreground 82 "✅ Soft killed '$NAME' (PID $PID)"
            end
        end
    else
        echo "⚡ Running processes (sorted by CPU):"
        set -l PROCS (ps -eo pid,user,%cpu,%mem,comm 2>/dev/null | sed 1d | sort -k3 -rn | head -n 20)
        if test (count $PROCS) -eq 0
            echo "❌ No processes found."
            return 1
        end

        for i in (seq (count $PROCS))
            printf "  [%2d] %s\n" $i "$PROCS[$i]"
        end

        read -l -P "Select process to kill (1-"(count $PROCS)"): " choice
        if string match -rq '^[0-9]+$' -- "$choice"; and test "$choice" -ge 1; and test "$choice" -le (count $PROCS)
            set -l SELECTED $PROCS[$choice]
            set -l PID (echo "$SELECTED" | awk '{print $1}')
            set -l NAME (echo "$SELECTED" | awk '{print $5}')
            read -l -P "Kill '$NAME' (PID $PID)? [y/N] " ans
            if string match -riq '^[yY]$' -- "$ans"
                kill -15 "$PID" 2>/dev/null; and echo "✅ Soft killed '$NAME' (PID $PID)"
            end
        end
    end
end

# 4. Interactive Quick Directory Search & CD
function fcd
    set -l DIR ""
    if command -v fzf >/dev/null 2>&1
        DIR=(find . -maxdepth 4 -not -path '*/.*' -type d 2>/dev/null | fzf --prompt="Select Directory: ")
    else if command -v gum >/dev/null 2>&1
        DIR=(find . -maxdepth 4 -not -path '*/.*' -type d 2>/dev/null | gum filter --height 5 --placeholder="Select Directory...")
    end

    if test -n "$DIR"
        cd "$DIR" || return
    end
end

# =====================================================
# 📋 TODO & NOTES UTILITIES (Fish | Linux/macOS/BSD)
# Cross-shell safe: bash 4+ and zsh 5+
# All bugs fixed — see ARCHITECTURE.txt for details
# =====================================================

# ---------------------------------------------------------------------------
# Helper: Portable in-place line deletion.
# GNU sed (Linux) uses `sed -i "Nd"`.
# BSD sed (macOS/FreeBSD) requires `sed -i '' "Nd"`.
# Detection: GNU sed responds to --version; BSD sed does not.
# ---------------------------------------------------------------------------
function _fb_sed_delete_line
    set -l n "$argv[1]" file="$argv[2]"
    if sed --version >/dev/null 2>&1
        sed -i "${n}d" "$file"
    else
        sed -i '' "${n}d" "$file"
    end
end

# ---------------------------------------------------------------------------
# Helper: Clipboard copy — Wayland → X11/xclip → X11/xsel → macOS pbcopy.
# Falls back gracefully with an informative error message.
# ---------------------------------------------------------------------------
function _fb_copy_to_clipboard
    set -l content "$argv[1]"

    # Try Wayland first if WAYLAND_DISPLAY is active
    if [ -n "${WAYLAND_DISPLAY:-}" ] && command -v wl-copy >/dev/null 2>&1
        if printf '%s' "$content" | wl-copy 2>/dev/null
            printf '📋 Copied to Wayland clipboard!\n'
            return 0
        end
    end

    # Try X11 xclip
    if command -v xclip >/dev/null 2>&1
        if printf '%s' "$content" | xclip -selection clipboard 2>/dev/null
            printf '📋 Copied to X11 clipboard!\n'
            return 0
        end
    end

    # Try X11 xsel
    if command -v xsel >/dev/null 2>&1
        if printf '%s' "$content" | xsel --clipboard --input 2>/dev/null
            printf '📋 Copied to X11 clipboard!\n'
            return 0
        end
    end

    # Try macOS pbcopy
    if command -v pbcopy >/dev/null 2>&1
        if printf '%s' "$content" | pbcopy 2>/dev/null
            printf '📋 Copied to macOS clipboard!\n'
            return 0
        end
    end

    # Fallback attempt wl-copy if WAYLAND_DISPLAY wasn't exported but server exists
    if command -v wl-copy >/dev/null 2>&1
        if printf '%s' "$content" | wl-copy 2>/dev/null
            printf '📋 Copied to Wayland clipboard!\n'
            return 0
        end
    end

    printf '❌ Clipboard copy failed (no working display or clipboard tool found).\n'
    return 1
end

# ---------------------------------------------------------------------------
# Helper: Print numbered task list. Shared across todo sub-commands.
# ---------------------------------------------------------------------------
function _fb_todo_show_list
    set -l file "$argv[1]"
    if test -s "$file"
        printf '\n--- 📋 YOUR TO-DO LIST ---\n'
        nl -w2 -s'. ' "$file"
        printf '\n'
    else
        printf '📋 No tasks pending! Super productive 🎉\n'
    end
end

# ------------------------------------------------------------------------------
# 1. TODO MANAGER
# Usage: todo | todo add "Task" | todo list | todo done [num] | todo clear
# Deps:  none required — fzf/gum are optional for interactive mode
# Fixes: out-of-range number crash | sed regex injection | echo -e portability
#        local+cmd exit code loss | inconsistent tool detection
# ------------------------------------------------------------------------------
function todo
    set -l TODO_FILE "$HOME/.todo_list.txt"
    touch "$TODO_FILE"

    set -l action "${1:-}"

    switch $action

        case add
            shift
    set -l task "${*:-}"
            # Prompt interactively when task not passed as argument
            if test -z "$task"
                if command -v gum >/dev/null 2>&1
                    task=(gum input --placeholder "Type your task here...")
                else
                    printf 'Task: '
                    read -r task
                end
            end
            if test -n "$task"
                printf '%s\n' "$task" >> "$TODO_FILE"
                printf '✔ Added: "%s"\n' "$task"
            else
                printf '❌ Task cannot be empty!\n'
            end

        case done|rm
            shift
    set -l num "${1:-}"

            if test ! -s "$TODO_FILE"
                printf '📋 No tasks to complete!\n'
                return 0
            end

    set -l total
            total=(wc -l < "$TODO_FILE")

            # ── Branch 1: direct line number provided ──────────────────────
            if test "$num" =~ ^[0-9]+$
                # BUG FIX: validate range — prevents sed crash on line 0
                # or on a number larger than the file (both silent data-loss bugs)
                if (( num < 1 || num > total ))
                    printf '❌ Invalid task number! Valid range: 1–%d\n' "$total"
                    return 1
                end
                # BUG FIX: separate local declaration from command substitution
                # so that a failed `sed` is not swallowed by `local`'s exit-0
    set -l task_text
                task_text=(sed -n "${num}p" "$TODO_FILE")
                _fb_sed_delete_line "$num" "$TODO_FILE"
                printf '🎉 Completed: "%s"\n' "$task_text"
                return 0
            end

            # ── Branch 2: fzf interactive selection ────────────────────────
            if command -v fzf >/dev/null 2>&1
    set -l selected
                set selected (cat -n "$TODO_FILE" | fzf \
                    --prompt="Select task to complete ➔ " \
                    --height=40% --reverse --border)
                if test -n "$selected"
                    set -l line_num task_text
                    line_num=(printf '%s' "$selected" | awk '{print $argv[1]}')
                    # awk strips leading whitespace+number cleanly — no sed regex needed
                    set task_text (printf '%s' "$selected" | awk '{$argv[1]=""; sub(/^[[:space:]]+/,""); print}')
                    _fb_sed_delete_line "$line_num" "$TODO_FILE"
                    printf '🎉 Completed: "%s"\n' "$task_text"
                end

            # ── Branch 3: gum interactive selection ────────────────────────
            else if command -v gum >/dev/null 2>&1
    set -l task_to_remove
                task_to_remove=(gum choose --header="Select task to mark as Done:" < "$TODO_FILE")
                if test -n "$task_to_remove"
                    # BUG FIX: grep -F (fixed-string) — immune to regex injection.
                    # Original code used sed with user data as a regex pattern,
                    # which crashed on tasks containing / & . * ^ $ [ etc.
    set -l match_line
                    set match_line (grep -Fn "$task_to_remove" "$TODO_FILE" 2>/dev/null \
                        | head -1 | cut -d: -f1)
                    if test -n "$match_line"
                        _fb_sed_delete_line "$match_line" "$TODO_FILE"
                        printf '🎉 Completed: "%s"\n' "$task_to_remove"
                    else
                        printf '❌ Could not locate the task in file!\n'
                    end
                end

            # ── Branch 4: no interactive tool — show numbered list ──────────
            else
                printf '❌ Pass a task number (e.g. todo done 1) or install fzf/gum!\n'
                _fb_todo_show_list "$TODO_FILE"
            end

        case clear
            > "$TODO_FILE"
            printf '🗑️  All tasks cleared!\n'

        case list|ls
            _fb_todo_show_list "$TODO_FILE"

        case -h|--help
            printf 'Usage:\n'
            printf '  todo              – Open interactive menu / view tasks\n'
            printf '  todo add <task>   – Add a new task\n'
            printf '  todo list         – List all tasks\n'
            printf '  todo done [num]   – Complete a task (interactive if no num)\n'
            printf '  todo clear        – Clear all tasks\n'

        case ""
            # Full gum menu when available; plain numbered list otherwise
            if command -v gum >/dev/null 2>&1
                gum style \
                    --foreground 212 --border normal \
                    --margin "1" --padding "1" \
                    "✨ FANCYBASH TO-DO MANAGER ✨"
    set -l MENU_CHOICE
                set MENU_CHOICE (gum choose \
                    "➕ Add Task" "✅ Complete Task" \
                    "📋 View Tasks" "🗑️  Clear All" "❌ Exit")
                switch $MENU_CHOICE
                    "➕ Add Task")      todo add ;;
                    "✅ Complete Task") todo done ;;
                    "📋 View Tasks")    _fb_todo_show_list "$TODO_FILE" ;;
                    "🗑️  Clear All")
                        if gum confirm "Are you sure you want to clear all tasks?"
                            todo clear
                        end
                    case *) return 0 ;;
                end
            else
                _fb_todo_show_list "$TODO_FILE"
            end
            ;;

        *)
            printf '❌ Unknown command: %s\n' "$action"
            printf "Run 'todo --help' for usage.\n"
            return 1
            ;;
    end
end

# ------------------------------------------------------------------------------
# 2. NOTES MANAGER
# Usage: notes | notes add | notes search | notes --help
# Deps:  none required — fzf/gum/bat/glow are optional
# Fixes: no fallback in notes add | fzf crash when missing | bat preview broken
#        grep regex injection in search | echo -e portability | ls -d error
# ------------------------------------------------------------------------------
function _fb_get_note_body
    set -l file "$argv[1]"
    if grep -q "^---*" "$file" 2>/dev/null
        awk 'flag{print} /^---*$/ && !flag{flag=1}' "$file" | sed '1{/^$/d}'
    else
        cat "$file"
    end
end

function _fb_has_vscode
    command -v code >/dev/null 2>&1; or command -v codium >/dev/null 2>&1; or begin; command -v flatpak >/dev/null 2>&1; and flatpak info com.visualstudio.code >/dev/null 2>&1; end
end

function _fb_open_vscode
    set -l target "$argv[1]"
    if command -v code >/dev/null 2>&1
        code "$target"
    else if command -v codium >/dev/null 2>&1
        codium "$target"
    else if command -v flatpak >/dev/null 2>&1; and flatpak info com.visualstudio.code >/dev/null 2>&1
        flatpak run com.visualstudio.code "$target"
    end
end

function notes
    set -l NOTE_DIR "$HOME/.my_notes"
    mkdir -p "$NOTE_DIR/General"

    set -l action "${1:-}"

    # Pick the best available markdown/syntax previewer for fzf --preview
    set -l PREVIEW_CMD
    if command -v bat >/dev/null 2>&1
        PREVIEW_CMD="bat --color=always --style=numbers,changes"
    else if command -v batcat >/dev/null 2>&1
        PREVIEW_CMD="batcat --color=always --style=numbers,changes"
    else if command -v glow >/dev/null 2>&1
        PREVIEW_CMD="glow -s dark"
    else
        PREVIEW_CMD="cat"
    end

    switch $action

        case add
    set -l category ""

            # ── Category selection: fzf → gum → plain read ─────────────────
            # BUG FIX: original used `ls -d */` which errors if no dirs exist;
            # replaced with `find -mindepth 1 -maxdepth 1 -type d`.
            # BUG FIX: `notes add` had zero fallback when gum was absent.
            if command -v fzf >/dev/null 2>&1
    set -l cats
                set cats (find "$NOTE_DIR" -mindepth 1 -maxdepth 1 -type d \
                    -exec basename {} \; 2>/dev/null)
                set category (printf '➕ Create New Category\n%s\n' "$cats" | \
                    grep -v '^$' | \
                    fzf --prompt="📁 Select Category ➔ " \
                        --height=40% --border)

            else if command -v gum >/dev/null 2>&1
    set -l cats
                set cats (find "$NOTE_DIR" -mindepth 1 -maxdepth 1 -type d \
                    -exec basename {} \; 2>/dev/null)
                set category (printf '➕ Create New Category\n%s\n' "$cats" | \
                    grep -v '^$' | \
                    gum choose --header="📁 Select Category:")

            else
                printf 'Available categories:\n'
                find "$NOTE_DIR" -mindepth 1 -maxdepth 1 -type d \
                    -exec basename {} \; 2>/dev/null | nl -w2 -s'. '
                printf 'Category name (Enter = General): '
                read -r category
                [ -z "$category" ] && category="General"
            end

            [ -z "$category" ] && return 0

            if test "$category" = "➕ Create New Category"
                if command -v gum >/dev/null 2>&1
                    category=(gum input --placeholder "New category name...")
                else
                    printf 'New category name: '
                    read -r category
                end
                [ -z "$category" ] && return 0
                mkdir -p "$NOTE_DIR/$category"
            end

            # ── Note title ──────────────────────────────────────────────────
    set -l title ""
            if command -v gum >/dev/null 2>&1
                set title (gum input --placeholder "Note Title (e.g. Docker Commands)...")
            else
                printf 'Note title: '
                read -r title
            end
            [ -z "$title" ] && return 0

    set -l file_path "$NOTE_DIR/$category/$title.txt"
            mkdir -p "$NOTE_DIR/$category"

            # ── Overwrite check ─────────────────────────────────────────────
            if test -f "$file_path"
    set -l overwrite "n"
                if command -v gum >/dev/null 2>&1
                    gum confirm "Note already exists! Overwrite?" && overwrite="y"
                else
                    printf 'Note already exists! Overwrite? [y/N]: '
                    read -r overwrite
                end
                test "$overwrite" != "y" && "$overwrite" != "Y" && return 0
            end

            # Write plain-text header (no markdown — anyone can read it)
            printf '%s\n' "$title" > "$file_path"
            printf 'Created: %s\n%s\n\n' "(date '+%Y-%m-%d %H:%M:%S')" "(printf -- '-%.0s' {1..40})" >> "$file_path"

            # ── Content input: gum write → $EDITOR / VS Code ─────────────
            if command -v gum >/dev/null 2>&1
                set -l mode
                if _fb_has_vscode
                    set mode (gum choose \
                        "📥 Quick Input via Gum" \
                        "💻 Open in VS Code" \
                        "📝 Use CLI Editor ((if test -n "$EDITOR"; echo "$EDITOR"; else; echo "nano"; end))")
                else
                    set mode (gum choose \
                        "📥 Quick Input via Gum" \
                        "📝 Use CLI Editor ((if test -n "$EDITOR"; echo "$EDITOR"; else; echo "nano"; end))")
                end

                if test "$mode" == *"Quick Input"*
                    set -l content
                    set content (gum write --placeholder "Type your note here... (Ctrl+D to save)")
                    printf '%s\n' "$content" >> "$file_path"
                else if test "$mode" == *"VS Code"*
                    _fb_open_vscode "$file_path"
                else
                    (if test -n "$EDITOR"; echo "$EDITOR"; else; echo "nano"; end) "$file_path"
                end
            else
                if _fb_has_vscode
                    printf 'Select editor mode:\n'
                    printf '  1. 📥 Quick Input via Terminal\n'
                    printf '  2. 💻 VS Code\n'
                    printf '  3. 📝 CLI Editor (%s)\n' "(if test -n "$EDITOR"; echo "$EDITOR"; else; echo "nano"; end)"
                    read -l -P "Choose [1-3] (default: 1): " choice < /dev/tty
                    switch $choice
                        case 2
                            _fb_open_vscode "$file_path"
                        case 3
                            (if test -n "$EDITOR"; echo "$EDITOR"; else; echo "nano"; end) "$file_path"
                        case '*'
                            printf 'Enter note content (Ctrl+D to finish):\n'
                            cat >> "$file_path"
                    end
                else
                    printf 'Select editor mode:\n'
                    printf '  1. 📥 Quick Input via Terminal\n'
                    printf '  2. 📝 CLI Editor (%s)\n' "(if test -n "$EDITOR"; echo "$EDITOR"; else; echo "nano"; end)"
                    read -l -P "Choose [1-2] (default: 1): " choice < /dev/tty
                    switch $choice
                        case 2
                            (if test -n "$EDITOR"; echo "$EDITOR"; else; echo "nano"; end) "$file_path"
                        case '*'
                            printf 'Enter note content (Ctrl+D to finish):\n'
                            cat >> "$file_path"
                    end
                end
            end

            printf '✔ Saved to [%s/%s.txt]\n' "$category" "$title"

        case search|find
    set -l query ""
            if command -v gum >/dev/null 2>&1
                query=(gum input --placeholder "Type text to search inside notes...")
            else
                printf 'Search query: '
                read -r query
            end
            [ -z "$query" ] && return 0

            # ── fzf-less fallback: plain grep ───────────────────────────────
            if ! command -v fzf >/dev/null 2>&1
                printf '⚠️  fzf not found — showing raw grep results:\n\n'
                # BUG FIX: grep -F (fixed-string) prevents query being
                # treated as a regex — avoids injection if query has . * + etc.
                grep -rnF "$query" "$NOTE_DIR" 2>/dev/null
                return 0
            end

            set -l match_line
            set match_line (grep -rnF "$query" "$NOTE_DIR" 2>/dev/null | \
                while IFS= read -r line
                    set -l f (printf '%s' "$line" | cut -d: -f1)
                    set -l rest (string sub -s (math (string length "$f:") + 1) "$line")
                    set -l rel (string replace "$NOTE_DIR/" "" "$f")
                    printf '%s:%s\t%s\n' "$rel" "$rest" "$line"
                end | fzf \
                --height=60% --border \
                --delimiter='\t' --with-nth=1 \
                --prompt="Matching Lines ➔ " \
                --preview 'f=(echo {2} | cut -d: -f1); l=(echo {2} | cut -d: -f2); { command -v bat >/dev/null 2>&1 && bat --color=always --highlight-line "$l" "$f" 2>/dev/null; } || { command -v batcat >/dev/null 2>&1 && batcat --color=always --highlight-line "$l" "$f" 2>/dev/null; } || cat "$f"')

            if test -n "$match_line"
                set -l raw_match (printf '%s' "$match_line" | cut -f2)
                set -l file (printf '%s' "$raw_match" | cut -d: -f1)
                (if test -n "$EDITOR"; echo "$EDITOR"; else; echo "nano"; end) "$file"
            end

        case -h|--help
            printf 'Usage:\n'
            printf '  notes          – Browse and preview notes interactively\n'
            printf '  notes add      – Add a new note under a category\n'
            printf '  notes search   – Search text inside all notes\n'

        case ""
            # ── Plain list fallback when fzf is absent ──────────────────────
            # BUG FIX: original code crashed here when fzf was not installed
            if ! command -v fzf >/dev/null 2>&1
                printf '📁 Notes in %s:\n\n' "$NOTE_DIR"
                find "$NOTE_DIR" -type f -name "*.txt" 2>/dev/null | \
                    while IFS= read -r f; do
                        printf '  [%s] %s\n' \
                            "(basename "(dirname "$f")")" \
                            "(basename "$f" .txt)"
                    end
                return 0
            end

            set -l selected_line
            set selected_line (find "$NOTE_DIR" -type f -name "*.txt" 2>/dev/null | \
                while IFS= read -r f
                    set -l rel (string replace "$NOTE_DIR/" "" "$f")
                    set -l cat_name (dirname "$rel")
                    set -l title_name (basename "$rel" .txt)
                    printf '📁 %s ➔ %s\t%s\n' "$cat_name" "$title_name" "$f"
                end | fzf \
                --height=70% --reverse --border \
                --delimiter='\t' --with-nth=1 \
                --prompt="🔎 Search Notes ➔ " \
                --preview "$PREVIEW_CMD {2} 2>/dev/null || cat {2}" \
                --preview-window=right:60%)

            [ -z "$selected_line" ] && return 0
            set -l selected (printf '%s' "$selected_line" | cut -f2)

            set -l title category note_action
            title=(basename "$selected" .txt)
            set category (basename "(dirname "$selected")")

            # ── Action menu: gum → plain numbered prompt ────────────────────
            if command -v gum >/dev/null 2>&1
                gum style --foreground 212 --border normal \
                    "📝 Note: $title | 📁 Category: $category"
                if _fb_has_vscode
                    set note_action (gum choose \
                        "👁️  Full View" "✏️  Edit Note" "💻 Open in VS Code" \
                        "📋 Copy Content" "🗑️  Delete Note" "🔙 Cancel")
                else
                    set note_action (gum choose \
                        "👁️  Full View" "✏️  Edit Note" \
                        "📋 Copy Content" "🗑️  Delete Note" "🔙 Cancel")
                end
            else
                printf '\n📝 Note: %s | 📁 Category: %s\n\n' "$title" "$category"
                printf '  1. 👁️  Full View\n'
                printf '  2. ✏️  Edit Note\n'
                printf '  3. 📋 Copy Content\n'
                printf '  4. 🗑️  Delete Note\n'
                printf '  5. Cancel\n\n'
                printf 'Choose [1-5]: '
    set -l choice
                read -r choice
                switch $choice
                    case 1) note_action="👁️  Full View" ;;
                    case 2) note_action="✏️  Edit Note" ;;
                    case 3) note_action="📋 Copy Content" ;;
                    case 4) note_action="🗑️  Delete Note" ;;
                    case *) return 0 ;;
                end
            end

            switch $note_action
                "👁️  Full View")
                    if command -v glow >/dev/null 2>&1
                        glow -p "$selected"
                    else if command -v bat >/dev/null 2>&1
                        bat "$selected"
                    else if command -v batcat >/dev/null 2>&1
                        batcat "$selected"
                    else
                        less "$selected"
                    end
                "✏️  Edit Note")
                    (if test -n "$EDITOR"; echo "$EDITOR"; else; echo "nano"; end) "$selected"
                "💻 Open in VS Code")
                    _fb_open_vscode "$selected"
                "📋 Copy Content")
                    _fb_copy_to_clipboard (_fb_get_note_body "$selected")
                "🗑️  Delete Note")
    set -l confirm_del "n"
                    if command -v gum >/dev/null 2>&1
                        gum confirm "Delete '$title'?" && confirm_del="y"
                    else
                        printf "Delete '%s'? [y/N]: " "$title"
                        read -r confirm_del
                    end
                    if test "$confirm_del" == "y" || "$confirm_del" == "Y"
                        rm "$selected"
                        printf '🗑️  Note deleted!\n'
                    end
            end
            ;;

        *)
            printf '❌ Unknown command: %s\n' "$action"
            printf "Run 'notes --help' for usage.\n"
            return 1
            ;;
    end
end

# ==============================================================================
# 🎬 3. FFMPEG MULTIMEDIA SUITE (ffmedia / ffstudio / fftool / fancy_ffmpeg)
# Usage: ffmedia [action]
# Interactive multimedia toolbox leveraging ffmpeg, fzf, and gum.
# ==============================================================================

function _fb_media_select_file
    set -l prompt "(if test -n "$argv[1]"; echo "$argv[1]"; else; echo "Select File:"; end)"
    set -l pattern "${2:-}"
    set -l selected ""

    if command -v fzf >/dev/null 2>&1
        if test -n "$pattern"
            selected=(find . -maxdepth 3 -type f 2>/dev/null | grep -iE "$pattern" | fzf --prompt="$prompt " --height=40% --reverse)
        else
            selected=(find . -maxdepth 3 -type f 2>/dev/null | fzf --prompt="$prompt " --height=40% --reverse)
        end
    else if command -v gum >/dev/null 2>&1
        selected=(gum file .)
    end

    if test -z "$selected"
        printf '%s ' "$prompt"
        read -r -e selected
    end
    echo "$selected"
end

function _fb_media_choose_opt
    set -l title "$argv[1]"
    shift
    set -l options ("$@")
    set -l choice ""

    if command -v gum >/dev/null 2>&1
        gum style --foreground 212 --bold "$title"
        choice=(printf '%s\n' "$options" | gum choose --height=15)
    else if command -v fzf >/dev/null 2>&1
        choice=(printf '%s\n' "$options" | fzf --prompt="$title ➔ " --height=40% --reverse)
    else
        printf '\n=== %s ===\n' "$title"
    set -l idx 1
        for opt in "$options"; do
            printf '  %2d) %s\n' "$idx" "$opt"
            ((idx++))
        end
        printf 'Select option [1-%d]: ' "(count $options)"
    set -l num
        read -r num
        if test "$num" =~ ^[0-9]+$ && (( num >= 1 && num <= (count $options) ))
            choice="${options[((num-1))]}"
        end
    end
    echo "$choice"
end

function _fb_media_input
    set -l prompt "$argv[1]"
    set -l default_val "${2:-}"
    set -l val ""

    if command -v gum >/dev/null 2>&1
        val=(gum input --placeholder "$prompt" --value "$default_val")
    else
        if test -n "$default_val"
            printf '%s [%s]: ' "$prompt" "$default_val"
        else
            printf '%s: ' "$prompt"
        end
        read -r val
        [ -z "$val" ] && val="$default_val"
    end
    echo "$val"
end

function ffmedia
    _fb_ensure_dep ffmpeg ffmpeg ffmpeg ffmpeg || return 1

    set -l action "${1:-}"

    if test -z "$action"
        set action (_fb_media_choose_opt "🎬 FFmedia All-in-One Multimedia Suite" \
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
    end

    [ -z "$action" ] || test "$action" = *"Exit"* && return 0

    switch $action
        "1. "*|*"Compress"*|compress)
    set -l file (_fb_media_select_file "Select video to compress:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$file" ] && return 0
    set -l preset (_fb_media_choose_opt "Select Compression Preset:" \
                case "1) Balanced Quality (CRF 23 - Recommended)" \
                case "2) High Compression (CRF 28 - ~50-70% size reduction)" \
                case "3) Extreme Compression (CRF 32 - ~70-80% size reduction)"
    set -l crf 23
            test "$preset" = *"2)"* && crf=28
            test "$preset" = *"3)"* && crf=32
    set -l ext "${file##*.}"
    set -l base "${file%.*}"
    set -l out "${base}_compressed.${ext}"
            printf '\n⚡ Compressing "%s" (CRF %s)...\n' "$file" "$crf"
            ffmpeg -i "$file" -vcodec libx264 -crf "$crf" -preset fast -acodec aac "$out"
            printf '\n✅ Done! Output saved as: %s\n' "$out"

        "2. "*|*"Trim"*|trim)
    set -l file (_fb_media_select_file "Select video to trim:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$file" ] && return 0
    set -l start (_fb_media_input "Start timestamp (HH:MM:SS or seconds)" "00:00:00")
    set -l dur (_fb_media_input "Duration (HH:MM:SS or seconds, e.g. 10)" "00:00:10")
    set -l ext "${file##*.}"
    set -l base "${file%.*}"
    set -l out "${base}_trimmed.${ext}"
            printf '\n⚡ Trimming "%s" (Start: %s, Duration: %s)...\n' "$file" "$start" "$dur"
            ffmpeg -ss "$start" -i "$file" -t "$dur" -c copy "$out"
            printf '\n✅ Lossless trim complete: %s\n' "$out"

        "3. "*|*"Concat"*|concat)
            printf 'Select files to concat. Enter file paths separated by space (or wildcard like *.mp4):\n'
    set -l input_pattern (_fb_media_input "Files or wildcard (e.g. video1.mp4 video2.mp4 or clip*.mp4)" "")
            [ -z "$input_pattern" ] && return 0
    set -l list_file (mktemp)
            for f in $input_pattern; do
                if test -f "$f"
    set -l escaped_path (realpath "$f" | sed "s/'/'\\\\''/g")
                    printf "file '%s'\n" "$escaped_path" >> "$list_file"
                end
            end
            if test ! -s "$list_file"
                printf '❌ No valid files found!\n'
                rm -f "$list_file"
                return 1
            end
    set -l out "merged_(date +%Y%m%d_%H%M%S).mp4"
            printf '\n⚡ Merging video files...\n'
            ffmpeg -f concat -safe 0 -i "$list_file" -c copy "$out"
            rm -f "$list_file"
            printf '\n✅ Videos merged into: %s\n' "$out"

        "4. "*|*"Resolution"*|resolution)
    set -l file (_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$file" ] && return 0
    set -l mode (_fb_media_choose_opt "Select Target Resolution / Aspect Ratio:" \
                case "1) 1080p Full HD (1920x1080)" \
                case "2) 720p HD (1280x720)" \
                case "3) 480p SD (854x480)" \
                case "4) Crop 16:9 to 9:16 Portrait (Reels/Shorts)" \
                case "5) 9:16 Portrait with Blurred Background"
    set -l base "${file%.*}"
    set -l out "${base}_res.${file##*.}"
            switch $mode
                case *"1)"*) ffmpeg -i "$file" -vf "scale=1920:1080:force_original_aspect_ratio=decrease,pad=1920:1080:(ow-iw)/2:(oh-ih)/2" -c:a copy "$out" ;;
                case *"2)"*) ffmpeg -i "$file" -vf "scale=1280:720:force_original_aspect_ratio=decrease,pad=1280:720:(ow-iw)/2:(oh-ih)/2" -c:a copy "$out" ;;
                case *"3)"*) ffmpeg -i "$file" -vf "scale=854:480:force_original_aspect_ratio=decrease,pad=854:480:(ow-iw)/2:(oh-ih)/2" -c:a copy "$out" ;;
                case *"4)"*) ffmpeg -i "$file" -vf "crop=ih*9/16:ih" -c:a copy "$out" ;;
                case *"5)"*) ffmpeg -i "$file" -filter_complex "[0:v]scale=1080:1920:force_original_aspect_ratio=increase,crop=1080:1920,boxblur=20:10[bg];[0:v]scale=1080:1920:force_original_aspect_ratio=decrease[fg];[bg][fg]overlay=(W-w)/2:(H-h)/2" -c:a copy "$out" ;;
            end
            printf '\n✅ Resolution adjusted: %s\n' "$out"
            ;;

        "5. "*|*"Speed Control"*|speed)
    set -l file (_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$file" ] && return 0
    set -l spd_opt (_fb_media_choose_opt "Select Speed:" \
                "1) 0.25x (Super Slow-Mo)" \
                "2) 0.5x (Slow Motion)" \
                "3) 1.5x (Faster)" \
                "4) 2.0x (Time-lapse 2x)" \
                "5) 4.0x (Time-lapse 4x)")
    set -l pts "1.0" atempo="1.0"
            test "$spd_opt" = *"1)"* && pts="4.0" && atempo="0.5,atempo=0.5"
            test "$spd_opt" = *"2)"* && pts="2.0" && atempo="0.5"
            test "$spd_opt" = *"3)"* && pts="0.66667" && atempo="1.5"
            test "$spd_opt" = *"4)"* && pts="0.5" && atempo="2.0"
            test "$spd_opt" = *"5)"* && pts="0.25" && atempo="2.0,atempo=2.0"
    set -l out "${file%.*}_speed.${file##*.}"
            if ffprobe -i "$file" -show_streams -select_streams a 2>&1 | grep -q "codec_type=audio"
                ffmpeg -i "$file" -filter_complex "[0:v]setpts=${pts}*PTS[v];[0:a]atempo=${atempo}[a]" -map "[v]" -map "[a]" "$out"
            else
                ffmpeg -i "$file" -vf "setpts=${pts}*PTS" "$out"
            end
            printf '\n✅ Speed changed: %s\n' "$out"
            ;;

        "6. "*|*"Rotate"*|rotate)
    set -l file (_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$file" ] && return 0
    set -l rot_opt (_fb_media_choose_opt "Select Rotation / Flip Option:" \
                "1) Rotate 90° Clockwise" \
                "2) Rotate 90° Counter-Clockwise" \
                "3) Rotate 180°" \
                "4) Flip Horizontally (Mirror)" \
                "5) Flip Vertically")
    set -l vf "transpose=1"
            test "$rot_opt" = *"2)"* && vf="transpose=2"
            test "$rot_opt" = *"3)"* && vf="transpose=2,transpose=2"
            test "$rot_opt" = *"4)"* && vf="hflip"
            test "$rot_opt" = *"5)"* && vf="vflip"
    set -l out "${file%.*}_rotated.${file##*.}"
            ffmpeg -i "$file" -vf "$vf" -c:a copy "$out"
            printf '\n✅ Rotation/Flip complete: %s\n' "$out"
            ;;

        "7. "*|*"Watermark"*|watermark)
    set -l file (_fb_media_select_file "Select main video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$file" ] && return 0
    set -l type (_fb_media_choose_opt "Watermark Type:" "1) Image Logo" "2) Text Banner")
    set -l out "${file%.*}_watermarked.${file##*.}"
            if test "$type" == *"1)"*
    set -l logo (_fb_media_select_file "Select watermark image (PNG/JPG):" "\.(png|jpg|jpeg)$")
                [ -z "$logo" ] && return 0
    set -l pos (_fb_media_choose_opt "Watermark Position:" \
                    "1) Top-Right" "2) Top-Left" "3) Bottom-Right" "4) Bottom-Left" "5) Center")
    set -l overlay "main_w-overlay_w-10:10" # default top-right
                test "$pos" = *"2)"* && overlay="10:10"
                test "$pos" = *"3)"* && overlay="main_w-overlay_w-10:main_h-overlay_h-10"
                test "$pos" = *"4)"* && overlay="10:main_h-overlay_h-10"
                test "$pos" = *"5)"* && overlay="(main_w-overlay_w)/2:(main_h-overlay_h)/2"
                ffmpeg -i "$file" -i "$logo" -filter_complex "[1:v]scale=150:-1[logo];[0:v][logo]overlay=${overlay}" -c:a copy "$out"
            else
    set -l text (_fb_media_input "Watermark Text" "FancyBash")
    set -l safe_text (echo "$text" | sed "s/'/\\\\'/g")
                ffmpeg -i "$file" -vf "drawtext=text='${safe_text}':x=w-tw-20:y=h-th-20:fontsize=36:fontcolor=white@0.8:box=1:boxcolor=black@0.4:boxborderw=5" -c:a copy "$out"
            end
            printf '\n✅ Watermark applied: %s\n' "$out"
            ;;

        "8. "*|*"Video Grid"*|grid)
    set -l grid_mode (_fb_media_choose_opt "Grid Layout:" "1) 2 Videos Side-by-Side" "2) 4 Videos (2x2 Grid)")
            if test "$grid_mode" == *"1)"*
    set -l f1 (_fb_media_select_file "Select Video 1:" "\.(mp4|mkv|mov|avi|webm)$")
    set -l f2 (_fb_media_select_file "Select Video 2:" "\.(mp4|mkv|mov|avi|webm)$")
                [ -z "$f1" ] || [ -z "$f2" ] && return 0
    set -l out "comparison_side_by_side.mp4"
                ffmpeg -i "$f1" -i "$f2" -filter_complex "[0:v]scale=-1:720[v0];[1:v]scale=-1:720[v1];[v0][v1]hstack=inputs=2[v]" -map "[v]" -c:v libx264 "$out"
            else
    set -l f1 (_fb_media_select_file "Select Video 1 (Top-Left):" "\.(mp4|mkv|mov|avi|webm)$")
    set -l f2 (_fb_media_select_file "Select Video 2 (Top-Right):" "\.(mp4|mkv|mov|avi|webm)$")
    set -l f3 (_fb_media_select_file "Select Video 3 (Bottom-Left):" "\.(mp4|mkv|mov|avi|webm)$")
    set -l f4 (_fb_media_select_file "Select Video 4 (Bottom-Right):" "\.(mp4|mkv|mov|avi|webm)$")
                [ -z "$f1" ] || [ -z "$f2" ] || [ -z "$f3" ] || [ -z "$f4" ] && return 0
    set -l out "grid_2x2.mp4"
                ffmpeg -i "$f1" -i "$f2" -i "$f3" -i "$f4" -filter_complex "[0:v]scale=640:360[v0];[1:v]scale=640:360[v1];[2:v]scale=640:360[v2];[3:v]scale=640:360[v3];[v0][v1][v2][v3]xstack=inputs=4:layout=0_0|w0_0|0_h0|w0_h0[v]" -map "[v]" -c:v libx264 "$out"
            end
            printf '\n✅ Grid video created: %s\n' "$out"
            ;;

        "9. "*|*"Extract Audio"*|audio-extract)
    set -l file (_fb_media_select_file "Select media file:" "\.(mp4|mkv|mov|avi|webm|m4a|flv)$")
            [ -z "$file" ] && return 0
    set -l fmt (_fb_media_choose_opt "Select Output Audio Format:" "1) MP3" "2) AAC" "3) WAV" "4) FLAC" "5) M4A")
    set -l ext "mp3" acodec="libmp3lame"
            test "$fmt" = *"2)"* && ext="aac" && acodec="aac"
            test "$fmt" = *"3)"* && ext="wav" && acodec="pcm_s16le"
            test "$fmt" = *"4)"* && ext="flac" && acodec="flac"
            test "$fmt" = *"5)"* && ext="m4a" && acodec="aac"
    set -l out "${file%.*}.${ext}"
            ffmpeg -i "$file" -vn -acodec "$acodec" "$out"
            printf '\n✅ Audio extracted to: %s\n' "$out"
            ;;

        "10. "*|*"Mute Video"*|mute)
    set -l file (_fb_media_select_file "Select video to mute:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$file" ] && return 0
    set -l out "${file%.*}_muted.${file##*.}"
            ffmpeg -i "$file" -an -c:v copy "$out"
            printf '\n✅ Video muted: %s\n' "$out"
            ;;

        "11. "*|*"Replace"*|audio-replace)
    set -l vid (_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$vid" ] && return 0
    set -l aud (_fb_media_select_file "Select audio track file:" "\.(mp3|wav|aac|m4a|flac|ogg)$")
            [ -z "$aud" ] && return 0
    set -l mode (_fb_media_choose_opt "Audio Integration Mode:" \
                "1) Replace original audio completely" \
                "2) Mix new audio with existing video sound")
    set -l out "${vid%.*}_audio_merged.${vid##*.}"
            if test "$mode" = *"1)"* || ! ffprobe -i "$vid" -show_streams -select_streams a 2>&1 | grep -q "codec_type=audio"
                ffmpeg -i "$vid" -i "$aud" -c:v copy -c:a aac -map 0:v:0 -map 1:a:0 -shortest "$out"
            else
                ffmpeg -i "$vid" -i "$aud" -filter_complex "[0:a][1:a]amix=inputs=2:duration=first[a]" -map 0:v -map "[a]" -c:v copy "$out"
            end
            printf '\n✅ Audio track processed: %s\n' "$out"
            ;;

        "12. "*|*"Loudness"*|loudness)
    set -l file (_fb_media_select_file "Select media file:" "\.(mp4|mkv|mov|avi|webm|mp3|wav)$")
            [ -z "$file" ] && return 0
    set -l std (_fb_media_choose_opt "Target Loudness Standard:" \
                "1) YouTube / Podcast (-14 LUFS - Recommended)" \
                "2) EBU R128 Broadcast (-23 LUFS)")
    set -l lufs "-14"
            test "$std" = *"2)"* && lufs="-23"
    set -l out "${file%.*}_normalized.${file##*.}"
            ffmpeg -i "$file" -af "loudnorm=I=${lufs}:LRA=11:TP=-1.5" "$out"
            printf '\n✅ Loudness normalized (%s LUFS): %s\n' "$lufs" "$out"
            ;;

        "13. "*|*"Visualizer"*|visualizer)
    set -l aud (_fb_media_select_file "Select audio file:" "\.(mp3|wav|aac|m4a|flac|ogg)$")
            [ -z "$aud" ] && return 0
    set -l viz (_fb_media_choose_opt "Select Visualizer Style:" \
                "1) Vector Waveform (Neon Green Line)" \
                "2) Frequency Spectrum (Rainbow Combined)" \
                "3) CQT Color Spectrum" \
                "4) Audio Stereo Histogram")
    set -l filter "showwaves=s=1280x720:mode=line:colors=0x00FF99[v]"
            test "$viz" = *"2)"* && filter="showspectrum=s=1280x720:mode=combined:color=rainbow[v]"
            test "$viz" = *"3)"* && filter="showcqt=s=1280x720[v]"
            test "$viz" = *"4)"* && filter="ahistogram=s=1280x720[v]"
    set -l out "${aud%.*}_visualizer.mp4"
            ffmpeg -i "$aud" -filter_complex "$filter" -map "[v]" -map 0:a -c:v libx264 -c:a aac -shortest "$out"
            printf '\n✅ Audio visualizer video created: %s\n' "$out"
            ;;

        "14. "*|*"Audio Speed"*|audio-speed)
    set -l aud (_fb_media_select_file "Select audio file:" "\.(mp3|wav|aac|m4a|flac|ogg)$")
            [ -z "$aud" ] && return 0
    set -l speed (_fb_media_input "Audio speed factor (e.g. 1.25, 1.5, 0.8)" "1.25")
    set -l out "${aud%.*}_speed_${speed}.${aud##*.}"
            ffmpeg -i "$aud" -af "atempo=${speed}" "$out"
            printf '\n✅ Audio speed changed (pitch preserved): %s\n' "$out"
            ;;

        "15. "*|*"Snapshot"*|snapshot)
    set -l vid (_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$vid" ] && return 0
    set -l time (_fb_media_input "Timestamp (HH:MM:SS or seconds)" "00:00:05")
    set -l fmt (_fb_media_choose_opt "Image Format:" "1) JPEG (.jpg)" "2) PNG (.png)")
    set -l ext "jpg"
            test "$fmt" = *"2)"* && ext="png"
    set -l out "${vid%.*}_snap_${time//:/}_.${ext}"
            ffmpeg -ss "$time" -i "$vid" -vframes 1 -q:v 2 "$out"
            printf '\n✅ Snapshot extracted: %s\n' "$out"
            ;;

        "16. "*|*"Bulk Frame"*|bulk-frames)
    set -l vid (_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$vid" ] && return 0
    set -l interval (_fb_media_input "Extract frame every N seconds" "1")
    set -l base (basename "${vid%.*}")
    set -l outdir "${base}_frames"
            mkdir -p "$outdir"
            ffmpeg -i "$vid" -vf "fps=1/${interval}" "${outdir}/frame_%04d.jpg"
            printf '\n✅ Frames extracted into video folder: %s/\n' "$outdir"
            ;;

        "17. "*|*"GIF"*|gif)
    set -l vid (_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$vid" ] && return 0
    set -l start (_fb_media_input "Start timestamp" "00:00:00")
    set -l dur (_fb_media_input "GIF Duration in seconds" "5")
    set -l res (_fb_media_choose_opt "GIF Resolution & Quality:" \
                "1) 480p @ 15fps (Standard)" \
                "2) 720p @ 20fps (HD High Quality)" \
                "3) 360p @ 12fps (Compact)")
    set -l scale "480" fps="15"
            test "$res" = *"2)"* && scale="720" && fps="20"
            test "$res" = *"3)"* && scale="360" && fps="12"
    set -l out "${vid%.*}_pro.gif"
    set -l tmp_palette "/tmp/ff_palette_$$.png"
            printf '\n⚡ Generating palette and rendering ultra-sharp GIF...\n'
            ffmpeg -ss "$start" -t "$dur" -i "$vid" -vf "fps=${fps},scale=${scale}:-1:flags=lanczos,palettegen" -y "$tmp_palette"
            ffmpeg -ss "$start" -t "$dur" -i "$vid" -i "$tmp_palette" -filter_complex "fps=${fps},scale=${scale}:-1:flags=lanczos[x];[x][1:v]paletteuse" "$out"
            rm -f "$tmp_palette"
            printf '\n✅ Pro-Quality GIF created: %s\n' "$out"
            ;;

        "18. "*|*"Contact Sheet"*|contact-sheet)
    set -l vid (_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$vid" ] && return 0
    set -l grid (_fb_media_choose_opt "Grid Layout:" "1) 3x3 Grid (9 Thumbnails)" "2) 4x4 Grid (16 Thumbnails)")
    set -l tiles "3x3"
            test "$grid" = *"2)"* && tiles="4x4"
    set -l out "${vid%.*}_contact_sheet.png"
            ffmpeg -i "$vid" -vf "fps=1/10,scale=320:-1,tile=${tiles}" -vsync vfr "$out"
            printf '\n✅ Mosaic Thumbnail Grid created: %s\n' "$out"
            ;;

        "19. "*|*"Screen Recorder"*|screen-record)
    set -l out "screencast_(date +%Y%m%d_%H%M%S).mp4"
    set -l os_type (uname -s)
    set -l audio_opt (_fb_media_choose_opt "Select Screen Record Mode:" \
                "1) Full Screen Video + System Audio (Mic/Speaker)" \
                "2) Full Screen Video Only (Silent)")

            printf '\n🎥 Starting Terminal Screen Recorder...\n'
            printf '📌 Press "q" or Ctrl+C in this terminal to stop recording.\n\n'

            if test "$os_type" = "Linux"
    set -l display "(if test -n "$DISPLAY"; echo "$DISPLAY"; else; echo ":0.0"; end)"
    set -l res ""
                if command -v xrandr >/dev/null 2>&1
                    res=(xrandr 2>/dev/null | grep '*' | awk '{print $argv[1]}' | head -n1)
                else if command -v xdpyinfo >/dev/null 2>&1
                    res=(xdpyinfo 2>/dev/null | grep 'dimensions:' | awk '{print $argv[2]}')
                end
                [ -z "$res" ] && res="1920x1080"

                if test "$audio_opt" = *"1)"* && (command -v pactl >/dev/null 2>&1 || command -v pulseaudio >/dev/null 2>&1)
                    ffmpeg -f x11grab -video_size "$res" -i "$display" -f pulse -i default -c:v libx264 -preset ultrafast -pix_fmt yuv420p -c:a aac "$out"
                else
                    ffmpeg -f x11grab -video_size "$res" -i "$display" -c:v libx264 -preset ultrafast -pix_fmt yuv420p "$out"
                end
            else if test "$os_type" = "Darwin"
                if test "$audio_opt" == *"1)"*
                    ffmpeg -f avfoundation -i "0:0" -c:v libx264 -preset ultrafast -pix_fmt yuv420p -c:a aac "$out" 2>/dev/null || \
                    ffmpeg -f avfoundation -i "1:0" -c:v libx264 -preset ultrafast -pix_fmt yuv420p -c:a aac "$out"
                else
                    ffmpeg -f avfoundation -i "0" -c:v libx264 -preset ultrafast -pix_fmt yuv420p "$out" 2>/dev/null || \
                    ffmpeg -f avfoundation -i "1" -c:v libx264 -preset ultrafast -pix_fmt yuv420p "$out"
                end
            else
                if test "$audio_opt" == *"1)"*
                    ffmpeg -f gdigrab -i desktop -f dshow -i audio="Microphone" -c:v libx264 -preset ultrafast -pix_fmt yuv420p -c:a aac "$out" 2>/dev/null || \
                    ffmpeg -f gdigrab -i desktop -c:v libx264 -preset ultrafast -pix_fmt yuv420p "$out"
                else
                    ffmpeg -f gdigrab -i desktop -c:v libx264 -preset ultrafast -pix_fmt yuv420p "$out"
                end
            end
            printf '\n✅ Screen recording saved as: %s\n' "$out"
            ;;

        "20. "*|*"Subtitle Burn"*|subtitle-burn)
    set -l vid (_fb_media_select_file "Select video file:" "\.(mp4|mkv|mov|avi|webm)$")
            [ -z "$vid" ] && return 0
    set -l sub (_fb_media_select_file "Select subtitle file (.srt/.ass):" "\.(srt|ass)$")
            [ -z "$sub" ] && return 0
    set -l out "${vid%.*}_subtitled.${vid##*.}"
    set -l safe_sub (echo "$sub" | sed "s/'/\\\\'/g" | sed "s/:/\\\\:/g")
            ffmpeg -i "$vid" -vf "subtitles='${safe_sub}'" -c:a copy "$out"
            printf '\n✅ Subtitle burned into video: %s\n' "$out"
            ;;

        "21. "*|*"Subtitle Extraction"*|subtitle-extract)
    set -l vid (_fb_media_select_file "Select video file:" "\.(mkv|mp4|mov)$")
            [ -z "$vid" ] && return 0
    set -l out "${vid%.*}.srt"
            ffmpeg -i "$vid" -map 0:s:0 "$out"
            printf '\n✅ Subtitle extracted: %s\n' "$out"
            ;;

        "22. "*|*"Privacy Clean"*|privacy-clean)
    set -l file (_fb_media_select_file "Select media file:" "\.(mp4|mkv|mov|avi|webm|mp3|wav|jpg|png)$")
            [ -z "$file" ] && return 0
    set -l out "${file%.*}_clean.${file##*.}"
            ffmpeg -i "$file" -map_metadata -1 -c copy "$out"
            printf '\n✅ Privacy clean complete (metadata wiped): %s\n' "$out"
            ;;

        "23. "*|*"Format Conversion"*|convert)
    set -l file (_fb_media_select_file "Select media file to convert:" "\.(mp4|mkv|mov|avi|webm|ts|flv|mp3|wav|aac)$")
            [ -z "$file" ] && return 0
    set -l fmt (_fb_media_choose_opt "Select Target Format:" \
                "1) MP4 (.mp4)" \
                "2) MKV (.mkv)" \
                "3) WEBM (.webm)" \
                "4) MOV (.mov)" \
                "5) AVI (.avi)" \
                "6) MP3 (.mp3)" \
                "7) WAV (.wav)")
    set -l ext "mp4"
            test "$fmt" = *"2)"* && ext="mkv"
            test "$fmt" = *"3)"* && ext="webm"
            test "$fmt" = *"4)"* && ext="mov"
            test "$fmt" = *"5)"* && ext="avi"
            test "$fmt" = *"6)"* && ext="mp3"
            test "$fmt" = *"7)"* && ext="wav"
    set -l out "${file%.*}.${ext}"
            ffmpeg -i "$file" "$out"
            printf '\n✅ Format converted to: %s\n' "$out"
            ;;

        "24. "*|*"Batch"*|batch)
    set -l folder (_fb_media_input "Enter folder path (or press enter for current directory)" ".")
            [ ! -d "$folder" ] && printf '❌ Directory not found!\n' && return 1
    set -l batch_task (_fb_media_choose_opt "Select Batch Operation:" \
                "1) Bulk Video Compress" \
                "2) Bulk Format Convert to MP4" \
                "3) Bulk Metadata Wiping (Privacy Clean)" \
                "4) Bulk Audio Extraction (MP3)" \
                "5) Bulk Video Muting")
            printf '\n⚡ Running batch processing on folder: %s...\n' "$folder"
            for f in "$folder"/*.{mp4,mkv,mov,avi,webm}; do
                [ -f "$f" ] || continue
    set -l base "${f%.*}"
                switch $batch_task
                    case *"1)"*) ffmpeg -i "$f" -vcodec libx264 -crf 26 -preset fast "${base}_batch_compressed.mp4" -y ;;
                    case *"2)"*) ffmpeg -i "$f" "${base}_converted.mp4" -y ;;
                    case *"3)"*) ffmpeg -i "$f" -map_metadata -1 -c copy "${base}_clean.${f##*.}" -y ;;
                    case *"4)"*) ffmpeg -i "$f" -vn -acodec libmp3lame "${base}_audio.mp3" -y ;;
                    case *"5)"*) ffmpeg -i "$f" -an -c:v copy "${base}_muted.${f##*.}" -y ;;
                end
            end
            printf '\n✅ Batch processing complete!\n'
            ;;

        *)
            printf '❌ Invalid action: %s\n' "$action"
            return 1
            ;;
    end
end

# Aliases for FFmedia Suite
alias ffstudio='ffmedia'
alias fftool='ffmedia'
alias fancy_ffmpeg='ffmedia'

# ==============================================================================
# 🔐 HARDENED MULTI-VAULT MANAGER (vault / secvault / fvault / fancy_vault)
# ==============================================================================
# Universal Linux & macOS AES-256 Memory-Guarded Directory Vault Manager
# Usage: vault [lock|unlock|list|config|--help]

set -g _fb_vault_store "$HOME/.secret_vaults"
set -g _fb_vault_iterations 500000
set -g _fb_vault_max_attempts 3

function _fb_vault_ensure_store
    mkdir -p "$_fb_vault_store" 2>/dev/null
    chmod 700 "$_fb_vault_store" 2>/dev/null
    # Clean up broken symlinks in $HOME left behind by unmounted/rebooted RAM vaults
    for sym in "$HOME"/*
        if test -L "$sym"; and not test -e "$sym"
            set -l target (readlink "$sym" 2>/dev/null)
            if string match -q "*dev/shm*" "$target"; or string match -q "*secret_ram*" "$target"
                rm -f "$sym" 2>/dev/null
            end
        end
    end
end

function _fb_vault_detect_ram_base
    set -l target_ram ""
    if test -d "/dev/shm"; and test -w "/dev/shm"
        set target_ram "/dev/shm"
    else
        set -l uid (id -u 2>/dev/null; or echo 1000)
        set -l tmp_base (set -q TMPDIR; and echo "$TMPDIR"; or echo "/tmp")
        set target_ram "$tmp_base/.secret_ram_$uid"
        mkdir -p "$target_ram" 2>/dev/null
        chmod 700 "$target_ram" 2>/dev/null
    fi
    echo "$target_ram"
end

function _fb_vault_banner
    if type -q gum
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
end

function _fb_vault_fancy_input
    set -l title $argv[1]
    set -l badge $argv[2]
    set -l icon (set -q argv[3]; and echo $argv[3]; or echo "🔑")

    if type -q gum
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
        set -l val ""
        printf "%s %s %s: " "$icon" "$title" "$badge" >&2
        stty -echo 2>/dev/null
        read -l val < /dev/tty
        stty echo 2>/dev/null
        printf "\n" >&2
        echo "$val"
    fi
end

function _fb_vault_send_alert
    set -l msg $argv[1]
    set -l bot_token "$FB_VAULT_TELEGRAM_BOT_TOKEN"
    set -l chat_id "$FB_VAULT_TELEGRAM_CHAT_ID"
    if test -n "$bot_token"; and test -n "$chat_id"
        curl -s -X POST "https://api.telegram.org/bot{$bot_token}/sendMessage" \
             -d chat_id="$chat_id" \
             -d text="⚠️ VAULT ALERT: $msg" >/dev/null 2>&1 &
    fi
end

function _fb_vault_open_explorer
    set -l target_path $argv[1]
    if string match -q "darwin*" "$OSTYPE"
        open "$target_path" &>/dev/null &
    else if type -q xdg-open
        xdg-open "$target_path" &>/dev/null &
    fi
end

function _fb_vault_secure_delete
    set -l target $argv[1]
    test -z "$target"; and return 0
    if test -f "$target"
        if type -q shred
            shred -u -n 3 -z "$target" 2>/dev/null; or rm -f "$target"
        else if string match -q "darwin*" "$OSTYPE"
            rm -P -f "$target" 2>/dev/null; or rm -f "$target"
        else
            dd if=/dev/urandom of="$target" bs=1M count=1 2>/dev/null; or true
            rm -f "$target"
        fi
    else if test -d "$target"; and not test -L "$target"
        if type -q shred
            find "$target" -type f -exec shred -u -n 3 -z {} + 2>/dev/null
        else if string match -q "darwin*" "$OSTYPE"
            find "$target" -type f -exec rm -P {} + 2>/dev/null
        fi
        rm -rf "$target"
    else if test -L "$target"
        rm -f "$target"
    fi
end

function _fb_vault_close_session
    set -l folder_name $argv[1]
    test -z "$folder_name"; and return 0
    set -l ram_base (_fb_vault_detect_ram_base)
    set -l ram_dir "$ram_base/$folder_name"
    set -l vault_dir "$HOME/$folder_name"

    test -L "$vault_dir"; or test -d "$vault_dir"; and rm -rf "$vault_dir" 2>/dev/null
    _fb_vault_secure_delete "$ram_dir"
end

function _fb_vault_lock_specific
    set -l folder_name $argv[1]
    _fb_vault_ensure_store
    set -l encrypted_file "$_fb_vault_store/$folder_name.enc"
    set -l panic_hash_file "$_fb_vault_store/$folder_name.panic"
    set -l ram_base (_fb_vault_detect_ram_base)
    set -l ram_dir "$ram_base/$folder_name"
    set -l vault_dir "$HOME/$folder_name"

    _fb_vault_banner
    if type -q gum
        gum style --foreground 220 --bold "[>] Relocking '$folder_name'..."
    else
        printf '\033[1;33m[*] Relocking "%s"...\033[0m\n' "$folder_name"
    fi

    set -l pass1 (_fb_vault_fancy_input "Master Password" "[$folder_name]" "🔐")
    set -l pass2 (_fb_vault_fancy_input "Confirm Master Password" "[RE-ENTER]" "🔄")

    if test "$pass1" != "$pass2"; or test -z "$pass1"
        printf '\033[1;31m  [X] Passwords do not match or empty! Lock cancelled.\033[0m\n'
        read -l -P "Press Enter to continue..." _ < /dev/tty
        return 1
    fi

    set -l panic1 (_fb_vault_fancy_input "Panic Password (Optional)" "[OPTIONAL]" "🚨")
    if test -n "$panic1"
        printf '%s' "$panic1" | openssl dgst -sha256 | awk '{print $2}' > "$panic_hash_file"
    fi

    set -l tmp_archive "$ram_base/.tmp_vault_{$folder_name}_%self.tar.gz"
    rm -f "$tmp_archive" 2>/dev/null

    echo ""
    set -l enc_ok 1
    if type -q gum
        gum spin --spinner dot --title "Archiving & Encrypting AES-256..." -- \
            tar -czf "$tmp_archive" -C "$ram_base" "$folder_name"
    else
        tar -czf "$tmp_archive" -C "$ram_base" "$folder_name" 2>/dev/null
    fi

    if test $status -eq 0; and test -s "$tmp_archive"
        openssl enc -aes-256-cbc -pbkdf2 -iter $_fb_vault_iterations -pass pass:"$pass1" -in "$tmp_archive" -out "$encrypted_file" 2>/dev/null
        if test $status -eq 0; and test -s "$encrypted_file"
            set enc_ok 0
        fi
    fi
    _fb_vault_secure_delete "$tmp_archive"

    if test $enc_ok -eq 0
        chmod 600 "$encrypted_file" 2>/dev/null
        test -L "$vault_dir"; and rm -f "$vault_dir"
        _fb_vault_secure_delete "$ram_dir"

        printf '\033[1;32m  [✓] "%s" locked & encrypted successfully!\033[0m\n' "$folder_name"
    else
        _fb_vault_secure_delete "$encrypted_file"
        printf '\033[1;31m  [X] Encryption failed! Memory vault preserved.\033[0m\n'
    fi
    read -l -P "Press Enter to continue..." _ < /dev/tty
end

function _fb_vault_lock
    _fb_vault_ensure_store
    _fb_vault_banner
    set -l target_dir $argv[1]

    if test -z "$target_dir"
        if type -q fzf
            set target_dir (find "$HOME" -maxdepth 2 -type d ! -path '*/.*' ! -path '*/node_modules*' ! -path '*/Library*' 2>/dev/null | \
                fzf --height=12 --layout=reverse --border=rounded \
                    --prompt="📁 Select folder to lock > " \
                    --color="border:99,header:220,pointer:212,fg:255")
        else
            read -l -P "Enter directory path to lock: " target_dir < /dev/tty
        fi
    fi

    if test -z "$target_dir"; or not test -d "$target_dir"
        printf '\033[1;31m[!] No valid directory selected.\033[0m\n'
        read -l -P "Press Enter to continue..." _ < /dev/tty
        return 1
    fi

    set -l folder_name (basename "$target_dir")
    set -l encrypted_file "$_fb_vault_store/$folder_name.enc"
    set -l panic_hash_file "$_fb_vault_store/$folder_name.panic"
    set -l ram_base (_fb_vault_detect_ram_base)
    set -l ram_dir "$ram_base/$folder_name"

    set -l pass1 (_fb_vault_fancy_input "New Master Password" "[$folder_name]" "🛡️")
    if test -z "$pass1"
        printf '\033[1;31m  [X] Password cannot be empty!\033[0m\n'
        read -l -P "Press Enter to continue..." _ < /dev/tty
        return 1
    fi

    set -l pass2 (_fb_vault_fancy_input "Confirm Password" "[RE-ENTER]" "🔄")
    if test "$pass1" != "$pass2"
        printf '\033[1;31m  [X] Passwords do not match! Aborting.\033[0m\n'
        read -l -P "Press Enter to continue..." _ < /dev/tty
        return 1
    fi

    set -l panic1 (_fb_vault_fancy_input "Panic Password (Optional)" "[OPTIONAL]" "🚨")
    if test -n "$panic1"
        printf '%s' "$panic1" | openssl dgst -sha256 | awk '{print $2}' > "$panic_hash_file"
    fi

    set -l parent_dir (dirname "$target_dir")
    set -l tmp_archive "$ram_base/.tmp_vault_{$folder_name}_%self.tar.gz"
    rm -f "$tmp_archive" 2>/dev/null

    echo ""
    set -l enc_ok 1
    if type -q gum
        gum spin --spinner dot --title "Encrypting '$folder_name' with AES-256..." -- \
            tar -czf "$tmp_archive" -C "$parent_dir" "$folder_name"
    else
        tar -czf "$tmp_archive" -C "$parent_dir" "$folder_name" 2>/dev/null
    fi

    if test $status -eq 0; and test -s "$tmp_archive"
        openssl enc -aes-256-cbc -pbkdf2 -iter $_fb_vault_iterations -pass pass:"$pass1" -in "$tmp_archive" -out "$encrypted_file" 2>/dev/null
        if test $status -eq 0; and test -s "$encrypted_file"
            set enc_ok 0
        fi
    fi
    _fb_vault_secure_delete "$tmp_archive"

    if test $enc_ok -eq 0
        chmod 600 "$encrypted_file"
        test -L "$target_dir"; and rm -f "$target_dir"

        if type -q gum
            gum spin --spinner monkey --title "Securely wiping original directory..." -- sleep 1
        fi
        _fb_vault_secure_delete "$target_dir"
        _fb_vault_secure_delete "$ram_dir"

        printf '\033[1;32m  [✓] "%s" locked successfully!\033[0m\n' "$folder_name"
    else
        _fb_vault_secure_delete "$encrypted_file"
        printf '\033[1;31m  [X] Encryption failed! Original directory preserved.\033[0m\n'
    fi

    read -l -P "Press Enter to continue..." _ < /dev/tty
end

function _fb_vault_unlock
    _fb_vault_ensure_store
    _fb_vault_banner

    set -l available_vaults (find "$_fb_vault_store" -maxdepth 1 -name "*.enc" -exec basename {} .enc \; 2>/dev/null)

    if test (count $available_vaults) -eq 0
        printf '\033[1;31m  [!] No locked vaults found!\033[0m\n'
        read -l -P "Press Enter to continue..." _ < /dev/tty
        return 0
    fi

    set -l selected_folder $argv[1]
    if test -z "$selected_folder"
        if type -q fzf
            set selected_folder (string join "\n" $available_vaults | \
                fzf --height=10 --layout=reverse --border=rounded \
                    --prompt="🔓 Select vault to unlock > " \
                    --color="border:99,header:220,pointer:212,fg:255")
        else
            printf 'Available Vaults:\n'
            for v in $available_vaults
                printf ' - %s\n' "$v"
            end
            read -l -P "Enter vault folder name to unlock: " selected_folder < /dev/tty
        fi
    fi

    if test -z "$selected_folder"
        return 0
    fi

    set -l encrypted_file "$_fb_vault_store/$selected_folder.enc"
    set -l panic_hash_file "$_fb_vault_store/$selected_folder.panic"
    set -l ram_base (_fb_vault_detect_ram_base)
    set -l ram_dir "$ram_base/$selected_folder"
    set -l vault_dir "$HOME/$selected_folder"

    set -l attempts 0

    while test $attempts -lt $_fb_vault_max_attempts
        set -l remaining (math $_fb_vault_max_attempts - $attempts)
        set -l pass (_fb_vault_fancy_input "Enter Secret Password" "[$selected_folder | Attempts remaining: $remaining]" "🔑")

        test -z "$pass"; and continue

        # Panic Password Check
        if test -f "$panic_hash_file"
            set -l input_hash (printf '%s' "$pass" | openssl dgst -sha256 | awk '{print $2}')
            set -l saved_panic_hash (cat "$panic_hash_file")

            if test "$input_hash" = "$saved_panic_hash"
                _fb_vault_send_alert "PANIC PASSWORD USED FOR $selected_folder! Data wiped."
                _fb_vault_secure_delete "$encrypted_file"
                _fb_vault_secure_delete "$panic_hash_file"

                mkdir -p "$vault_dir"
                echo "Confidential Project Notes "(date +%Y)"..." > "$vault_dir/notes.txt"

                _fb_vault_open_explorer "$vault_dir"

                printf '\033[1;32m  [✓] Access Granted!\033[0m\n'
                read -l -P "Press Enter to continue..." _ < /dev/tty
                return 0
            fi
        fi

        rm -rf "$ram_dir" 2>/dev/null

        set -l tmp_dec_archive "$ram_base/.tmp_dec_{$selected_folder}_%self.tar.gz"
        rm -f "$tmp_dec_archive" 2>/dev/null

        echo ""
        if type -q gum
            gum spin --spinner line --title "Decrypting & mounting in memory (RAM)..." -- \
                openssl enc -d -aes-256-cbc -pbkdf2 -iter $_fb_vault_iterations -pass pass:"$pass" -in "$encrypted_file" -out "$tmp_dec_archive"
        else
            openssl enc -d -aes-256-cbc -pbkdf2 -iter $_fb_vault_iterations -pass pass:"$pass" -in "$encrypted_file" -out "$tmp_dec_archive" 2>/dev/null
        fi

        set -l dec_ok 1
        if test -s "$tmp_dec_archive"
            tar -xzf "$tmp_dec_archive" -C "$ram_base" 2>/dev/null
            if test $status -eq 0; and test -d "$ram_dir"
                set dec_ok 0
            fi
        fi
        _fb_vault_secure_delete "$tmp_dec_archive"

        if test $dec_ok -eq 0
            printf '\033[1;32m  [✓] "%s" unlocked successfully! (Loaded in RAM)\033[0m\n' "$selected_folder"

            test -L "$vault_dir"; or test -d "$vault_dir"; and rm -rf "$vault_dir" 2>/dev/null
            ln -s "$ram_dir" "$vault_dir"

            _fb_vault_open_explorer "$vault_dir"

            printf '\033[1;36m  [!] Directory opened in file explorer. (Encrypted backup preserved in ~/.secret_vaults/)\033[0m\n'

            echo ""
            set -l relock_choice ""
            if type -q gum
                set relock_choice (gum choose "Interactive Vault Session (Auto-relock on exit)" "10-Minute Auto-Relock Timer" "Relock Now")
                if test $status -ne 0; or test -z "$relock_choice"
                    printf '\033[1;33m  [!] Selection cancelled (ESC). Relocking vault...\033[0m\n'
                    _fb_vault_close_session "$selected_folder"
                    return 0
                end
            else
                printf 'Select Relock Mode:\n'
                printf '  1) Interactive Vault Session (Auto-relock on exit/terminal close)\n'
                printf '  2) 10-Minute Auto-Relock Timer\n'
                printf '  3) Relock Now\n'
                read -l -P "Enter choice [1-3] (default: 1): " relock_choice < /dev/tty
                test -z "$relock_choice"; and set relock_choice "1"
            fi

            if string match -ri ".*session.*" "$relock_choice"; or test "$relock_choice" = "1"
                printf '\033[1;32m  [🔓] Entering Vault Shell Session. Type "exit" or close terminal to auto-relock.\033[0m\n'
                begin
                    function __fb_vault_cleanup_on_exit --on-event fish_exit --inherit-variable selected_folder
                        _fb_vault_close_session "$selected_folder"
                    end
                    set -l user_shell "$SHELL"
                    test -z "$user_shell"; and set user_shell fish
                    $user_shell
                end
                _fb_vault_close_session "$selected_folder"
                return 0
            else if string match -ri ".*10-minute.*" "$relock_choice"; or test "$relock_choice" = "2"
                printf '\033[1;33m  [*] 10-minute auto-relock background timer started...\033[0m\n'
                sh -c "sleep 600 && (test -L '$vault_dir' && rm -f '$vault_dir'); (test -d '$ram_dir' && rm -rf '$ram_dir')" >/dev/null 2>&1 &
                return 0
            else
                _fb_vault_close_session "$selected_folder"
                printf '\033[1;32m  [✓] "%s" relocked & RAM purged.\033[0m\n' "$selected_folder"
                return 0
            fi
        else
            rm -rf "$ram_dir" 2>/dev/null
            set attempts (math $attempts + 1)
            set remaining (math $_fb_vault_max_attempts - $attempts)
            set -l delay (math $attempts \* 3)

            printf '\033[1;31m  [X] Invalid Password!\033[0m\n'

            if test $remaining -gt 0
                printf '\033[1;33m  [!] Anti-brute-force delay: Waiting %d seconds...\033[0m\n' "$delay"
                sleep $delay
            fi
        fi
    end

    # Self-Destruct Sequence
    clear
    printf '\033[1;31m🚨 [ALERT] Failed password attempts limit reached (%d/3)!\033[0m\n' "$_fb_vault_max_attempts"
    printf '\033[1;31mSELF-DESTRUCT: Permanently wiping "%s"...\033[0m\n' "$selected_folder"

    _fb_vault_send_alert "3 Failed attempts on $selected_folder! Self-destruct activated."

    _fb_vault_secure_delete "$encrypted_file"
    _fb_vault_secure_delete "$panic_hash_file"
    test -L "$vault_dir"; and rm -f "$vault_dir"
    _fb_vault_secure_delete "$vault_dir"
    _fb_vault_secure_delete "$ram_dir"

    printf '\033[1;32m  [✓] All vault data securely destroyed.\033[0m\n'
    return 1
end

function _fb_vault_list
    _fb_vault_ensure_store
    _fb_vault_banner
    printf '\033[1;36m📋 Encrypted Vaults List:\033[0m\n\n'
    set -l count 0
    set -l ram_base (_fb_vault_detect_ram_base)
    for f in "$_fb_vault_store"/*.enc
        if test -f "$f"
            set -l name (basename "$f" .enc)
            set -l size (du -sh "$f" 2>/dev/null | cut -f1)
            set -l ram_dir "$ram_base/$name"
            if test -d "$ram_dir"
                printf '  • 🔓 \033[1;32m%-20s\033[0m (Size: %-8s) \033[1;36m[UNLOCKED in RAM]\033[0m\n' "$name" "$size"
            else
                printf '  • 🔒 \033[1;33m%-20s\033[0m (Size: %-8s) \033[1;30m[LOCKED]\033[0m\n' "$name" "$size"
            fi
            set count (math $count + 1)
        fi
    end
    if test $count -eq 0
        printf '  \033[1;30m(No encrypted vaults found)\033[0m\n'
    fi
    printf '\n'
    read -l -P "Press Enter to continue..." _ < /dev/tty
end

function _fb_vault_config
    _fb_vault_banner
    printf '\033[1;36m⚙️  Telegram Alert Configuration:\033[0m\n\n'
    read -l -P "Telegram Bot Token: " token < /dev/tty
    read -l -P "Telegram Chat ID: " chat_id < /dev/tty

    if test -n "$token"; and test -n "$chat_id"
        set -gx FB_VAULT_TELEGRAM_BOT_TOKEN "$token"
        set -gx FB_VAULT_TELEGRAM_CHAT_ID "$chat_id"
        printf '\033[1;32m[✓] Telegram alert configuration saved successfully!\033[0m\n'
    else
        printf '\033[1;33m[!] Telegram configuration left empty.\033[0m\n'
    fi
    read -l -P "Press Enter to continue..." _ < /dev/tty
end

function _fb_vault_delete
    _fb_vault_ensure_store
    _fb_vault_banner

    # Collect available locked vaults
    set -l available_vaults
    for f in "$_fb_vault_store"/*.enc
        if test -f "$f"
            set available_vaults $available_vaults (basename "$f" .enc)
        end
    end

    if test (count $available_vaults) -eq 0
        printf '\033[1;31m  [!] No locked vaults found to delete!\033[0m\n'
        read -l -P "Press Enter to continue..." _ < /dev/tty
        return 0
    end

    # Let user select vault
    set -l selected_folder ""
    if type -q fzf
        set selected_folder (printf '%s\n' $available_vaults | \
            fzf --height=10 --layout=reverse --border=rounded \
                --prompt="🗑️  Select vault to DELETE > " \
                --color="border:196,header:220,pointer:196,fg:255")
    else if type -q gum
        set selected_folder (gum choose --header="🗑️  Select vault to DELETE:" $available_vaults)
    else
        printf 'Available Vaults:\n'
        for v in $available_vaults
            printf '  - %s\n' "$v"
        end
        read -l -P "Enter vault folder name to delete: " selected_folder < /dev/tty
    end

    test -z "$selected_folder"; and return 0

    set -l encrypted_file "$_fb_vault_store/$selected_folder.enc"
    set -l panic_hash_file "$_fb_vault_store/$selected_folder.panic"
    set -l ram_base (_fb_vault_detect_ram_base)
    set -l tmp_dec "$ram_base/.tmp_verify_{$selected_folder}_%self.tar.gz"

    printf '\n'
    if type -q gum
        gum style --foreground 196 --bold --border rounded --border-foreground 196 \
            --padding "0 2" --width 58 \
            "⚠️  WARNING: Permanently delete vault '$selected_folder'?" \
            "This action CANNOT be undone!"
    else
        printf '\033[1;31m  ⚠️  WARNING: This will PERMANENTLY delete vault "%s"!\033[0m\n' "$selected_folder"
        printf '\033[1;31m  This action CANNOT be undone!\033[0m\n'
    end
    printf '\n'

    # Verify master password before deleting
    set -l pass (_fb_vault_fancy_input "Enter Master Password to CONFIRM Delete" "[$selected_folder]" "🗑️")
    if test -z "$pass"
        printf '\033[1;33m  [!] Password empty. Delete cancelled.\033[0m\n'
        read -l -P "Press Enter to continue..." _ < /dev/tty
        return 1
    end

    # Verify password by attempting decryption and testing tar archive integrity
    rm -f "$tmp_dec" 2>/dev/null
    openssl enc -d -aes-256-cbc -pbkdf2 -iter $_fb_vault_iterations \
        -pass pass:"$pass" -in "$encrypted_file" -out "$tmp_dec" 2>/dev/null

    set -l verify_ok 1
    if test -s "$tmp_dec"
        tar -tzf "$tmp_dec" >/dev/null 2>&1; and set verify_ok 0
    end
    rm -f "$tmp_dec" 2>/dev/null

    if test $verify_ok -ne 0
        printf '\033[1;31m  [X] Wrong password! Delete aborted. Vault is safe.\033[0m\n'
        read -l -P "Press Enter to continue..." _ < /dev/tty
        return 1
    end

    # Password verified — permanently destroy
    _fb_vault_secure_delete "$encrypted_file"
    if test -f "$panic_hash_file"
        _fb_vault_secure_delete "$panic_hash_file"
    end

    printf '\033[1;32m  [✓] Vault "%s" permanently deleted and securely wiped.\033[0m\n' "$selected_folder"
    _fb_vault_send_alert "Vault $selected_folder permanently deleted by user."
    read -l -P "Press Enter to continue..." _ < /dev/tty
end

function vault
    _fb_ensure_dep openssl openssl openssl openssl; or return 1
    _fb_ensure_dep tar tar tar tar; or return 1

    set -l action $argv[1]

    switch "$action"
        case lock
            _fb_vault_lock $argv[2]
        case unlock
            _fb_vault_unlock $argv[2]
        case list
            _fb_vault_list
        case delete remove
            _fb_vault_delete
        case config
            _fb_vault_config
        case --help -h help
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
        case ""
            while true
                _fb_vault_banner
                set -l choice ""
                if type -q fzf
                    set choice (printf "🔒 Lock Directory\n🔓 Unlock Directory\n📋 List Vaults\n⚙️ Telegram Config\n❌ Exit" | \
                        fzf --height=10 \
                            --layout=reverse \
                            --border=rounded \
                            --margin=1,2 \
                            --prompt="⚡ Select Option > " \
                            --color="border:99,header:220,pointer:212,fg:255")
                else if type -q gum
                    set choice (gum choose "🔒 Lock Directory" "🔓 Unlock Directory" "📋 List Vaults" "⚙️ Telegram Config" "❌ Exit")
                else
                    printf '1) 🔒 Lock Directory\n2) 🔓 Unlock Directory\n3) 📋 List Vaults\n4) ⚙️ Telegram Config\n5) ❌ Exit\n'
                    read -l -P "Select option: " choice < /dev/tty
                fi

                switch "$choice"
                    case "*Lock Directory*" 1
                        _fb_vault_lock
                    case "*Unlock Directory*" 2
                        _fb_vault_unlock
                    case "*List Vaults*" 3
                        _fb_vault_list
                    case "*Delete Vault*" 4
                        _fb_vault_delete
                    case "*Telegram Config*" 5
                        _fb_vault_config
                    case "*Exit*" 6
                        break
                    case "*"
                        break
                end
            end
        case "*"
            printf '❌ Invalid command: %s. Run "vault --help" for details.\n' "$action"
            return 1
    end
end

# Aliases for Hardened Multi-Vault Manager
alias secvault='vault'
alias fvault='vault'
alias fancy_vault='vault'

# =====================================================
# End of config.fish
# =====================================================

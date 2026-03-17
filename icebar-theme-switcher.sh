#!/usr/bin/env bash

# ╭─────────────────────────────────────────────────────────────────────────╮
# │   ICEBAR THEME SWITCHER                                                 │
# │   Lists themes in ./themes, lets you pick one,                          │
# │   and installs its config.ron to ~/.config/icebar/                      │
# │                                                                         │
# │   Flags:                                                                │
# │     --no-exit      -n          Loop back to theme list after install.   │
# │     --force        -f          Skip confirmation prompts.               │
# │     --cycle        -c          Cycle through all themes one by one.     │
# │     --workspace    -w  <wm>    Bypass workspace picker. <wm>: Sway,     │
# │                                Hypr, Niri, or None.                     │
# │     --focused      -fw <wm>    Bypass focused window picker. <wm>:      │
# │                                Sway, Hypr, Niri, or None.               │
# │     --help         -h          Show this help message and exit.          │
# ╰─────────────────────────────────────────────────────────────────────────╯

# ── Flags ──────────────────────────────────────────────────────────────────
NO_EXIT=false
FORCE=false
CYCLE=false
BYPASS_WM=""    # Sway | Hypr | Niri | None
BYPASS_FW=""    # Sway | Hypr | Niri | None

# ── Help ───────────────────────────────────────────────────────────────────
print_help()
{
    echo
    echo -e "${CYAN}${BOLD}  icebar-theme-switcher${RESET}"
    echo
    echo -e "  ${BWHITE}Usage:${RESET}"
    echo -e "    ${DIM}./icebar-theme-switcher.sh [flags]${RESET}"
    echo
    echo -e "  ${BWHITE}Flags:${RESET}"
    echo -e "    ${CYAN}-n${RESET}, ${CYAN}--no-exit${RESET}              Loop back to the theme list after installing."
    echo -e "    ${CYAN}-f${RESET}, ${CYAN}--force${RESET}                Skip all confirmation prompts."
    echo -e "    ${CYAN}-c${RESET}, ${CYAN}--cycle${RESET}                Cycle through every theme one by one."
    echo -e "    ${CYAN}-w${RESET}, ${CYAN}--workspace${RESET}  ${DIM}<wm>${RESET}      Bypass workspace module picker."
    echo -e "    ${CYAN}-fw${RESET}, ${CYAN}--focused${RESET}   ${DIM}<wm>${RESET}      Bypass focused window module picker."
    echo -e "    ${CYAN}-h${RESET}, ${CYAN}--help${RESET}                 Show this help message and exit."
    echo
    echo -e "  ${BWHITE}<w> and ${BWHITE}<fw> values:${RESET}  ${WHITE}Sway${RESET}  ${WHITE}Hypr${RESET}  ${WHITE}Niri${RESET}  ${WHITE}None${RESET}"
    echo
    echo -e "  ${BWHITE}Examples:${RESET}"
    echo -e "    ${DIM}./icebar-theme-switcher.sh${RESET}"
    echo -e "    ${DIM}./icebar-theme-switcher.sh --cycle -w Hypr -fw Hypr${RESET}"
    echo -e "    ${DIM}./icebar-theme-switcher.sh --force --workspace Niri${RESET}"
    echo -e "    ${DIM}./icebar-theme-switcher.sh -n${RESET}"
    echo
}

# Resolve a wm shorthand to the matching module name fragment
# resolve_wm_arg <arg> <type>  — type is "workspace" or "focused"
resolve_wm_arg()
{
    local val="${1,,}"   # lowercase
    local type="$2"
    case "$val" in
        sway)  echo "Sway" ;;
        hypr)  echo "Hypr" ;;
        niri)  echo "Niri" ;;
        none)  echo "None" ;;
        *)
            echo -e "  ${RED}${BOLD}✗${RESET}  Unknown ${type} compositor '${1}'. Valid values: Sway, Hypr, Niri, None." >&2
            exit 1
            ;;
    esac
}

i=1
while [[ $i -le $# ]]; do
    arg="${!i}"
    case "$arg" in
        --help|-h)      print_help; exit 0 ;;
        --no-exit|-n)   NO_EXIT=true ;;
        --force|-f)     FORCE=true   ;;
        --cycle|-c)     CYCLE=true   ;;
        --workspace|-w)
            i=$(( i + 1 ))
            [[ $i -gt $# ]] && { echo -e "  ${RED}${BOLD}✗${RESET}  --workspace/-w requires a value (Sway, Hypr, Niri, None)." >&2; exit 1; }
            BYPASS_WM="$(resolve_wm_arg "${!i}" "workspace")"
            ;;
        --focused|-fw)
            i=$(( i + 1 ))
            [[ $i -gt $# ]] && { echo -e "  ${RED}${BOLD}✗${RESET}  --focused/-fw requires a value (Sway, Hypr, Niri, None)." >&2; exit 1; }
            BYPASS_FW="$(resolve_wm_arg "${!i}" "focused window")"
            ;;
    esac
    i=$(( i + 1 ))
done

# ── Paths ──────────────────────────────────────────────────────────────────
THEMES_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/themes"
ICEBAR_DIR="$HOME/.config/icebar"
ICEBAR_CONFIG="$ICEBAR_DIR/config.ron"

# ── Colors ─────────────────────────────────────────────────────────────────
RESET='\033[0m'
BOLD='\033[1m'
DIM='\033[2m'
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
WHITE='\033[0;37m'
BWHITE='\033[1;37m'

# ── Helpers ────────────────────────────────────────────────────────────────
print_header() {
    echo
    echo -e "${CYAN}${BOLD}  ╭──────────────────────────────────────────╮${RESET}"
    echo -e "${CYAN}${BOLD}  │         ICEBAR THEME SWITCHER            │${RESET}"
    echo -e "${CYAN}${BOLD}  ╰──────────────────────────────────────────╯${RESET}"
    echo
}

print_success() { echo -e "  ${GREEN}${BOLD}✓${RESET}  $1"; }
print_error()   { echo -e "  ${RED}${BOLD}✗${RESET}  $1"; }
print_info()    { echo -e "  ${CYAN}→${RESET}  $1"; }
print_warn()    { echo -e "  ${YELLOW}!${RESET}  $1"; }

divider() { echo -e "  ${DIM}──────────────────────────────────────────${RESET}"; }

# ── Generic module picker ──────────────────────────────────────────────────
# Usage: pick_module <found_module> <label> <modules_array...>
# Sets CHOSEN_MODULE and CHOSEN_MODULE_LABEL, or empty string if "none" picked.
pick_module() {
    local found_module="$1"
    local section_label="$2"
    shift 2
    local modules=("$@")

    # Build labels and descriptions from module names
    local labels=()
    local descriptions=()
    for m in "${modules[@]}"; do
        local wm
        case "$m" in
            *Sway*)  wm="Sway"     ;;
            *Hypr*)  wm="Hyprland" ;;
            *Niri*)  wm="Niri"     ;;
            *)       wm=""         ;;
        esac
        labels+=("$wm")
        if [[ -n "$wm" ]]; then
            descriptions+=("${m}  ${DIM}(${wm})${RESET}")
        else
            descriptions+=("${m}")
        fi
    done

    echo
    divider
    echo
    echo -e "  ${MAGENTA}${BOLD}  ${section_label} Detected${RESET}"
    echo
    print_warn "This theme uses the ${BOLD}${found_module}${RESET} module."
    print_info "These modules are window-manager specific and only work"
    print_info "in their respective compositors."
    echo
    echo -e "  ${BWHITE}Which module would you like to use?${RESET}"
    echo
    for i in "${!modules[@]}"; do
        local marker="  "
        if [[ "${modules[$i]}" == "$found_module" ]]; then
            marker="${GREEN}${BOLD}*${RESET} "
        fi
        local num
        num=$(printf "%2d" $((i + 1)))
        echo -e "  ${DIM}${num}.${RESET}  ${marker}${descriptions[$i]}"
    done
    local none_num=$(( ${#modules[@]} + 1 ))
    echo -e "  ${DIM}${none_num}.${RESET}    ${WHITE}Continue without module${RESET}"
    echo
    echo -e "  ${DIM}(${GREEN}*${RESET}${DIM} = currently set in this theme)${RESET}"
    echo

    # ── Bypass via flag (--workspace / --focused / --force) ─────────────────
    local bypass=""
    if [[ "$_PICK_BYPASS" == "None" ]]; then
        CHOSEN_MODULE=""
        CHOSEN_MODULE_LABEL=""
        print_info "Bypassed: no module selected."
        return
    elif [[ -n "$_PICK_BYPASS" ]]; then
        # Find the module whose name contains the bypass fragment
        for i in "${!modules[@]}"; do
            if [[ "${modules[$i]}" == *"${_PICK_BYPASS}"* ]]; then
                bypass="${modules[$i]}"
                CHOSEN_MODULE="$bypass"
                CHOSEN_MODULE_LABEL="${labels[$i]}"
                print_info "Bypassed: using ${CYAN}${BOLD}${CHOSEN_MODULE}${RESET}"
                return
            fi
        done
        print_warn "Bypass value '${_PICK_BYPASS}' did not match any module — falling through to prompt."
    elif [[ "$FORCE" == true ]]; then
        # In force mode keep the theme's existing module
        CHOSEN_MODULE="$found_module"
        CHOSEN_MODULE_LABEL="${labels[0]}"
        for i in "${!modules[@]}"; do
            if [[ "${modules[$i]}" == "$found_module" ]]; then
                CHOSEN_MODULE_LABEL="${labels[$i]}"
                break
            fi
        done
        print_info "Force mode: keeping existing module ${CYAN}${BOLD}${CHOSEN_MODULE}${RESET}"
        return
    fi

    while true; do
        echo -ne "  ${BWHITE}Choose a module ${DIM}[1-${none_num}]${RESET}${BWHITE} or ${DIM}[q]${RESET}${BWHITE} to quit:${RESET} "
        read -r mod_selection

        if [[ "$mod_selection" =~ ^[qQ]$ ]]; then
            echo
            print_info "Aborted. No changes made."
            echo
            exit 0
        fi

        if [[ "$mod_selection" =~ ^[0-9]+$ ]]; then
            if (( mod_selection >= 1 && mod_selection <= ${#modules[@]} )); then
                CHOSEN_MODULE="${modules[$((mod_selection - 1))]}"
                CHOSEN_MODULE_LABEL="${labels[$((mod_selection - 1))]}"
                return
            elif (( mod_selection == none_num )); then
                CHOSEN_MODULE=""
                CHOSEN_MODULE_LABEL=""
                print_info "Continuing without module."
                return
            fi
        fi

        print_warn "Invalid choice. Enter a number between 1 and ${none_num}, or q to quit."
    done
}

# ── Main loop (repeated when --no-exit / -n or --cycle / -c is set) ───────
CYCLE_INDEX=0
while true; do

# ── Sanity checks ──────────────────────────────────────────────────────────
if [[ ! -d "$THEMES_DIR" ]]; then
    print_header
    print_error "Themes folder not found: ${BOLD}$THEMES_DIR${RESET}"
    print_info  "Create a folder called ${BOLD}themes${RESET} next to this script"
    print_info  "and place theme subdirectories inside it, each containing a ${BOLD}config.ron${RESET}"
    echo
    exit 1
fi

# ── Collect themes ─────────────────────────────────────────────────────────
mapfile -t THEMES < <(
    for dir in "$THEMES_DIR"/*/; do
        [[ -f "$dir/config.ron" ]] && basename "$dir"
    done | sort
)

if [[ ${#THEMES[@]} -eq 0 ]]; then
    print_header
    print_error "No themes found in ${BOLD}$THEMES_DIR${RESET}"
    print_info  "Each theme must be a subdirectory containing a ${BOLD}config.ron${RESET} file"
    echo
    exit 1
fi

# ── Cycle mode: auto-select next theme ────────────────────────────────────
if [[ "$CYCLE" == true ]]; then
    if (( CYCLE_INDEX >= ${#THEMES[@]} )); then
        echo
        print_info "All ${#THEMES[@]} themes have been cycled through."
        echo
        exit 0
    fi
    CHOSEN_THEME="${THEMES[$CYCLE_INDEX]}"
    CHOSEN_CONFIG="$THEMES_DIR/$CHOSEN_THEME/config.ron"
    CYCLE_INDEX=$(( CYCLE_INDEX + 1 ))

    print_header
    echo -e "  ${DIM}Theme ${CYCLE_INDEX} of ${#THEMES[@]}${RESET}"
    echo
    divider
    echo
    echo -e "  ${BWHITE}Selected:${RESET}  ${CYAN}${BOLD}${CHOSEN_THEME}${RESET}"
    echo
else

# ── Display theme list ─────────────────────────────────────────────────────
print_header

echo -e "  ${BWHITE}Available themes:${RESET}"
echo
for i in "${!THEMES[@]}"; do
    num=$(printf "%2d" $((i + 1)))
    echo -e "  ${DIM}${num}.${RESET}  ${WHITE}${THEMES[$i]}${RESET}"
done

echo
divider
echo

# ── Prompt selection ───────────────────────────────────────────────────────
while true; do
    echo -ne "  ${BWHITE}Select a theme ${DIM}[1-${#THEMES[@]}]${RESET}${BWHITE} or ${DIM}[q]${RESET}${BWHITE} to quit:${RESET} "
    read -r selection

    if [[ "$selection" =~ ^[qQ]$ ]]; then
        echo
        print_info "Aborted. No changes made."
        echo
        exit 0
    fi

    if [[ "$selection" =~ ^[0-9]+$ ]] &&
       (( selection >= 1 && selection <= ${#THEMES[@]} )); then
        CHOSEN_THEME="${THEMES[$((selection - 1))]}"
        CHOSEN_CONFIG="$THEMES_DIR/$CHOSEN_THEME/config.ron"
        break
    fi

    print_warn "Invalid choice. Enter a number between 1 and ${#THEMES[@]}, or q to quit."
done

fi  # end of cycle/manual selection

# ── Workspace module detection ─────────────────────────────────────────────
WM_MODULES=("SwayWorkspaces" "HyprWorkspaces" "NiriWorkspaces")

FOUND_WM_MODULE=""
for module in "${WM_MODULES[@]}"; do
    if grep -v "^[[:space:]]*//" "$CHOSEN_CONFIG" | grep -q "modules:.*$module"; then
        FOUND_WM_MODULE="$module"
        break
    fi
done

CHOSEN_WM_MODULE=""
CHOSEN_WM_LABEL=""

if [[ -n "$FOUND_WM_MODULE" ]]; then
    _PICK_BYPASS="$BYPASS_WM" pick_module "$FOUND_WM_MODULE" "Workspace Module" "${WM_MODULES[@]}"
    CHOSEN_WM_MODULE="$CHOSEN_MODULE"
    CHOSEN_WM_LABEL="$CHOSEN_MODULE_LABEL"
    if [[ -n "$CHOSEN_WM_MODULE" ]]; then
        echo
        print_info "Workspace module: ${CYAN}${BOLD}${CHOSEN_WM_MODULE}${RESET}${CHOSEN_WM_LABEL:+  ${DIM}(${CHOSEN_WM_LABEL})${RESET}}"
    fi
fi

# ── FocusedWindow module detection ─────────────────────────────────────────
FW_MODULES=("FocusedWindowSway" "FocusedWindowHypr" "FocusedWindowNiri")

FOUND_FW_MODULE=""
for module in "${FW_MODULES[@]}"; do
    if grep -v "^[[:space:]]*//" "$CHOSEN_CONFIG" | grep -q "modules:.*$module"; then
        FOUND_FW_MODULE="$module"
        break
    fi
done

CHOSEN_FW_MODULE=""
CHOSEN_FW_LABEL=""

if [[ -n "$FOUND_FW_MODULE" ]]; then
    _PICK_BYPASS="$BYPASS_FW" pick_module "$FOUND_FW_MODULE" "FocusedWindow Module" "${FW_MODULES[@]}"
    CHOSEN_FW_MODULE="$CHOSEN_MODULE"
    CHOSEN_FW_LABEL="$CHOSEN_MODULE_LABEL"
    if [[ -n "$CHOSEN_FW_MODULE" ]]; then
        echo
        print_info "FocusedWindow module: ${CYAN}${BOLD}${CHOSEN_FW_MODULE}${RESET}${CHOSEN_FW_LABEL:+  ${DIM}(${CHOSEN_FW_LABEL})${RESET}}"
    fi
fi

echo
divider
echo

# ── Ensure icebar config dir exists ───────────────────────────────────────
mkdir -p "$ICEBAR_DIR"

# ── Handle existing config ─────────────────────────────────────────────────
if [[ -f "$ICEBAR_CONFIG" ]]; then
    if [[ "$FORCE" == true || "$CYCLE" == true ]]; then
        # Force/cycle mode: overwrite silently
        print_info "Overwriting existing config."
    else
        print_warn "A config already exists at ${BOLD}$ICEBAR_CONFIG${RESET}"
        echo
        echo -e "  ${BWHITE}What would you like to do?${RESET}"
        echo
        echo -e "  ${DIM}1.${RESET}  ${WHITE}Create a backup and install the new theme${RESET}"
        echo -e "  ${DIM}2.${RESET}  ${WHITE}Overwrite the current config${RESET}"
        echo -e "  ${DIM}q.${RESET}  ${WHITE}Cancel${RESET}"
        echo

        while true; do
            echo -ne "  ${BWHITE}Choose an option ${DIM}[1/2/q]:${RESET} "
            read -r conflict_choice

            case "$conflict_choice" in

                1)
                    TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
                    BACKUP_PATH="$ICEBAR_DIR/config.ron.backup_$TIMESTAMP"
                    if cp "$ICEBAR_CONFIG" "$BACKUP_PATH"; then
                        echo
                        print_success "Backup created: ${DIM}$BACKUP_PATH${RESET}"
                    else
                        echo
                        print_error "Failed to create backup. Aborting."
                        echo
                        exit 1
                    fi
                    break
                    ;;

                2)
                    echo
                    print_warn "${BOLD}${RED}This will permanently overwrite your current config.${RESET}"
                    echo
                    echo -ne "  ${BWHITE}Are you sure you want to overwrite? ${DIM}[yes/no]:${RESET} "
                    read -r confirm

                    if [[ "$confirm" == "yes" ]]; then
                        echo
                        print_info "Proceeding with overwrite..."
                        break
                    else
                        echo
                        print_info "Overwrite cancelled. No changes made."
                        echo
                        exit 0
                    fi
                    ;;

                [qQ])
                    echo
                    print_info "Cancelled. No changes made."
                    echo
                    exit 0
                    ;;

                *)
                    print_warn "Invalid option. Enter 1, 2, or q."
                    ;;
            esac
        done
    fi
fi

# ── Copy the chosen config ─────────────────────────────────────────────────
echo
if cp "$CHOSEN_CONFIG" "$ICEBAR_CONFIG"; then

    # ── Patch workspace module ─────────────────────────────────────────────
    # Sed is scoped to lines containing *_modules: to avoid touching comments
    # or other config keys that might contain the same module name string.
    if [[ -n "$FOUND_WM_MODULE" ]]; then
        if [[ -z "$CHOSEN_WM_MODULE" ]]; then
            # Strip only the token from the modules line, preserving everything else
            sed -i "/modules:.*${FOUND_WM_MODULE}/{s/,[ ]*${FOUND_WM_MODULE}//g;s/${FOUND_WM_MODULE}[ ]*,[ ]*//g;s/${FOUND_WM_MODULE}//g}" "$ICEBAR_CONFIG"
            print_info "Workspace module removed: ${DIM}${FOUND_WM_MODULE}${RESET}"
        elif [[ "$CHOSEN_WM_MODULE" != "$FOUND_WM_MODULE" ]]; then
            sed -i "/modules:/s/${FOUND_WM_MODULE}/${CHOSEN_WM_MODULE}/g" "$ICEBAR_CONFIG"
            print_info "Workspace module replaced: ${DIM}${FOUND_WM_MODULE}${RESET} → ${CYAN}${BOLD}${CHOSEN_WM_MODULE}${RESET}"
        fi
    fi

    # ── Patch FocusedWindow module ─────────────────────────────────────────
    if [[ -n "$FOUND_FW_MODULE" ]]; then
        if [[ -z "$CHOSEN_FW_MODULE" ]]; then
            sed -i "/modules:.*${FOUND_FW_MODULE}/{s/,[ ]*${FOUND_FW_MODULE}//g;s/${FOUND_FW_MODULE}[ ]*,[ ]*//g;s/${FOUND_FW_MODULE}//g}" "$ICEBAR_CONFIG"
            print_info "FocusedWindow module removed: ${DIM}${FOUND_FW_MODULE}${RESET}"
        elif [[ "$CHOSEN_FW_MODULE" != "$FOUND_FW_MODULE" ]]; then
            sed -i "/modules:/s/${FOUND_FW_MODULE}/${CHOSEN_FW_MODULE}/g" "$ICEBAR_CONFIG"
            print_info "FocusedWindow module replaced: ${DIM}${FOUND_FW_MODULE}${RESET} → ${CYAN}${BOLD}${CHOSEN_FW_MODULE}${RESET}"
        fi
    fi

    divider
    echo
    print_success "${BOLD}Theme installed successfully!${RESET}"
    echo
    print_info "Theme:   ${CYAN}${BOLD}${CHOSEN_THEME}${RESET}"
    [[ -n "$CHOSEN_WM_MODULE" ]] && \
    print_info "Workspaces:    ${CYAN}${BOLD}${CHOSEN_WM_MODULE}${RESET}${CHOSEN_WM_LABEL:+  ${DIM}(${CHOSEN_WM_LABEL})${RESET}}"
    [[ -n "$CHOSEN_FW_MODULE" ]] && \
    print_info "FocusedWindow: ${CYAN}${BOLD}${CHOSEN_FW_MODULE}${RESET}${CHOSEN_FW_LABEL:+  ${DIM}(${CHOSEN_FW_LABEL})${RESET}}"
    print_info "Config:  ${DIM}$ICEBAR_CONFIG${RESET}"
    echo
    divider
    echo
    echo -e "  ${DIM}If you have 'bar_check_reload_interval_ms' set to 'None', you will need to restart icebar to apply the new theme.${RESET}"
    echo

    # ── Loop back or exit ──────────────────────────────────────────────────
    if [[ "$CYCLE" == true ]]; then
        if (( CYCLE_INDEX < ${#THEMES[@]} )); then
            echo
            echo -ne "  ${BWHITE}Try next theme (${THEMES[$CYCLE_INDEX]})? ${DIM}[y/n/q]:${RESET} "
            read -r cycle_ans
            case "${cycle_ans,,}" in
                y|"") echo; continue ;;
                q)    echo; print_info "Stopped cycling."; echo; exit 0 ;;
                *)
                    echo
                    print_info "Keeping current theme. Goodbye."
                    echo
                    exit 0
                    ;;
            esac
        fi
    elif [[ "$NO_EXIT" == true ]]; then
        echo -ne "  ${BWHITE}Switch to another theme? ${DIM}[y/n]:${RESET} "
        read -r again
        if [[ "$again" =~ ^[yY]$ ]]; then
            echo
            continue
        else
            echo
            print_info "Goodbye."
            echo
            exit 0
        fi
    fi

else
    echo
    print_error "Failed to copy config. Check permissions for ${BOLD}$ICEBAR_DIR${RESET}"
    echo
    exit 1
fi

break
done

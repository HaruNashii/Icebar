#!/usr/bin/env bash

# ╭─────────────────────────────────────────────────────────────────────────╮
# │   ICEBAR THEME SWITCHER                                                 │
# │   Lists themes in ./themes, lets you pick one,                 	    │
# │   and installs its config.ron to ~/.config/icebar/                      │
# ╰─────────────────────────────────────────────────────────────────────────╯

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

echo
divider
echo
echo -e "  ${BWHITE}Selected:${RESET}  ${CYAN}${BOLD}${CHOSEN_THEME}${RESET}"
echo

# ── Workspace module detection ─────────────────────────────────────────────
# Check which workspace module(s) exist in the chosen theme's config
WM_MODULES=("SwayWorkspaces" "HyprWorkspaces" "NiriWorkspaces")
WM_LABELS=("Sway" "Hyprland" "Niri")
WM_DESCRIPTIONS=(
    "SwayWorkspaces  ${DIM}(Sway)${RESET}"
    "HyprWorkspaces  ${DIM}(Hyprland)${RESET}"
    "NiriWorkspaces  ${DIM}(Niri)${RESET}"
)

FOUND_MODULE=""
for module in "${WM_MODULES[@]}"; do
    if grep -q "$module" "$CHOSEN_CONFIG"; then
        FOUND_MODULE="$module"
        break
    fi
done

CHOSEN_WM_MODULE=""   # will hold the module name to write into the final config

if [[ -n "$FOUND_MODULE" ]]; then
    echo
    divider
    echo
    echo -e "  ${MAGENTA}${BOLD}  Workspace Module Detected${RESET}"
    echo
    print_warn "This theme uses the ${BOLD}${FOUND_MODULE}${RESET} module."
    print_info "Workspace modules are window-manager specific and only work"
    print_info "in their respective compositors."
    echo
    echo -e "  ${BWHITE}Which workspace module would you like to use?${RESET}"
    echo
    for i in "${!WM_MODULES[@]}"; do
        marker="  "
        # Highlight the one currently in the file
        if [[ "${WM_MODULES[$i]}" == "$FOUND_MODULE" ]]; then
            marker="${GREEN}${BOLD}*${RESET} "
        fi
        num=$(printf "%2d" $((i + 1)))
        echo -e "  ${DIM}${num}.${RESET}  ${marker}${WM_DESCRIPTIONS[$i]}"
    done
    echo
    echo -e "  ${DIM}(${GREEN}*${RESET}${DIM} = currently set in this theme)${RESET}"
    echo

    while true; do
        echo -ne "  ${BWHITE}Choose a module ${DIM}[1-${#WM_MODULES[@]}]${RESET}${BWHITE} or ${DIM}[q]${RESET}${BWHITE} to quit:${RESET} "
        read -r wm_selection

        if [[ "$wm_selection" =~ ^[qQ]$ ]]; then
            echo
            print_info "Aborted. No changes made."
            echo
            exit 0
        fi

        if [[ "$wm_selection" =~ ^[0-9]+$ ]] &&
           (( wm_selection >= 1 && wm_selection <= ${#WM_MODULES[@]} )); then
            CHOSEN_WM_MODULE="${WM_MODULES[$((wm_selection - 1))]}"
            CHOSEN_WM_LABEL="${WM_LABELS[$((wm_selection - 1))]}"
            break
        fi

        print_warn "Invalid choice. Enter a number between 1 and ${#WM_MODULES[@]}, or q to quit."
    done

    echo
    print_info "Workspace module: ${CYAN}${BOLD}${CHOSEN_WM_MODULE}${RESET}  ${DIM}(${CHOSEN_WM_LABEL})${RESET}"
fi

echo
divider
echo

# ── Ensure icebar config dir exists ───────────────────────────────────────
mkdir -p "$ICEBAR_DIR"

# ── Handle existing config ─────────────────────────────────────────────────
if [[ -f "$ICEBAR_CONFIG" ]]; then
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
                # ── Backup ────────────────────────────────────────────────
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
                # ── Overwrite confirmation ────────────────────────────────
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

# ── Copy the chosen config ─────────────────────────────────────────────────
echo
if cp "$CHOSEN_CONFIG" "$ICEBAR_CONFIG"; then

    # ── Patch workspace module if the user picked one ──────────────────────
    if [[ -n "$CHOSEN_WM_MODULE" && -n "$FOUND_MODULE" && "$CHOSEN_WM_MODULE" != "$FOUND_MODULE" ]]; then
        # Replace every occurrence of the old module name with the new one
        sed -i "s/${FOUND_MODULE}/${CHOSEN_WM_MODULE}/g" "$ICEBAR_CONFIG"
        print_info "Workspace module replaced: ${DIM}${FOUND_MODULE}${RESET} → ${CYAN}${BOLD}${CHOSEN_WM_MODULE}${RESET}"
    fi

    divider
    echo
    print_success "${BOLD}Theme installed successfully!${RESET}"
    echo
    print_info "Theme:   ${CYAN}${BOLD}${CHOSEN_THEME}${RESET}"
    [[ -n "$CHOSEN_WM_MODULE" ]] && \
    print_info "Module:  ${CYAN}${BOLD}${CHOSEN_WM_MODULE}${RESET}  ${DIM}(${CHOSEN_WM_LABEL})${RESET}"
    print_info "Config:  ${DIM}$ICEBAR_CONFIG${RESET}"
    echo
    divider
    echo
    echo -e "  ${DIM}If you have 'bar_check_reload_interval_ms' set to 'None', you will need to restart icebar to apply the new theme.${RESET}"
    echo
else
    echo
    print_error "Failed to copy config. Check permissions for ${BOLD}$ICEBAR_DIR${RESET}"
    echo
    exit 1
fi

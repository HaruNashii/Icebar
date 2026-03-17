#!/usr/bin/env bash
# Fixes color syntax in all theme config.ron files and src/helpers/fs.rs:
#   RGB(r, g, b)           -> RGB((r, g, b))
#   RGBA(r, g, b, a)       -> RGBA((r, g, b, a))
#   _side_separator_color: (r, g, b)  -> _side_separator_color: RGB((r, g, b))

set -euo pipefail

ROOT="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}"

echo "=== fix_colortype_syntax.sh ==="
echo "Root: $ROOT"
echo ""

CHANGED=0

fix_file()
{
    local file="$1"
    [[ -f "$file" ]] || return
    grep -qE '\bRGB\(|\bRGBA\(|separator_color:' "$file" 2>/dev/null || return

    perl -i -pe '
        s/\bRGB\((?!\()(\d[^)]*)\)/RGB((\1))/g;
        s/\bRGBA\((?!\()(\d[^)]*)\)/RGBA((\1))/g;
        s/(\w+_side_separator_color:\s*)\((\d[^)]*)\)/\1RGB((\2))/g;
    ' "$file"

    echo "  fixed: $file"
    CHANGED=$((CHANGED + 1))
}

while IFS= read -r -d '' f; do
    fix_file "$f"
done < <(find "$ROOT/themes" -name "config.ron" -print0 2>/dev/null)

fix_file "$ROOT/src/helpers/fs.rs"

LIVE_CONFIG="$HOME/.config/icebar/config.ron"
if [[ -f "$LIVE_CONFIG" ]]; then
    echo ""
    echo "Live config found at $LIVE_CONFIG"
    read -r -p "Fix it too? [y/N] " ans
    if [[ "${ans,,}" == "y" ]]; then
        fix_file "$LIVE_CONFIG"
    fi
fi

echo ""
if [[ $CHANGED -eq 0 ]]; then
    echo "Nothing to fix."
else
    echo "Done. $CHANGED file(s) updated."
fi

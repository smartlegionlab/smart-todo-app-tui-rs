#!/bin/bash
set -e

APP_ID="smart-todo-app-tui"
BIN_NAME="smart-todo-app-tui"
BIN_DIR="$HOME/.local/bin"
APPS_DIR="$HOME/.local/share/applications"
MENU_ENTRY="$APPS_DIR/$APP_ID.desktop"
DATA_DIR="$HOME/.local/share/$APP_ID"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

info()  { echo -e "${GREEN}==>${NC} $*"; }
warn()  { echo -e "${YELLOW}==>${NC} $*"; }
error() { echo -e "${RED}==>${NC} $*" >&2; }
step()  { echo -e "${BOLD}${BLUE}[*]${NC} $*"; }

banner() {
    echo ""
    echo -e "${BOLD}${RED}==============================================${NC}"
    echo -e "${BOLD}${RED}  Smart TODO TUI - Uninstaller${NC}"
    echo -e "${BOLD}${RED}==============================================${NC}"
    echo ""
}

banner

echo "Will remove:"
echo "  - Binary:      $BIN_DIR/$BIN_NAME"
echo "  - Menu entry:  $MENU_ENTRY"
echo ""
echo -e "${BOLD}${GREEN}Your tasks are NOT touched.${NC}"
echo "  $DATA_DIR/todos.json"
echo ""

step "Removing binary..."
if [ -f "$BIN_DIR/$BIN_NAME" ]; then
    rm -f "$BIN_DIR/$BIN_NAME"
    info "Removed: $BIN_DIR/$BIN_NAME"
else
    warn "Not found: $BIN_DIR/$BIN_NAME"
fi

step "Removing menu entry..."
if [ -f "$MENU_ENTRY" ]; then
    rm -f "$MENU_ENTRY"
    info "Removed: $MENU_ENTRY"
else
    warn "Not found: $MENU_ENTRY"
fi

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$APPS_DIR" >/dev/null 2>&1 || true
fi

echo ""
echo -e "${BOLD}${GREEN}  Uninstall complete${NC}"
echo ""
echo "  Kept: $DATA_DIR/todos.json"
echo "  To delete your tasks too:"
echo "    rm -rf $DATA_DIR"
echo ""
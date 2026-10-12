#!/bin/bash
set -e

REPO_URL="https://github.com/smartlegionlab/smart-todo-app-tui-rs.git"
APP_NAME="Smart TODO TUI"
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
    echo -e "${BOLD}${GREEN}==============================================${NC}"
    echo -e "${BOLD}${GREEN}  Smart TODO TUI - Installer${NC}"
    echo -e "${BOLD}${GREEN}==============================================${NC}"
    echo ""
}

banner

echo "This installer will:"
echo "  1. Download the application source code"
echo "  2. Build it with cargo (requires Rust toolchain)"
echo "  3. Install the binary to:  $BIN_DIR/$BIN_NAME"
echo "  4. Register the app in your application menu"
echo ""
echo "Your tasks will be stored in:"
echo "  $DATA_DIR/todos.json"
echo ""

step "Checking cargo..."
if ! command -v cargo >/dev/null 2>&1; then
    error "cargo is required but not installed."
    error "Install Rust from https://rustup.rs and try again."
    exit 1
fi
CARGO_VER="$(cargo --version 2>&1)"
info "Found: $CARGO_VER"

step "Checking git..."
if ! command -v git >/dev/null 2>&1; then
    error "git is required but not installed."
    exit 1
fi
info "git found."

step "Cloning repository..."
TMP_CLONE="$(mktemp -d)"
info "From: $REPO_URL"
info "Into: $TMP_CLONE"
git clone --depth 1 "$REPO_URL" "$TMP_CLONE" >/dev/null 2>&1
info "Repository cloned."

step "Building release binary..."
pushd "$TMP_CLONE" >/dev/null
cargo build --release >/dev/null 2>&1
popd >/dev/null
info "Build finished."

step "Installing binary..."
mkdir -p "$BIN_DIR"
install -m 755 "$TMP_CLONE/target/release/$BIN_NAME" "$BIN_DIR/$BIN_NAME"
info "Binary installed: $BIN_DIR/$BIN_NAME"

step "Preparing data directory..."
mkdir -p "$DATA_DIR"
info "Data directory: $DATA_DIR"

step "Registering application in the menu..."
mkdir -p "$APPS_DIR"

DESKTOP_CONTENT="[Desktop Entry]
Version=1.0
Type=Application
Name=$APP_NAME
Comment=A smart TUI todo app written in Rust
Exec=$BIN_DIR/$BIN_NAME
Icon=utilities-terminal
Terminal=true
Categories=Utility;
StartupNotify=true
Keywords=todo;tasks;tui;terminal;
"

echo "$DESKTOP_CONTENT" > "$MENU_ENTRY"
chmod +x "$MENU_ENTRY"
info "Menu entry created: $MENU_ENTRY"

rm -rf "$TMP_CLONE"
info "Temporary clone removed."

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$APPS_DIR" >/dev/null 2>&1 || true
fi

echo ""
echo -e "${BOLD}${GREEN}==============================================${NC}"
echo -e "${BOLD}${GREEN}  Installation complete${NC}"
echo -e "${BOLD}${GREEN}==============================================${NC}"
echo ""
echo "  Binary:          $BIN_DIR/$BIN_NAME"
echo "  Menu entry:      $MENU_ENTRY"
echo "  Tasks file:      $DATA_DIR/todos.json"
echo ""
echo "  Launch from terminal:"
echo "    $BIN_NAME"
echo ""
echo "  Or from the application menu: \"$APP_NAME\""
echo ""
echo "  If '$BIN_DIR' is not in your PATH, add it:"
echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
echo ""
echo "  Uninstall:"
echo "    curl -fsSL https://raw.githubusercontent.com/smartlegionlab/smart-todo-app-tui-rs/master/uninstall.sh | bash"
echo ""
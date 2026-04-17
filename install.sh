#!/usr/bin/env sh
# UniLang Installer Bootstrap
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/AIWithHitesh/unilang/main/install.sh | sh
#   curl -fsSL ... | sh -s -- --lite
#   curl -fsSL ... | sh -s -- --full
#   curl -fsSL ... | sh -s -- --path ~/.local/bin
#
# Flags (all optional — omitting them launches the interactive wizard):
#   --lite            Install UniLang Lite edition (no wizard)
#   --full            Install UniLang Full edition (no wizard)
#   --path <dir>      Override install directory
#   --version <tag>   Install a specific version (e.g. v0.1.0)
#   --list-drivers    List available driver groups and exit
#   --help            Show this help

set -eu

REPO="AIWithHitesh/unilang"
INSTALLER_BASE="https://github.com/${REPO}/releases"

# ── Colour helpers (silently degraded when not a terminal) ────────────────────
tty_bold=""   ; tty_reset=""  ; tty_cyan=""
tty_green=""  ; tty_yellow="" ; tty_red=""
if [ -t 1 ]; then
    tty_bold=$(  printf '\033[1m'  )
    tty_reset=$( printf '\033[0m'  )
    tty_cyan=$(  printf '\033[36m' )
    tty_green=$( printf '\033[32m' )
    tty_yellow=$(printf '\033[33m' )
    tty_red=$(   printf '\033[31m' )
fi

ohai() { printf "${tty_bold}${tty_cyan}==>${tty_reset}${tty_bold} %s${tty_reset}\n" "$@"; }
ok()   { printf "${tty_green}  ✓${tty_reset} %s\n" "$@"; }
warn() { printf "${tty_yellow}  ⚠${tty_reset} %s\n" "$@"; }
err()  { printf "${tty_red}  ✗${tty_reset} %s\n" "$@" >&2; }
die()  { err "$@"; exit 1; }

# ── Detect OS and architecture ────────────────────────────────────────────────
detect_target() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)

    case "${OS}" in
        linux*)  OS_TAG="linux"  ;;
        darwin*) OS_TAG="macos"  ;;
        *)       die "Unsupported OS: ${OS}. Please install manually." ;;
    esac

    case "${ARCH}" in
        x86_64 | amd64)  ARCH_TAG="x86_64"   ;;
        arm64  | aarch64) ARCH_TAG="aarch64"  ;;
        *)                die "Unsupported architecture: ${ARCH}." ;;
    esac

    echo "${ARCH_TAG}-${OS_TAG}"
}

# ── Download a file with progress ─────────────────────────────────────────────
download() {
    URL="$1"
    DEST="$2"
    if command -v curl >/dev/null 2>&1; then
        curl -fSL --progress-bar -o "${DEST}" "${URL}"
    elif command -v wget >/dev/null 2>&1; then
        wget -q --show-progress -O "${DEST}" "${URL}"
    else
        die "Neither curl nor wget is available. Please install one and retry."
    fi
}

# ── Fetch latest release tag from GitHub API ──────────────────────────────────
latest_tag() {
    API_URL="https://api.github.com/repos/${REPO}/releases/latest"
    if command -v curl >/dev/null 2>&1; then
        TAG=$(curl -fsSL "${API_URL}" | tr ',' '\n' | grep '"tag_name"' | head -1 \
              | sed 's/.*"tag_name": *"\(.*\)".*/\1/')
    elif command -v wget >/dev/null 2>&1; then
        TAG=$(wget -qO- "${API_URL}" | tr ',' '\n' | grep '"tag_name"' | head -1 \
              | sed 's/.*"tag_name": *"\(.*\)".*/\1/')
    fi
    if [ -z "${TAG:-}" ]; then
        warn "Could not determine latest release from GitHub API. Using v0.1.0."
        TAG="v0.1.0"
    fi
    echo "${TAG}"
}

# ── Print banner ──────────────────────────────────────────────────────────────
print_banner() {
    printf "\n"
    printf "${tty_bold}${tty_cyan}"
    printf "  ╔══════════════════════════════════════════════════════╗\n"
    printf "  ║          UniLang Installer                           ║\n"
    printf "  ║          The Universal Programming Language          ║\n"
    printf "  ║          https://github.com/AIWithHitesh/unilang     ║\n"
    printf "  ╚══════════════════════════════════════════════════════╝\n"
    printf "${tty_reset}\n"
}

# ── Main ──────────────────────────────────────────────────────────────────────
main() {
    # Forward all script arguments to the installer binary
    FORWARD_ARGS=""
    for arg in "$@"; do
        FORWARD_ARGS="${FORWARD_ARGS} ${arg}"
    done

    # Show help without downloading anything
    if echo "${FORWARD_ARGS}" | grep -q -- "--help"; then
        print_banner
        printf "Usage:\n"
        printf "  install.sh [--lite|--full] [--path DIR] [--version TAG]\n\n"
        printf "Flags:\n"
        printf "  --lite             Install Lite edition (no wizard)\n"
        printf "  --full             Install Full edition (no wizard)\n"
        printf "  --path <dir>       Override install directory\n"
        printf "  --version <tag>    Install specific version (e.g. v0.1.0)\n"
        printf "  --list-drivers     List available drivers and exit\n"
        exit 0
    fi

    print_banner

    TARGET=$(detect_target)
    ok "Detected platform: ${TARGET}"

    # Determine version to install
    VERSION_ARG=""
    for arg in "$@"; do
        if [ "${PREV_ARG:-}" = "--version" ]; then
            VERSION_ARG="${arg}"
        fi
        PREV_ARG="${arg}"
    done
    if [ -n "${VERSION_ARG}" ]; then
        TAG="${VERSION_ARG}"
    else
        ohai "Fetching latest release info..."
        TAG=$(latest_tag)
    fi
    ok "Version: ${TAG}"

    # Download the installer binary
    INSTALLER_URL="${INSTALLER_BASE}/download/${TAG}/unilang-installer-${TARGET}"
    TMPDIR=$(mktemp -d)
    INSTALLER_PATH="${TMPDIR}/unilang-installer"

    ohai "Downloading installer for ${TARGET}..."
    if ! download "${INSTALLER_URL}" "${INSTALLER_PATH}" 2>/dev/null; then
        warn "Release binary not yet published for ${TARGET}."
        warn "To build from source: cargo install --git https://github.com/${REPO} unilang-installer"
        rm -rf "${TMPDIR}"
        exit 1
    fi

    chmod +x "${INSTALLER_PATH}"
    ok "Installer downloaded"

    # Run the installer, forwarding all flags
    ohai "Launching interactive installer..."
    # shellcheck disable=SC2086
    exec "${INSTALLER_PATH}" ${FORWARD_ARGS}
}

main "$@"

# Installation

This guide walks you through installing UniLang on macOS, Windows, and Linux. By the end, you will have the `unilang` command-line tool ready to use.

---

## macOS

### Option 1: Download DMG (Recommended)

1. Go to the [GitHub Releases](https://github.com/hiteshkumar2187/unilang/releases) page.
2. Download the correct DMG for your Mac:
   - **Apple Silicon** (M1/M2/M3/M4): `unilang-cli-macos-arm64.dmg`
   - **Intel**: `unilang-cli-macos-x86_64.dmg`
   - Not sure? Click Apple menu > "About This Mac." If it says "Apple M1" (or M2/M3/M4), choose Apple Silicon.
3. Open the downloaded `.dmg` file.
4. Copy the binaries to `/usr/local/bin/`:
   ```bash
   sudo cp /Volumes/UniLang/unilang /usr/local/bin/
   sudo cp /Volumes/UniLang/unilang-lsp /usr/local/bin/
   ```
5. Eject the DMG volume.
6. If macOS blocks the app ("unidentified developer"):
   - Go to **System Settings** > **Privacy & Security**.
   - Click **"Open Anyway"** next to the UniLang message.
7. Verify:
   ```bash
   unilang --version
   ```

### Option 2: Download tar.gz

1. Download the correct archive from [Releases](https://github.com/hiteshkumar2187/unilang/releases):
   - Apple Silicon: `unilang-cli-macos-arm64.tar.gz`
   - Intel: `unilang-cli-macos-x86_64.tar.gz`
2. Extract and install:
   ```bash
   cd ~/Downloads
   tar xzf unilang-cli-macos-*.tar.gz
   sudo cp unilang-cli-*/bin/* /usr/local/bin/
   ```
3. Verify:
   ```bash
   unilang --version
   ```

### Option 3: Homebrew (Coming Soon)

Homebrew support is planned for a future release. Use Option 1 or 2 in the meantime.

---

## Windows

### Download ZIP Archive

1. Go to [GitHub Releases](https://github.com/hiteshkumar2187/unilang/releases) and download `unilang-cli-windows-x86_64.zip`.
2. Right-click the ZIP and select **"Extract All..."**.
3. Extract to a permanent location such as:
   ```
   C:\Program Files\UniLang\
   ```
4. Add UniLang to your system PATH:
   - Press the **Windows key**, type **"Environment Variables"**, and open it.
   - Under **System variables**, find **Path**, click **Edit**.
   - Click **New** and add:
     ```
     C:\Program Files\UniLang\bin
     ```
   - Click **OK** on all dialogs.
5. Open a **new** Command Prompt or PowerShell window.
6. Verify:
   ```cmd
   unilang --version
   ```

---

## Linux

### Download tar.gz

These steps work on Ubuntu, Debian, Fedora, Arch, and most other distributions.

1. Download `unilang-cli-linux-x86_64.tar.gz` from [Releases](https://github.com/hiteshkumar2187/unilang/releases).
   - ARM-based systems (e.g., Raspberry Pi 4 64-bit): use `unilang-cli-linux-aarch64.tar.gz` if available.
2. Extract and install:
   ```bash
   cd ~/Downloads
   tar xzf unilang-cli-linux-x86_64.tar.gz
   sudo cp unilang-cli-*/bin/* /usr/local/bin/
   ```
3. Verify:
   ```bash
   unilang --version
   ```

---

## Build from Source (All Platforms)

Building from source requires **Rust 1.75 or later**.

### 1. Install Rust (if needed)

Go to [https://rustup.rs](https://rustup.rs) and follow the instructions:

- **macOS / Linux:**
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  ```
- **Windows:** Download and run `rustup-init.exe` from the website.

Verify Rust is installed:
```bash
rustc --version    # Should show 1.75.0 or later
```

### 2. Clone and build

```bash
git clone https://github.com/hiteshkumar2187/unilang.git
cd unilang
cargo build --release
```

### 3. Install the binaries

**macOS / Linux:**
```bash
sudo cp target/release/unilang-cli /usr/local/bin/unilang
sudo cp target/release/unilang-lsp /usr/local/bin/unilang-lsp
```

**Windows (run as Administrator):**
```cmd
copy target\release\unilang-cli.exe "C:\Program Files\UniLang\bin\unilang.exe"
copy target\release\unilang-lsp.exe "C:\Program Files\UniLang\bin\unilang-lsp.exe"
```

### 4. Verify

```bash
unilang --version
```

---

## Verify Installation

After installing via any method, confirm everything works:

```bash
# Check version
unilang --version
# Expected: unilang 0.1.0

# Check help
unilang --help

# Run a quick test
echo 'print("UniLang is installed!")' > test.uniL
unilang run test.uniL
# Expected: UniLang is installed!
```

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| `command not found: unilang` | Ensure binary is on your PATH. macOS/Linux: check `/usr/local/bin/unilang` exists. Windows: verify PATH includes the bin directory. |
| Permission denied (macOS/Linux) | Use `sudo` when copying to `/usr/local/bin/` |
| macOS Gatekeeper blocks the app | System Settings > Privacy & Security > "Open Anyway" |
| Rust build fails ("edition 2021 not supported") | Run `rustup update` then rebuild |
| Still stuck? | [Open an issue](https://github.com/hiteshkumar2187/unilang/issues) with your OS, command, and error message |

---

**Next**: [[Quick Start]] | [[IDE Setup]]

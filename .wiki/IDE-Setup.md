# IDE Setup

UniLang has editor support for VS Code, Cursor, IntelliJ, PyCharm, Eclipse, and a standalone IDE. This page covers setup for all of them.

---

## VS Code / Cursor

The UniLang extension provides syntax highlighting, code snippets, bracket matching, and optional language server support.

### Step-by-step

1. Go to [GitHub Releases](https://github.com/hiteshkumar2187/unilang/releases) and download `unilang-vscode.vsix`.
2. Install via command line:
   ```bash
   code --install-extension ~/Downloads/unilang-vscode.vsix
   ```
   Or via the UI: open Command Palette (`Cmd+Shift+P` / `Ctrl+Shift+P`), select **"Extensions: Install from VSIX..."**, and pick the file.
3. Restart VS Code.
4. Open or create a `.uniL` file to confirm syntax highlighting works.

### Configure the Language Server (Optional)

The language server provides real-time error highlighting and diagnostics.

1. Verify `unilang-lsp` is installed:
   ```bash
   which unilang-lsp    # macOS/Linux
   where unilang-lsp    # Windows
   ```
2. Open VS Code Settings (`Cmd+,` / `Ctrl+,`), search for `unilang`, and set **"UniLang: LSP Path"** to the binary path (e.g., `/usr/local/bin/unilang-lsp`).
3. Alternatively, add to `settings.json`:
   ```json
   {
       "unilang.lsp.path": "/usr/local/bin/unilang-lsp"
   }
   ```
4. Restart VS Code.

### Cursor IDE

Cursor is built on VS Code, so the same extension works without modifications. Use `cursor` instead of `code` when installing via command line:

```bash
cursor --install-extension ~/Downloads/unilang-vscode.vsix
```

### Snippet Prefixes

| Prefix | Description |
|--------|-------------|
| `defpy` | Python-style function |
| `defjava` | Java-style method |
| `classpy` | Python class with `__init__` |
| `classjava` | Java class with constructor |
| `ifpy` / `ifjava` | If block (Python / Java style) |
| `for` / `forj` | For loop (Python / Java style) |
| `try` / `trycatch` | Try/except / Try/catch block |
| `main` | Java main method |
| `mlmodel` | ML model skeleton |

---

## IntelliJ IDEA / PyCharm (JetBrains)

The plugin works with all JetBrains IDEs: IntelliJ IDEA, PyCharm, WebStorm, CLion, GoLand, Rider, and others. Requires IDE version 2023.1 or later.

### Step-by-step

1. Go to [GitHub Releases](https://github.com/hiteshkumar2187/unilang/releases) and download `unilang-jetbrains.zip`.
2. **Do not unzip the file.** The IDE installs directly from the `.zip`.
3. Open your JetBrains IDE.
4. Go to **Settings** (or **Preferences** on macOS) > **Plugins**.
5. Click the **gear icon** at the top and select **"Install Plugin from Disk..."**.
6. Select the downloaded `unilang-jetbrains.zip` and click **OK**.
7. Click **OK** to close Settings.
8. Restart the IDE when prompted.
9. Create a `test.uniL` file to verify syntax highlighting.

### Features

- Python keywords displayed in blue, Java keywords in purple
- Strings, numbers, and comments each highlighted distinctly
- Real-time error annotations (unmatched brackets, TODO/FIXME highlights)
- Code completion for all UniLang keywords
- Custom file icon for `.uniL` files

---

## Eclipse

Requires Eclipse version 2023-03 or later.

### Step-by-step (Dropins Method)

1. Go to [GitHub Releases](https://github.com/hiteshkumar2187/unilang/releases) and download `unilang-eclipse.jar`.
2. Find the `dropins` folder inside your Eclipse installation directory:
   - **macOS:** Right-click `Eclipse.app` > "Show Package Contents" > `Contents/Eclipse/dropins/`
   - **Windows:** Typically `C:\eclipse\dropins\`
   - **Linux:** Typically `~/eclipse/dropins/`
3. Copy `unilang-eclipse.jar` into the `dropins` folder:
   ```bash
   # macOS example
   cp ~/Downloads/unilang-eclipse.jar /Applications/Eclipse.app/Contents/Eclipse/dropins/

   # Linux example
   cp ~/Downloads/unilang-eclipse.jar ~/eclipse/dropins/
   ```
4. Restart Eclipse.
5. Create a `test.uniL` file to verify syntax highlighting.

### Alternative: Install via Eclipse UI

1. Open Eclipse > **Help** > **"Install New Software..."**.
2. Click **"Add..."** > **"Archive..."** and browse to the downloaded JAR.
3. Check **"UniLang Support"** and click **Next** through the wizard.
4. Accept the license and click **Finish**.
5. Restart Eclipse.

### Customizing Colors

Go to **Window** > **Preferences** (or **Eclipse** > **Preferences** on macOS) > **UniLang** > **Syntax Coloring** to adjust highlight colors for your theme.

---

## UniLang IDE (Standalone)

UniLang IDE is a standalone editor built specifically for UniLang development. Everything works out of the box with no plugins or configuration.

### Downloads

| Platform | File | Size |
|----------|------|------|
| macOS | `UniLang-IDE.dmg` | ~80 MB |
| Windows | `UniLang-IDE-Setup.exe` | ~60 MB |
| Linux | `UniLang-IDE.AppImage` | ~70 MB |

### macOS Installation

1. Download `UniLang-IDE.dmg` from [Releases](https://github.com/hiteshkumar2187/unilang/releases).
2. Open the DMG and drag **"UniLang IDE"** to **Applications**.
3. Launch from Applications. If macOS blocks it, go to **System Settings** > **Privacy & Security** > **"Open Anyway"**.

### Windows Installation

1. Download and run `UniLang-IDE-Setup.exe`.
2. If SmartScreen blocks it, click **"More info"** then **"Run anyway"**.
3. Follow the installation wizard.

### Linux Installation

1. Download `UniLang-IDE.AppImage`.
2. Make it executable and run:
   ```bash
   chmod +x UniLang-IDE.AppImage
   ./UniLang-IDE.AppImage
   ```
3. For system-wide installation:
   ```bash
   sudo mkdir -p /opt/unilang-ide
   sudo mv UniLang-IDE.AppImage /opt/unilang-ide/
   sudo chmod +x /opt/unilang-ide/UniLang-IDE.AppImage
   sudo ln -s /opt/unilang-ide/UniLang-IDE.AppImage /usr/local/bin/unilang-ide
   ```

### Features

- Full syntax highlighting for `.uniL` files
- Integrated terminal
- File tree panel
- Build, Run, and Check toolbar buttons
- Keyboard shortcuts: `Cmd/Ctrl+R` (run), `Cmd/Ctrl+B` (build), `Cmd/Ctrl+S` (save)

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl/Cmd + R` | Run current file |
| `Ctrl/Cmd + B` | Build (compile) |
| `Ctrl/Cmd + S` | Save |
| `Ctrl/Cmd + N` | New file |
| `Ctrl/Cmd + F` | Find in file |
| `Ctrl/Cmd + /` | Toggle comment |
| `Ctrl/Cmd + +/-` | Increase/decrease font size |
| `` Ctrl/Cmd + ` `` | Toggle terminal |

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| No syntax highlighting | Ensure file ends with `.uniL` (case-sensitive). Restart the editor. |
| Language server not starting | Run `unilang-lsp --version` in terminal. Check LSP path in settings. |
| Eclipse plugin not loading | Verify JAR is in the correct `dropins` directory (inside `.app` on macOS). |
| JetBrains plugin install fails | Check IDE version is 2023.1+. Do not unzip the `.zip` file. |
| UniLang IDE blocked on macOS | System Settings > Privacy & Security > "Open Anyway" |
| UniLang IDE fails on Linux | Install FUSE: `sudo apt install libfuse2` (Ubuntu/Debian) |

---

**Previous**: [[Quick Start]] | **Next**: [[Language Overview]]

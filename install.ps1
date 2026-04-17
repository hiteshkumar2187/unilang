# UniLang Installer Bootstrap for Windows (PowerShell)
# Usage:
#   irm https://raw.githubusercontent.com/AIWithHitesh/unilang/main/install.ps1 | iex
#
# Or with flags:
#   & ([scriptblock]::Create((irm 'https://.../install.ps1'))) --lite
#   & ([scriptblock]::Create((irm 'https://.../install.ps1'))) --full
#   & ([scriptblock]::Create((irm 'https://.../install.ps1'))) --path 'C:\tools'
#
# Flags (all optional — omitting them launches the interactive TUI wizard):
#   --lite             Install UniLang Lite edition (no wizard)
#   --full             Install UniLang Full edition (no wizard)
#   --path <dir>       Override install directory
#   --version <tag>    Install a specific version (e.g. v0.1.0)
#   --list-drivers     List available driver groups and exit

param(
    [switch]$Lite,
    [switch]$Full,
    [string]$Path    = "",
    [string]$Version = "",
    [switch]$ListDrivers,
    [switch]$Help
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$Repo       = "AIWithHitesh/unilang"
$ApiBase    = "https://api.github.com/repos/$Repo"
$ReleaseBase = "https://github.com/$Repo/releases/download"
$Target     = "x86_64-windows"   # Windows CI only builds x86_64

# ── Colour helpers ────────────────────────────────────────────────────────────
function Write-Banner {
    $c = [System.ConsoleColor]::Cyan
    $b = [System.ConsoleColor]::White
    Write-Host ""
    Write-Host "  ╔══════════════════════════════════════════════════════╗" -ForegroundColor $c
    Write-Host "  ║          UniLang Installer                           ║" -ForegroundColor $c
    Write-Host "  ║          The Universal Programming Language          ║" -ForegroundColor $c
    Write-Host "  ║          https://github.com/AIWithHitesh/unilang     ║" -ForegroundColor $c
    Write-Host "  ╚══════════════════════════════════════════════════════╝" -ForegroundColor $c
    Write-Host ""
}
function Write-Ok($msg)   { Write-Host "  ✓ $msg" -ForegroundColor Green  }
function Write-Warn($msg) { Write-Host "  ⚠ $msg" -ForegroundColor Yellow }
function Write-Step($msg) { Write-Host "==> $msg" -ForegroundColor Cyan   }
function Write-Err($msg)  { Write-Host "  ✗ $msg" -ForegroundColor Red    }

# ── Fetch latest release tag ──────────────────────────────────────────────────
function Get-LatestTag {
    try {
        $response = Invoke-RestMethod -Uri "$ApiBase/releases/latest" `
                        -Headers @{ "User-Agent" = "unilang-installer" }
        return $response.tag_name
    } catch {
        Write-Warn "Could not fetch latest release from GitHub API. Using v0.1.0."
        return "v0.1.0"
    }
}

# ── Download file with progress ───────────────────────────────────────────────
function Download-File($Url, $Dest) {
    Write-Step "Downloading from $Url"
    $ProgressPreference = "SilentlyContinue"   # Invoke-WebRequest is slow with progress
    try {
        Invoke-WebRequest -Uri $Url -OutFile $Dest -UseBasicParsing
    } catch {
        throw "Download failed: $_"
    }
    $ProgressPreference = "Continue"
}

# ── Main ──────────────────────────────────────────────────────────────────────
if ($Help) {
    Write-Banner
    Write-Host "Usage: install.ps1 [-Lite] [-Full] [-Path <dir>] [-Version <tag>]"
    Write-Host ""
    Write-Host "Flags:"
    Write-Host "  -Lite              Install Lite edition (no wizard)"
    Write-Host "  -Full              Install Full edition (no wizard)"
    Write-Host "  -Path <dir>        Override install directory"
    Write-Host "  -Version <tag>     Install specific version (e.g. v0.1.0)"
    Write-Host "  -ListDrivers       List available drivers and exit"
    exit 0
}

Write-Banner

# Determine version
if ($Version -ne "") {
    $Tag = $Version
} else {
    Write-Step "Fetching latest release info..."
    $Tag = Get-LatestTag
}
Write-Ok "Version: $Tag"
Write-Ok "Platform: $Target"

# Build installer download URL
$InstallerFile = "unilang-installer-$Target.exe"
$InstallerUrl  = "$ReleaseBase/$Tag/$InstallerFile"

# Download to temp
$TmpDir = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), "unilang-install-$([System.Guid]::NewGuid().ToString('N'))")
New-Item -ItemType Directory -Path $TmpDir | Out-Null
$InstallerPath = [System.IO.Path]::Combine($TmpDir, $InstallerFile)

try {
    Download-File $InstallerUrl $InstallerPath
    Write-Ok "Installer downloaded"
} catch {
    Write-Err "Could not download installer: $_"
    Write-Warn "Release binary may not be published yet for this version."
    Write-Warn "Build from source: cargo install --git https://github.com/$Repo unilang-installer"
    Remove-Item -Recurse -Force $TmpDir -ErrorAction SilentlyContinue
    exit 1
}

# Build argument list to forward to the installer
$ForwardArgs = @()
if ($Lite)        { $ForwardArgs += "--lite"          }
if ($Full)        { $ForwardArgs += "--full"          }
if ($Path -ne "") { $ForwardArgs += "--path"; $ForwardArgs += $Path    }
if ($Version -ne "") { $ForwardArgs += "--version"; $ForwardArgs += $Version }
if ($ListDrivers) { $ForwardArgs += "--list-drivers"  }

# Run the installer
Write-Step "Launching interactive installer..."
try {
    & $InstallerPath @ForwardArgs
    $ExitCode = $LASTEXITCODE
} catch {
    Write-Err "Failed to run installer: $_"
    $ExitCode = 1
}

# Cleanup temp dir
Remove-Item -Recurse -Force $TmpDir -ErrorAction SilentlyContinue

exit $ExitCode

use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};

use crate::download::{self, fetch_latest_release, verify_checksum};
use crate::install::{self, path_hint};

const GITHUB_REPO: &str = "AIWithHitesh/unilang";

// ─── Driver definitions ───────────────────────────────────────────────────────

struct DriverGroup {
    label: &'static str,
    drivers: &'static [Driver],
}

struct Driver {
    name: &'static str,
    desc: &'static str,
    lite_default: bool,
}

/// All driver groups shown in the Custom picker.
fn driver_groups() -> Vec<DriverGroup> {
    vec![
        DriverGroup {
            label: "── SQL Databases ──────────────────────────────────────",
            drivers: &[
                Driver {
                    name: "SQLite",
                    desc: "bundled, zero-config (always included)",
                    lite_default: true,
                },
                Driver {
                    name: "MySQL",
                    desc: "requires MySQL/MariaDB server",
                    lite_default: false,
                },
                Driver {
                    name: "PostgreSQL",
                    desc: "requires PostgreSQL server",
                    lite_default: false,
                },
            ],
        },
        DriverGroup {
            label: "── NoSQL / Document ────────────────────────────────────",
            drivers: &[Driver {
                name: "MongoDB",
                desc: "document store",
                lite_default: true,
            }],
        },
        DriverGroup {
            label: "── Cache ───────────────────────────────────────────────",
            drivers: &[
                Driver {
                    name: "Redis",
                    desc: "in-memory cache & pub/sub",
                    lite_default: true,
                },
                Driver {
                    name: "Memcached",
                    desc: "simple in-memory cache",
                    lite_default: false,
                },
            ],
        },
        DriverGroup {
            label: "── Messaging / Streaming ───────────────────────────────",
            drivers: &[
                Driver {
                    name: "Kafka (in-memory)",
                    desc: "zero-config, no broker needed",
                    lite_default: true,
                },
                Driver {
                    name: "RabbitMQ",
                    desc: "requires RabbitMQ broker",
                    lite_default: false,
                },
                Driver {
                    name: "NATS",
                    desc: "requires NATS server",
                    lite_default: false,
                },
            ],
        },
        DriverGroup {
            label: "── Search ──────────────────────────────────────────────",
            drivers: &[Driver {
                name: "Elasticsearch",
                desc: "full-text search cluster",
                lite_default: true,
            }],
        },
        DriverGroup {
            label: "── Email ───────────────────────────────────────────────",
            drivers: &[Driver {
                name: "SMTP",
                desc: "send emails via any SMTP server",
                lite_default: true,
            }],
        },
        DriverGroup {
            label: "── Time-series ─────────────────────────────────────────",
            drivers: &[Driver {
                name: "InfluxDB",
                desc: "time-series database",
                lite_default: false,
            }],
        },
        DriverGroup {
            label: "── Object Storage ──────────────────────────────────────",
            drivers: &[Driver {
                name: "Amazon S3",
                desc: "S3-compatible object storage (SigV4)",
                lite_default: false,
            }],
        },
        DriverGroup {
            label: "── Monitoring ──────────────────────────────────────────",
            drivers: &[Driver {
                name: "Prometheus",
                desc: "metrics collection & export",
                lite_default: false,
            }],
        },
        DriverGroup {
            label: "── Real-time ───────────────────────────────────────────",
            drivers: &[Driver {
                name: "WebSocket",
                desc: "WebSocket server & client",
                lite_default: false,
            }],
        },
    ]
}

// The canonical lite driver names (used when mapping Custom → binary edition).
const LITE_DRIVERS: &[&str] = &[
    "SQLite",
    "MongoDB",
    "Redis",
    "Kafka (in-memory)",
    "Elasticsearch",
    "SMTP",
];

// ─── Public API ───────────────────────────────────────────────────────────────

pub struct InstallOptions {
    pub edition: String,
    pub tag: String,
    pub target: String,
    pub install_dir: String,
}

/// Run the full interactive wizard. Returns `Ok(())` on success.
pub fn run() -> Result<(), String> {
    let opts = wizard(None, None, None)?;
    execute(opts)
}

/// Run in non-interactive mode (CI / scripted).
pub fn run_noninteractive(
    edition: &str,
    install_dir: Option<String>,
    version_tag: Option<String>,
) -> Result<(), String> {
    let release = match version_tag {
        Some(tag) => {
            let version = tag.trim_start_matches('v').to_string();
            download::ReleaseInfo { tag, version }
        }
        None => fetch_latest_release(GITHUB_REPO)?,
    };

    let dir = install_dir.unwrap_or_else(|| "/usr/local/bin".to_string());
    let target = download::detect_target();

    let opts = InstallOptions {
        edition: edition.to_string(),
        tag: release.tag,
        target: target.to_string(),
        install_dir: dir,
    };

    execute(opts)
}

/// Print the list of available driver groups and exit.
pub fn list_drivers() {
    println!("{}", style("Available driver groups:").bold().cyan());
    for group in driver_groups() {
        println!("\n  {}", style(group.label).dim());
        for d in group.drivers {
            let default_marker = if d.lite_default {
                style(" [Lite default]").green().to_string()
            } else {
                String::new()
            };
            println!(
                "    • {} — {}{}",
                style(d.name).bold(),
                d.desc,
                default_marker
            );
        }
    }
}

// ─── Wizard screens ───────────────────────────────────────────────────────────

fn wizard(
    forced_edition: Option<&str>,
    forced_dir: Option<String>,
    forced_tag: Option<String>,
) -> Result<InstallOptions, String> {
    print_banner();

    // Fetch release info early so we can display the version in the summary.
    let release = match forced_tag {
        Some(ref tag) => download::ReleaseInfo {
            tag: tag.clone(),
            version: tag.trim_start_matches('v').to_string(),
        },
        None => fetch_latest_release(GITHUB_REPO)?,
    };

    let target = download::detect_target();
    let theme = ColorfulTheme::default();

    // Check if stdin is a TTY; fall back to Lite + system path if not.
    let is_tty = console::Term::stdout().is_term();
    if !is_tty {
        eprintln!(
            "{}",
            style("Non-interactive terminal detected; using Lite edition + /usr/local/bin.").dim()
        );
        return Ok(InstallOptions {
            edition: "lite".to_string(),
            tag: release.tag,
            target: target.to_string(),
            install_dir: "/usr/local/bin".to_string(),
        });
    }

    // ── Screen 1: Edition ─────────────────────────────────────────────────────
    let edition_str = if let Some(e) = forced_edition {
        e.to_string()
    } else {
        let choices = &[
            "UniLang Lite   — core runtime + 6 essential drivers (recommended)",
            "UniLang Full   — all 15+ drivers pre-installed",
            "Custom         — pick exactly which drivers to include",
        ];
        let idx = Select::with_theme(&theme)
            .with_prompt("Choose your edition")
            .items(choices)
            .default(0)
            .interact()
            .map_err(|e| format!("Prompt error: {}", e))?;
        match idx {
            0 => "lite".to_string(),
            1 => "full".to_string(),
            _ => "custom".to_string(),
        }
    };

    // ── Screen 2: Driver selection (Custom only) ──────────────────────────────
    let resolved_edition = if edition_str == "custom" {
        pick_custom_drivers(&theme)?
    } else {
        edition_str
    };

    // ── Screen 3: Install location ────────────────────────────────────────────
    let install_dir = if let Some(d) = forced_dir {
        d
    } else {
        pick_install_dir(&theme)?
    };

    // ── Screen 4: Confirm ─────────────────────────────────────────────────────
    println!();
    println!(
        "  {}  UniLang {} v{}",
        style("Edition:").bold(),
        capitalise(&resolved_edition),
        release.version
    );
    println!("  {}  {}", style("Platform:").bold(), style(target).cyan());
    println!("  {}  {}/unilang", style("Install:").bold(), install_dir);
    println!();

    let proceed = Confirm::with_theme(&theme)
        .with_prompt("Proceed with installation?")
        .default(true)
        .interact()
        .map_err(|e| format!("Prompt error: {}", e))?;

    if !proceed {
        return Err("Installation cancelled by user.".to_string());
    }

    Ok(InstallOptions {
        edition: resolved_edition,
        tag: release.tag,
        target: target.to_string(),
        install_dir,
    })
}

/// Screen 2 — MultiSelect driver picker.
fn pick_custom_drivers(theme: &ColorfulTheme) -> Result<String, String> {
    let groups = driver_groups();

    // Flatten groups into (display_string, is_lite_default, name) tuples.
    // Insert group header rows that cannot be toggled (we handle them as
    // separator labels by prepending a non-selectable prefix).
    let mut items: Vec<String> = Vec::new();
    let mut defaults: Vec<bool> = Vec::new();
    let mut driver_names: Vec<Option<&str>> = Vec::new();

    for group in &groups {
        // Group header — shown but always off, we track it with None.
        items.push(format!("{}", style(group.label).dim()));
        defaults.push(false);
        driver_names.push(None);

        for d in group.drivers {
            items.push(format!("  {} — {}", style(d.name).bold(), d.desc));
            defaults.push(d.lite_default);
            driver_names.push(Some(d.name));
        }
    }

    let selections = MultiSelect::with_theme(theme)
        .with_prompt(
            "Select drivers to include (↑↓ navigate · space toggle · a toggle all · enter confirm)",
        )
        .items(&items)
        .defaults(&defaults)
        .interact()
        .map_err(|e| format!("Prompt error: {}", e))?;

    // Collect the names of selected drivers (skip header rows).
    let selected_names: Vec<&str> = selections
        .iter()
        .filter_map(|&idx| driver_names[idx])
        .collect();

    println!(
        "\n  {}",
        style("Note: Custom granular builds will be available in a future release.").dim()
    );

    // Map selection to closest binary.
    let all_lite = selected_names
        .iter()
        .all(|name| LITE_DRIVERS.contains(name));

    let edition = if all_lite { "lite" } else { "full" };
    println!("  Mapped to {} binary.\n", style(edition).bold().cyan());

    Ok(edition.to_string())
}

/// Screen 3 — Install directory.
fn pick_install_dir(theme: &ColorfulTheme) -> Result<String, String> {
    let choices = &[
        "/usr/local/bin   system-wide (recommended, may need sudo)",
        "~/.local/bin      user-local (no sudo needed)",
        "Custom path       enter manually",
    ];

    let idx = Select::with_theme(theme)
        .with_prompt("Choose installation directory")
        .items(choices)
        .default(0)
        .interact()
        .map_err(|e| format!("Prompt error: {}", e))?;

    match idx {
        0 => Ok("/usr/local/bin".to_string()),
        1 => {
            let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());
            Ok(format!("{}/.local/bin", home))
        }
        _ => {
            let path: String = Input::with_theme(theme)
                .with_prompt("Enter installation path")
                .validate_with(|p: &String| {
                    if p.trim().is_empty() {
                        Err("Path cannot be empty")
                    } else {
                        Ok(())
                    }
                })
                .interact_text()
                .map_err(|e| format!("Prompt error: {}", e))?;
            Ok(path.trim().to_string())
        }
    }
}

// ─── Execution ────────────────────────────────────────────────────────────────

fn execute(opts: InstallOptions) -> Result<(), String> {
    let url = download::binary_url(&opts.tag, &opts.edition, &opts.target);

    println!("\n  {} {}", style("Downloading:").bold(), style(&url).dim());

    // Stage into a temp directory.
    let tmp_dir = std::env::temp_dir();
    let ext = if opts.target.ends_with("windows") {
        "zip"
    } else {
        "tar.gz"
    };
    let archive_name = format!("unilang-{}-{}.{}", opts.edition, opts.target, ext);
    let archive_path = tmp_dir.join(&archive_name);
    let archive_str = archive_path.to_string_lossy().to_string();

    download::download_binary(&url, &archive_str)?;

    // Verify checksum.
    println!("  {} SHA-256 checksum…", style("Verifying").bold());
    match verify_checksum(&url, &archive_str) {
        Ok(()) => println!("  {}", style("Checksum OK").green()),
        Err(e) => {
            // Warn but don't abort — the release might not have a .sha256 yet.
            eprintln!("  {} (skipping): {}", style("Checksum warning").yellow(), e);
        }
    }

    // Extract.
    println!("  {} archive…", style("Extracting").bold());
    let extract_dir = tmp_dir.join(format!("unilang-extract-{}", opts.edition));
    let extract_str = extract_dir.to_string_lossy().to_string();

    let binary_path = if opts.target.ends_with("windows") {
        install::extract_zip(&archive_str, &extract_str)?
    } else {
        install::extract_targz(&archive_str, &extract_str)?
    };

    // Install.
    println!(
        "  {} to {}…",
        style("Installing").bold(),
        style(&opts.install_dir).cyan()
    );
    let installed = install::install_binary(&binary_path, &opts.install_dir)?;

    println!();
    println!(
        "  {} UniLang {} installed at {}",
        style("✓").green().bold(),
        style(format!("v{}", opts.tag.trim_start_matches('v'))).green(),
        style(&installed).green().bold()
    );

    // PATH hint — only show if the install dir isn't already on PATH.
    let path_env = std::env::var("PATH").unwrap_or_default();
    let canonical_dir = std::fs::canonicalize(&opts.install_dir)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| opts.install_dir.clone());

    if !path_env.split(':').any(|p| {
        let cp = std::fs::canonicalize(p)
            .map(|x| x.to_string_lossy().to_string())
            .unwrap_or_else(|_| p.to_string());
        cp == canonical_dir
    }) {
        println!();
        println!("  {}", style("PATH setup:").bold());
        for line in path_hint(&opts.install_dir).lines() {
            println!("    {}", line);
        }
    }

    println!();
    println!(
        "  Run {} to get started.",
        style("unilang --help").bold().cyan()
    );

    // Clean up temp files.
    let _ = std::fs::remove_file(&archive_str);
    let _ = std::fs::remove_dir_all(&extract_str);

    Ok(())
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn print_banner() {
    println!();
    println!(
        "{}",
        style(" ┌──────────────────────────────────────────────────────┐").cyan()
    );
    println!(
        "{}  {}  {}",
        style(" │").cyan(),
        style("UniLang Installer").bold().cyan(),
        style(format!(
            "                    v{}       ",
            env!("CARGO_PKG_VERSION")
        ))
        .cyan()
    );
    println!(
        "{}  {}  {}",
        style(" │").cyan(),
        style("The Universal Programming Language").cyan(),
        style("                 │").cyan()
    );
    println!(
        "{}  {}  {}",
        style(" │").cyan(),
        style("https://github.com/AIWithHitesh/unilang").dim(),
        style("          │").cyan()
    );
    println!(
        "{}",
        style(" └──────────────────────────────────────────────────────┘").cyan()
    );
    println!();
}

fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

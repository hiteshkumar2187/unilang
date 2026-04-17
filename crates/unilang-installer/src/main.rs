mod download;
mod install;
mod ui;

fn main() {
    // Print panics cleanly instead of the default backtrace dump.
    std::panic::set_hook(Box::new(|info| {
        eprintln!("installer error: {}", info);
    }));

    let args: Vec<String> = std::env::args().skip(1).collect();

    // ── --list-drivers ────────────────────────────────────────────────────────
    if args.iter().any(|a| a == "--list-drivers") {
        ui::list_drivers();
        return;
    }

    // ── --version ────────────────────────────────────────────────────────────
    if args.iter().any(|a| a == "--version" || a == "-V") {
        // If the next token after --version looks like a tag we print it;
        // otherwise print the crate version.
        let ver_arg = flag_value(&args, "--version");
        if let Some(tag) = ver_arg {
            println!(
                "unilang-installer {} (installing {})",
                env!("CARGO_PKG_VERSION"),
                tag
            );
        } else {
            println!("unilang-installer {}", env!("CARGO_PKG_VERSION"));
        }
        return;
    }

    // ── Non-interactive flags ─────────────────────────────────────────────────
    let is_lite = args.iter().any(|a| a == "--lite");
    let is_full = args.iter().any(|a| a == "--full");

    if is_lite || is_full {
        let edition = if is_full { "full" } else { "lite" };
        let install_dir = flag_value(&args, "--path");
        let version_tag = flag_value(&args, "--version");

        if let Err(e) = ui::run_noninteractive(edition, install_dir, version_tag) {
            eprintln!("Installation failed: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // ── Interactive wizard ────────────────────────────────────────────────────
    if let Err(e) = ui::run() {
        eprintln!("Installation failed: {}", e);
        std::process::exit(1);
    }
}

/// Return the string value following `flag` in the argument list, if present.
/// E.g. `flag_value(&args, "--path")` returns `Some("/usr/local/bin")` for
/// `["--path", "/usr/local/bin"]`.
fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
}

// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.

//! `unilang new` — interactive wizard and non-interactive scaffolding.

use dialoguer::{Confirm, Input, MultiSelect, Select};
use std::fs;
use std::path::{Path, PathBuf};

// ── All available driver names (used in wizard + non-interactive) ─────────────

const ALL_DRIVERS: &[&str] = &[
    "mysql",
    "postgres",
    "sqlite",
    "redis",
    "kafka",
    "email",
    "elasticsearch",
    "mongodb",
    "rabbitmq",
    "nats",
    "s3",
    "influxdb",
    "websocket",
];

// ── Public config struct ──────────────────────────────────────────────────────

struct ProjectConfig {
    name: String,
    project_type: String,
    deps: Vec<String>,
    project_dir: PathBuf,
    no_git: bool,
}

// ── Entry point ───────────────────────────────────────────────────────────────

/// Main handler for `unilang new`.
pub fn cmd_new(
    name: Option<String>,
    project_type: Option<String>,
    deps: Option<String>,
    path: Option<String>,
    no_git: bool,
    yes: bool,
) {
    let config = if yes || (name.is_some() && project_type.is_some() && deps.is_some()) {
        build_config_non_interactive(name, project_type, deps, path, no_git)
    } else {
        build_config_interactive(name, project_type, deps, path, no_git)
    };

    create_project(&config);
}

// ── Non-interactive path ──────────────────────────────────────────────────────

fn build_config_non_interactive(
    name: Option<String>,
    project_type: Option<String>,
    deps: Option<String>,
    path: Option<String>,
    no_git: bool,
) -> ProjectConfig {
    let name = name.unwrap_or_else(|| "my-project".to_string());
    let project_type = project_type.unwrap_or_else(|| "app".to_string());
    let deps: Vec<String> = deps
        .map(|d| {
            d.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();

    let base = path
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let project_dir = base.join(&name);

    ProjectConfig {
        name,
        project_type,
        deps,
        project_dir,
        no_git,
    }
}

// ── Interactive wizard ────────────────────────────────────────────────────────

fn build_config_interactive(
    name: Option<String>,
    project_type: Option<String>,
    deps: Option<String>,
    path: Option<String>,
    no_git: bool,
) -> ProjectConfig {
    println!("Welcome to the UniLang project wizard!\n");

    // Project name
    let name: String = match name {
        Some(n) => n,
        None => Input::new()
            .with_prompt("Project name")
            .default("my-project".to_string())
            .interact_text()
            .expect("Failed to read project name"),
    };

    // Project type
    let project_type: String = match project_type {
        Some(t) => t,
        None => {
            let types = &["web", "app", "cli"];
            let idx = Select::new()
                .with_prompt("Project type")
                .items(types)
                .default(0)
                .interact()
                .expect("Failed to read project type");
            types[idx].to_string()
        }
    };

    // Dependencies / drivers
    let deps: Vec<String> = match deps {
        Some(d) => d
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        None => {
            let selections = MultiSelect::new()
                .with_prompt("Select drivers (space to toggle, enter to confirm)")
                .items(ALL_DRIVERS)
                .interact()
                .expect("Failed to read driver selection");
            selections
                .into_iter()
                .map(|i| ALL_DRIVERS[i].to_string())
                .collect()
        }
    };

    // Parent directory
    let base: PathBuf = match path {
        Some(p) => PathBuf::from(p),
        None => {
            let current = std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .display()
                .to_string();
            let entered: String = Input::new()
                .with_prompt("Parent directory")
                .default(current)
                .interact_text()
                .expect("Failed to read parent directory");
            PathBuf::from(entered)
        }
    };

    // Git init?
    let no_git = if no_git {
        true
    } else {
        !Confirm::new()
            .with_prompt("Initialise a git repository?")
            .default(true)
            .interact()
            .expect("Failed to read git preference")
    };

    let project_dir = base.join(&name);

    ProjectConfig {
        name,
        project_type,
        deps,
        project_dir,
        no_git,
    }
}

// ── File generation ───────────────────────────────────────────────────────────

fn create_project(config: &ProjectConfig) {
    let dir = &config.project_dir;

    // Create directory tree
    let src_dir = dir.join("src");
    fs::create_dir_all(&src_dir).unwrap_or_else(|e| {
        eprintln!(
            "error: cannot create directory '{}': {}",
            src_dir.display(),
            e
        );
        std::process::exit(1);
    });

    // Write each file
    write_file(&dir.join("unilang.toml"), &render_toml(config));
    write_file(&src_dir.join("main.uniL"), &render_main(config));
    write_file(&dir.join(".gitignore"), gitignore_content());
    write_file(&dir.join("README.md"), &render_readme(config));

    println!("Created project '{}' at {}", config.name, dir.display());

    // Git init
    if !config.no_git {
        let dir_str = dir.to_str().unwrap_or(".");
        git_init(dir_str);
    }

    println!("\nGet started:");
    println!("  cd {}", config.name);
    println!("  unilang run src/main.uniL");
}

fn write_file(path: &Path, content: &str) {
    fs::write(path, content).unwrap_or_else(|e| {
        eprintln!("error: cannot write '{}': {}", path.display(), e);
        std::process::exit(1);
    });
}

// ── Template renderers ────────────────────────────────────────────────────────

fn render_toml(config: &ProjectConfig) -> String {
    let drivers_list = config
        .deps
        .iter()
        .map(|d| format!("\"{}\"", d))
        .collect::<Vec<_>>()
        .join(", ");

    let web_section = if config.project_type == "web" {
        "\n[web]\nhost = \"0.0.0.0\"\nport = 8080\n"
    } else {
        ""
    };

    format!(
        r#"[project]
name = "{name}"
version = "0.1.0"
description = ""
type = "{ptype}"
edition = "2026"

[dependencies]
drivers = [{drivers}]
{web}
[build]
main = "src/main.uniL"
"#,
        name = config.name,
        ptype = config.project_type,
        drivers = drivers_list,
        web = web_section,
    )
}

fn driver_snippets(deps: &[String]) -> String {
    let mut out = String::new();
    for dep in deps {
        let snippet = match dep.as_str() {
            "sqlite" => {
                "# db = db_connect(\"./data.db\")\n\
                 # db_exec(db, \"CREATE TABLE IF NOT EXISTS items (id INTEGER PRIMARY KEY, name TEXT)\")\n"
            }
            "redis" => {
                "# redis_connect(\"redis://localhost:6379\")\n\
                 # redis_set(\"key\", \"value\")\n"
            }
            "mysql" => "# mysql_connect(\"mysql://root:pass@localhost/mydb\")\n",
            "postgres" => "# pg_connect(\"postgres://user:pass@localhost/mydb\")\n",
            "kafka" => "# kafka_produce(\"my-topic\", \"hello\")\n",
            "email" => {
                "# smtp_send(\"smtp://user:pass@smtp.example.com\", \"to@example.com\", \"Subject\", \"Body\")\n"
            }
            "elasticsearch" => "# es_connect(\"http://localhost:9200\")\n",
            "mongodb" => "# mongo_connect(\"mongodb://localhost:27017\")\n",
            "rabbitmq" => "# rabbitmq_connect(\"amqp://guest:guest@localhost:5672\")\n",
            "nats" => "# nats_connect(\"nats://localhost:4222\")\n",
            "s3" => {
                "# s3_put(\"my-bucket\", \"key\", \"content\", \"us-east-1\", \"ACCESS_KEY\", \"SECRET_KEY\")\n"
            }
            "influxdb" => {
                "# influxdb_write(\"http://localhost:8086\", \"mydb\", \"measurement,tag=value field=1.0\")\n"
            }
            "websocket" => "# ws_connect(\"ws://localhost:9001\")\n",
            _ => "",
        };
        out.push_str(snippet);
    }
    out
}

fn render_main(config: &ProjectConfig) -> String {
    let snippets = driver_snippets(&config.deps);
    let prefix = if snippets.is_empty() {
        String::new()
    } else {
        format!("{}\n", snippets)
    };

    let name = &config.name;
    match config.project_type.as_str() {
        "web" => {
            let body = format!(
                "# {name} \u{2014} a web service built with UniLang\n\
                 # Start server: unilang run src/main.uniL\n\
                 \n\
                 def handle_root(req):\n\
                 \treturn {{ \"status\": 200, \"body\": \"Hello from {name}!\" }}\n\
                 \n\
                 def handle_health(req):\n\
                 \treturn {{ \"status\": 200, \"body\": {{ \"status\": \"ok\", \"service\": \"{name}\" }} }}\n\
                 \n\
                 routes = {{\n\
                 \t\"GET /\":        handle_root,\n\
                 \t\"GET /health\":  handle_health,\n\
                 }}\n\
                 \n\
                 print(\"Starting {name} on port 8080...\")\n\
                 serve(8080, routes)\n",
                name = name,
            );
            format!("{}{}", prefix, body)
        }
        "cli" => {
            let body = format!(
                "# {name} \u{2014} a CLI tool built with UniLang\n\
                 # Run: unilang run src/main.uniL\n\
                 \n\
                 def usage():\n\
                 \tprint(\"Usage: unilang run src/main.uniL <command>\")\n\
                 \tprint(\"\")\n\
                 \tprint(\"Commands:\")\n\
                 \tprint(\"  help     Show help\")\n\
                 \tprint(\"  version  Show version\")\n\
                 \n\
                 def main():\n\
                 \tprint(\"{name} v0.1.0\")\n\
                 \tusage()\n\
                 \n\
                 main()\n",
                name = name,
            );
            format!("{}{}", prefix, body)
        }
        _ => {
            let body = format!(
                "# {name} \u{2014} a desktop application built with UniLang\n\
                 # Run: unilang run src/main.uniL\n\
                 \n\
                 def main():\n\
                 \tprint(\"=== {name} ===\")\n\
                 \t# Add your application logic here\n\
                 \n\
                 main()\n",
                name = name,
            );
            format!("{}{}", prefix, body)
        }
    }
}

fn gitignore_content() -> &'static str {
    "target/\n*.uniLbc\n.env\n*.db\n"
}

fn render_readme(config: &ProjectConfig) -> String {
    let type_desc = match config.project_type.as_str() {
        "web" => "A web service project.",
        "cli" => "A command-line tool project.",
        _ => "A desktop application project.",
    };

    let deps_section = if config.deps.is_empty() {
        "None".to_string()
    } else {
        config
            .deps
            .iter()
            .map(|d| format!("- {}", d))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "# {name}\n\
         \n\
         A UniLang project.\n\
         \n\
         ## Getting Started\n\
         \n\
         ```bash\n\
         unilang run src/main.uniL\n\
         ```\n\
         \n\
         ## Project Type\n\
         \n\
         {type_desc}\n\
         \n\
         ## Dependencies\n\
         \n\
         {deps}\n",
        name = config.name,
        type_desc = type_desc,
        deps = deps_section,
    )
}

// ── Git helpers ───────────────────────────────────────────────────────────────

fn git_init(project_dir: &str) {
    std::process::Command::new("git")
        .args(["init", project_dir])
        .status()
        .ok();
    std::process::Command::new("git")
        .args(["-C", project_dir, "add", "."])
        .status()
        .ok();
    std::process::Command::new("git")
        .args([
            "-C",
            project_dir,
            "commit",
            "-m",
            "Initial commit (UniLang scaffold)",
        ])
        .status()
        .ok();
}

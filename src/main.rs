mod config;

use clap::{Parser, CommandFactory};
use clap_complete::{generate, Generator, Shell};
use config::Config;
use std::path::PathBuf;
use std::env;
use std::process;
use std::io;

/// Expands $HOME and ~ in a path string for portability.
/// Bare relative paths (e.g. projects/foo) are treated as home-relative.
fn expand_home(path: &str) -> PathBuf {
    let path_str = path.trim();
    if path_str.is_empty() {
        return PathBuf::from(path);
    }
    // ~ or ~/path
    if path_str.starts_with("~/") || path_str == "~" {
        if let Some(home) = dirs::home_dir() {
            return if path_str == "~" {
                home
            } else {
                home.join(&path_str[2..])
            };
        }
    }
    // $HOME or $HOME/path
    if path_str.contains("$HOME") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(path_str.replace("$HOME", &home));
        }
    }
    // Absolute path or explicit cwd-relative (./ or ../) — use as-is
    if path_str.starts_with('/')
        || path_str.starts_with("./")
        || path_str.starts_with("..")
    {
        return PathBuf::from(path_str);
    }
    // Bare relative path (e.g. projects/foo) — assume they meant ~/path
    if let Some(home) = dirs::home_dir() {
        return home.join(path_str);
    }
    PathBuf::from(path)
}

#[derive(Parser)]
#[command(author, version, about = "Navigate to project directories")]
struct Cli {
    /// Project name to navigate to
    #[arg(index = 1)]
    project_name: Option<String>,

    /// Path to config file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Generate shell completion
    #[arg(long = "completion", value_enum)]
    shell: Option<Shell>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() {
    let cli = Cli::parse();

    // Handle completion generation
    if let Some(shell) = cli.shell {
        let mut cmd = Cli::command();
        print_completions(shell, &mut cmd);
        return;
    }

    match Config::load(cli.config.as_ref()) {
        Ok(config) => {
            if let Some(project_name) = cli.project_name {
                match config.projects.get(&project_name) {
                    Some(project_configs) => {
                        if let Some(first_config) = project_configs.first() {
                            let path = expand_home(&first_config.path);

                            if !path.exists() {
                                eprintln!("Project path does not exist: {}", path.display());
                                process::exit(1);
                            }

                            println!("CHANGE_DIR={}", path.display());
                        } else {
                            eprintln!("No path configuration found for project: {}", project_name);
                            process::exit(1);
                        }
                    }
                    None => {
                        eprintln!("Project '{}' not found in configuration", project_name);
                        println!("\nAvailable projects:");
                        for project_name in config.projects.keys() {
                            println!("  - {}", project_name);
                        }
                        process::exit(1);
                    }
                }
            } else {
                println!("Available projects:");
                for (project_name, project_configs) in &config.projects {
                    let path = expand_home(&project_configs[0].path);
                    println!("  {} -> {}", project_name, path.display());
                }
            }
        }
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            println!("Creating default configuration file...");

            let config_path = cli.config.clone().or_else(|| Config::config_path().ok());
            if let Some(config_path) = config_path {
                if let Err(e) = Config::create_default_config(&config_path) {
                    eprintln!("Failed to create default config: {}", e);
                } else {
                    println!("Default configuration created at: {}", config_path.display());
                }
            }
            process::exit(1);
        }
    }
}

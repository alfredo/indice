mod config;

use clap::Parser;
use config::Config;
use std::path::PathBuf;
use std::env;
use std::process;

#[derive(Parser)]
#[command(author, version, about = "Navigate to project directories")]
struct Cli {
    /// Project name to navigate to
    #[arg(index = 1)]
    project_name: Option<String>,

    /// Path to config file
    #[arg(short, long)]
    config: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    match Config::load() {
        Ok(config) => {
            if let Some(project_name) = cli.project_name {
                match config.projects.get(&project_name) {
                    Some(project_configs) => {
                        if let Some(first_config) = project_configs.first() {
                            let path = PathBuf::from(&first_config.path);

                            if !path.exists() {
                                eprintln!("Project path does not exist: {}", path.display());
                                process::exit(1);
                            }

                            // Print only what's necessary for the shell function
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
                // No project specified, list available projects
                println!("Available projects:");
                for (project_name, project_configs) in &config.projects {
                    println!("  {} -> {}", project_name, project_configs[0].path);
                }
            }
        }
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            println!("Creating default configuration file...");

            if let Ok(config_path) = Config::config_path() {
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

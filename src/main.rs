mod config;
mod git;
mod llm;
mod shell;
mod util;

use clap::{Parser, Subcommand};
use std::process;
use colored::Colorize;

#[derive(Parser)]
#[command(name = "gitsensei", about = "AI-powered git commit autocompletion", version)]
struct Cli {
    #[arg(long, global = true)]
    debug: bool, // Add global debug flag

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Suggest { prefix: Option<String> },
    Init,
    Hook { shell: String },
    Config, // Add this
}

fn main() {
    let cli = Cli::parse();
    let cfg = config::Config::load(); // Removed .unwrap_or_default()

    match cli.command {
        Commands::Suggest { prefix } => {
            let prefix_str = prefix.unwrap_or_default();
            
            // If debug is on, we print the context before sending
            if cli.debug {
                let raw_diff = git::provider::get_staged_diff();
                let bundled = git::bundler::bundle_diff(&raw_diff, cfg.aggressive_minification);
                println!("{}", "--- DEBUG: RAW DIFF ---".yellow());
                println!("{}", raw_diff);
                println!("{}", "--- DEBUG: BUNDLED DIFF ---".yellow());
                println!("{}", bundled);
            }

            match llm::client::generate_suggestion(&cfg, &prefix_str) {
                Ok(suggestion) => println!("{}", suggestion),
                Err(e) => {
                    util::log_error(&format!("AI Error: {}", e));
                    std::process::exit(1);
                }
            }
        }
        Commands::Init => {
            util::log_info("Setting up GitSensei...");
            let _ = process::Command::new("ollama")
                .args(["pull", &cfg.model])
                .status();
            util::log_success(&format!("Model {} is ready.", cfg.model));
        }
        Commands::Hook { shell } => {
            shell::install::print_hook(&shell);
        }
        Commands::Config => {
            let path = config::Config::get_path();
            util::log_info(&format!("Config path: {}", path.display()));
            
            // Check if it exists
            if path.exists() {
                util::log_success("Config file is present.");
            } else {
                util::log_error("Config file is missing!");
            }
        }
    }
}

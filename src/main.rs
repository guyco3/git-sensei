mod config;
mod git;
mod llm;
mod shell;
mod util;

use clap::{Parser, Subcommand};
use std::process;

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
}

fn main() {
    let cli = Cli::parse();
    let cfg = config::Config::load().unwrap_or_default();

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
                Err(_) => std::process::exit(0),
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
    }
}

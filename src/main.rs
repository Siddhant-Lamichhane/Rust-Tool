mod cli;
mod math;
mod config;
mod dft;
mod format;

use clap::Parser;
use env_logger;
use log;
use crate::format::format_complex;

fn main() {
    // Initialize logging.
    env_logger::init();
    log::info!("Starting CLI Tool");

    // Parse command line arguments.
    let cli_args = cli::Cli::parse();

    // Load configuration if a config file is specified.
    let settings = if let Some(config_path) = &cli_args.config {
        match config::load_config(config_path) {
            Ok(s) => {
                log::info!("Loaded configuration from: {}", config_path);
                Some(s)
            }
            Err(e) => {
                eprintln!("Error loading configuration: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Dispatch the command.
    match cli_args.command {
        cli::Commands::Greet { name } => cli::handle_greet(name, settings.as_ref()),
        cli::Commands::Farewell { name } => cli::handle_farewell(name, settings.as_ref()),
        cli::Commands::Add { numbers } => math::handle_add(&numbers),
        cli::Commands::Sub { numbers } => math::handle_sub(&numbers),
        cli::Commands::Mul { numbers } => math::handle_mul(&numbers),
        cli::Commands::Div { numbers } => math::handle_div(&numbers),
        cli::Commands::Dft { input } => {
            let result = dft::dft(&input);
            println!("DFT Result:");
            for (i, value) in result.iter().enumerate() {
                println!("Index {}: {}", i, format_complex(value));
            }
            log::info!("DFT subcommand executed with input: {:?}", input);
        },
        cli::Commands::Version => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            log::info!("Version subcommand executed");
        }
    }
}

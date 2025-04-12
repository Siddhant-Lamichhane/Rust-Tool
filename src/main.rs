mod cli;
mod math;
mod config;
mod dft;
mod fft;
mod format;

use clap::Parser;
use env_logger;
use log;
use crate::format::format_complex;
use num_complex::Complex;

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
        cli::Commands::Fft { input } => {
            // For FFT, check that the input count is a power of two.
            if !input.len().is_power_of_two() {
                eprintln!("Input length for FFT must be a power of two.");
                return;
            }
            // Convert input into complex numbers with zero imaginary parts.
            let input_complex: Vec<Complex<f64>> = input.iter()
                .map(|&x| Complex::new(x, 0.0))
                .collect();
            let result = fft::fft(&input_complex);
            println!("FFT Result:");
            for (i, value) in result.iter().enumerate() {
                println!("Index {}: {}", i, format_complex(value));
            }
            log::info!("FFT subcommand executed with input: {:?}", input);
        },
        cli::Commands::Version => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            log::info!("Version subcommand executed");
        }
    }
}

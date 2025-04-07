use clap::{Parser, Subcommand};
use crate::config::Settings; // This may not be used in the functions if you don't need it here.

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to a configuration file
    #[arg(short, long)]
    pub config: Option<String>,

    /// Subcommands available for this tool
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Greet someone
    Greet {
        /// The name of the person to greet
        #[arg(short, long)]
        name: String,
    },

    /// Bid farewell to someone
    Farewell {
        /// The name of the person to say goodbye to
        #[arg(short, long)]
        name: String,
    },

    Add {
        /// Numbers to add
        #[arg(short, long, num_args = 1..)]
        numbers: Vec<f64>,
    },

    Sub {
        /// Numbers to subtract
        #[arg(short, long, num_args = 1..)]
        numbers: Vec<f64>,
    },

    Mul {
        /// Numbers to multiply
        #[arg(short, long, num_args = 1..)]
        numbers: Vec<f64>,
    },

    Div {
        /// Numbers to divide
        #[arg(short, long, num_args = 1..)]
        numbers: Vec<f64>,
    },

    /// Compute the Discrete Fourier Transform (DFT)
    Dft {
        /// Real numbers to transform (comma-separated or space-separated)
        #[arg(short, long, num_args = 1..)]
        input: Vec<f64>,
    },

    /// Display the version information
    Version,
}

/// Handles the Greet subcommand.
pub fn handle_greet(name: String, settings: Option<&Settings>) {
    if let Some(s) = settings {
        if let Some(prefix) = &s.greeting_prefix {
            println!("{} {}", prefix, name);
            return;
        }
    }
    println!("Hello, {}!", name);
}

/// Handles the Farewell subcommand.
pub fn handle_farewell(name: String, settings: Option<&Settings>) {
    if let Some(s) = settings {
        if let Some(suffix) = &s.farewell_suffix {
            println!("Goodbye, {}{}", name, suffix);
            return;
        }
    }
    println!("Goodbye, {}!", name);
}

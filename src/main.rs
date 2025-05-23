use clap::{Args, Parser, Subcommand};
use rust_template::{add, divide, multiply, subtract};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Add { a, b } => add(a, b),
        Command::Subtract { a, b } => subtract(a, b),
        Command::Multiply { a, b } => multiply(a, b),
        Command::Divide { a, b } => divide(a, b),
    };

    println!("Result: {}", result);
}

/// A CLI calculator application that performs basic arithmetic operations.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[clap(flatten)]
    globals: GlobalArgs,
}

#[derive(Args)]
struct GlobalArgs {
    /// Set verbosity. `-v` = 1, `-vvv` = 3
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbosity: u8,
}

#[derive(Subcommand)]
enum Command {
    /// Adds two numbers
    Add { a: f64, b: f64 },

    /// Subtracts the second number from the first
    Subtract { a: f64, b: f64 },

    /// Multiplies two numbers
    Multiply { a: f64, b: f64 },

    /// Divides the first number by the second
    Divide { a: f64, b: f64 },
}

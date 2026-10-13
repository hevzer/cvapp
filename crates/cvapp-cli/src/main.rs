use clap::Parser;
use colored::Colorize;
use cvapp_cli::cli;
use std::process;

fn main() {
    let args = cli::Cli::parse();

    if let Err(err) = cli::run_cli(args) {
        eprintln!("{} {}", "Error:".red().bold(), err);
        process::exit(1);
    }
}

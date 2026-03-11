use clap::Parser;

use config::Cli;

mod config;
mod counter;
mod output;
mod process;

fn main() {
    let cli = Cli::parse();

    process::process(&cli);
}

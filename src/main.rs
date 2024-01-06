use clap::Parser;

use crate::cli::{Args, Commands};
use crate::commands::{branch, delete_branches, rebase};

mod cli;
mod commands;

fn main() {
    let cli = Args::parse();

    let result = match cli.command {
        Some(Commands::Branch { name, base }) => branch::run(&name, base, cli.verbose),
        Some(Commands::Rebase { base }) => rebase::run(base, cli.verbose),
        Some(Commands::DeleteBranches { dry_run }) => delete_branches::run(dry_run, cli.verbose),
        Some(Commands::Fixup { number }) => commands::fixup::run(number, cli.verbose),
        None => Err("No command specified, please run with --help for more info"),
    };

    if let Err(err) = result {
        println!("{err}");
    }
}

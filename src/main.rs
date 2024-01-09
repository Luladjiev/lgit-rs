use clap::Parser;

use crate::cli::{Args, Commands};
use crate::commands::{branch, delete_branches, get_base, rebase, Cmd};

mod cli;
mod commands;

fn main() {
    let cli = Args::parse();

    let command = Cmd {};

    let result = match cli.command {
        Some(Commands::Branch { name, base }) => {
            let base = get_base(&command, base, cli.verbose);

            branch::run(&command, name, base, cli.verbose)
        }
        Some(Commands::Rebase { base }) => {
            let base = get_base(&command, base, cli.verbose);

            rebase::run(&command, &base, cli.verbose)
        }
        Some(Commands::DeleteBranches { dry_run }) => {
            delete_branches::run(&command, dry_run, cli.verbose)
        }
        Some(Commands::Fixup { number }) => commands::fixup::run(&command, number, cli.verbose),
        None => Err("No command specified, please run with --help for more info"),
    };

    if let Err(err) = result {
        println!("{err}");
    }
}

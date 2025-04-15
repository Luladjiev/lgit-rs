use clap::Parser;

use crate::cli::{Args, Commands};
use crate::commands::{autosquash, branch, checkout, cherry_pick, delete_branches, rebase, Cmd};
use crate::utils::get_base;

mod cli;
mod commands;
mod utils;

fn main() {
    let cli = Args::parse();

    let command = Cmd {};

    let result = match cli.command {
        Some(Commands::Autosquash { number, base }) => {
            let base = get_base(&command, base, cli.verbose);

            autosquash::run(&command, &base, number, cli.verbose)
        }
        Some(Commands::Branch { name, base }) => {
            let base = get_base(&command, base, cli.verbose);

            branch::run(&command, &name, &base, cli.verbose)
        }
        Some(Commands::Checkout { name }) => checkout::run(&command, name, cli.verbose),
        Some(Commands::DeleteBranches { dry_run }) => {
            delete_branches::run(&command, dry_run, cli.verbose)
        }
        Some(Commands::Fixup { number }) => commands::fixup::run(&command, number, cli.verbose),
        Some(Commands::Rebase { base }) => {
            let base = get_base(&command, base, cli.verbose);

            rebase::run(&command, &base, cli.verbose)
        }
        Some(Commands::CherryPick { branch, number }) => {
            cherry_pick::run(&command, &branch, number, cli.verbose)
        }
        None => Err("No command specified, please run with --help for more info"),
    };

    if let Err(err) = result {
        println!("{err}");
    }
}

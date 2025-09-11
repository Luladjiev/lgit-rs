use clap::Parser;

use crate::cli::{Args, Commands};
use crate::commands::{
    autosquash, branch, checkout, cherry_pick, delete_branches, git_fallback, rebase, Cmd,
};
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
        Some(Commands::Checkout { name, remote, all }) => {
            checkout::run(&command, name, remote, all, cli.verbose)
        }
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
        Some(Commands::External(args)) => {
            // Handle 'co' alias for checkout
            // if !args.is_empty() && args[0] == "co" {
            //     let name = args.get(1).map(|s| s.to_string());
            //     // Parse remaining flags - for now, just handle basic case
            //     let remote = args.iter().any(|arg| arg == "-r" || arg == "--remote");
            //     let all = args.iter().any(|arg| arg == "-a" || arg == "--all");
            //     checkout::run(&command, name, remote, all, cli.verbose)
            // } else {
            git_fallback::run(&command, &args, cli.verbose)
            // }
        }
        None => Err(Some(
            "No command specified, please run with --help for more info".to_string(),
        )),
    };

    if let Err(Some(err)) = result {
        println!("{err}");
    }
}

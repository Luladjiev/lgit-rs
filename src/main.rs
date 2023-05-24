mod cli;
mod commands;
use clap::Parser;
use cli::{Args, Commands};
use commands::{branch, delete_branches, rebase};

fn main() {
    let cli = Args::parse();

    let result = match cli.command {
        Some(Commands::Branch { name, base }) => branch(name, base, cli.verbose),
        Some(Commands::Rebase { base }) => rebase(base, cli.verbose),
        Some(Commands::DeleteBranches { dry_run }) => delete_branches(dry_run, cli.verbose),
        None => "No command provided".to_string(),
    };

    println!("{}", result);
}

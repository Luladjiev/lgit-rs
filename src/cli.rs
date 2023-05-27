use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long, default_value_t = false, help = "Verbose output")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Create a new branch from latest BASE branch")]
    Branch {
        #[arg(help = "Name of the branch to create")]
        name: String,

        #[arg(short, long, help = "Base branch to branch from")]
        base: Option<String>,
    },

    #[command(about = "Rebase current branch on top of latest BASE branch")]
    Rebase {
        #[arg(short, long, help = "Base branch to branch from")]
        base: Option<String>,
    },

    #[command(about = "Delete all branches for which remotes are gone. Use with caution!")]
    DeleteBranches {
        #[arg(short, long, help = "Dry run, don't delete anything")]
        dry_run: bool,
    },
}

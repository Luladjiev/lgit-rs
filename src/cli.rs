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
    #[command(
        about = "Rebase branch to combine fixup / squash commits with their corresponding commits",
        long_about = "Rebase branch to combine fixup / squash commits with their corresponding commits.\n\n\
            Requires --number to be passed if it's used on the main / master branch.",
        visible_alias = "as"
    )]
    Autosquash {
        #[arg(short, long, help = "Number of commits to rebase")]
        number: Option<u32>,

        #[arg(short, long, help = "Base branch to rebase from")]
        base: Option<String>,
    },

    #[command(
        about = "Create a new branch from latest BASE branch",
        visible_alias = "b"
    )]
    Branch {
        #[arg(help = "Name of the branch to create")]
        name: String,

        #[arg(short, long, help = "Base branch to branch from")]
        base: Option<String>,
    },

    #[command(about = "Switch branches", visible_alias = "co")]
    Checkout {
        #[arg(help = "Name of the branch to checkout")]
        name: Option<String>,
    },

    #[command(about = "Delete all branches for which remotes are gone. Use with caution!")]
    DeleteBranches {
        #[arg(short, long, help = "Dry run, don't delete anything")]
        dry_run: bool,
    },

    #[command(about = "Commit as a fixup", visible_alias = "f")]
    Fixup {
        #[arg(short, long, default_value_t = 25, help = "Number of commits to list")]
        number: u32,
    },

    #[command(
        about = "Rebase current branch on top of latest BASE branch",
        visible_alias = "r"
    )]
    Rebase {
        #[arg(short, long, help = "Base branch to rebase onto")]
        base: Option<String>,
    },
}

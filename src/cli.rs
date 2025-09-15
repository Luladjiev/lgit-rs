use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
#[command(allow_external_subcommands = true)]
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

        #[arg(short, long, help = "List only remote branches")]
        remote: bool,

        #[arg(short, long, help = "List all branches (local and remote)")]
        all: bool,
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

    #[command(
        about = "Interactively cherry-pick commits from another branch",
        visible_alias = "cp"
    )]
    CherryPick {
        #[arg(help = "Branch to cherry-pick commits from")]
        branch: String,

        #[arg(short, long, default_value_t = 25, help = "Number of commits to show")]
        number: u32,
    },

    #[command(
        about = "Interactively delete local branches",
        visible_alias = "db"
    )]
    DeleteBranch {
        #[arg(short, long, help = "Force delete branches (uses -D instead of -d)")]
        force: bool,
    },

    #[command(external_subcommand)]
    External(Vec<String>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_args_parse_verbose_flag() {
        let args = Args::try_parse_from(&["lgit", "--verbose", "branch", "test-branch"]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(args.verbose);

        match args.command {
            Some(Commands::Branch { name, base: _ }) => {
                assert_eq!(name, "test-branch");
            }
            _ => panic!("Expected Branch command"),
        }
    }

    #[test]
    fn test_args_parse_short_verbose_flag() {
        let args = Args::try_parse_from(&["lgit", "-v", "autosquash", "--number", "5"]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(args.verbose);

        match args.command {
            Some(Commands::Autosquash { number, base: _ }) => {
                assert_eq!(number, Some(5));
            }
            _ => panic!("Expected Autosquash command"),
        }
    }

    #[test]
    fn test_args_parse_no_verbose() {
        let args = Args::try_parse_from(&["lgit", "branch", "test-branch"]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(!args.verbose);
    }

    #[test]
    fn test_branch_command_with_base() {
        let args = Args::try_parse_from(&["lgit", "branch", "feature-branch", "--base", "develop"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::Branch { name, base }) => {
                assert_eq!(name, "feature-branch");
                assert_eq!(base, Some("develop".to_string()));
            }
            _ => panic!("Expected Branch command"),
        }
    }

    #[test]
    fn test_branch_alias() {
        let args = Args::try_parse_from(&["lgit", "b", "feature-branch"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::Branch { name, base: _ }) => {
                assert_eq!(name, "feature-branch");
            }
            _ => panic!("Expected Branch command"),
        }
    }

    #[test]
    fn test_checkout_command_with_flags() {
        let args = Args::try_parse_from(&["lgit", "checkout", "--remote", "--all"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::Checkout { name, remote, all }) => {
                assert_eq!(name, None);
                assert!(remote);
                assert!(all);
            }
            _ => panic!("Expected Checkout command"),
        }
    }

    #[test]
    fn test_checkout_alias() {
        let args = Args::try_parse_from(&["lgit", "co", "main"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::Checkout {
                name,
                remote: _,
                all: _,
            }) => {
                assert_eq!(name, Some("main".to_string()));
            }
            _ => panic!("Expected Checkout command"),
        }
    }

    #[test]
    fn test_delete_branches_dry_run() {
        let args = Args::try_parse_from(&["lgit", "delete-branches", "--dry-run"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::DeleteBranches { dry_run }) => {
                assert!(dry_run);
            }
            _ => panic!("Expected DeleteBranches command"),
        }
    }

    #[test]
    fn test_fixup_with_number() {
        let args = Args::try_parse_from(&["lgit", "fixup", "--number", "10"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::Fixup { number }) => {
                assert_eq!(number, 10);
            }
            _ => panic!("Expected Fixup command"),
        }
    }

    #[test]
    fn test_fixup_default_number() {
        let args = Args::try_parse_from(&["lgit", "fixup"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::Fixup { number }) => {
                assert_eq!(number, 25); // default value
            }
            _ => panic!("Expected Fixup command"),
        }
    }

    #[test]
    fn test_autosquash_with_number() {
        let args = Args::try_parse_from(&["lgit", "autosquash", "--number", "3"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::Autosquash { number, base: _ }) => {
                assert_eq!(number, Some(3));
            }
            _ => panic!("Expected Autosquash command"),
        }
    }

    #[test]
    fn test_autosquash_alias() {
        let args = Args::try_parse_from(&["lgit", "as"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::Autosquash { number: _, base: _ }) => {
                // Success - autosquash alias works
            }
            _ => panic!("Expected Autosquash command"),
        }
    }

    #[test]
    fn test_rebase_with_base() {
        let args = Args::try_parse_from(&["lgit", "rebase", "--base", "main"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::Rebase { base }) => {
                assert_eq!(base, Some("main".to_string()));
            }
            _ => panic!("Expected Rebase command"),
        }
    }

    #[test]
    fn test_cherry_pick_command() {
        let args =
            Args::try_parse_from(&["lgit", "cherry-pick", "feature-branch", "--number", "5"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::CherryPick { branch, number }) => {
                assert_eq!(branch, "feature-branch");
                assert_eq!(number, 5);
            }
            _ => panic!("Expected CherryPick command"),
        }
    }

    #[test]
    fn test_cherry_pick_alias() {
        let args = Args::try_parse_from(&["lgit", "cp", "dev"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::CherryPick { branch, number }) => {
                assert_eq!(branch, "dev");
                assert_eq!(number, 25); // default value
            }
            _ => panic!("Expected CherryPick command"),
        }
    }

    #[test]
    fn test_delete_branch_command() {
        let args = Args::try_parse_from(&["lgit", "delete-branch"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::DeleteBranch { force }) => {
                assert!(!force); // default value
            }
            _ => panic!("Expected DeleteBranch command"),
        }
    }

    #[test]
    fn test_delete_branch_with_force() {
        let args = Args::try_parse_from(&["lgit", "delete-branch", "--force"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::DeleteBranch { force }) => {
                assert!(force);
            }
            _ => panic!("Expected DeleteBranch command"),
        }
    }

    #[test]
    fn test_delete_branch_alias() {
        let args = Args::try_parse_from(&["lgit", "db", "--force"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::DeleteBranch { force }) => {
                assert!(force);
            }
            _ => panic!("Expected DeleteBranch command"),
        }
    }

    #[test]
    fn test_external_command() {
        let args = Args::try_parse_from(&["lgit", "status", "--short"]);

        assert!(args.is_ok());
        match args.unwrap().command {
            Some(Commands::External(external_args)) => {
                assert_eq!(external_args, vec!["status", "--short"]);
            }
            _ => panic!("Expected External command"),
        }
    }

    #[test]
    fn test_no_command() {
        let args = Args::try_parse_from(&["lgit"]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(args.command.is_none());
    }
}

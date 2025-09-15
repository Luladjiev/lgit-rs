use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;

use crate::commands::Exec;

pub fn run<T: Exec>(
    cmd: &T,
    name: Option<String>,
    remote: bool,
    all: bool,
    verbose: bool,
) -> Result<(), Option<String>> {
    if let Some(name) = name {
        return do_checkout(cmd, &name, verbose);
    }

    let branch = get_branches(cmd, remote, all, verbose)?;

    do_checkout(cmd, &branch, verbose)
}

fn do_checkout<T: Exec>(cmd: &T, branch: &str, verbose: bool) -> Result<(), Option<String>> {
    cmd.exec(&["checkout", branch], verbose, false)
        .map_err(|()| format!("Failed to checkout branch '{}'", branch))?;

    Ok(())
}

fn get_branches<T: Exec>(
    cmd: &T,
    remote: bool,
    all: bool,
    verbose: bool,
) -> Result<String, String> {
    let remotes: Vec<String> = cmd
        .exec(&["remote"], verbose, false)
        .map_err(|()| "Failed to get git remotes (check network connection)".to_string())?
        .lines()
        .map(String::from)
        .collect();

    // Determine which branches to list based on flags
    let branch_args = if all {
        vec!["branch", "-a", "--format", "%(refname)"]
    } else if remote {
        vec!["branch", "-r", "--format", "%(refname)"]
    } else {
        vec!["branch", "--format", "%(refname)"]
    };

    let mut branches: Vec<String> = cmd
        .exec(&branch_args, verbose, false)
        .map_err(|()| {
            let branch_type = if all {
                "all branches (local and remote)"
            } else if remote {
                "remote branches"
            } else {
                "local branches"
            };
            format!("Failed to list {}", branch_type)
        })?
        .lines()
        .map(|line| {
            let mut line = String::from(line);

            // Clean up remote prefixes
            for remote in &remotes {
                line = line.replace(&format!("refs/remotes/{}/", &remote), "");
            }

            // Clean up local branch prefix
            line.replace("refs/heads/", "").trim().to_string()
        })
        .filter(|branch| branch != "HEAD")
        .collect();

    branches.sort();
    branches.dedup();

    if branches.is_empty() {
        let branch_type = if all {
            "branches (local and remote)"
        } else if remote {
            "remote branches"
        } else {
            "local branches"
        };
        return Err(format!("No {} found", branch_type));
    }

    let option = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which branch to checkout?")
        .default(0)
        .items(&branches)
        .interact()
        .map_err(|err| {
            if verbose {
                println!("{err}");
            }

            format!("Failed to select branch: {}", err)
        })?;

    let branch = branches.get(option);

    if branch.is_none() {
        return Err(format!("Invalid branch selection index: {}", option));
    }

    let branch = branch.unwrap();

    Ok(branch.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::MockCmd;

    #[test]
    fn test_run_with_specific_branch() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "feature-branch"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        let result = run(
            &command,
            Some("feature-branch".to_string()),
            false,
            false,
            false,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_checkout_failure() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "nonexistent-branch"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Err(()));

        let result = run(
            &command,
            Some("nonexistent-branch".to_string()),
            false,
            false,
            false,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_do_checkout_success() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "main"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        let result = do_checkout(&command, "main", false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_do_checkout_failure() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "invalid"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Err(()));

        let result = do_checkout(&command, "invalid", false);

        assert!(result.is_err());
    }

    // Note: We can't easily test get_branches success case because it uses interactive FuzzySelect
    // which requires user input. We focus on testing the error cases and the individual functions.

    #[test]
    fn test_get_branches_no_branches_found() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["remote"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok("origin".to_string()));

        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["branch", "--format", "%(refname)"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        let result = get_branches(&command, false, false, false);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No local branches found");
    }

    #[test]
    fn test_get_branches_remote_failure() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["remote"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Err(()));

        let result = get_branches(&command, false, false, false);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to get git remotes (check network connection)");
    }

    #[test]
    fn test_get_branches_branch_list_failure() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["remote"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok("origin".to_string()));

        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["branch", "--format", "%(refname)"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Err(()));

        let result = get_branches(&command, false, false, false);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to list local branches");
    }
}

use dialoguer::MultiSelect;

use crate::commands::Exec;

pub fn run(cmd: &dyn Exec, branch: &str, number: u32, verbose: bool) -> Result<(), Option<String>> {
    let commits = get_commits(cmd, branch, number, verbose)?;

    let selections = MultiSelect::new()
        .with_prompt("Select commits to cherry-pick (use space to select, enter to confirm)")
        .items(&commits)
        .interact()
        .map_err(|_| "Failed to get user input".to_string())?;

    if selections.is_empty() {
        return Err(Some("No commits selected".to_string()));
    }

    let selected_commits: Result<Vec<&str>, String> = selections
        .iter()
        .map(|&i| match commits[i].split_whitespace().next() {
            Some(commit) => Ok(commit),
            None => Err(format!("Invalid commit format: {}\n", commits[i])),
        })
        .collect::<Result<Vec<&str>, String>>();

    let mut selected_commits = match selected_commits {
        Ok(commits) => commits,
        Err(err) => {
            if verbose {
                println!("{err}");
            }

            return Err(Some("Failed to parse commits format".to_string()));
        }
    };

    selected_commits.reverse();

    for commit in selected_commits {
        if cmd.exec(&["cherry-pick", commit], verbose, false).is_err() {
            return Err(Some("Failed to cherry-pick commit".to_string()));
        }
    }

    Ok(())
}

fn get_commits(
    cmd: &dyn Exec,
    branch: &str,
    number: u32,
    verbose: bool,
) -> Result<Vec<String>, String> {
    let output = cmd
        .exec(
            &[
                "log",
                branch,
                "--pretty=format:%h %s",
                &format!("-n {number}"),
            ],
            verbose,
            false,
        )
        .map_err(|()| "Failed to get commit history".to_string())?;

    Ok(output.lines().map(String::from).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::MockCmd;

    #[test]
    fn test_get_commits_success() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["log", "feature-branch", "--pretty=format:%h %s", "-n 5"]
                    && !(*verbose)
                    && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| {
                Ok("abc123 First commit\ndef456 Second commit\n789ghi Third commit".to_string())
            });

        let result = get_commits(&command, "feature-branch", 5, false);

        assert!(result.is_ok());
        let commits = result.unwrap();
        assert_eq!(commits.len(), 3);
        assert_eq!(commits[0], "abc123 First commit");
        assert_eq!(commits[1], "def456 Second commit");
        assert_eq!(commits[2], "789ghi Third commit");
    }

    #[test]
    fn test_get_commits_failure() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == [
                    "log",
                    "nonexistent-branch",
                    "--pretty=format:%h %s",
                    "-n 10",
                ] && !(*verbose)
                    && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Err(()));

        let result = get_commits(&command, "nonexistent-branch", 10, false);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to get commit history");
    }

    #[test]
    fn test_get_commits_empty_output() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["log", "empty-branch", "--pretty=format:%h %s", "-n 1"]
                    && !(*verbose)
                    && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        let result = get_commits(&command, "empty-branch", 1, false);

        assert!(result.is_ok());
        let commits = result.unwrap();
        // Empty string when split into lines becomes an empty vector
        assert_eq!(commits.len(), 0);
    }

    #[test]
    fn test_get_commits_with_verbose() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["log", "test-branch", "--pretty=format:%h %s", "-n 3"]
                    && *verbose
                    && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok("abc123 Test commit".to_string()));

        let result = get_commits(&command, "test-branch", 3, true);

        assert!(result.is_ok());
    }
}

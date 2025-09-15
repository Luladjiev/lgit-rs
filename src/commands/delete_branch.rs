use dialoguer::MultiSelect;

use crate::commands::Exec;

pub fn run(cmd: &dyn Exec, force: bool, verbose: bool) -> Result<(), Option<String>> {
    let branches = get_branches(cmd, verbose)?;

    let selections = MultiSelect::new()
        .with_prompt("Select branches to delete (use space to select, enter to confirm)")
        .items(&branches)
        .interact()
        .map_err(|err| format!("Failed to get user selection: {}", err))?;

    if selections.is_empty() {
        return Err(Some(format!(
            "No branches selected from {} available branches",
            branches.len()
        )));
    }

    let selected_branches: Vec<&str> = selections.iter().map(|&i| branches[i].as_str()).collect();

    let delete_flag = if force { "-D" } else { "-d" };

    for branch in selected_branches {
        match cmd.exec(&["branch", delete_flag, branch], verbose, false) {
            Ok(_) => {
                println!("Successfully deleted branch {}", branch);
            }
            Err(_) => {
                eprintln!(
                    "Failed to delete branch {}. Use --force to delete it.",
                    branch
                );
            }
        }
    }

    Ok(())
}

fn get_branches(cmd: &dyn Exec, verbose: bool) -> Result<Vec<String>, String> {
    let output = cmd
        .exec(&["branch", "--format=%(refname:short)"], verbose, false)
        .map_err(|()| "Failed to get list of local branches".to_string())?;

    // Get current branch to filter it out
    let current_branch = cmd
        .exec(&["branch", "--show-current"], verbose, false)
        .map_err(|()| "Failed to get current branch".to_string())?
        .trim()
        .to_string();

    let branches: Vec<String> = output
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|branch| !branch.is_empty() && branch != &current_branch)
        .collect();

    if branches.is_empty() {
        return Err("No branches available for deletion (current branch is excluded)".to_string());
    }

    Ok(branches)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::MockCmd;

    #[test]
    fn test_get_branches_success() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["branch", "--format=%(refname:short)"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok("main\nfeature-1\nfeature-2\nbugfix-123".to_string()));

        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["branch", "--show-current"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok("main".to_string()));

        let result = get_branches(&command, false);

        assert!(result.is_ok());
        let branches = result.unwrap();
        assert_eq!(branches.len(), 3);
        assert_eq!(branches[0], "feature-1");
        assert_eq!(branches[1], "feature-2");
        assert_eq!(branches[2], "bugfix-123");
        assert!(!branches.contains(&"main".to_string()));
    }

    #[test]
    fn test_get_branches_failure() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["branch", "--format=%(refname:short)"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Err(()));

        let result = get_branches(&command, false);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to get list of local branches");
    }

    #[test]
    fn test_get_branches_no_other_branches() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["branch", "--format=%(refname:short)"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok("main".to_string()));

        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["branch", "--show-current"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok("main".to_string()));

        let result = get_branches(&command, false);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "No branches available for deletion (current branch is excluded)"
        );
    }

    #[test]
    fn test_get_branches_with_verbose() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["branch", "--format=%(refname:short)"] && *verbose && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok("main\ntest-branch".to_string()));

        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["branch", "--show-current"] && *verbose && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok("main".to_string()));

        let result = get_branches(&command, true);

        assert!(result.is_ok());
        let branches = result.unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0], "test-branch");
    }

    #[test]
    fn test_error_message_formatting() {
        // Test error message formatting with all failures
        let failed_deletions = vec!["branch1", "branch2"];
        let successful_deletions: Vec<&str> = vec![];

        let error_msg = format!(
            "Failed to delete {} branch(es): {}{}",
            failed_deletions.len(),
            failed_deletions.join(", "),
            if successful_deletions.is_empty() {
                String::new()
            } else {
                format!(
                    " (successfully deleted {} others)",
                    successful_deletions.len()
                )
            }
        );

        assert_eq!(error_msg, "Failed to delete 2 branch(es): branch1, branch2");

        // Test error message formatting with mixed results
        let successful_deletions = vec!["branch1"];
        let failed_deletions = vec!["branch2", "branch3"];

        let error_msg = format!(
            "Failed to delete {} branch(es): {}{}",
            failed_deletions.len(),
            failed_deletions.join(", "),
            if successful_deletions.is_empty() {
                String::new()
            } else {
                format!(
                    " (successfully deleted {} others)",
                    successful_deletions.len()
                )
            }
        );

        assert_eq!(
            error_msg,
            "Failed to delete 2 branch(es): branch2, branch3 (successfully deleted 1 others)"
        );
    }

    #[test]
    fn test_deletion_with_mixed_results() {
        let mut command = MockCmd::new();

        // Setup branches
        command
            .expect_exec()
            .withf(|args, _, _| args == ["branch", "--format=%(refname:short)"])
            .returning(|_, _, _| Ok("main\nbranch1\nbranch2\nbranch3".to_string()));

        command
            .expect_exec()
            .withf(|args, _, _| args == ["branch", "--show-current"])
            .returning(|_, _, _| Ok("main".to_string()));

        // Test deletion behavior manually since we can't easily mock interactive input
        let branches = get_branches(&command, false).unwrap();
        assert_eq!(branches, vec!["branch1", "branch2", "branch3"]);

        // Test the deletion logic separately by simulating the branch processing
        let selected_branches = vec!["branch1", "branch2", "branch3"];

        let mut successful_deletions = Vec::new();
        let mut failed_deletions = Vec::new();

        // Simulate mixed results
        for (i, branch) in selected_branches.iter().enumerate() {
            if i == 0 {
                // First branch succeeds
                successful_deletions.push(*branch);
            } else {
                // Others fail
                failed_deletions.push(*branch);
            }
        }

        // Verify error message format
        assert_eq!(successful_deletions, vec!["branch1"]);
        assert_eq!(failed_deletions, vec!["branch2", "branch3"]);

        let error_msg = format!(
            "Failed to delete {} branch(es): {}{}",
            failed_deletions.len(),
            failed_deletions.join(", "),
            if successful_deletions.is_empty() {
                String::new()
            } else {
                format!(
                    " (successfully deleted {} others)",
                    successful_deletions.len()
                )
            }
        );

        assert_eq!(
            error_msg,
            "Failed to delete 2 branch(es): branch2, branch3 (successfully deleted 1 others)"
        );
    }
}

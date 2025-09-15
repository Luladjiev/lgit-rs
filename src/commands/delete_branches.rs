use crate::commands::Exec;

pub fn run<T: Exec>(command: &T, dry_run: bool, verbose: bool) -> Result<(), Option<String>> {
    match delete_branches(command, dry_run, verbose) {
        Ok(output) => {
            println!("{output}");
            Ok(())
        }
        Err(err) => Err(Some(err)),
    }
}

fn delete_branches<T: Exec>(command: &T, dry_run: bool, verbose: bool) -> Result<String, String> {
    command
        .exec(&["fetch", "--prune"], verbose, false)
        .map_err(|()| "Failed to fetch and prune from remote (check network connection)".to_string())?;

    let branches = command
        .exec(&["branch", "-vv"], verbose, false)
        .map_err(|()| "Failed to get branch information with tracking details".to_string())?;

    let mut result = Vec::new();

    for line in branches.lines() {
        if line.starts_with('*') || !line.contains(": gone]") {
            continue;
        }

        let branch_name = line
            .split_whitespace()
            .next()
            .ok_or_else(|| format!("Failed to parse branch name from line: '{}'", line.trim()))?;

        if !dry_run {
            command
                .exec(&["branch", "-D", branch_name], verbose, false)
                .map_err(|()| format!("Failed to delete branch '{}'", branch_name))?;
        }

        result.push(format!("Deleted branch {branch_name}"));
    }

    Ok(if result.is_empty() {
        "No branches to delete".to_string()
    } else {
        result.join("\n")
    })
}

#[cfg(test)]
mod tests {
    use crate::commands::delete_branches::delete_branches;
    use crate::commands::MockCmd;

    fn cmd_fetch_prune() -> MockCmd {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["fetch", "--prune"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        command
    }

    fn cmd_fetch_prune_branch() -> MockCmd {
        let mut command = cmd_fetch_prune();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| args == ["branch", "-vv"] && !(*verbose) && !(*inherit_stderr))
            .times(1)
            .returning(|_, _, _| Ok("  branch1 [origin/branch1: gone]\n  branch2 [origin/branch2: gone]\n* branch3 [origin/branch3]".to_string()));

        command
    }

    #[test]
    fn delete_branches_does_not_delete_when_dry_run() {
        let command = cmd_fetch_prune_branch();

        let result = delete_branches(&command, true, false);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "Deleted branch branch1\nDeleted branch branch2"
        );
    }

    #[test]
    fn delete_branches_does_not_delete_current_branch() {
        let mut command = cmd_fetch_prune_branch();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["branch", "-D", "branch1"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["branch", "-D", "branch2"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        let result = delete_branches(&command, false, false);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "Deleted branch branch1\nDeleted branch branch2"
        );
    }

    #[test]
    fn delete_branches_returns_error_when_delete_fails() {
        let mut command = cmd_fetch_prune_branch();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["branch", "-D", "branch1"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Err(()));

        let result = delete_branches(&command, false, false);

        assert!(result.is_err());
    }

    #[test]
    fn delete_branches_no_branches_to_delete() {
        let mut command = cmd_fetch_prune();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["branch", "-vv"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok("* branch3 [origin/branch3]".to_string()));

        let result = delete_branches(&command, false, false);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "No branches to delete");
    }
}

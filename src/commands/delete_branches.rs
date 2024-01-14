use crate::commands::Exec;

pub fn run<T: Exec>(command: &T, dry_run: bool, verbose: bool) -> Result<(), &str> {
    match delete_branches(command, dry_run, verbose) {
        Ok(output) => {
            println!("{output}");
            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn delete_branches<T: Exec>(
    command: &T,
    dry_run: bool,
    verbose: bool,
) -> Result<String, &'static str> {
    command
        .exec(&["fetch", "--prune"], verbose)
        .map_err(|()| "Failed to fetch")?;

    let branches = command
        .exec(&["branch", "-vv"], verbose)
        .map_err(|()| "Failed to get branches")?;

    let mut result = Vec::new();

    for line in branches.lines() {
        if line.starts_with('*') || !line.contains(": gone]") {
            continue;
        }

        let branch_name = line
            .split_whitespace()
            .next()
            .ok_or("Failed to parse branch name")?;

        if !dry_run {
            command
                .exec(&["branch", "-D", branch_name], verbose)
                .map_err(|()| "Failed to delete branch")?;
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
            .withf(|args, verbose| args == ["fetch", "--prune"] && !(*verbose))
            .times(1)
            .returning(|_, _| Ok(String::new()));

        command
    }

    fn cmd_fetch_prune_branch() -> MockCmd {
        let mut command = cmd_fetch_prune();
        command
            .expect_exec()
            .withf(|args, verbose| args == ["branch", "-vv"] && !(*verbose))
            .times(1)
            .returning(|_, _| Ok("  branch1 [origin/branch1: gone]\n  branch2 [origin/branch2: gone]\n* branch3 [origin/branch3]".to_string()));

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
            .withf(|args, verbose| args == ["branch", "-D", "branch1"] && !(*verbose))
            .times(1)
            .returning(|_, _| Ok(String::new()));
        command
            .expect_exec()
            .withf(|args, verbose| args == ["branch", "-D", "branch2"] && !(*verbose))
            .times(1)
            .returning(|_, _| Ok(String::new()));

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
            .withf(|args, verbose| args == ["branch", "-D", "branch1"] && !(*verbose))
            .times(1)
            .returning(|_, _| Err(()));

        let result = delete_branches(&command, false, false);

        assert!(result.is_err());
    }

    #[test]
    fn delete_branches_no_branches_to_delete() {
        let mut command = cmd_fetch_prune();
        command
            .expect_exec()
            .withf(|args, verbose| args == ["branch", "-vv"] && !(*verbose))
            .times(1)
            .returning(|_, _| Ok("* branch3 [origin/branch3]".to_string()));

        let result = delete_branches(&command, false, false);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "No branches to delete");
    }
}

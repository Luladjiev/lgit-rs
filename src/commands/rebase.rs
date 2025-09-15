use crate::commands::Exec;
use crate::utils::{refresh_base, stash, unstash};

pub fn run<T: Exec>(command: &T, base: &str, verbose: bool) -> Result<(), Option<String>> {
    let unsaved_changes = stash(command, verbose)?;

    refresh_base(command, base, verbose)
        .map_err(|()| format!("Failed to refresh base branch '{}'", base))?;

    command
        .exec(&["checkout", "-"], verbose, false)
        .map_err(|()| "Failed to checkout back to initial branch (git checkout -)".to_string())?;

    command
        .exec(&["rebase", base], verbose, false)
        .map_err(|()| format!("Failed to rebase onto '{}'", base))?;

    if unsaved_changes {
        unstash(command, verbose)?;
    }

    println!("Rebased onto {base}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::MockCmd;

    #[test]
    fn test_run_success_without_stash() {
        let mut command = MockCmd::new();

        // Mock stash check - no changes
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["status", "--porcelain"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock refresh_base - checkout base
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "main"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock refresh_base - pull
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["pull"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock checkout back to original branch
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "-"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock rebase
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["rebase", "main"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        let result = run(&command, "main", false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_success_with_stash() {
        let mut command = MockCmd::new();

        // Mock stash check - has changes
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["status", "--porcelain"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok("M file.txt".to_string()));

        // Mock stash save
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["stash", "-u"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok("Saved working directory".to_string()));

        // Mock refresh_base - checkout base
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "main"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock refresh_base - pull
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["pull"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock checkout back to original branch
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "-"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock rebase
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["rebase", "main"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock unstash
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["stash", "pop"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok("Applied stash".to_string()));

        let result = run(&command, "main", false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_stash_failure() {
        let mut command = MockCmd::new();

        // Mock stash check - failure
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["status", "--porcelain"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Err(()));

        let result = run(&command, "main", false);

        assert!(result.is_err());
    }

    #[test]
    fn test_run_refresh_base_failure() {
        let mut command = MockCmd::new();

        // Mock stash check - no changes
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["status", "--porcelain"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock refresh_base - checkout failure
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "main"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Err(()));

        let result = run(&command, "main", false);

        assert!(result.is_err());
    }

    #[test]
    fn test_run_checkout_back_failure() {
        let mut command = MockCmd::new();

        // Mock stash check - no changes
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["status", "--porcelain"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock refresh_base - checkout base
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "main"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock refresh_base - pull
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["pull"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock checkout back - failure
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "-"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Err(()));

        let result = run(&command, "main", false);

        assert!(result.is_err());
    }

    #[test]
    fn test_run_rebase_failure() {
        let mut command = MockCmd::new();

        // Mock stash check - no changes
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["status", "--porcelain"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock refresh_base - checkout base
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "main"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock refresh_base - pull
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["pull"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock checkout back to original branch
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "-"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        // Mock rebase - failure
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["rebase", "main"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Err(()));

        let result = run(&command, "main", false);

        assert!(result.is_err());
    }
}

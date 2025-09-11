use crate::commands::Exec;
use crate::utils::{refresh_base, stash, unstash};

pub fn run<T: Exec>(
    command: &T,
    name: &str,
    base: &str,
    verbose: bool,
) -> Result<(), Option<String>> {
    let unsaved_changes = stash(command, verbose)?;

    refresh_base(command, base, verbose)
        .map_err(|()| "Failed to refresh base branch".to_string())?;

    command
        .exec(&["checkout", "-b", name], verbose, false)
        .map_err(|()| "Failed to create branch".to_string())?;

    if unsaved_changes {
        unstash(command, verbose)?;
    }

    println!("Created branch {name}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::commands::branch::run;
    use crate::commands::MockCmd;

    #[test]
    fn test_run_with_master_branch() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .times(1)
            .withf(|args, verbose, inherit_stderr| {
                args == ["status", "--porcelain"] && !(*verbose) && !(*inherit_stderr)
            })
            .returning(|_, _, _| Ok(String::new()));
        command
            .expect_exec()
            .times(1)
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "main"] && !(*verbose) && !(*inherit_stderr)
            })
            .returning(|_, _, _| Ok(String::new()));
        command
            .expect_exec()
            .times(1)
            .withf(|args, verbose, inherit_stderr| {
                args == ["pull"] && !(*verbose) && !(*inherit_stderr)
            })
            .returning(|_, _, _| Ok(String::new()));
        command
            .expect_exec()
            .times(1)
            .withf(|args, verbose, inherit_stderr| {
                args == ["checkout", "-b", "test"] && !(*verbose) && !(*inherit_stderr)
            })
            .returning(|_, _, _| Ok(String::new()));
        assert_eq!(run(&command, "test", "main", false), Ok(()));
    }
}

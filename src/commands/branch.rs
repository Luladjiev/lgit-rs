use crate::commands::Exec;
use crate::utils::{refresh_base, stash, unstash};

pub fn run<T: Exec>(
    command: &T,
    name: &str,
    base: &str,
    verbose: bool,
) -> Result<(), &'static str> {
    stash(command, verbose)?;

    refresh_base(command, base, verbose).map_err(|()| "Failed to refresh base branch")?;

    command
        .exec(&["checkout", "-b", &name], verbose)
        .map_err(|()| "Failed to create branch")?;

    unstash(command, verbose)?;

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
            .withf(|args, verbose| args == ["stash", "-u"] && !(*verbose))
            .returning(|_, _| Ok(String::new()));
        command
            .expect_exec()
            .times(1)
            .withf(|args, verbose| args == ["checkout", "main"] && !(*verbose))
            .returning(|_, _| Ok(String::new()));
        command
            .expect_exec()
            .times(1)
            .withf(|args, verbose| args == ["pull"] && !(*verbose))
            .returning(|_, _| Ok(String::new()));
        command
            .expect_exec()
            .times(1)
            .withf(|args, verbose| args == ["checkout", "-b", "test"] && !(*verbose))
            .returning(|_, _| Ok(String::new()));
        command
            .expect_exec()
            .times(1)
            .withf(|args, verbose| args == ["stash", "pop"] && !(*verbose))
            .returning(|_, _| Ok(String::new()));
        assert_eq!(run(&command, "test", "main", false), Ok(()));
    }
}

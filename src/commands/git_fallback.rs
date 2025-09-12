use crate::commands::Exec;

pub fn run(cmd: &dyn Exec, args: &[String], verbose: bool) -> Result<(), Option<String>> {
    let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    match cmd.exec(&str_args, verbose, true) {
        Ok(_) => Ok(()),
        Err(_) => Err(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::MockCmd;

    #[test]
    fn test_run_success() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["status", "--short"] && !(*verbose) && *inherit_stderr
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        let args = vec!["status".to_string(), "--short".to_string()];
        let result = run(&command, &args, false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_failure() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["invalid-command"] && !(*verbose) && *inherit_stderr
            })
            .times(1)
            .returning(|_, _, _| Err(()));

        let args = vec!["invalid-command".to_string()];
        let result = run(&command, &args, false);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), None);
    }

    #[test]
    fn test_run_empty_args() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args.is_empty() && !(*verbose) && *inherit_stderr
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        let args: Vec<String> = vec![];
        let result = run(&command, &args, false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_verbose() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["log", "--oneline"] && *verbose && *inherit_stderr
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        let args = vec!["log".to_string(), "--oneline".to_string()];
        let result = run(&command, &args, true);

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_multiple_args() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["commit", "-m", "test message", "--author", "Test Author"]
                    && !(*verbose)
                    && *inherit_stderr
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        let args = vec![
            "commit".to_string(),
            "-m".to_string(),
            "test message".to_string(),
            "--author".to_string(),
            "Test Author".to_string(),
        ];
        let result = run(&command, &args, false);

        assert!(result.is_ok());
    }
}

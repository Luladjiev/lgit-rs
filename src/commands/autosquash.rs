use crate::commands::Exec;

pub fn run<T: Exec>(
    cmd: &T,
    base: &str,
    number: Option<u32>,
    verbose: bool,
) -> Result<(), Option<String>> {
    let mut args = vec![
        "-c",
        "sequence.editor=:", // used in order to prevent --interactive blocking the autosquash
        "rebase",
        "--interactive",
        "--autosquash",
    ];
    let arg: String;

    match number {
        Some(number) => {
            arg = format!("HEAD~{number}");
            args.push(&arg);
        }
        None => args.push(base),
    }

    cmd.exec(&args, verbose, false)
        .map(|_| ())
        .map_err(|()| Some("Failed to auto squash commits".to_string()))
}

#[cfg(test)]
mod tests {
    use crate::commands::MockCmd;

    use super::*;

    #[test]
    fn test_with_number_supplied() {
        let mut cmd = MockCmd::new();
        cmd.expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == [
                    "-c",
                    "sequence.editor=:",
                    "rebase",
                    "--interactive",
                    "--autosquash",
                    "HEAD~1",
                ] && !(*verbose)
                    && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        let result = run(&cmd, "HEAD~1", None, false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_with_base_supplied() {
        let mut cmd = MockCmd::new();
        cmd.expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == [
                    "-c",
                    "sequence.editor=:",
                    "rebase",
                    "--interactive",
                    "--autosquash",
                    "main",
                ] && !(*verbose)
                    && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        let result = run(&cmd, "main", None, false);

        assert!(result.is_ok());
    }
}

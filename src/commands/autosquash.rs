use crate::commands::Exec;

pub fn run<T: Exec>(
    cmd: &T,
    base: &str,
    number: Option<u32>,
    verbose: bool,
) -> Result<(), &'static str> {
    let mut args = vec![
        "-c",
        "sequence.editor=:", // used in order to prevent --interactive blocking the autosquash
        "rebase",
        "--interactive",
        "--autosquash",
    ];
    let arg: String;

    if let Some(number) = number {
        arg = format!("HEAD~{number}");

        args.push(&arg);
    } else {
        args.push(base);
    }

    let result = cmd.exec(&args, verbose);

    match result {
        Ok(_) => Ok(()),
        Err(()) => Err("Failed to auto squash commits"),
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::MockCmd;

    use super::*;

    #[test]
    fn test_with_number_supplied() {
        let mut cmd = MockCmd::new();
        cmd.expect_exec()
            .withf(|args, verbose| {
                args == [
                    "-c",
                    "sequence.editor=:",
                    "rebase",
                    "--interactive",
                    "--autosquash",
                    "HEAD~1",
                ] && !(*verbose)
            })
            .times(1)
            .returning(|_, _| Ok(String::new()));

        let result = run(&cmd, "HEAD~1", None, false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_with_base_supplied() {
        let mut cmd = MockCmd::new();
        cmd.expect_exec()
            .withf(|args, verbose| {
                args == [
                    "-c",
                    "sequence.editor=:",
                    "rebase",
                    "--interactive",
                    "--autosquash",
                    "main",
                ] && !(*verbose)
            })
            .times(1)
            .returning(|_, _| Ok(String::new()));

        let result = run(&cmd, "main", None, false);

        assert!(result.is_ok());
    }
}

use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;

use crate::commands::Exec;

pub fn run<T: Exec>(command: &T, number: u32, verbose: bool) -> Result<(), Option<String>> {
    let commit = get_sha(command, number, verbose)?;
    let result = command.exec(&["commit", "--fixup", commit.as_str()], verbose, false);

    match result {
        Ok(_) => Ok(()),
        Err(()) => Err(Some(format!("Failed to fixup commit '{}'", commit))),
    }
}

fn get_sha<T: Exec>(command: &T, number: u32, verbose: bool) -> Result<String, String> {
    let options = get_log(command, number, verbose);
    let options = options?;

    let selected_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which commit you want to fix?")
        .default(0)
        .items(&options)
        .interact()
        .map_err(|err| {
            if verbose {
                println!("{err}");
            }

            format!("Failed to select commit: {}", err)
        })?;

    let option = options.get(selected_index);

    if option.is_none() {
        return Err(format!("Invalid commit selection index: {}", selected_index));
    }

    let option = option.unwrap();

    let sha = option.split_whitespace().next().unwrap();

    Ok(sha.to_string())
}

fn get_log<T: Exec>(command: &T, number: u32, verbose: bool) -> Result<Vec<String>, String> {
    let log = command
        .exec(
            &["log", "--format=%h %s", "-n", &number.to_string()],
            verbose,
            false,
        )
        .map_err(|()| format!("Failed to fetch git log (last {} commits)", number))?;

    let log = log.lines().map(String::from);
    let log = log.collect();

    Ok(log)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::MockCmd;

    #[test]
    fn test_get_log_success() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["log", "--format=%h %s", "-n", "5"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| {
                Ok("abc123 First commit\ndef456 Second commit\n789ghi Third commit".to_string())
            });

        let result = get_log(&command, 5, false);

        assert!(result.is_ok());
        let commits = result.unwrap();
        assert_eq!(commits.len(), 3);
        assert_eq!(commits[0], "abc123 First commit");
        assert_eq!(commits[1], "def456 Second commit");
        assert_eq!(commits[2], "789ghi Third commit");
    }

    #[test]
    fn test_get_log_failure() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["log", "--format=%h %s", "-n", "10"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Err(()));

        let result = get_log(&command, 10, false);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Failed to fetch git log (last 10 commits)");
    }

    #[test]
    fn test_get_log_empty_output() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["log", "--format=%h %s", "-n", "1"] && !(*verbose) && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok(String::new()));

        let result = get_log(&command, 1, false);

        assert!(result.is_ok());
        let commits = result.unwrap();
        // Empty string when split into lines becomes an empty vector
        assert_eq!(commits.len(), 0);
    }

    #[test]
    fn test_get_log_verbose() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose, inherit_stderr| {
                args == ["log", "--format=%h %s", "-n", "3"] && *verbose && !(*inherit_stderr)
            })
            .times(1)
            .returning(|_, _, _| Ok("abc123 Test commit".to_string()));

        let result = get_log(&command, 3, true);

        assert!(result.is_ok());
    }

    #[test]
    fn test_get_sha_from_valid_commit() {
        // This test focuses on the SHA extraction logic
        // Since get_sha() uses interactive selection, we test the SHA parsing indirectly
        // by testing the logic that would extract SHA from a commit line
        let commit_line = "abc123 This is a test commit message";
        let sha = commit_line.split_whitespace().next().unwrap();
        assert_eq!(sha, "abc123");
    }

    #[test]
    fn test_get_sha_from_empty_commit() {
        // Test edge case where commit line might be empty
        let commit_line = "";
        let sha = commit_line.split_whitespace().next();
        assert!(sha.is_none());
    }

    #[test]
    fn test_get_sha_from_malformed_commit() {
        // Test with just a SHA and no message
        let commit_line = "def456";
        let sha = commit_line.split_whitespace().next().unwrap();
        assert_eq!(sha, "def456");
    }
}

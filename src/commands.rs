use std::process::{self, Stdio};

use mockall::mock;

pub mod autosquash;
pub mod branch;
pub mod checkout;
pub mod cherry_pick;
pub mod delete_branches;
pub mod fixup;
pub mod git_fallback;
pub mod rebase;

pub trait Exec {
    fn exec(&self, args: &[&str], verbose: bool, inherit_stdio: bool) -> Result<String, ()>;
}

pub struct Cmd {}

impl Exec for Cmd {
    fn exec(&self, args: &[&str], verbose: bool, inherit_stdio: bool) -> Result<String, ()> {
        let cmd = "git";

        if verbose {
            println!("Executing: {} {}\n", cmd, args.join(" "));
        }

        match inherit_stdio {
            true => {
                let status = process::Command::new(cmd)
                    .args(args)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .status()
                    .expect("Failed to execute command");

                if status.success() {
                    Ok(String::new())
                } else {
                    Err(())
                }
            }
            false => {
                let output = process::Command::new(cmd)
                    .args(args)
                    .output()
                    .expect("Failed to execute command");

                let status = output.status;

                if status.success() {
                    let output = String::from_utf8(output.stdout).unwrap();

                    if verbose {
                        println!("{output}");
                    }

                    Ok(output)
                } else {
                    Err(())
                }
            }
        }
    }
}

mock! {
    pub Cmd {}

    impl Exec for Cmd {
        fn exec<'a>(&self, args: &[&'a str], verbose: bool, inherit_stdio: bool) -> Result<String, ()>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmd_exec_success_with_output() {
        let cmd = Cmd {};

        // Test with a simple git command that should succeed in most environments
        let result = cmd.exec(&["version"], false, false);

        // We expect this to succeed and return some output
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
        assert!(output.starts_with("git version"));
    }

    #[test]
    fn test_cmd_exec_verbose_output() {
        let cmd = Cmd {};

        // Test verbose flag - should still work but might produce different console output
        let result = cmd.exec(&["version"], true, false);

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.starts_with("git version"));
    }

    #[test]
    fn test_cmd_exec_invalid_command() {
        let cmd = Cmd {};

        // Test with an invalid git command
        let result = cmd.exec(&["invalid-command-that-does-not-exist"], false, false);

        // This should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_cmd_exec_inherit_stdio() {
        let cmd = Cmd {};

        // Test inherit_stdio flag - this should return empty string on success
        let result = cmd.exec(&["version"], false, true);

        // With inherit_stdio=true, successful commands return empty string
        if result.is_ok() {
            assert_eq!(result.unwrap(), "");
        }
    }

    #[test]
    fn test_cmd_exec_help() {
        let cmd = Cmd {};

        // Test with help argument which should always work
        let result = cmd.exec(&["--help"], false, false);

        // git --help should succeed and show help
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.is_empty());
        assert!(output.contains("Git") || output.contains("usage"));
    }
}

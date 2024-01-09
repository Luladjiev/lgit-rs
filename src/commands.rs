use std::process;

use mockall::mock;

pub mod branch;
pub mod delete_branches;
pub mod fixup;
pub mod rebase;

pub trait Exec {
    fn exec(&self, args: &[&str], verbose: bool) -> Result<String, ()>;
}

pub struct Cmd {}

impl Exec for Cmd {
    fn exec(&self, args: &[&str], verbose: bool) -> Result<String, ()> {
        let cmd = "git";
        let output = process::Command::new(cmd)
            .args(args)
            .output()
            .expect("Failed to execute command");

        if verbose {
            println!("Executing: {} {}\n", cmd, args.join(" "));
        }

        if output.status.success() {
            let output = String::from_utf8(output.stdout).unwrap();

            if verbose {
                println!("{output}");
            }

            Ok(output)
        } else {
            if verbose {
                println!("{}", String::from_utf8(output.stderr).unwrap());
            }

            Err(())
        }
    }
}

pub fn get_default_branch<T: Exec>(
    command: &T,
    verbose: bool,
) -> Result<&'static str, &'static str> {
    for branch in ["main", "master"] {
        if search_branch(command, branch, verbose).is_ok() {
            return Ok(branch);
        }
    }

    Err("Failed to determine default branch")
}

pub fn get_base<T: Exec>(command: &T, base: Option<String>, verbose: bool) -> String {
    base.unwrap_or_else(|| match get_default_branch(command, verbose) {
        Ok(branch) => branch.to_string(),
        Err(error) => panic!("{}", error),
    })
}

fn refresh_base<'a, T: Exec>(command: &T, base: &'a str, verbose: bool) -> Result<&'a str, ()> {
    let result = command.exec(&["checkout", base], verbose);

    if let Err(()) = result {
        return Err(());
    }

    let result = command.exec(&["pull"], verbose);

    match result {
        Ok(_) => Ok(base),
        Err(()) => Err(()),
    }
}

fn search_branch<T: Exec>(command: &T, branch: &str, verbose: bool) -> Result<(), &'static str> {
    let result = command.exec(&["branch", "-l", branch], verbose);

    match result {
        Ok(output) => {
            if output.is_empty() {
                Err("Branch not found")
            } else {
                Ok(())
            }
        }
        Err(()) => Err("Failed to list branch"),
    }
}

mock! {
    pub Cmd {}

    impl Exec for Cmd {
        fn exec<'a>(&self, args: &[&'a str], verbose: bool) -> Result<String, ()>;
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::MockCmd;

    fn cmd_branch_not_found() -> MockCmd {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose| args == ["branch", "-l", "main"] && !(*verbose))
            .times(1)
            .returning(|_, _| Ok(String::new()));

        command
    }

    fn cmd_branch_main_found() -> MockCmd {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose| args == ["branch", "-l", "main"] && !(*verbose))
            .times(1)
            .returning(|_, _| Ok("* main".to_string()));

        command
    }

    fn cmd_default_branch_master_found() -> MockCmd {
        let mut command = cmd_branch_not_found();
        command
            .expect_exec()
            .withf(|args, verbose| args == ["branch", "-l", "master"] && !(*verbose))
            .times(1)
            .returning(|_, _| Ok("* master".to_string()));

        command
    }

    fn cmd_checkout_main() -> MockCmd {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose| args == ["checkout", "main"] && !(*verbose))
            .times(1)
            .returning(|_, _| Ok(String::new()));

        command
    }

    #[test]
    fn test_get_default_branch_main() {
        let command = cmd_branch_main_found();
        let branch = super::get_default_branch(&command, false);

        assert_eq!(branch, Ok("main"));
    }

    #[test]
    fn test_get_default_branch_master() {
        let command = cmd_default_branch_master_found();
        let branch = super::get_default_branch(&command, false);

        assert_eq!(branch, Ok("master"));
    }

    #[test]
    fn test_get_base_default_to_main() {
        let command = cmd_branch_main_found();
        let base = super::get_base(&command, None, false);

        assert_eq!(base, "main");
    }

    #[test]
    fn test_get_base_default_to_master() {
        let command = cmd_default_branch_master_found();
        let base = super::get_base(&command, None, false);

        assert_eq!(base, "master");
    }

    #[test]
    fn test_get_base_supplied_base() {
        let command = MockCmd::new();
        let base = super::get_base(&command, Some("test".to_string()), false);

        assert_eq!(base, "test");
    }

    #[test]
    fn test_refresh_base_success() {
        let mut command = cmd_checkout_main();
        command
            .expect_exec()
            .withf(|args, verbose| args == ["pull"] && !(*verbose))
            .times(1)
            .returning(|_, _| Ok(String::new()));

        let result = super::refresh_base(&command, "main", false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_refresh_base_checkout_failure() {
        let mut command = MockCmd::new();
        command
            .expect_exec()
            .withf(|args, verbose| args == ["checkout", "main"] && !(*verbose))
            .times(1)
            .returning(|_, _| Err(()));

        let result = super::refresh_base(&command, "main", false);

        assert!(result.is_err());
    }

    #[test]
    fn test_refresh_base_pull_failure() {
        let mut command = cmd_checkout_main();
        command
            .expect_exec()
            .withf(|args, verbose| args == ["pull"] && !(*verbose))
            .times(1)
            .returning(|_, _| Err(()));

        let result = super::refresh_base(&command, "main", false);

        assert!(result.is_err());
    }

    #[test]
    fn test_search_branch_found() {
        let command = cmd_branch_main_found();
        let result = super::search_branch(&command, "main", false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_search_branch_not_found() {
        let command = cmd_branch_not_found();
        let result = super::search_branch(&command, "main", false);

        assert!(result.is_err());
    }
}

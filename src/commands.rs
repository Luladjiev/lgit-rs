use std::process;

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

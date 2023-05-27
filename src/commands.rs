use std::process;

pub mod branch;
pub mod delete_branches;
pub mod rebase;

fn command(cmd: &str, args: Vec<&str>, verbose: bool) -> Result<String, ()> {
    let output = process::Command::new(cmd)
        .args(&args)
        .output()
        .expect("Failed to execute command");

    if verbose {
        println!("Executing: {} {}", cmd, args.join(" "));
    }

    match output.status.success() {
        true => {
            let output = String::from_utf8(output.stdout).unwrap();

            if verbose {
                println!("{}", output);
            }

            Ok(output)
        }
        false => {
            if verbose {
                println!("{}", String::from_utf8(output.stderr).unwrap());
            }

            Err(())
        }
    }
}

fn refresh_base(base: Option<String>, verbose: bool) -> Result<String, ()> {
    let base = base.unwrap_or_else(|| match get_default_branch(verbose) {
        Ok(branch) => String::from(branch),
        Err(error) => panic!("{}", error),
    });

    let result = command("git", vec!["checkout", &base], verbose);

    if let Err(()) = result {
        return Err(());
    }

    let result = command("git", vec!["pull"], verbose);

    match result {
        Ok(_) => Ok(base),
        Err(()) => Err(()),
    }
}

fn get_default_branch(verbose: bool) -> Result<&'static str, &'static str> {
    for branch in ["main", "master"] {
        if search_branch(branch, verbose).is_ok() {
            return Ok(branch);
        }
    }

    Err("Failed to determine default branch")
}

fn search_branch(branch: &str, verbose: bool) -> Result<(), &'static str> {
    let result = command("git", vec!["branch", "-l", branch], verbose);

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

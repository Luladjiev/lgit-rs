use std::process;

pub fn delete_branches(dry_run: bool, verbose: bool) -> Result<String, String> {
    let result = command("git", vec!["fetch", "--prune"], verbose);

    if let Err(()) = result {
        return Err(String::from("Failed to fetch"));
    }

    let result = command("git", vec!["branch", "-vv"], verbose);

    match result {
        Ok(branches) => try_delete_branches(branches, dry_run, verbose),
        Err(()) => Err(String::from("Failed to get branches")),
    }
}

fn try_delete_branches(branches: String, dry_run: bool, verbose: bool) -> Result<String, String> {
    let mut result = Vec::new();

    for line in branches.lines() {
        if line.starts_with('*') {
            continue;
        }

        if line.contains(": gone]") {
            let branch_split = line.split_whitespace().next();

            let branch_name = match branch_split {
                Some(branch_name) => branch_name,
                None => return Err(String::from("Failed to parse branch name")),
            };

            if !dry_run {
                let output = command("git", vec!["branch", "-D", branch_name], verbose);

                if let Err(()) = output {
                    return Err(format!("Failed to delete branch {}", branch_name));
                }
            }

            result.push(format!("Deleted branch {}", branch_name));
        }
    }

    if result.is_empty() {
        return Ok(String::from("No branches to delete"));
    }

    Ok(result.join("\n"))
}

pub fn branch(name: String, base: String, verbose: bool) -> Result<String, String> {
    let result = refresh_base(&base, verbose);

    if let Err(()) = result {
        return Err(format!("Failed to refresh {}", base));
    }

    let result = command("git", vec!["checkout", "-b", &name], verbose);

    match result {
        Ok(_) => Ok(format!("Branch {} created", name)),
        Err(()) => Err(format!("Failed to checkout branch {}", name)),
    }
}

pub fn rebase(base: String, verbose: bool) -> Result<String, String> {
    let result = refresh_base(&base, verbose);

    if let Err(()) = result {
        return Err(format!("Failed to refresh {}", base));
    }

    let result = command("git", vec!["checkout", "-"], verbose);

    if let Err(()) = result {
        return Err(format!("Failed to checkout back to initial branch"));
    }

    let result = command("git", vec!["rebase", &base], verbose);

    if let Err(()) = result {
        return Err(format!("Failed to rebase onto {}", base));
    }

    Ok(format!("Rebased onto {}", base))
}

fn refresh_base(base: &str, verbose: bool) -> Result<String, ()> {
    let result = command("git", vec!["checkout", base], verbose);

    if let Err(()) = result {
        return Err(());
    }

    command("git", vec!["pull"], verbose)
}

fn command(cmd: &str, args: Vec<&str>, verbose: bool) -> Result<String, ()> {
    let mut proc = process::Command::new(cmd);

    if verbose {
        println!("Executing: {} {}", cmd, args.join(" "));

        proc.stdout(process::Stdio::inherit())
            .stderr(process::Stdio::inherit());
    }

    let output = proc.args(args).output().expect("Failed to execute command");

    match output.status.success() {
        true => Ok(String::from_utf8(output.stdout).unwrap()),
        false => Err(()),
    }
}

use crate::commands::command;

pub fn run(dry_run: bool, verbose: bool) -> Result<(), &'static str> {
    let result = command("git", vec!["fetch", "--prune"], verbose);

    if let Err(()) = result {
        return Err("Failed to fetch");
    }

    let result = command("git", vec!["branch", "-vv"], verbose);

    match result {
        Ok(branches) => try_delete_branches(branches, dry_run, verbose),
        Err(()) => Err("Failed to get branches"),
    }
}

fn try_delete_branches(branches: String, dry_run: bool, verbose: bool) -> Result<(), &'static str> {
    let mut result = Vec::new();

    for line in branches.lines() {
        if line.starts_with('*') {
            continue;
        }

        if line.contains(": gone]") {
            let branch_split = line.split_whitespace().next();

            let branch_name = match branch_split {
                Some(branch_name) => branch_name,
                None => return Err("Failed to parse branch name"),
            };

            if !dry_run {
                let output = command("git", vec!["branch", "-D", branch_name], verbose);

                if let Err(()) = output {
                    return Err("Failed to delete branch");
                }
            }

            result.push(format!("Deleted branch {}", branch_name));
        }
    }

    if result.is_empty() {
        println!("No branches to delete");
    } else {
        println!("{}", result.join("\n"))
    }

    Ok(())
}

use crate::commands::Exec;

pub fn run<T: Exec>(command: &T, dry_run: bool, verbose: bool) -> Result<(), &'static str> {
    let result = command.exec(&["fetch", "--prune"], verbose);

    if let Err(()) = result {
        return Err("Failed to fetch");
    }

    let result = command.exec(&["branch", "-vv"], verbose);

    match result {
        Ok(branches) => try_delete_branches(command, &branches, dry_run, verbose),
        Err(()) => Err("Failed to get branches"),
    }
}

fn try_delete_branches<T: Exec>(
    command: &T,
    branches: &str,
    dry_run: bool,
    verbose: bool,
) -> Result<(), &'static str> {
    let mut result = Vec::new();

    for line in branches.lines() {
        if line.starts_with('*') {
            continue;
        }

        if line.contains(": gone]") {
            let branch_split = line.split_whitespace().next();

            let Some(branch_name) = branch_split else {
                return Err("Failed to parse branch name");
            };

            if !dry_run {
                let output = command.exec(&["branch", "-D", branch_name], verbose);

                if let Err(()) = output {
                    return Err("Failed to delete branch");
                }
            }

            result.push(format!("Deleted branch {branch_name}"));
        }
    }

    if result.is_empty() {
        println!("No branches to delete");
    } else {
        println!("{}", result.join("\n"));
    }

    Ok(())
}

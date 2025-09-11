use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;

use crate::commands::Exec;

pub fn run<T: Exec>(
    cmd: &T,
    name: Option<String>,
    remote: bool,
    all: bool,
    verbose: bool,
) -> Result<(), &str> {
    if let Some(name) = name {
        return do_checkout(cmd, &name, verbose);
    }

    let branch = get_branches(cmd, remote, all, verbose)?;

    do_checkout(cmd, &branch, verbose)
}

fn do_checkout<T: Exec>(cmd: &T, branch: &str, verbose: bool) -> Result<(), &'static str> {
    cmd.exec(&["checkout", branch], verbose)
        .map_err(|()| "Failed to checkout branch")?;

    Ok(())
}

fn get_branches<T: Exec>(cmd: &T, remote: bool, all: bool, verbose: bool) -> Result<String, &str> {
    let remotes: Vec<String> = cmd
        .exec(&["remote"], verbose)
        .map_err(|()| "Failed to get remotes")?
        .lines()
        .map(String::from)
        .collect();

    // Determine which branches to list based on flags
    let branch_args = if all {
        vec!["branch", "-a", "--format", "%(refname)"]
    } else if remote {
        vec!["branch", "-r", "--format", "%(refname)"]
    } else {
        vec!["branch", "--format", "%(refname)"]
    };

    let mut branches: Vec<String> = cmd
        .exec(&branch_args, verbose)
        .map_err(|()| "Failed to list branches")?
        .lines()
        .map(|line| {
            let mut line = String::from(line);

            // Clean up remote prefixes
            for remote in &remotes {
                line = line.replace(&format!("refs/remotes/{}/", &remote), "");
            }

            // Clean up local branch prefix
            line.replace("refs/heads/", "").trim().to_string()
        })
        .filter(|branch| branch != "HEAD")
        .collect();

    branches.sort();
    branches.dedup();

    if branches.is_empty() {
        return Err("No branches found");
    }

    let option = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which branch to checkout?")
        .default(0)
        .items(&branches)
        .interact()
        .map_err(|err| {
            if verbose {
                println!("{err}");
            }

            "There was an error determining the branch"
        })?;

    let branch = branches.get(option);

    if branch.is_none() {
        return Err("There was an error getting the branch");
    }

    let branch = branch.unwrap();

    Ok(branch.to_string())
}

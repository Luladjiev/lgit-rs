use dialoguer::MultiSelect;

use crate::commands::Exec;

pub fn run(cmd: &dyn Exec, branch: &str, number: u32, verbose: bool) -> Result<(), &'static str> {
    let commits = get_commits(cmd, branch, number, verbose)?;

    let selections = MultiSelect::new()
        .with_prompt("Select commits to cherry-pick (use space to select, enter to confirm)")
        .items(&commits)
        .interact()
        .map_err(|_| "Failed to get user input")?;

    if selections.is_empty() {
        return Err("No commits selected");
    }

    let selected_commits: Result<Vec<&str>, String> = selections
        .iter()
        .map(|&i| match commits[i].split_whitespace().next() {
            Some(commit) => Ok(commit),
            None => Err(format!("Invalid commit format: {}\n", commits[i])),
        })
        .collect::<Result<Vec<&str>, String>>();

    let mut selected_commits = match selected_commits {
        Ok(commits) => commits,
        Err(err) => {
            if verbose {
                println!("{}", err);
            }

            return Err("Failed to parse commits format");
        }
    };

    selected_commits.reverse();

    for commit in selected_commits {
        if let Err(_) = cmd.exec(&["cherry-pick", commit], verbose) {
            return Err("Failed to cherry-pick commit");
        }
    }

    Ok(())
}

fn get_commits(
    cmd: &dyn Exec,
    branch: &str,
    number: u32,
    verbose: bool,
) -> Result<Vec<String>, &'static str> {
    let output = cmd
        .exec(
            &[
                "log",
                branch,
                "--pretty=format:%h %s",
                &format!("-n {}", number),
            ],
            verbose,
        )
        .map_err(|_| "Failed to get commit history")?;

    Ok(output.lines().map(String::from).collect())
}

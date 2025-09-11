use dialoguer::MultiSelect;

use crate::commands::Exec;

pub fn run(cmd: &dyn Exec, branch: &str, number: u32, verbose: bool) -> Result<(), String> {
    let commits = get_commits(cmd, branch, number, verbose)?;

    let selections = MultiSelect::new()
        .with_prompt("Select commits to cherry-pick (use space to select, enter to confirm)")
        .items(&commits)
        .interact()
        .map_err(|_| "Failed to get user input".to_string())?;

    if selections.is_empty() {
        return Err("No commits selected".to_string());
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
                println!("{err}");
            }

            return Err("Failed to parse commits format".to_string());
        }
    };

    selected_commits.reverse();

    for commit in selected_commits {
        if cmd.exec(&["cherry-pick", commit], verbose).is_err() {
            return Err("Failed to cherry-pick commit".to_string());
        }
    }

    Ok(())
}

fn get_commits(
    cmd: &dyn Exec,
    branch: &str,
    number: u32,
    verbose: bool,
) -> Result<Vec<String>, String> {
    let output = cmd
        .exec(
            &[
                "log",
                branch,
                "--pretty=format:%h %s",
                &format!("-n {number}"),
            ],
            verbose,
        )
        .map_err(|()| "Failed to get commit history".to_string())?;

    Ok(output.lines().map(String::from).collect())
}

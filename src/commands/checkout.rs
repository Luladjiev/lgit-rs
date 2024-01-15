use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;
use regex::Regex;

use crate::commands::Exec;

pub fn run<T: Exec>(cmd: &T, name: Option<String>, verbose: bool) -> Result<(), &str> {
    if let Some(name) = name {
        return do_checkout(cmd, &name, verbose);
    }

    let branch = get_branches(cmd, verbose)?;

    do_checkout(cmd, &branch, verbose)
}

fn do_checkout<T: Exec>(cmd: &T, branch: &str, verbose: bool) -> Result<(), &'static str> {
    cmd.exec(&["checkout", branch], verbose)
        .map_err(|()| "Failed to checkout branch")?;

    Ok(())
}

fn get_branches<T: Exec>(cmd: &T, verbose: bool) -> Result<String, &str> {
    let remotes: Vec<String> = cmd
        .exec(&["remote"], verbose)
        .map_err(|()| "Failed to get remotes")?
        .lines()
        .map(String::from)
        .collect();

    let remotes = remotes.join("|");
    let remotes = format!(r"refs(\/remotes)?\/(heads|{remotes})\/");
    let re = Regex::new(&remotes).unwrap();

    let mut branches: Vec<String> = cmd
        .exec(&["branch", "-a", "--format", "%(refname)"], verbose)
        .map_err(|()| "Failed to list branches")?
        .lines()
        .map(|line| re.replace(line, "").trim().to_string())
        .filter(|branch| branch != "HEAD")
        .collect();

    branches.sort();
    branches.dedup();

    let option = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which branch to checkout?")
        .default(0)
        .items(&branches)
        .interact()
        .map_err(|err| {
            if verbose {
                println!("{err}");
            }

            "There was an error determining the commit"
        })?;

    let branch = branches.get(option);

    if branch.is_none() {
        return Err("There was an error getting the commit");
    }

    let branch = branch.unwrap();

    Ok(branch.to_string())
}

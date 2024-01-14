use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;

use crate::commands::Exec;

pub fn run<T: Exec>(command: &T, number: u32, verbose: bool) -> Result<(), &'static str> {
    let commit = get_sha(command, number, verbose)?;
    let result = command.exec(&["commit", "--fixup", commit.as_str()], verbose);

    match result {
        Ok(_) => Ok(()),
        Err(()) => Err("Failed to fixup commit"),
    }
}

fn get_sha<T: Exec>(command: &T, number: u32, verbose: bool) -> Result<String, &'static str> {
    let options = get_log(command, number, verbose);
    let options = options?;

    let option = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which commit you want to fix?")
        .default(0)
        .items(&options)
        .interact()
        .map_err(|err| {
            if verbose {
                println!("{err}");
            }

            "There was an error determining the commit"
        })?;

    let option = options.get(option);

    if option.is_none() {
        return Err("There was an error getting the commit");
    }

    let option = option.unwrap();

    let sha = option.split_whitespace().next().unwrap();

    Ok(sha.to_string())
}

fn get_log<T: Exec>(command: &T, number: u32, verbose: bool) -> Result<Vec<String>, &'static str> {
    let log = command
        .exec(
            &["log", "--format=%h %s", "-n", &number.to_string()],
            verbose,
        )
        .map_err(|()| "Failed to fetch git log")?;

    let log = log.lines().map(String::from);
    let log = log.collect();

    Ok(log)
}

use inquire::{InquireError, Select};

use crate::commands::command;

pub fn run(number: i32, verbose: bool) -> Result<(), &'static str> {
    let commit = get_sha(number, verbose);
    let commit = commit?;

    let result = command("git", &["commit", "--fixup", commit.as_str()], verbose);

    match result {
        Ok(_) => Ok(()),
        Err(()) => Err("Failed to fixup commit"),
    }
}

fn get_sha(number: i32, verbose: bool) -> Result<String, &'static str> {
    let options = get_log(number, verbose);
    let options = options?;
    let options = options.iter().map(String::as_str).collect();

    let ans: Result<&str, InquireError> =
        Select::new("Which commit you want to fix?", options).prompt();

    match ans {
        Ok(choice) => {
            let sha = choice.split_whitespace().next().unwrap();
            Ok(sha.to_string())
        }
        Err(_) => Err("There was an error getting the commit sha"),
    }
}

fn get_log(number: i32, verbose: bool) -> Result<Vec<String>, &'static str> {
    let log = command(
        "git",
        &["log", "--format=%h %s", "-n", &number.to_string()],
        verbose,
    );

    if let Err(()) = log {
        return Err("Failed to fetch git log");
    }

    let log = log.unwrap();
    let log = log.lines().map(String::from);
    let log = log.collect();

    Ok(log)
}

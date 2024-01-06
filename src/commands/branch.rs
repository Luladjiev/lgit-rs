use crate::commands::{command, refresh_base};

pub fn run(name: &str, base: Option<String>, verbose: bool) -> Result<(), &'static str> {
    let result = refresh_base(base, verbose);

    if let Err(()) = result {
        return Err("Failed to refresh base branch");
    }

    let result = command("git", &["checkout", "-b", name], verbose);

    if let Err(()) = result {
        return Err("Failed to create branch");
    }

    println!("Created branch {name}");

    Ok(())
}

use crate::commands::{command, refresh_base};

pub fn run(base: Option<String>, verbose: bool) -> Result<(), &'static str> {
    let result = refresh_base(base, verbose);

    let base = match result {
        Ok(base) => base,
        Err(()) => return Err("Failed to refresh base branch"),
    };

    let result = command("git", vec!["checkout", "-"], verbose);

    if let Err(()) = result {
        return Err("Failed to checkout back to initial branch");
    }

    let result = command("git", vec!["rebase", &base], verbose);

    if let Err(()) = result {
        return Err("Failed to rebase");
    }

    println!("Rebased onto {}", base);

    Ok(())
}

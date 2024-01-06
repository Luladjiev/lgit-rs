use crate::commands::{command, refresh_base};

pub fn run(base: Option<String>, verbose: bool) -> Result<(), &'static str> {
    let result = refresh_base(base, verbose);

    let Ok(base) = result else { return Err("Failed to refresh base branch"); };

    let result = command("git", &["checkout", "-"], verbose);

    if let Err(()) = result {
        return Err("Failed to checkout back to initial branch");
    }

    let result = command("git", &["rebase", &base], verbose);

    if let Err(()) = result {
        return Err("Failed to rebase");
    }

    println!("Rebased onto {base}");

    Ok(())
}

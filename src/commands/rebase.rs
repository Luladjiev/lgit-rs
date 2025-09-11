use crate::commands::Exec;
use crate::utils::{refresh_base, stash, unstash};

pub fn run<T: Exec>(command: &T, base: &str, verbose: bool) -> Result<(), Option<String>> {
    let unsaved_changes = stash(command, verbose)?;

    refresh_base(command, base, verbose)
        .map_err(|()| "Failed to refresh base branch".to_string())?;

    command
        .exec(&["checkout", "-"], verbose, false)
        .map_err(|()| "Failed to checkout back to initial branch".to_string())?;

    command
        .exec(&["rebase", base], verbose, false)
        .map_err(|()| "Failed to rebase".to_string())?;

    if unsaved_changes {
        unstash(command, verbose)?;
    }

    println!("Rebased onto {base}");

    Ok(())
}

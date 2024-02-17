use crate::commands::Exec;
use crate::utils::{refresh_base, stash, unstash};

pub fn run<T: Exec>(command: &T, base: &str, verbose: bool) -> Result<(), &'static str> {
    let unsaved_changes = stash(command, verbose)?;

    refresh_base(command, base, verbose).map_err(|()| "Failed to refresh base branch")?;

    command
        .exec(&["checkout", "-"], verbose)
        .map_err(|()| "Failed to checkout back to initial branch")?;

    command
        .exec(&["rebase", base], verbose)
        .map_err(|()| "Failed to rebase")?;

    if unsaved_changes {
        unstash(command, verbose)?;
    }

    println!("Rebased onto {base}");

    Ok(())
}

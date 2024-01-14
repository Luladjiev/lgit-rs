use crate::commands::Exec;
use crate::utils::refresh_base;

pub fn run<T: Exec>(command: &T, base: &str, verbose: bool) -> Result<(), &'static str> {
    let result = refresh_base(command, base, verbose);

    let Ok(base) = result else {
        return Err("Failed to refresh base branch");
    };

    let result = command.exec(&["checkout", "-"], verbose);

    if let Err(()) = result {
        return Err("Failed to checkout back to initial branch");
    }

    let result = command.exec(&["rebase", base], verbose);

    if let Err(()) = result {
        return Err("Failed to rebase");
    }

    println!("Rebased onto {base}");

    Ok(())
}

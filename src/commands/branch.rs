use crate::commands::{refresh_base, Exec};

pub fn run<T: Exec>(
    command: &T,
    name: String,
    base: String,
    verbose: bool,
) -> Result<(), &'static str> {
    let result = refresh_base(command, &base, verbose);

    if let Err(()) = result {
        return Err("Failed to refresh base branch");
    }

    let result = command.exec(&["checkout", "-b", &name], verbose);

    if let Err(()) = result {
        return Err("Failed to create branch");
    }

    println!("Created branch {name}");

    Ok(())
}

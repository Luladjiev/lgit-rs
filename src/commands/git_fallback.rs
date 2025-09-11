use crate::commands::Exec;

pub fn run(cmd: &dyn Exec, args: &[String], verbose: bool) -> Result<(), Option<String>> {
    let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    match cmd.exec(&str_args, verbose, true) {
        Ok(_) => Ok(()),
        Err(_) => Err(None),
    }
}

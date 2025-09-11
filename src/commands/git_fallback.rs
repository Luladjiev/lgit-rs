use crate::commands::Exec;

pub fn run(cmd: &dyn Exec, args: &[String], verbose: bool) -> Result<(), &'static str> {
    let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    match cmd.exec(&str_args, verbose) {
        Ok(output) => {
            if !output.trim().is_empty() {
                println!("{}", output.trim());
            }
            Ok(())
        }
        Err(_) => Err("Git command failed"),
    }
}

use crate::commands::Exec;

pub fn run(cmd: &dyn Exec, args: &[String], verbose: bool) -> Result<(), String> {
    let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    match cmd.exec(&str_args, verbose) {
        Ok(output) => {
            if !output.trim().is_empty() {
                println!("{}", output.trim());
            }
            Ok(())
        }
        Err(_) => {
            let command = format!("git {}", args.join(" "));
            let error_msg = format!("Git command '{}' failed", command);
            Err(error_msg)
        }
    }
}

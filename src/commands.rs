use std::io::{stderr, stdout, Write};
use std::process;

pub fn delete_branches(dry_run: bool, verbose: bool) -> String {
    command("git", vec!["fetch", "--prune"], verbose);

    let branches = command("git", vec!["branch", "-vv"], verbose);

    let mut result = Vec::new();

    for line in branches.lines() {
        if line.starts_with('*') {
            continue;
        }

        if line.contains(": gone]") {
            let branch_name = line.split_whitespace().next().unwrap();

            if !dry_run {
                process::Command::new("git")
                    .args(["branch", "-d", branch_name])
                    .output()
                    .expect("Failed to delete branch");
            }

            result.push(format!("Deleted branch {}", branch_name));
        }
    }

    if result.is_empty() {
        return "No branches to delete".to_string();
    }

    result.join("\n")
}

pub fn branch(name: String, base: String, verbose: bool) -> String {
    refresh_base(&base, verbose);

    command("git", vec!["checkout", "-b", &name], verbose);

    format!("Branch {} created", name)
}

pub fn rebase(base: String, verbose: bool) -> String {
    refresh_base(&base, verbose);

    command("git", vec!["rebase", &base], verbose);

    format!("Rebased onto {}", base)
}

fn refresh_base(base: &str, verbose: bool) {
    command("git", vec!["checkout", base], verbose);
}

fn command(cmd: &'static str, args: Vec<&str>, verbose: bool) -> String {
    let output = process::Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to execute command");

    if verbose {
        stdout().write_all(&output.stdout).unwrap();
        stderr().write_all(&output.stderr).unwrap();
    }

    String::from_utf8(output.stdout).unwrap()
}

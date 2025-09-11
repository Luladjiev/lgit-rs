use std::process::{self, Stdio};

use mockall::mock;

pub mod autosquash;
pub mod branch;
pub mod checkout;
pub mod cherry_pick;
pub mod delete_branches;
pub mod fixup;
pub mod git_fallback;
pub mod rebase;

pub trait Exec {
    fn exec(&self, args: &[&str], verbose: bool) -> Result<String, ()>;
}

pub struct Cmd {}

impl Exec for Cmd {
    fn exec(&self, args: &[&str], verbose: bool) -> Result<String, ()> {
        let cmd = "git";
        
        if verbose {
            println!("Executing: {} {}\n", cmd, args.join(" "));
        }

        let status = process::Command::new(cmd)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("Failed to execute command");

        if status.success() {
            Ok(String::new())
        } else {
            Err(())
        }
    }
}

mock! {
    pub Cmd {}

    impl Exec for Cmd {
        fn exec<'a>(&self, args: &[&'a str], verbose: bool) -> Result<String, ()>;
    }
}

use std::process;

use mockall::mock;

pub mod autosquash;
pub mod branch;
pub mod checkout;
pub mod cherry_pick;
pub mod delete_branches;
pub mod fixup;
pub mod rebase;

pub trait Exec {
    fn exec(&self, args: &[&str], verbose: bool) -> Result<String, ()>;
}

pub struct Cmd {}

impl Exec for Cmd {
    fn exec(&self, args: &[&str], verbose: bool) -> Result<String, ()> {
        let cmd = "git";
        let output = process::Command::new(cmd)
            .args(args)
            .output()
            .expect("Failed to execute command");

        if verbose {
            println!("Executing: {} {}\n", cmd, args.join(" "));
        }

        if output.status.success() {
            let output = String::from_utf8(output.stdout).unwrap();

            if verbose {
                println!("{output}");
            }

            Ok(output)
        } else {
            if verbose {
                println!("{}", String::from_utf8(output.stderr).unwrap());
            }

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

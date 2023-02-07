//make a very simple shell

use std::ffi::CString;
use std::env;
use std::io; 
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, execvp}};

fn main() {
    let path = env::current_dir();
    println!("The current directory is {}", path.expect("REASON").display());
    //Ok(())
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Continuing execution in parent process, new child has pid: {}", child);
            waitpid(child, None).unwrap();
            println!("Returned to parent - child is finished.");
        }
        Ok(ForkResult::Child) => {
            let cmd = externalize("ls -l -a");
            println!("{cmd:?}");
            match execvp::<CString>(cmd[0].as_c_str(), &cmd) {
                Ok(_) => {println!("Child finished");},
                Err(e) => {println!("Could not execute: {e}");},
            }
        }
        Err(_) => println!("Fork failed"),
     }
}

fn externalize(command: &str) -> Vec<CString> {
    command.split_whitespace()
        .map(|s| CString::new(s).unwrap())
        .collect()
}
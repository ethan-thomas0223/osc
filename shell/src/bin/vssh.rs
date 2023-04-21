//create a very simple shell

use std::ffi::CString;
use std::io::Write;
use std::path::Path;
use std::env;
use std::io; 
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, execvp}};
use anyhow::Result;




fn main() {

    loop {
        match handle_client() {
            Ok(all_good) => {
                if !all_good {
                    break;
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
}




fn handle_client() -> anyhow::Result<bool> {
    let mut cmd = String::new();
    //let path = env::current_dir();
    print!("-$- ");
    //reset stdout after evey execution of cmd, std gets reset with new input
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut cmd)?;
    //finds keyword to kill program other than ctrl c 
    if cmd.trim() == "exit" || cmd.trim() == "quit"{
        print!("exiting program \n");
        Ok(false)
    } else {
        let commands: Vec<&str> = cmd.split_whitespace().collect();
        //println!("You entered the follwoing command: {commands:?}");
        if commands[0].contains("cd") {
            //makes new path and sets that path to current dir, prints it out
            //bugs related to white space in filepath
            let mut filepath: Vec<&str> = cmd.split("cd").collect();
            let mut path = Path::new(filepath[1].trim());
            env::set_current_dir(path);
            let path = env::current_dir();
            println!("The current directory is {}", path.expect("REASON").display());
        }else {
            match unsafe{fork()}? {
                ForkResult::Parent { child } => {
                    println!("Continuing execution in parent process, new child has pid: {}", child);
                    waitpid(child, None).unwrap();
                    println!("Returned to parent - child is finished.");
                }

                ForkResult::Child => {
                    let cmd2 = externalize(cmd.as_str());
                    match execvp(cmd2[0].as_c_str(), &cmd2) {
                        Ok(_) => {println!("Child finished");},
                        Err(e) => {println!("Error: {e}");},
                    }
                }
            }
        }
        Ok(true)
    }

}


fn externalize(command: &str) -> Vec<CString> {
    command.split_whitespace()
    .map(|s| CString::new(s).unwrap()).collect()

}


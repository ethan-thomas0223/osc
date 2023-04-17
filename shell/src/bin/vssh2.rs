//create an enhanced shell

use std::ffi::CString;
//use std::intrinsics::rotate_right;
use std::io::Write;
use std::path::Path;
use std::env;
use std::io; 
//use anyhow::Ok;
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, execvp}};
use anyhow::Result;
//use check::*;
use nix::unistd::dup2;
use nix::unistd::close;
use nix::unistd::pipe;

//& at the end means run in BG, ie don't want for child to finish
// | means output of right goes into left
// > means filename follows and result goes into that filename (last arg), create file if none
// < means input should be taken from the filename (1st arg), if 404 abort 


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
        //can produce bugs/errors with the cd command if filepath contains spaces
        //works if just type filepath without any spaces in it
        let commands: Vec<&str> = cmd.split_whitespace().collect();
        //println!("You entered the follwoing command: {commands:?}");
        if commands[0].contains("cd") {
            //makes new path and sets that path to current dir, prints it out
            let mut path = Path::new(commands[1]);
            env::set_current_dir(path).unwrap();
            let path = env::current_dir();
            println!("The current directory is {}", path.expect("REASON").display());
        }else {
            match unsafe{fork()}? {
                ForkResult::Parent { child } => {
                    println!("Continuing execution in parent process, new child has pid: {}", child);
                    //if no ampersand sign, runs normally, otherwise don't wait for child completion
                    if !commands.contains(&"&") {
                        waitpid(child, None).unwrap();
                    }
                    println!("Returned to parent - child is finished.");
                }

                ForkResult::Child => {
                    if commands.contains(&"|"){
                        //print!("here \n");
                        let mut piped: Vec<&str> = cmd.split("|").collect();
                        let mut counter = 0;
                        for arg in &piped{
                            if counter < piped.len() - 1{
                                println!("here \n");
                                //idk whay this either doesn't run or doesn't return anything
                                //either ask hop or meet with ferrer
                                
                                pipline(piped[counter + 1], piped[counter]).unwrap();
                               
                                
                            };
                            counter += 1; 
                        }
                        counter = 0;
                        //pipline(right_arg, left_arg)
                    }else{
                        let cmd2 = externalize(cmd.as_str());
                        match execvp(cmd2[0].as_c_str(), &cmd2) {
                            Ok(_) => {println!("Child finished");},
                            Err(e) => {println!("Error: {e}");},
                        }
                    }
                    
                }
            }
        }
        Ok(true)
    }

}

fn pipline(right_arg: &str, left_arg: &str) -> anyhow::Result<bool>  {
    //println!("Execute ls -l | wc");

    //let ls: Vec<CString> = vec![CString::new("ls")?, CString::new("-l")?];
    //let wc: Vec<CString> = vec![CString::new("wc")?];
    let ls = externalize(left_arg);
    let wc = externalize(right_arg);
    match unsafe {fork()}? {
        nix::unistd::ForkResult::Parent { child } => {
            println!("wc pid is {child}");
            waitpid(child, None).unwrap();
            println!("Finished!");
        },
        nix::unistd::ForkResult::Child => {
            let (wc_in, ls_out) = pipe()?;
            match unsafe {fork()}? {
                nix::unistd::ForkResult::Parent { child: _ } => {
                    close(ls_out)?;
                    dup2(wc_in, 0)?;
                    execvp(&wc[0], &wc)?;
                }
                nix::unistd::ForkResult::Child => {
                    close(wc_in)?;
                    dup2(ls_out, 1)?;
                    execvp(&ls[0], &ls)?;
                }
            }
        }
    }
    Ok(true)
}

fn externalize(command: &str) -> Vec<CString> {
    command.split_whitespace()
    .map(|s| CString::new(s).unwrap()).collect()

}
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
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, dup2, pipe};


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
            let mut filepath: Vec<&str> = cmd.split("cd").collect();
            let mut path = Path::new(filepath[1].trim());
            env::set_current_dir(path)?;
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
                        let mut args: Vec<&str> = cmd.split(&"|").collect();
                        let mut cur_fd_out = 1; 
                        let num_args = args.len()-1; 
                        if cmd.contains(">"){
                            let commands:Vec<&str> = args.last().expect("REASON").split(">").collect();
                            let flags: OFlag = [OFlag::O_CREAT, OFlag::O_WRONLY, OFlag::O_TRUNC].iter().copied().collect();
                            let mode: Mode = [Mode::S_IRUSR, Mode::S_IWUSR].iter().copied().collect();
                            let newfd = open(commands[1].trim(), flags, mode)?;
                            args[num_args] = commands[0];
                            cur_fd_out = newfd;
                        }
                        //let ls: Vec<CString> = vec![CString::new("ls")?, CString::new("-l")?];
                        //let wc: Vec<CString> = vec![CString::new("wc")?];
                        
                        for command in args.iter().skip(1).rev(){
                            let (output, input) = pipe()?;
                                
                            match unsafe {fork()} {
                                
                                Ok(nix::unistd::ForkResult::Parent { child: _ }) => {
                                    close(input)?;
                                    dup2(cur_fd_out, 1)?;
                                    dup2(output, 0)?;
                                    let cmd2 = externalize(command);
                                    execvp(&cmd2[0], &cmd2)?;
                                }
                        
                                Ok(nix::unistd::ForkResult::Child) => {
                                    close(output)?;
                                    cur_fd_out = input;
                                }
                                Err(e) => {println!("Error: {e}");},
                                
                            } 

                        }
                        if cmd.contains("<") {

                            let commands:Vec<&str> = cmd.split("<").collect();
                            let cur_fd_out = open(commands[1].trim(), OFlag::O_RDONLY, Mode::empty())?;
                            dup2(cur_fd_out, 0)?;
                            args[0] = commands[0];
                            
                        }
                        let beg = externalize(args[0]);
                        dup2(cur_fd_out, 1)?;

                        match execvp(beg[0].as_c_str(), &beg) {
                            Ok(_) => {println!("Child finished");},
                            Err(e) => {println!("Error: {e}");},
                        }
                        
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


fn pipeline(cmd: String) -> anyhow::Result<bool> {
    //println!("Execute ls -l | wc");
    let mut args: Vec<&str> = cmd.split(&"|").collect();
    let mut cur_fd_out = 1; 
    let num_args = args.len()-1; 
    if cmd.contains(">"){
        let commands:Vec<&str> = args.last().expect("REASON").split(">").collect();
        let flags: OFlag = [OFlag::O_CREAT, OFlag::O_WRONLY, OFlag::O_TRUNC].iter().copied().collect();
        let mode: Mode = [Mode::S_IRUSR, Mode::S_IWUSR].iter().copied().collect();
        let newfd = open(commands[1].trim(), flags, mode)?;
        args[num_args] = commands[0];
        cur_fd_out = newfd;
    }
    //let ls: Vec<CString> = vec![CString::new("ls")?, CString::new("-l")?];
    //let wc: Vec<CString> = vec![CString::new("wc")?];
    
    for command in args.iter().skip(1).rev(){
        let (output, input) = pipe()?;
            
        match unsafe {fork()} {
            
            Ok(nix::unistd::ForkResult::Parent { child: _ }) => {
                close(input)?;
                dup2(cur_fd_out, 1)?;
                dup2(output, 0)?;
                let cmd2 = externalize(command);
                execvp(&cmd2[0], &cmd2)?;
            }
    
            Ok(nix::unistd::ForkResult::Child) => {
                close(output)?;
                cur_fd_out = input;
            }
            Err(e) => {println!("Error: {e}");},
            
        } 

    }
    if cmd.contains("<") {

        let commands:Vec<&str> = cmd.split("<").collect();
        let cur_fd_out = open(commands[1].trim(), OFlag::O_RDONLY, Mode::empty())?;
        dup2(cur_fd_out, 0)?;
        args[0] = commands[0];
        
    }
    let beg = externalize(args[0]);
    dup2(cur_fd_out, 1)?;

    match execvp(beg[0].as_c_str(), &beg) {
        Ok(_) => {println!("Child finished");},
        Err(e) => {println!("Error: {e}");},
    }
    Ok(true)
}

fn externalize(command: &str) -> Vec<CString> {
    command.split_whitespace()
    .map(|s| CString::new(s).unwrap()).collect()

}
//make an enhanced shell

use std::ffi::CString;
use std::env;
use std::io; 
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, execvp}};
use check::*;



fn main() -> anyhow::Result<()> {
    let path = env::current_dir();
    println!("The current directory is {}", path.expect("REASON").display());
    
    //essentially just working on the pipeline enhancement here 
    let mut args: Vec<String> = std::env::args().skip(1).split("|").collect();  
    let mut isbg = CmdChecker.checkBG(args);
    let mut file_out = CmdChecker.checkOut(args);
    let mut file_in = CmdChecker.checkIn(args);
    if len(args) > 1{
        //meaning there would at least 2 args, therefore a pipe -> run the pipeline
        match unsafe {fork()}? {
            nix::unistd::ForkResult::Parent { child: _ } => {
                println!("Continuing execution in parent process, new child has pid: {}", child);  
                //checking to see if bg program, 
                //seems like this isn't the right spot though and is bug inducing
                if isbg == false {
                    //wait on child if not true
                    waitpid(child, None).unwrap();
                }else{
                    //if true remove ampersand and don't wait on child
                    args[0].remove(len(args[0]-1));
                }
                println!("Returned to parent - child is finished.");
            }
            nix::unistd::ForkResult::Child => {
                pipeline(args);
            }
        }
    }else{
        //there's no pipe, so execute command as is 
        let cmd = externalize("{args[0]}");
            println!("{cmd:?}");
            match execvp::<CString>(cmd[0].as_c_str(), &cmd) {
                Ok(_) => {println!("Child finished");},
                Err(e) => {println!("Could not execute: {e}");},
            }
    }
    Ok(())
}


fn pipeline(args: Vec<String>) -> anyhow::Result<()>{
    //last command runs 1st, 1st command runs last?
    let mut curarg = len(args) - 1;
    let mut fd = 1; 
    for arg in args.rev() {
        if curarg > 1 {
          
            let mut rhs: Vec<CString> = vec![arg.split_whitespace().as_c_str()?]; 

            match unsafe {fork()}? {
                nix::unistd::ForkResult::Parent { child } => {
                    println!("wc pid is {child}");
                    waitpid(child, None).unwrap();
                    println!("Finished!");
                },
                nix::unistd::ForkResult::Child => {
                    let (rhs_in, lhs_out) = pipe()?;
                    match unsafe {fork()}? {
                        nix::unistd::ForkResult::Parent { child: _ } => {
                            close(lhs_out)?;
                            let flags: OFlag = [OFlag::O_CREAT, OFlag::O_WRONLY, OFlag::O_TRUNC].iter().copied().collect();
                            let mode: Mode = [Mode::S_IRUSR, Mode::S_IWUSR].iter().copied().collect();
                            //let file_out = open("wc.out", flags, mode)?;
                            //dup2(file_out, 1)?;
                            dup2(fd, 1)?;
                            dup2(lhs_out, 0)?;
                            execvp(&rhs[0], &rhs)?;
                        }
                        nix::unistd::ForkResult::Child => {
                            close(rhs_in)?;
                            fd = rhs_in; 
                            //dup2(lhs_out, 1)?;
                            //execvp(&lhs[0], &lhs)?;
                        }
                    }
                }
            }
        }
        curarg -= 1;
        
    }
    let nargs = externalize(args[0]);
    dup2(fd, 1);
    execvp(&nargs[0], &nargs);
    
    Ok(())
}

fn externalize(command: &str) -> Vec<CString> {
    command.split_whitespace()
        .map(|s| CString::new(s).unwrap())
        .collect()
}






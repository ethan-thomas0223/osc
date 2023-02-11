//make a very simple shell

use std::ffi::CString;
use std::env;
use std::io; 
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, execvp}};

fn main() -> anyhow::Result<()> {
    let path = env::current_dir();
    println!("The current directory is {}", path.expect("REASON").display());
    let mut args: Vec<String> = std::env::args().skip(1).split("|").collect();  

    if len(args) > 1{
        //meaning there would at least 2 args, therefore a pipe -> run the pipeline
        pipeline(args);
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
    //reverse iterate through args?

    //convert args to CString   
    let mut lastarg = len(args);
    for arg in args{

        //let csarg1:Vec<CString> = vec![CString::new("ls")?, CString::new("-l")?];
        let csarg1: Vec<CString> = vec![args[lastarg].split_whitespace().as_c_str()?]; 
        let csarg2:Vec<CString> = vec![args[lastarg-1].split_whitespace().as_c_str()?];

        match unsafe {fork()}? {
            nix::unistd::ForkResult::Parent { child } => {
                println!("wc pid is {child}");
                waitpid(child, None).unwrap();
                println!("Finished!");
            },
            nix::unistd::ForkResult::Child => {
                let (csarg1_in, csarg2_out) = pipe()?;
                match unsafe {fork()}? {
                    nix::unistd::ForkResult::Parent { child: _ } => {
                        close(csarg2_out)?;
                        let flags: OFlag = [OFlag::O_CREAT, OFlag::O_WRONLY, OFlag::O_TRUNC].iter().copied().collect();
                        let mode: Mode = [Mode::S_IRUSR, Mode::S_IWUSR].iter().copied().collect();
                        //let file_out = open("wc.out", flags, mode)?;
                        //dup2(file_out, 1)?;
                        dup2(csarg1_in, 0)?;
                        execvp(&csarg1[0], &csarg1)?;
                    }
                    nix::unistd::ForkResult::Child => {
                        close(csarg1_in)?;
                        dup2(csarg2_out, 1)?;
                        execvp(&csarg2[0], &csarg2)?;
                    }
                }
            }
        }
    }
    

    OK(())
}

fn externalize(command: &str) -> Vec<CString> {
    command.split_whitespace()
        .map(|s| CString::new(s).unwrap())
        .collect()
}
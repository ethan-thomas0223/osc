use std::env;
use std::io; 

struct CmdChecker {
    cmd: String,
}
impl Type{
    //check if pipe, return bool
    fn checkPipe(&self) -> bool { 
        let mut ispipe = false;
        for arg in self.cmd{
            if arg == "|" {
                ispipe = true;
            }else{
                ispipe = false;
            }
        }
        if ispipe == true {
            return 
        }else{
            return 
        }
    }
    //check for outfile, remove from args, and return filename as str
    fn checkOut(&self) -> String {
        for arg in self.cmd{
            if arg == ">" {
                return cmd.split(">").collect().last().to_string();
            }else[
                return "".to_string();
            ]
        }
    }
    //check for input file, remove from args, and return filename as str
    fn checkPipe(&self) -> String {
        for arg in self.cmd{
            if arg == "<" {
                let args = cmd.split("<").collect();
                return args[0].to_string();
            }else[
                return "".to_string();
            ]
        }
    }
    //check to see ampersand,
    fn checkBG(&self) -> bool {
        for arg in self.cmd{
            if arg == "&" {
                return true;
            }else[
                return false;
            ]
        }
    }
}
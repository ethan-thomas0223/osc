use std::env;
use std::io; 

struct CmdChecker {
    cmd: Vec<String>,
}
impl CmdChecker{
    //already check if pipe, 
    
    //check for outfile, remove from args, and return filename as str
    fn checkOut(&self) -> String {
        for args in self.cmd{
            for arg in args{
                if arg == ">" {
                    return args.last().to_string();
                }else{
                    return "".to_string();
                }
            }
        }          
    }
    
    //check for input file, remove from args, and return filename as str
    fn checkIn(&self) -> String {
        for args in self.cmd{
            for arg in args{
                if arg == "<" {
                    return args[0].to_string();
                }else {
                    return "".to_string();
                }
            }
        }
    }
    //check to see ampersand, 
    fn checkBG(&self) -> bool {
        for arg in self.cmd{
            if arg == "&" {
                return true;
            }else{
                return false;
            }
        }
    }
}
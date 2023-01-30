use std::io;
use std::fs; 


fn main() {
    //print out all filenames and directories in current directory
    //basically the ls command 
    let paths = fs::read_dir("./").unwrap();   

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}
    




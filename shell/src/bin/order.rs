//Works like cat, except the output lines must be sorted before being output. 
//All lines from all files will be mixed together and then sorted. 
//If the “-r” command-line argument is provided, 
//they should be sorted in reverse order.

use std::io;
use std::fs; 

fn main() {
    let mut rev = false; 
    //let mut path = "";  
    let mut counter = 0;  
    for arg in std::env::args().skip(1){
        counter += 1;
        if arg.contains("-r"){
            rev = true; 
        }

        if counter > 1 {
            let path = &arg;
            get_order(rev, path); 

        }
        
    }
    
}

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader; 

fn get_order(rev: bool, path: &str) -> std::io::Result<()> {
    let f = File::open(path)?;
    let f = BufReader::new(f);
    let mut linelist = Vec::new(); 
    for line in f.lines() {
        linelist.push(line);
    }
    if rev == true {
        linelist.sort_by(|w1, w2| w2.cmp(&w1));
    }
    else{
        linelist.sort(); 
    }
    Ok(())
}
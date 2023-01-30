//Prints out the first ten lines of each file listed in the command-line arguments. 
//If the first argument begins with a dash, use the number immediately following the dash instead of ten.
//cargo run --bin start -15 filepath filepath



use std::fs; 

fn main() {
    //let paths = fs::read_dir("./").unwrap(); 
    
    
    let counter: i32 = 0;
    let lcount: i32 = 10; 
    for arg in std::env::args().skip(1){
        counter += 1;
        if counter == 1{
            if arg.contains("-"){
                let strnum = &arg[1..]; 
                let num: i32 = strnum.parse().unwrap();
                lcount = num; 
            }

        }else if counter > 1{
            println!("{}", read_lines(lcount, &arg)); 
        }
    }

    
}
    
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader; 
use std::io; 

fn read_lines<>(numLines: i32, filepath: &str) -> io::Result<()> {
    let f = File::open(filepath)?;
    let f = BufReader::new(f);
    let i = 0; 
    for line in f.lines() {
        
        if i != numLines{
            println!("{}", line.unwrap());
        }else{
            break; 
        }
        i += 1;
    }

    Ok(())
}



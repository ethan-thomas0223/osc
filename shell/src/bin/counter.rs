//Prints out the number of words, lines, and characters for each file listed in its command-line arguments. 
//If the first argument begins with a dash, the letters “w”, “l”, and “c” 
//immediately following the dash indicate which of words, lines, and characters get displayed.
use std::fs; 

fn main() {
    //let paths = fs::read_dir("./").unwrap(); 

    
    let mut counter: i32 = 0;
    let mut lcount: i32 = 10; 
    for arg in std::env::args().skip(1){
        counter += 1;
        if counter == 1{
            if arg.contains("-"){
                let strnum = &arg[1..]; 
                let num: i32 = strnum.parse().unwrap();
                lcount = num; 
            }

        }else {
            println!("Hi!");
            read_lines(lcount, &arg).unwrap(); 
        }
    }

    
}
    
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader; 
use std::io; 

fn read_lines<>(numLines: i32, filepath: &str) -> std::io::Result<()> {
    let f = File::open(filepath)?;
    let f = BufReader::new(f);
    let mut i = 0; 
    for line in f.lines() {
        println!("{i}");
        if i < numLines{
            println!("{}", line.unwrap());
        }else{
            break; 
        }
        i += 1;
    }

    Ok(())
}

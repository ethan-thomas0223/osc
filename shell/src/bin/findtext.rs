//Output every line that contains a specified pattern. 
//The first command-line argument is the fixed-string pattern.
//Remaining arguments are the names of the files to inspect.
use std::fs; 


fn main() {
    let mut counter: i32 = 0;
    let mut target: String = " ".to_string();
    for arg in std::env::args().skip(1){
        counter += 1;
        if counter == 1{
            if arg.contains("-"){
                target = arg[1..].to_string();
                //println!("{target}");
            }
        }
        else {
            //println!("Hi!");
            //println!("{target}");
            read_lines(target.as_str(), &arg).unwrap(); 
        }
    }

}

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader; 
use std::io; 

fn read_lines<>(target: &str, filepath: &str) -> std::io::Result<()> {
    println!("{target}");
    println!("{filepath}");
    let f = File::open(filepath)?;
    let f = BufReader::new(f);
    for line in f.lines() {
        
        match line {
            Ok(line) => if line.contains(target) {println!("{}", line) },   
            Err(e) => println!{"errorno: {e}"} 
        } 
    } 

    Ok(())
}
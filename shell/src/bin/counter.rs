//Prints out the number of words, lines, and characters for each file listed in its command-line arguments. 
//If the first argument begins with a dash, the letters “w”, “l”, and “c” 
//immediately following the dash indicate which of words, lines, and characters get displayed.




use std::fs; 

fn main() {
    //let paths = fs::read_dir("./").unwrap(); 
    
    
    let counter: i32 = 0;
    let lcount: i32 = 10; 
    for arg in std::env::args().skip(1){
        counter += 1;
        if counter == 1{
            if arg.contains("-"){
                if arg[1] == "w"{
                    w = get_parts(arg);
                }
                else if arg[1] == "l"{
                    l = get_parts(arg);
                }else if arg[1] == "c"{
                    c = get_parts(arg);
                }
            }

        }else if counter > 1{
            println!("{}", get_parts(arg)); 
        }
    }

    
}
    
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader; 

fn get_parts(filepath: &str) -> std::io::Result<()> {
    let f = File::open(filepath)?;
    let f = BufReader::new(f);
    let linecounter = 0; 
    let wordcounter = 0;
    let charcounter = 0;
    for line in f.lines() {
        linecounter += 1;
        for char in line.unwrap(){
            charcounter += 1;
            if char == " "{
                wordcounter += 1;
            }
        }

        
            
        
    }
    println!("{}", wordcounter, linecounter, charcounter);
    Ok(())
}
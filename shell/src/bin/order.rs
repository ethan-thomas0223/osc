//Works like cat, except the output lines must be sorted before being output. 
//All lines from all files will be mixed together and then sorted. 
//If the “-r” command-line argument is provided, 
//they should be sorted in reverse order.

use std::fs; 


fn main() {
    let mut counter: i32 = 0;
    //let mut target: String = " ".to_string();
    let mut rev = false; 
    for arg in std::env::args().skip(1){
        counter += 1;
        if counter == 1{
            if arg.contains("-r"){
                //target = arg[1..].to_string();
                //println!("{target}");
                rev = true; 
            }
        }
        else {
            //println!("Hi!");
            //println!("{target}");
            get_order(rev, &arg).unwrap(); 
        }
    }

}

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader; 
use std::io; 

fn get_order(rev: bool, filepath: &str) -> std::io::Result<()> {
    println!("{rev}");
    println!("{filepath}");
    let f = File::open(filepath)?;
    let f = BufReader::new(f);
    let mut linelist: Vec<String> = Vec::new();
    for line in f.lines() {
        linelist.push(line? as String); 
    } 
    if rev == false {
        //println!("sort normal here");
        linelist.sort_by(|a: &String, b:&String| a.trim().to_lowercase().cmp(&b.trim().to_lowercase()));

    }else{
        //println!("sort reverse order here");
        //got some help from Andrei with the sort_by line
        linelist.sort_by(|a: &String, b:&String| b.trim().to_lowercase().cmp(&a.trim().to_lowercase()));
    }
    for l in linelist{
        println!("{l}");
    }   
    Ok(())
}
//Prints out the number of words, lines, and characters for each file listed in its command-line arguments. 
//If the first argument begins with a dash, the letters “w”, “l”, and “c” 
//immediately following the dash indicate which of words, lines, and characters get displayed.
use std::fs; 

fn main() {
    //let paths = fs::read_dir("./").unwrap(); 
    let mut counter: i32 = 0;
    let mut target = "".to_string();
    for arg in std::env::args().skip(1){
        counter += 1;
        if counter == 1{
            if arg.contains("-"){
                if arg.contains("-"){
                    target = arg[1..].to_string();
                    //println!("{target}");
                } 
            }
        }else {
            //println!("Hi!");
            read_lines(&target, &arg).unwrap(); 
        }
    }

    
}
    
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader; 
use std::io; 
use std::collections::HashMap;

fn read_lines<>(target: &str, filepath: &str) -> std::io::Result<()> {
    println!("{target}");
    println!("{filepath}");
    let f = File::open(filepath)?;
    let f = BufReader::new(f);
    let mut counts = HashMap::new(); 
    let mut linecount = "l";
    let mut wordcount = "w";
    let mut charcount = "c";
    counts.insert(linecount, 0);
    counts.insert(wordcount, 0);
    counts.insert(charcount, 0);
    for line in f.lines() {
        let count = counts.entry(linecount).or_insert(0);
        *count += 1;
        for word in line?.split_whitespace() {
            let count = counts.entry(wordcount).or_insert(0);
            *count += 1;
            for letter in word.chars() {
                let count = counts.entry(charcount).or_insert(0);
                *count += 1;
            }
        }
        if target == "w"{
            println!("words");
            //println!("{:?}", counts);
        }if target == "l"{
            println!("lines");
            //println!("{}", counts.get(&lines));
        }if target == "c"{
            println!("chars");
            //println!("{}", counts.get(&chars));
        }else{
            println!("All");
            //println!("{}", counts.get(&lines));
            //println!("{}", counts.get(&words));
            //println!("{}", counts.get(&chars));
        }
    }

    Ok(())
}

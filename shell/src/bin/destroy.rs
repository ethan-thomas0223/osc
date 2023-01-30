//use std::io;
use std::fs; 



fn main() -> std::io::Result<()> {
    // Delete every file in the list of command-line arguments

    for arg in std::env::args().skip(1) {
        println!("{arg}");
        fs::remove_file(arg)?; 
    }
    Ok(())
}


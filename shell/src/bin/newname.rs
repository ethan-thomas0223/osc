use std::fs; 
use std::io; 

fn main() -> std::io::Result<()>{
    //This program expects two command-line arguments. It will give a “usage” message if it does not receive them. 
    //It will change the name of the file given by the first argument to be the name given by the second argument

    //this accomplishes the renaming portion, needs usage message :)
    let mut curname = "".to_string(); 
    let mut newname = "".to_string();
    let mut counter = 0; 

    if std::env::args().skip(1).count() != 2 {
        println!("Usage: newname curFilepath newFilepath")
    }else{
        for arg in std::env::args().skip(1){
            
            counter += 1;
            if counter == 1{
                curname = arg;
                println!("{}", curname);
            }
            else {
                newname = arg;
                println!("{}", newname);
                
            }  
        
        }
    }
    fs::rename(curname, newname)?; // Rename a.txt to b.txt using filepaths as input

    //println!("{}", &newname);
    Ok(())
}
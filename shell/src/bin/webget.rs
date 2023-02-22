use openssl::ssl::{SslConnector, SslMethod};
use std::io;
use std::ffi::CString;
use std::env;
use std::io::Write; 
use std::io::BufReader; 
use std::io::BufRead;
use std::net::TcpStream; 

//Usage: webget url

fn main() {
    let mut url: String = std::env::args().last().unwrap();
    //got all my basic variables; Don't know what's next

    //parse to get message parts; skip over the https:// in url
    let mut parts = url.split("/").collect();
    let mut host: String = parts[3].to_string();
    let mut req = "".to_string();
    for entry in parts.iter() {
        req += "/{entry}";
    }
    send_message(host, 443, req); 

    //execvp(&cstring_cmd[0], &cstring_cmd);
}


fn send_message(host: &str, port: usize, message: &str) -> io::Result<()> {
    let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
    let connector = SslConnector::builder(SslMethod::tls())?.build();
    let mut stream = connector.connect(host, tcp).unwrap();
    stream.write(message.as_bytes())?;
    //create buff reader on the stream 
    let buf = BufReader::new(stream);
    let newmsg = "".to_string();
    //iterate through buf reader using lines
    //add to sequence of string the we are getting back (print to clarify)
    //break up string to get header, shave it off, save the rest to local file
    for line in buf.lines(){
        newmsg.push_str("{line} \n");
    }
    println!("{}", newmsg); 

    Ok(())
}
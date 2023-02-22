use openssl::ssl::{SslConnector, SslMethod};
use std::io;
use std::ffi::CString;
use std::env;

//Usage: webget url

fn main() {
    let mut args Vec<&str> = std::env::args().skip(1).split_whitespace().collect();
    let mut url &str = args.last();
    let mut cmd String = std::env::args().skip(1).to_string();
    let cstring_cmd Vec<CString> = externalize(&cmd);
    //got all my basic variables; Don't know what's next

    //parse to get message parts; skip over the https:// in url
    let mut parts Vec<String> = url.skip(7).split("/");
    let mut host String = parts[0].to_string();
    let mut req = "";
    for entry in parts.skip(1){
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
    let newmsg = "";
    //iterate through buf reader using lines
    //add to sequence of string the we are getting back (print to clarify)
    //break up string to get header, shave it off, save the rest to local file
    for line in buf.lines(){
        newmsg.push_str("{line} \n");
    }
    println!("{}", newmsg); 

    Ok(())
}

fn externalize(command: &str) -> Vec<CString> {
    command.split_whitespace()
        .map(|s| CString::new(s).unwrap())
        .collect()
}
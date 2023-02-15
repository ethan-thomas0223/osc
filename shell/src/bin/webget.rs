use openssl::ssl::{SslConnector, SslMethod};
use std::io;
use std::ffi::CString;
use std::env;

//Usage: webget url

fn main() {
    let mut args Vec<String> = std::env::args().skip(1).split_whitespace().collect();
    let mut url String = args.last();
    let mut cmd <String> = std::env::args().skip(1).to_string();
    let cstring_cmd Vec<CString> = externalize(&cmd);
    execvp(&cstring_cmd[0], &cstring_cmd);
}



fn send_message(host: &str, port: usize, message: &str) -> io::Result<()> {
    let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
    let connector = SslConnector::builder(SslMethod::tls())?.build();
    let mut stream = connector.connect(host, tcp).unwrap();
    stream.write(message.as_bytes())?;
    Ok(())
}

fn externalize(command: &str) -> Vec<CString> {
    command.split_whitespace()
        .map(|s| CString::new(s).unwrap())
        .collect()
}
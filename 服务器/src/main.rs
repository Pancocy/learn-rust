use std ::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

fn main() {
    let stream = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in stream.incoming(){
        let mut stream = stream.unwrap();
        //println!("connection established!!!!");
        handle_request(&mut stream);
    }
}
fn handle_request(stream:&mut TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";


    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    check_request_status(stream,status_line,file_name);
}
fn check_request_status(stream:&mut TcpStream,status_line:&str,file_name:&str){
    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    stream.flush().unwrap();
}
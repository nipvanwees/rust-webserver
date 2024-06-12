// listen to tcp streams
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // bind the listener to a port
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Invalid port");

    // an iterator of the receivedd connections on the bound ip
    // connection attempts
    for stream in listener.incoming() {
        println!("Connection attempted");
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// stream mutable because internal state might change when more data requested
fn handle_connection(mut stream: TcpStream) {
    // a buffer on the stack which is big enough to hold data of the basic request
    // so yes, the request size is limited to 1024 bytes on the stack: heap and arbitrary size
    // would complicate a lot more
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request_o = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", &request_o);

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        // if main index resource is retrieved
        send_response(stream, String::from("index.html"));
    } else {
        send_404(stream, String::from("404.html"));
    }

    // let resouce = get_requested_resource(request_o);
    // read the stream to the buffer: store the stream over tcp on the stack in memory
    // print the stream as a string, transformed from the entire buffer
    // from_utf8_lossy checks if converted to valid utf8
}

// fn get_requested_resource(request: &String) -> String {
// return String::from("/");
// }
fn send_404(mut stream: TcpStream, resource: String) {
    let contents = fs::read_to_string(resource).expect("could not find resource");
    let response = format!(
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn send_response(mut stream: TcpStream, resource: String) {
    let contents = fs::read_to_string(resource).expect("could not open file");
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nServer: RustyNips/1.1.2 (Nips)\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream
        .write(response.as_bytes())
        .expect("response send succesfully");
    stream.flush().unwrap();
}

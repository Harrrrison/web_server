use std::{
    fs,
    io::{prelude::*, BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();
    //    println!("Request: {:#?}", http_request);
    let (status_line, filename) = if req_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "main.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "error.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\n Contents-length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
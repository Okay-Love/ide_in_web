use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

enum ReqType {
    GET,
    POST,
    UNKNOWN,
}

struct ReqInfo<'a> {
    req_type: ReqType,
    req_path: &'a str,
}

fn split_req_info(info: &String) -> ReqInfo {
    let req = info.split_whitespace().collect::<Vec<_>>();

    let req_info = ReqInfo {
        req_type: if req[0] == "GET" {
            ReqType::GET
        } else if req[0] == "POST" {
            ReqType::POST
        } else {
            ReqType::UNKNOWN
        },
        req_path: req[1],
    };
    req_info
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let string_stream = String::from_utf8_lossy(&buffer[..]);

    println!("Request: {}", &string_stream);

    let string_stream = String::from_utf8(buffer[..].to_vec()).unwrap();

    let mut file = File::open("index.html").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    let req = split_req_info(&string_stream);
    println!("{}", &req.req_path);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const MAX_THREAD_COUNT: usize = 20;
const THREAD_SLEEP_TIME_MS: u64 = 1000;

enum ReqType {
    GET,
    POST,
    UNKNOWN,
}

struct ReqInfo<'a> {
    req_type: ReqType,
    req_raw_path: &'a str,
    req_path: Vec<&'a str>,
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
        req_raw_path: req[1],
        req_path: req[1].split("/").collect::<Vec<_>>(),
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
    println!("{}", &req.req_raw_path);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let html_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    let api_listener = TcpListener::bind("127.0.0.1:3001").unwrap();

    let thread_count = Arc::new(Mutex::new(0));

    let html_arc_count = Arc::clone(&thread_count);

    thread::spawn(move || {
        for stream in html_listener.incoming() {
            let stream = stream.unwrap();

            loop {
                if *html_arc_count.lock().unwrap() >= MAX_THREAD_COUNT {
                    thread::sleep(Duration::from_millis(THREAD_SLEEP_TIME_MS));
                } else {
                    *html_arc_count.lock().unwrap() += 1;
                    break;
                }
            }

            thread::spawn(|| handle_connection(stream));
        }
    });

    let api_arc_count = Arc::clone(&thread_count);

    for stream in api_listener.incoming() {
        let stream = stream.unwrap();

        loop {
            if *api_arc_count.lock().unwrap() >= MAX_THREAD_COUNT {
                thread::sleep(Duration::from_millis(THREAD_SLEEP_TIME_MS));
            } else {
                *api_arc_count.lock().unwrap() += 1;
                break;
            }
        }
        thread::spawn(|| handle_connection(stream));
    }
}

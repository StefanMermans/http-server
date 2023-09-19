use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpStream};

pub struct StatusLine {
    method: String,
    path: String,
    protocol: String,
}

#[derive(Debug)]
pub struct Request {
    pub protocol: String,
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub content: Option<Vec<u8>>,
    pub content_length: usize,
}

impl Request {
    pub fn new() -> Self {
        Self {
            method: String::new(),
            path: "".to_string(),
            protocol: "".to_string(),
            headers: HashMap::new(),
            content: None,
            content_length: 0,
        }
    }
}

pub fn parse_http(mut stream: &TcpStream) -> Request {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut index: usize = 0;
    let mut request = Request::new();

    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).expect("Failed to read line!");

        line = line.strip_suffix("\r\n")
            .or(line.strip_suffix("\n"))
            .unwrap().to_string();

        if index == 0 {
            let status_line = parse_status_line(&line);
            request.method = status_line.method;
            request.path = status_line.path;
            request.protocol = status_line.protocol;
            index += 1;
            continue;
        }

        if line.is_empty() {
            if let Some(len_str) = request.headers.get("Content-Length") {
                request.content_length = len_str.parse::<usize>().expect("Failed to parse Content-Length header!");
                request.content = Some(read_content(&mut buf_reader, request.content_length));
            }

            break;
        }

        let (key, value) = parse_line(&line).expect(format!("Failed to parse header {}", &line).as_str());
        request.headers.insert(key, value);

        index += 1;
    }

    request
}

fn parse_status_line(status_line: &String) -> StatusLine {
    let split: Vec<&str> = status_line.split(' ').collect();

    if split.len() != 3 {
        panic!("Status line is invalid \"{}\"", &status_line);
    }

    StatusLine {
        method: split[0].to_string(),
        path: split[1].to_string(),
        protocol: split[2].to_string(),
    }
}

fn read_content(buf_reader: &mut BufReader<&mut &TcpStream>, content_length: usize) -> Vec<u8> {
    let mut content_bytes = vec![0u8; content_length];
    buf_reader.read(&mut content_bytes).expect("Failed to read content");

    content_bytes
}

fn parse_line(line: &String) -> Result<(String, String), ()> {
    let index = match line.find(':') {
        Some(index) => index,
        None => return Err(()),
    };

    Ok((
        String::from(&line[0..index]),
        String::from(&line[index+2..])
    ))
}

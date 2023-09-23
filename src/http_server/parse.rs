use std::io::{BufRead, BufReader, Read};
use std::net::{TcpStream};
use crate::http_server::request::Request;

pub fn parse_http_stream(mut stream: &TcpStream) -> Request {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut index: usize = 0;
    let mut request = Request::new();

    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).expect("Failed to read line!");
        line = line.trim_end().to_string();

        if index == 0 {
            parse_status_line(&line, &mut request);
            index += 1;
            continue;
        }

        if line.is_empty() {
            read_content(&mut buf_reader, &mut request);
            break;
        }

        parse_header(&line, &mut request);

        index += 1;
    }

    request
}

fn parse_status_line(status_line: &String, request: &mut Request) {
    let split: Vec<&str> = status_line.split(' ').collect();

    if split.len() != 3 {
        panic!("Status line is invalid \"{}\"", &status_line);
    }

    request.method = split[0].to_string();
    request.path = split[1].to_string();
    request.protocol = split[2].to_string();
}

fn read_content(buf_reader: &mut BufReader<&mut &TcpStream>, request: &mut Request) {
    if let Some(len_str) = request.headers.get("Content-Length") {
        request.content_length = len_str.parse::<usize>().expect("Failed to parse Content-Length header!");

        let mut content_bytes = vec![0u8; request.content_length];
        buf_reader.read(&mut content_bytes).expect("Failed to read content");

        request.content = Some(content_bytes);
    }
}

fn parse_header(header_line: &String, request: &mut Request) {
    let index = match header_line.find(':') {
        Some(index) => index,
        None => return,
    };

    request.headers.insert(
        String::from(&header_line[0..index]),
        String::from(&header_line[index+2..])
    );
}

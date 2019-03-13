use std::net::*;
use std::fmt;
use std::io::Read;
use std::io::Write;
use std::thread;
include!("http_resource.rs");
include!("octave_pipe.rs");

#[derive(Debug)]
struct Buffer(Vec<u8>);

#[derive(Debug)]
enum HttpRequestType {
    Get,
    Post,
    Connect,
    Delete,
}

#[derive(Debug)]
struct HttpRequest {
    method: HttpRequestType,
    version: Buffer,
    resource: Buffer,
    headers: std::collections::HashMap<String, String>,
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Http{:?}Request {}: {{ \n", self.method, String::from_utf8(self.version.0.clone()).unwrap());
        write!(f, "\tresource: {}\n", String::from_utf8(self.resource.0.clone()).unwrap());
        for entry in &self.headers {
            write!(f, "\t{}: {}\n", entry.0, entry.1);
        }
        write!(f, "}} \n")
    }
}

fn read_http_full(mut stream: &TcpStream) -> Result<Buffer, std::io::Error> {
    let mut buffer = Buffer(Vec::new());
    let mut temp: [u8; 1] = [0u8];
    while !buffer.0.ends_with(&[13u8, 10u8, 13u8, 10u8]) {
        stream.read_exact(&mut temp)?;
        buffer.0.push(temp[0]);
    }
    Ok(buffer)
}

fn parse_header(mut vec: Vec<&str>) -> HttpRequest {
    use HttpRequestType::*;
    let req: Vec<&str> = vec[0].split(' ').collect();
    let m = match req[0] {
        "POST" => Post,
        "CONNECT" => Connect,
        "DELETE" => Delete,
        _ => Get,
    };
    let mut attrs: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for line in vec.drain(1..) {
        match line.find(':') {
            None => continue,
            Some(i) => {
                let (key, value) = line.split_at(i + 1);
                attrs.insert(key.trim_right_matches(":").to_owned(), value.trim().to_owned());
            }
        };
    };
    HttpRequest {
        method: m,
        version: Buffer(Vec::from(req[2])),
        resource: Buffer(Vec::from(req[1])),
        headers: attrs,
    }
}

fn handle(stream: TcpStream) -> Result<(), std::string::FromUtf8Error> {
    let mut stream = stream;
    let buffer = read_http_full(&stream);
    let addr = stream.peer_addr().expect("unknown ip address");
    if buffer.is_ok() {
        let header = String::from_utf8(buffer.unwrap().0)?;
        let headers: Vec<&str> = header.split("\r\n").collect();
        let request = parse_header(headers);
        println!("request specific from {:?}:\n {}", addr, request);
        let resource = String::from_utf8(request.resource.0.clone()).unwrap();
        println!("request resource : {}", resource);
        match(resource.as_str()) {
            "/favicon.ico" => {
                stream.write("HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n".as_bytes());
            },
            "/code.sub" => {
                handleSubmit(stream, request);
            },
            _ => {
                stream.write("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n".as_bytes());
                stream.write(grap_primary_page().as_bytes());
            }
        }
    } else {
        eprintln!("server error occurred - connection close {:?}", addr);
    }
    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080");
    if listener.is_err() {
	eprintln!("error bind");
        return;
    }
    for stream in listener.unwrap().incoming() {
        if stream.is_ok() {
            let _new_thread = thread::spawn(move || {
                handle(stream.unwrap());
            });
        }
    }
}

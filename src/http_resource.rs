use std::fs::File;

fn grap_primary_page() -> String {
    let mut f = File::open("./index.html").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents;
}

fn read_exact_bytes(mut stream: &TcpStream, length: u32) -> Result<Buffer, std::io::Error>  {
    let mut buffer = Buffer(Vec::new());
    let mut temp: [u8; 1] = [0u8];
    for _ in 0..length {
        stream.read_exact(&mut temp)?;
        buffer.0.push(temp[0]);
    }
    Ok(buffer)
}

fn handle_submit(stream: TcpStream, request: HttpRequest)  -> Result<(), std::string::FromUtf8Error>  {
    match request.method {
        HttpRequestType::Post => {
            println!("handling");
            let mut stream = stream;
            let length : u32 = request.headers["Content-Length"].parse().unwrap();
            let buffer = read_exact_bytes(&stream, length).expect("unable to read code");
            let content = String::from_utf8(buffer.0)?;
            let (out, err) = submit_octave(content);
            let send = out + err.as_str();
            stream.write("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n".as_bytes()).expect("http write failed");
            stream.write(format!("Content-Length: {}\r\n\r\n", send.len()).as_bytes()).expect("http write failed");
            stream.write(send.as_bytes()).expect("http write failed");
        },
        _ => {
            eprintln!("unrecognized submit type");
        }
    }
    Ok(())
}

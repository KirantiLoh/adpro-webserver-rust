use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf = BufReader::new(&mut stream);

    let http_req: Vec<_> = buf
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status = "HTTP/1.1 200 OK\r\n\r\n";
    let content = fs::read_to_string("static/hello.html").unwrap();
    let length = content.len();
    println!("Request: {:#?}", http_req);

    stream
        .write_all(format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}").as_bytes())
        .unwrap();
}

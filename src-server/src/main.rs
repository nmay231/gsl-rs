use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use itertools::Itertools;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4040").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream).ok();
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), ()> {
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let request_line = lines.next().ok_or(())?.or(Err(()))?;

    let (status_line, contents) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello world!".to_owned()),
        "GET /test HTTP/1.1" => (
            "HTTP/1.1 200 OK",
            "<div> dynamic content from server!!! </div>".to_owned(),
        ),
        "POST /test HTTP/1.1" => {
            let list = lines
                .map(|line| format!("<li>{}</li>", line.unwrap()))
                .join("");
            ("HTTP/1.1 200 OK", format!("<ul>{}</ul>", list))
        }
        first_line => (
            "HTTP/1.1 404 NOT FOUND",
            format!("uh oh... first_line=`{}`", first_line),
        ),
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}

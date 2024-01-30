use askama::Template;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    str::FromStr,
};

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

fn templates_dir() -> PathBuf {
    PathBuf::from_str(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .unwrap()
        .join("templates")
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3030").unwrap();

    let hello = HelloTemplate { name: "world" };
    println!("{}", hello.render().unwrap());
    fs::read_to_string(templates_dir().join("hello.html")).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream).ok();
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), ()> {
    let buf_reader = BufReader::new(&mut stream);
    // if buf_reader.lines().next().is_none() {
    //     stream.write_all("")
    // }
    let request_line = buf_reader.lines().next().ok_or(())?.or(Err(()))?;

    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", templates_dir().join("hello.html")),
        _ => ("HTTP/1.1 404 NOT FOUND", templates_dir().join("index.html")),
    };

    // let contents = fs::read_to_string(filename).unwrap();
    let contents = filename.to_str().unwrap();
    // let contents = "testing";
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}

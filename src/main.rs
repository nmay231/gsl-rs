// use anyhow::{Context, Result};
use askama::Template; // bring trait in scope
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct HelloTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();

    let hello = HelloTemplate { name: "world" }; // instantiate your struct
    println!("{}", hello.render().unwrap()); // then render it.
    fs::read_to_string("./templates/hello.html").unwrap();
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
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./templates/hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "./templates/index.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}

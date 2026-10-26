use std::io::Write;
use std::net::TcpStream;
use std::{io::Error, net::TcpListener};

use crate::{header::header, parse_buf::parse_buf, routes::route};

fn handle_connection(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut request = parse_buf(stream)?;
    let (method, url) = header(&mut request)?;
    let http_response = match (method, url) {
        (Some(x), Some(y)) => route(x, y),
        _ => {
            return Err(Error::other("Invalid Request"));
        }
    };
    let response = http_response.to_http();
    stream.write_all(response.as_bytes())?;
    Ok(())
}

pub fn server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server is running on 127.0.0.1:8080");

    for stream in listener.incoming() {
        println!("New request: {:?}", stream);

        match stream {
            Ok(mut stream) => match handle_connection(&mut stream) {
                Ok(_) => (),
                Err(e) => println!("{:?}", e),
            },
            Err(e) => {
                return Err(Error::other(e));
            }
        }
    }

    Ok(())
}

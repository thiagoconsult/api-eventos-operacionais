use std::io::Write;
use std::{io::Error, net::TcpListener};

use crate::{header::header, parse_buf::parse_buf, routes::route};

fn handle_connection(listener: &TcpListener) -> std::io::Result<()> {
    for stream in listener.incoming() {
        println!("New request: {:?}", stream);

        match stream {
            Ok(mut stream) => {
                let mut request = parse_buf(&mut stream)?;
                let (method, url) = header(&mut request)?;
                let http_response = match (method, url) {
                    (Some(x), Some(y)) => route(x, y),
                    _ => {
                        return Err(Error::other("Invalid Request"));
                    }
                };

                let response = http_response.to_http();

                stream.write_all(response.as_bytes())?;
            }
            Err(e) => {
                return Err(Error::other(e));
            }
        }
    }

    Ok(())
}

pub fn server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server is running on 127.0.0.1:8080");

    loop {
        match handle_connection(&listener) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

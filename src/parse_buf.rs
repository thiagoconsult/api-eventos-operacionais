use std::{
    io::{Error, Read},
    net::TcpStream,
};

pub fn parse_buf(stream: &mut TcpStream) -> std::io::Result<Vec<u8>> {
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut request: Vec<u8> = Vec::new();

    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    return Err(Error::other("0 KB"));
                }

                if request.len() + n > 8_192 {
                    return Err(Error::other("Maior que o permitido. Limite: 8KB"));
                }
                request.extend_from_slice(&buffer[..n]);

                for window in request.windows(4) {
                    if window == "\r\n\r\n".as_bytes() {
                        return Ok(request);
                    }
                }
            }
            Err(e) => return Err(Error::other(e)),
        }
    }
}

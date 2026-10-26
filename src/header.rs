use std::{io::Error, str::from_utf8};

pub fn header(request: &mut [u8]) -> std::io::Result<(Option<&str>, Option<&str>)> {
    let request_utf8 = match from_utf8(request) {
        Ok(req) => req,
        Err(e) => return Err(Error::other(e)),
    };

    let first_line = match request_utf8.lines().next() {
        Some(line) => line,
        None => return Err(Error::other("Falha ao ler request")),
    };

    let mut first_line_split = first_line.split_whitespace();

    let method = first_line_split.next();
    let url = first_line_split.next();
    let version_http = first_line_split.next();
    let garbage = first_line_split.next();

    if version_http != Some("HTTP/1.1") {
        return Err(Error::other("Versão HTTP Inválida"));
    }

    if garbage.is_some() {
        return Err(Error::other("Request Inválida"));
    }

    Ok((method, url))
}

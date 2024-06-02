use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::{routes, ROUTES};

pub type Route = (Method, &'static str, fn() -> Response<'static>);

pub struct Response<'a> {
    status: &'a str,
    content_type: &'a str,
    content: &'a [u8],
}

impl<'a> Response<'a> {
    pub const fn new(status: &'a str, content_type: &'a str, content: &'a [u8]) -> Response<'a> {
        Response {
            status,
            content_type,
            content,
        }
    }

    pub fn send(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        let response = format!(
            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            self.status,
            self.content_type,
            self.content.len()
        );

        stream.write_all(response.as_bytes())?;
        stream.write_all(self.content)?;
        stream.flush()?;
        Ok(())
    }
}

pub struct Request<'a> {
    method: Method,
    path: &'a str,
}

#[derive(PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Trace,
    Connect,
    Patch,
}

fn parse_request(buffer: &[u8]) -> Result<Request, &'static str> {
    let mut parts = buffer.splitn(3, |&x| x == b' ');

    let method = match parts.next() {
        Some(b"GET") => Method::Get,
        Some(b"POST") => Method::Post,
        Some(b"PUT") => Method::Put,
        Some(b"DELETE") => Method::Delete,
        Some(b"HEAD") => Method::Head,
        Some(b"OPTIONS") => Method::Options,
        Some(b"TRACE") => Method::Trace,
        Some(b"CONNECT") => Method::Connect,
        Some(b"PATCH") => Method::Patch,
        _ => return Err("Unsupported HTTP method"),
    };

    let path = std::str::from_utf8(parts.next().ok_or("Failed to parse path")?)
        .map_err(|_| "Failed to parse path")?;

    if !parts
        .next()
        .map_or(false, |part| part.starts_with(b"HTTP/1.1\r\n"))
    {
        return Err("Invalid HTTP version");
    }

    Ok(Request { method, path })
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if stream.read(&mut buffer).is_err() {
        eprintln!("Failed to read from stream");
        return;
    }

    let Ok(request) = parse_request(&buffer) else {
        let response = routes::not_found();
        if let Err(e) = response.send(&mut stream) {
            eprintln!("Failed to send response: {}", e);
        }
        return;
    };

    let response = ROUTES
        .iter()
        .find(|(method, path, _)| *method == request.method && *path == request.path)
        .map_or_else(routes::not_found, |(_, _, handler)| handler());

    if let Err(e) = response.send(&mut stream) {
        eprintln!("Failed to send response: {}", e);
    }
}

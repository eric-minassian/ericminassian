mod config;

use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    path::PathBuf,
};

use ericminassian::{
    file_cache::{load_file, FileCache},
    thread_pool::ThreadPool,
};

fn main() {
    dotenv::dotenv().ok();

    let config = config::Config::new();
    let cache = FileCache::default();

    println!("Listening on {}:{}", config.host, config.port);

    let listener = TcpListener::bind((config.host, config.port)).expect("Failed to bind to port");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to establish connection");

        let cache = cache.clone();
        pool.execute(move || {
            handle_connection(stream, &cache);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, cache: &FileCache) {
    let mut buffer = [0; 1024];
    #[allow(clippy::unwrap_used)]
    let _ = stream.read(&mut buffer).unwrap();

    let index = b"GET / HTTP/1.1\r\n";
    let resume = b"GET /resume HTTP/1.1\r\n";

    let (status_line, contents, content_type) = if buffer.starts_with(index) {
        (
            "HTTP/1.1 200 OK",
            load_file(cache, &PathBuf::from("templates/index.html")).expect("Failed to load index"),
            "text/html",
        )
    } else if buffer.starts_with(resume) {
        (
            "HTTP/1.1 200 OK",
            load_file(cache, &PathBuf::from("assets/Eric_Minassian_resume.pdf"))
                .expect("Failed to load resume"),
            "application/pdf",
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND",
            load_file(cache, &PathBuf::from("templates/404.html")).expect("Failed to load 404"),
            "text/html",
        )
    };

    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        status_line,
        content_type,
        contents.len()
    );

    stream
        .write_all(response.as_bytes())
        .expect("Failed to write response");
    stream
        .write_all(&contents)
        .expect("Failed to write response contents");
    stream.flush().expect("Failed to flush stream");
}

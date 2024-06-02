mod config;
mod routes;
mod server;

use std::net::TcpListener;

use ericminassian::thread_pool::ThreadPool;
use server::{handle_connection, Method, Response, Route};

const ROUTES: [Route; 2] = [
    (Method::Get, "/", routes::index::index),
    (Method::Get, "/resume", routes::resume::resume),
];

fn main() {
    dotenv::dotenv().ok();

    let config = config::Config::new();

    println!("Listening on {}:{}", config.host, config.port);

    let listener = TcpListener::bind((config.host, config.port)).expect("Failed to bind to port");
    let pool = ThreadPool::new(20);

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to establish connection");

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

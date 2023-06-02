use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

use super::read_cfg::read_cfg;

use threadpool::ThreadPool;

/// Starts the server with multiple threads.
pub fn start_server() {
    let config = read_cfg();
    println!(
        "Server running on: {}:{}",
        config.server.ip, config.server.port
    );

    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.server.ip, config.server.port
    ))
    .unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down...");
}

/// Handles the incoming request and returns the requested JSON content.
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut response: String;
    let mut count = 1;

    let config = read_cfg();

    stream.read(&mut buffer).unwrap();

    for route in config.routes.iter() {
        let contents: String;
        let bufstr = String::from(std::str::from_utf8(&buffer).unwrap());

        let path = format!("GET {} HTTP/1.1\r\n", route.path);

        let (status_line, filename) = ("HTTP/1.1 200 OK", format!("{}/{}", config.server.base_dir, route.file));
        contents = fs::read_to_string(filename).unwrap();

        // Configure response headers for successful request
        response = format!(
            "{}\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        // Return the JSON data for the requested path
        if bufstr.contains(&path) {
            stream.write_all(response.as_bytes()).unwrap();
        }
        // Handle 404 errors
        else if count == config.routes.iter().len() {
            let content404 = "{\"error\": 404}";
            let error_line = "HTTP/1.1 404 NOT FOUND";

            // Configure response headers for 404 error
            response = format!(
                "{}\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}",
                error_line,
                content404.len(),
                content404
            );

            // Return the 404 JSON data
            stream.write_all(response.as_bytes()).unwrap();
        }
        count += 1;
    }
    stream.flush().unwrap();
}

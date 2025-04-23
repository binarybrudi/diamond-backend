use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8088")?;
    println!("diamond listening on http://0.0.0.0:8088");

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Ok(n) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..n]);
        if request.starts_with("GET /hello ") {
            let response = "hello".to_string();
            let http_response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                response.len(),
                response
            );
            stream.write_all(http_response.as_bytes()).unwrap();
        } else {
            let response = "HTTP/1.1 404 Not Found\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}


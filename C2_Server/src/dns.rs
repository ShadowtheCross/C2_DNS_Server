use std::net::{TcpListener, TcpStream};
use std::io::{self, Read};
use std::thread;


fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0u8; 512];

    let bytes_read = stream.read(&mut buffer)?;

    let DNS_Request = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("DNS REQUEST Received: {}", request);

    Ok(())
}

pub fn dns_communication(ip: &str, port: u16) -> io::Result<()> {
    let listener = TcpListener::bind((ip, port))?;

    println!("DNS server listening on {}:{}", ip, port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("Error Processing Data: {}", e);
                    }
                    
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
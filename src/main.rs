mod api;
mod model;

use std::io::{self, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    // Specify the host and port you want to connect to
    let host = "192.168.2.40"; // IP address of the host (localhost in this example)
    let port = 30002; // Port on which the server is listening

    // Try to connect to the server via TCP
    match TcpStream::connect((host, port)) {
        Ok(mut stream) => {
            println!("Successfully connected to server at {}:{}", host, port);

            // Data to be sent
            let data = "set_digital_out(3, true)";

            // Send data over the stream
            stream.write_all(data.as_bytes())?;
            println!("Data sent: {}", data);

            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            Err(e)
        }
    }
}

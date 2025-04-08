use std::{io::Write, net::TcpStream};

pub fn start_tcp() {
    let mut _stream = TcpStream::connect("192.168.2.40:30002");
    //TcpStream::write(&mut _stream, "movep(p[0.12, 0.12, 0.12, 0.12, 0.12, 0.12])");
}

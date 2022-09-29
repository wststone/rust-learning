use std::net::TcpStream;

fn main() {
    TcpStream::connect("localhost:3000").unwrap();
    println!("Hello, world!");
}

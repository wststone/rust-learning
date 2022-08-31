use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("localhost:3000").unwrap();
    println!("Hello, world!");
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("connection established!")
    }
}

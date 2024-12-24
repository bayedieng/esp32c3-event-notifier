use std::io::Read;
use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("192.168.101.200:8000").unwrap();
    let mut stream_buf = [0; 1024];
    println!("Listening...");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        stream.read(&mut stream_buf).unwrap();
        let stream_string = String::from_utf8(stream_buf.to_vec()).unwrap();
        println!("{stream_string}")
    }
    println!("Hello, world!");
}

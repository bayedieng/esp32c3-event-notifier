use std::io::Read;
use std::net::TcpListener;
use std::thread;
fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    let mut stream_buf = [0; 1024];
    println!("Listening...");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let n_bytes = stream.read(&mut stream_buf).unwrap();
        println!("Client Connected: {}", stream.peer_addr().unwrap());
        println!(
            "Message From Client: {:?}",
            String::from_utf8(stream_buf[..n_bytes].to_vec()).unwrap()
        );
    }
}

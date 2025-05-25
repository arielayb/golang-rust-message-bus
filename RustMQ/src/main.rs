use inquire::{Confirm, Select, Text};
use rustmq::DataPackage;
use std::collections::VecDeque;
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::{string, thread};
use uuid::Uuid;

mod rustmq;

#[derive(Clone, Copy, Debug)]
pub struct InitQueue;

impl InitQueue {
    fn init_queue(self, stream: TcpStream, addr: SocketAddr) {
        let mut buffer = vec![0; 1024];
        if let Ok(mut stream) = TcpStream::connect(addr) {
            println!("RustMQ Stream created, {:?}", addr);
            let data_pkg = rustmq::DataPackage {
                msg: String::from(""),
                uuid: Uuid::new_v4().to_string(),
                msg_queue: VecDeque::new(),
            };
            //TODO: start setting up the queue creation and messages.
            loop {
                let bytes_read = stream.read(&mut buffer);
                if bytes_read > 0 {
                    let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("Received: {}", received_data); // Print the received data
                }
            }
        } else {
            println!("Couldn't create RustMQ Tcp Stream!")
        }

        // socket.close(None);
    }
}

fn main() {
    // Example using a text prompt
    let rustmq_url: String = Text::new("Enter RustMQ URL?")
        .with_default("127.0.0.1:8080")
        .prompt()
        .unwrap();
    println!("RustMq URL: {}", &rustmq_url);

    let queue: InitQueue = InitQueue {};
    let addr: SocketAddr = rustmq_url
        .parse()
        .expect("Cannot parse the url string! Did you include a port number?");
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || {
                // init queue/tcp connection
                queue.init_queue(stream, addr);
            })
            .join()
            .expect("recv msg thread completed.")
        }
    }
}

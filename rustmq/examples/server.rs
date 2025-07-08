use inquire::{Confirm, Select, Text};
use rustmq::rustmq_interface::DataPackage;
use std::collections::VecDeque;
use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use rustmq::tklog::{info, error};
use rustmq::{rustmq_interface, Uuid};
use rustmq::prelude::{tcp_stream_writer::TcpStreamWriter};

#[derive(Clone, Copy, Debug)]
pub struct InitQueue;

impl InitQueue {
    fn init_queue(self, stream: TcpStream, addr: SocketAddr) {
       //
        let dpkg = DataPackage;
    }
}

fn main() {
    // Example using a text prompt
    let rustmq_url: String = Text::new("Enter RustMQ URL?")
        .with_default("127.0.0.1:8080")
        .prompt()
        .unwrap();
    info!("RustMq URL: {}", &rustmq_url);

    let queue: InitQueue = InitQueue {};
    let addr: SocketAddr = rustmq_url
        .parse()
        .expect("Cannot parse the url string! Did you include a port number?");
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || {
                // init queue/tcp connection
                let _ = queue.init_queue(stream, addr);
            })
            .join()
            .expect("recv msg thread completed.")
        } else {
            error!("Couldn't create RustMQ Tcp Stream!")
        }
    }
}

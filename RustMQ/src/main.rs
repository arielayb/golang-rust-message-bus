use chrono;
use inquire::{Confirm, Select, Text};
use std::collections::VecDeque;
use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use tcp_stream_writer::TcpStreamWriter;
use uuid::Uuid;

mod rustmq;
mod tcp_stream_writer;

#[derive(Clone, Copy, Debug)]
pub struct InitQueue;

impl InitQueue {
    fn init_queue(self, stream: TcpStream, addr: SocketAddr) -> io::Result<()> {
        println!("RustMQ Stream created, {:?}", addr);
        let mut tcp_stream_writer = TcpStreamWriter::new(stream)?;
        //TODO: start setting up the queue creation and messages.
        let message: String = tcp_stream_writer.read_message()?;
        // let data_pkg = rustmq::DataPackage {
        //     msg: String::from(&message),
        //     uuid: Uuid::new_v4().to_string(),
        //     msg_queue: VecDeque::new(),
        // };
        // tcp_stream_writer.send_message(&message)?;
        println!("the message from client: {:?}", &message);

        Ok(())
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
        } else {
            println!("Couldn't create RustMQ Tcp Stream!")
        }
    }
}

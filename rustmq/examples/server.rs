use inquire::Text;
use rustmq::prelude::tcp_stream_writer::TcpStreamWriter;
use rustmq::rustmq_interface::RustMQ;
use rustmq::tklog::{error, info};
use rustmq::{Uuid, rustmq_interface::DataPackage};
use std::collections::VecDeque;
use std::io::{self, Read};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

#[derive(Clone, Copy, Debug)]
pub struct InitQueue;

/*impl InitQueue {
    fn init_queue(self, stream: TcpStream, addr: SocketAddr) -> io::Result<Vec<u8>> {
        println!("RustMQ Stream created, {:?}", addr);
        let mut tcp_stream_writer = TcpStreamWriter::new(stream)?;
        //TODO: start setting up the queue creation and messages.
        let message: Vec<u8> = tcp_stream_writer.read_message()?;
        let data_pkg = DataPackage {
            msg: message,
            uuid: Uuid::new_v4().to_string(),
            msg_queue: VecDeque::new(),
        };
        // tcp_stream_writer.send_message(&message)?;
        let stream_msg = String::from_utf8_lossy(&data_pkg.msg).into_owned();
        println!("the message from client: {}", stream_msg);

        Ok(data_pkg.msg)
    }
}*/

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
    println!("RustMQ Stream created, {:?}", addr);

    //let mut tcp_stream_writer = TcpStreamWriter::new(stream)?;
    //TODO: start setting up the queue creation and messages.
    //let message: Vec<u8> = tcp_stream_writer.read_message()?;
    /*let mut data_pkg = DataPackage {
        msg: vec![],
        uuid: Uuid::new_v4().to_string(),
        msg_queue: VecDeque::new(),
    };*/

    for stream in listener.incoming() {
        let mut data_pkg = DataPackage {
            msg: Box::new(Vec::new()),
            uuid: Uuid::new_v4().to_string(),
            msg_queue: VecDeque::new(),
        };

        if let Ok(stream) = stream {
            thread::spawn(move || {
                // init queue/tcp connection
                //let _q: Result<Vec<u8>, io::Error> = queue.init_queue(stream, addr);
                let _q: _ = data_pkg.rustmq_init_queue(addr, stream);
            })
            .join()
            .expect("recv msg thread completed.")
        } else {
            error!("Couldn't create RustMQ Tcp Stream!")
        }
    }
}

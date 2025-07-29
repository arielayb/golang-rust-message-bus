use crate::graph_system;
use crate::tcp_stream_writer::TcpStreamWriter;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;
use std::io::Bytes;
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::vec::Vec;
use tklog::{error, info};
pub use uuid::Uuid;

pub trait RustMQ {
    fn rustmq_init_queue(&mut self, addr: SocketAddr, stream: TcpStream) -> io::Result<()>;
    fn rustmq_create_queue(&mut self) -> &VecDeque<Box<Vec<u8>>>;
    fn rustmq_get_queue(&self) -> &VecDeque<Box<Vec<u8>>>;
    fn rustmq_set_queue(&mut self, msg_queue: VecDeque<Box<Vec<u8>>>);
    fn rustmq_stream(&self);
    fn rustmq_write(&mut self, message: Box<Vec<u8>>);
    fn rustmq_read(&self);
}

pub struct DataPackage {
    pub msg: Box<Vec<u8>>,
    pub uuid: String,
    pub msg_queue: VecDeque<Box<Vec<u8>>>,
}

impl RustMQ for DataPackage {
    fn rustmq_init_queue(&mut self, addr: SocketAddr, stream: TcpStream) -> io::Result<()> {
        info!("RustMQ Stream created, ", addr);
        let mut tcp_stream_writer = TcpStreamWriter::new(stream)?;
        //TODO: start setting up the queue creation and messages.
        //let msg = tcp_stream_writer.read_message()?;
        self.uuid = Uuid::new_v4().to_string();
        let msg: Box<Vec<u8>> = Box::new(tcp_stream_writer.read_message().unwrap());
        let msgstr: Vec<u8> = tcp_stream_writer.read_message().unwrap();
        println!("the message from client: {:?}", &msg.to_vec());
        self.msg_queue.push_back(msg);
        println!("the message from client: {:?}", &self.msg_queue[0]);
        info!("message in queue");

        Ok(())
    }

    fn rustmq_create_queue(&mut self) -> &VecDeque<Box<Vec<u8>>> {
        let data: VecDeque<Box<Vec<u8>>> = VecDeque::new();
        self.msg_queue = data;

        return &self.msg_queue;
    }

    fn rustmq_get_queue(&self) -> &VecDeque<Box<Vec<u8>>> {
        return &self.msg_queue;
    }

    fn rustmq_set_queue(&mut self, msg_queue: VecDeque<Box<Vec<u8>>>) {
        self.msg_queue = msg_queue;
    }

    fn rustmq_stream(&self) {}

    fn rustmq_write(&mut self, message: Box<Vec<u8>>) {
        self.msg = message;
    }

    fn rustmq_read(&self) {}
}

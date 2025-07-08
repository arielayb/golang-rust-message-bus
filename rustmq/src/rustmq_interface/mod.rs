use crate::graph_system;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;
use std::io::Bytes;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::vec::Vec;
use tklog::{info, error};
use crate::tcp_stream_writer::TcpStreamWriter;
pub use uuid::Uuid;

trait RustMQ<T> {
    fn rustmq_init_queue(&mut self, addr: SocketAddr, message: Vec<u8>, stream: TcpStream) -> io::Result<()>;
    fn rustmq_create_queue(&mut self) -> &VecDeque<T>;
    fn rustmq_get_queue(&self) -> &VecDeque<T>;
    fn rustmq_set_queue(&mut self, msg_queue: VecDeque<T>);
    fn rustmq_stream(&self);
    fn rustmq_write(&mut self, message: T);
    fn rustmq_read(&self);
}

pub struct DataPackage<T> {
    pub msg: Vec<u8>,
    pub uuid: String,
    pub msg_queue: VecDeque<T>,
}

impl<T> RustMQ<T> for DataPackage<T> {
    fn rustmq_init_queue(&mut self, addr: SocketAddr, mut message: Vec<u8>, stream: TcpStream) -> io::Result<()> {
        info!("RustMQ Stream created, ", addr);
        let mut tcp_stream_writer = TcpStreamWriter::new(stream)?;
        //TODO: start setting up the queue creation and messages.
        message = tcp_stream_writer.read_message()?;
        self.msg = String::from_utf8(message);
        self.uuid = Uuid::new_v4().to_string();
        self.msg_queue = VecDeque::new();

        // tcp_stream_writer.send_message(&message)?;
        println!("the message from client: {:?}", self.msg);
        info!("the message from client: {}", self.msg);

        Ok(())
    }

    fn rustmq_create_queue(&mut self) -> &VecDeque<T> {
        let data: VecDeque<T> = VecDeque::new();
        self.msg_queue = data;

        return &self.msg_queue;
    }

    fn rustmq_get_queue(&self) -> &VecDeque<T> {
        return &self.msg_queue;
    }

    fn rustmq_set_queue(&mut self, msg_queue: VecDeque<T>) {
        self.msg_queue = msg_queue;
    }

    fn rustmq_stream(&self) {}

    fn rustmq_write(&mut self, message: T) {
        self.msg = message;
    }

    fn rustmq_read(&self) {}
}

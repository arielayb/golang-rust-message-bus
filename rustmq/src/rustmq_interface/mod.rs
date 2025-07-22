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

trait RustMQ {
    fn rustmq_init_queue(&mut self, addr: SocketAddr, stream: TcpStream) -> io::Result<()>;
    fn rustmq_create_queue(&mut self) -> &VecDeque<u8>;
    fn rustmq_get_queue(&self) -> &VecDeque<u8>;
    fn rustmq_set_queue(&mut self, msg_queue: VecDeque<u8>);
    fn rustmq_stream(&self);
    fn rustmq_write(&mut self, message: Vec<u8>);
    fn rustmq_read(&self);
}

pub struct DataPackage {
    pub msg: Vec<u8>,
    pub uuid: String,
    pub msg_queue: VecDeque<u8>,
}

impl RustMQ for DataPackage {
    fn rustmq_init_queue(&mut self, addr: SocketAddr, stream: TcpStream) -> io::Result<()> {
        info!("RustMQ Stream created, ", addr);
        let mut tcp_stream_writer = TcpStreamWriter::new(stream)?;
        //TODO: start setting up the queue creation and messages.
        self.msg = tcp_stream_writer.read_message()?;
        self.uuid = Uuid::new_v4().to_string();
        
        self.msg_queue = VecDeque::new();

        // tcp_stream_writer.send_message(&message)?;
        println!("the message from client: {:?}", self.msg);
        info!("message in queue");

        Ok(())
    }

    fn rustmq_create_queue(&mut self) -> &VecDeque<u8> {
        let data: VecDeque<u8> = VecDeque::new();
        self.msg_queue = data;

        return &self.msg_queue;
    }

    fn rustmq_get_queue(&self) -> &VecDeque<u8> {
        return &self.msg_queue;
    }

    fn rustmq_set_queue(&mut self, msg_queue: VecDeque<u8>) {
        self.msg_queue = msg_queue;
    }

    fn rustmq_stream(&self) {}

    fn rustmq_write(&mut self, message: Vec<u8>) {
        self.msg = message;
    }

    fn rustmq_read(&self) {}
}

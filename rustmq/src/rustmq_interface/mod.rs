use crate::graph_system;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::vec::Vec;


trait RustMQ<T> {
    fn rustmq_create_queue(&mut self) -> &VecDeque<T>;
    fn rustmq_get_queue(&self) -> &VecDeque<T>;
    fn rustmq_set_queue(&mut self, msg_queue: VecDeque<T>);
    fn rustmq_stream(&self);
    fn rustmq_write(&mut self, message: T);
    fn rustmq_read(&self);
}

pub struct DataPackage<T> {
    pub msg: T,
    pub uuid: String,
    pub msg_queue: VecDeque<T>,
}

impl<T> RustMQ<T> for DataPackage<T> {
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

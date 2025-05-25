use std::collections::HashMap;
use std::collections::VecDeque;
use std::vec::Vec;

trait RustMQ<T> {
    fn rusty_queue(&self, data_pkt: VecDeque<T>);
    fn rusty_stream(&self);
    fn rusty_write(&self, message: T);
    fn rusty_read(&self);
}

pub struct DataPackage<T> {
    msg: String,
    uuid: String,
    msg_id: u32,
    data_queue: VecDeque<T>,
}

impl<T> RustMQ<T> for DataPackage<T> {
    fn rusty_queue(&self, data_pkt: VecDeque<T>) {}

    fn rusty_stream(&self) {}

    fn rusty_write(&self, message: T) {}

    fn rusty_read(&self) {}
}

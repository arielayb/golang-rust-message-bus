use std::collections::VecDeque;
use std::vec::Vec;
use std::collections::HashMap;

trait RustyMQ {
    fn rustyQueue<T>(self) -> VecDeque<T>;
    fn rustyStream<T>(self) -> Vec<T>;
}
use inquire::{Confirm, Select, Text};
use rustymq::DataPackage;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::{string, thread};

mod rustymq;

#[derive(Clone, Copy, Debug)]
pub struct InitQueue;

impl InitQueue {
    fn init_queue(self, stream: TcpStream, addr: SocketAddr) {
        if let Ok(mut stream) = TcpStream::connect(addr) {
            println!("RustyMQ Stream created, {:?}", addr);
            //TODO: start setting up the queue creation and messages.
            loop {}
        } else {
            println!("Couldn't create RustyMQ Tcp Stream!")
        }

        // socket.close(None);
    }
}

fn main() {
    // Example using a text prompt
    let rustymq_url: String = Text::new("Enter RustyMQ URL?")
        .with_default("127.0.0.1:8080")
        .prompt()
        .unwrap();
    println!("rustyMq URL: {}", &rustymq_url);

    let queue: InitQueue = InitQueue {};
    let addr: SocketAddr = rustymq_url
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
    // Example using a confirmation prompt
    // let confirm = Confirm::new("Are you sure?")
    // .with_default(false)
    // .prompt()
    // .unwrap();
    // println!("Confirmation: {}", confirm);

    // Example using a select prompt
    // let options = vec!["Option 1", "Option 2", "Option 3"];
    // let selected_option = Select::new("Select an option:", options)
    // .prompt()
    // .unwrap();
    // println!("Selected: {}", selected_option);
}

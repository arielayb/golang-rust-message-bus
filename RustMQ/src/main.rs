use inquire::{Confirm, Select, Text};
use std::net::UdpSocket;
use std::thread;

mod rustymq;

fn main() {
    // Example using a text prompt
    let rustymq_url: String = Text::new("Enter RustyMQ URL?")
    .with_default("rtmq://")
    .prompt()
    .unwrap();
    println!("rustyMq URL: {}", rustymq_url);

    thread::spawn(move || {  
        data.recv_msg();
    }).join().expect("recv msg thread completed.")

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

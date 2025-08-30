use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn send_multiple() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();

    // ! sending a message to the rx
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // ! receiving the message from the tx
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // ! send multiple messages 
    send_multiple();
}

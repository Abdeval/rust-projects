use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // ? depending on the place of this it will make
    // ? the first thread finishes than the main thread
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // todo: using resources from main thread to other thread
    let v = vec![1, 2, 3];
    let handle1 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle1.join().unwrap();
}


// mpsc stands for multiple producer, single consumer
// receiver has two methods: recv and try_recv
//   - recv: blocks tha main thread's execution and waits until a value is sent down the channel
//   - try_recv: doesn't block, but will return a Result<T, E> immediately: Ok(value) if a value is available and an Err if there aren't any values

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // println!("val is {val}"); // This will give an error because val is moved to tx.send(val)
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
   
    // let received = rx.recv().unwrap();
    for received in rx { // if there is no need to wait
        println!("Got: {received}");
    }
}
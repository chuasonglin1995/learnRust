use std::thread;
// use std::time::Duration;

// once main thread is done, spawned thread will be shut down


fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // problem is that rust cannot tell how long the spawned thread will run, hence we need 'move'
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}

// fn main() {
//     let handler = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i} from the spawned thread!");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..6 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1));
//     }


//     // wait for all spawned thread to finish
//     handler.join().unwrap();
// }
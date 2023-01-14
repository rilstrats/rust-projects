// use std::thread;
// use std::time::Duration;
use std::collections::HashMap;

fn main() {
    const API_URL: &str = "http://127.0.0.1:8790";
    const FILM: i32 = 6;

    let mut calls = 0;
}

async fn request(url: &str) {}

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     handle.join().unwrap()
// }

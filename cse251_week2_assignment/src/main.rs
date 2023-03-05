use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    const TOP_API_URL: &str = "http://127.0.0.1:8790";
    const FILM: i32 = 6;

    let mut calls = 0;

    // let (tx, rx) = mpsc::channel();

    let top: Top = reqwest::blocking::get(TOP_API_URL)
        .expect("Server isn't running!")
        .json()
        .unwrap();
    calls += 1;
    println!("{:?}", top);

    let film_url = top.films + &FILM.to_string();
    println!("{}", film_url);

    let film: Film = reqwest::blocking::get(film_url)
        .expect("Server isn't running!")
        .json()
        .unwrap();
    calls += 1;
    println!("{:?}", film);

    println!("{}", &film.characters.len());
    let (tx, rx) = mpsc::channel();
    let mut threads: Vec<thread::JoinHandle<()>> = vec![];
    for url in film.characters {
        let tx_clone = tx.clone();
        threads.push(thread::spawn(move || {
            let name: Name = reqwest::blocking::get(url)
                .expect("Server isn't running!")
                .json()
                .unwrap();
            tx_clone.send(name.name).unwrap();
            calls += 1;
        }));
    }

    for thread in threads {
        thread.join().unwrap()
    }

    // println!("{}",rx.)

    let mut count = 0;
    for name in rx {
        println!("{}", name);
        count += 1;
        println!("{}", count);
    }

    // let mut urls_dict: HashMap<String, Vec<String>>;

    // for (key, value) in film {
    // println!("{:?}", key)
    // match value {
    // Value::Array(x) => urls_dict.insert(key, value.unwrap()),
    // _ => continue,
    // };
    //     match film[section] {
    //         // Value::Array(x) => urls_dict[section] = x,
    //         Value::Array(x) => println!("{:?}", x),
    //         _ => continue,
    //     }
    // }
}

// fn request(url: &str) -> Result<> {
// let response = serde_json::from_value(reqwest::blocking::get(url)?.json()?);
// Ok(response)
// }

#[derive(Serialize, Deserialize, Debug)]
struct Top {
    people: String,
    planets: String,
    films: String,
    species: String,
    vehicles: String,
    starships: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Film {
    title: String,
    director: String,
    producer: String,
    release_date: String,
    characters: Vec<String>,
    planets: Vec<String>,
    starships: Vec<String>,
    vehicles: Vec<String>,
    species: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Name {
    name: String,
}

// fn thread_request(url: &str) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
//     thread::spawn(move || {
//         let response = reqwest::blocking::get(url)?.json::<HashMap<String, Value>>()?;
//         Ok(response)
//     })
// }

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

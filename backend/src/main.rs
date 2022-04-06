// #[macro_use] extern crate rocket;
use mongodb::{options::ClientOptions, sync::Client};
use mongodb::bson::{doc, Document};
use serde::{Serialize, Deserialize};

use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io::prelude::*;

extern crate notify;

use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;

// #[get("/hello")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    wins: u32,
    losses: u32
}

impl User {
    fn new(name: &str) -> User {
        User {
            name: name.to_string(),
            wins: 0,
            losses: 0
        }
    }
}

// fn wait_until_file_created() {
//     let (tx, rx) = channel();
//     let mut watcher = raw_watcher(tx).unwrap();
//     // Watcher can't be registered for file that don't exists.
//     // I use its parent directory instead, because I'm sure that it always exists
//     // let file_dir = file_path.parent().unwrap();
//     watcher.watch("../", RecursiveMode::NonRecursive).unwrap();
//     // watcher.watch("../../../", RecursiveMode::Recursive).unwrap();
//     // if !file_path.exists() {
//     //     loop {
//     //         match rx.recv_timeout(Duration::from_secs(2))? {
//     //             RawEvent { path: Some(p), op: Ok(op::CREATE), .. } => 
//     //                 if p == file_path {
//     //                     break
//     //                 },
//     //             _ => continue,
//     //         }
//     //     }
//     // }
//     loop {
//         match rx.recv() {
//            Ok(RawEvent{path: Some(path), op: Ok(op), cookie}) => {
//                println!("{:?} {:?} ({:?})", op, path, cookie);
//             //    fs::remove_file("User.txt").unwrap();
//            },
//            Ok(event) => println!("broken event: {:?}", event),
//            Err(e) => println!("watch error: {:?}", e),
//         }
//     }
// }

// fn rocket() -> Result<rocket::Rocket<rocket::Build>, mongodb::error::Error> {
//     let mut client_options = ClientOptions::parse(
//         "mongodb+srv://myUser:myPassword@mycluster.zvnqo.mongodb.net/MyCluster?retryWrites=true&w=majority",
//     )?;
//     let client = Client::with_options(client_options)?;
//     for db_name in client.list_database_names(None, None)? {
//         println!("{}", db_name);
//     }
//     println!("");
//     let database = client.database("Connect4DB");
//     for collection_name in database.list_collection_names(None)? {
//         println!("{}", collection_name);
//     }
//     let collection = database.collection::<User>("test");
//     println!("connected");
//     collection.delete_many(doc! { "losses": 0 }, None)?;
//     let docs = vec![
//         User {
//             name: "Aaron".to_string(),
//             wins: 0,
//             losses: 0
//         },
//         User {
//             name: "Calvin".to_string(),
//             wins: 0,
//             losses: 0
//         },
//         User {
//             name: "Ryden".to_string(),
//             wins: 0,
//             losses: 0
//         }
//     ];
//     collection.insert_many(docs, None)?;

//     let mut file = File::create("User.txt")?;

//     let cursor = collection.find(doc! { "wins": 0 }, None)?;
//     for result in cursor {
//         let user = &result?;
//         println!("title: {}", user.name);
//         println!("title: {}", user.wins);
//         println!("title: {}", user.losses);
//         // bincode::serialize_into(&mut file, &result?).unwrap();
//         // bincode::serialize_into(&mut file, &User {
//         //     name: "Ryden".to_string(),
//         //     wins: 0,
//         //     losses: 0
//         // }).unwrap();
//         file.write_all(serde_json::to_string(&user).unwrap().as_bytes())?;
//     }

//     Ok(rocket::build().mount("/", routes![index]))
// }

// #[rocket::main]
// async fn main() {
//     // launch server or report error
//     match rocket() {
//         Ok(rocket) => {
//             let error = rocket.launch().await;
//             // eprintln!("Failed to launch server: {}", error);
//         }
//         Err(error) => eprintln!("Failed to create server: {}", error),
//     }
// }

fn main() {
    let mut client_options = ClientOptions::parse(
        "mongodb+srv://myUser:myPassword@mycluster.zvnqo.mongodb.net/MyCluster?retryWrites=true&w=majority",
    ).unwrap();
    let client = Client::with_options(client_options).unwrap();
    for db_name in client.list_database_names(None, None).unwrap() {
        println!("{}", db_name);
    }
    println!("");
    let database = client.database("Connect4DB");
    for collection_name in database.list_collection_names(None).unwrap() {
        println!("{}", collection_name);
    }
    let collection = database.collection::<User>("test");
    println!("connected");
    collection.delete_many(doc! { "losses": 0 }, None).unwrap();
    let docs = vec![
        User {
            name: "Aaron".to_string(),
            wins: 0,
            losses: 0
        },
        User {
            name: "Calvin".to_string(),
            wins: 0,
            losses: 0
        },
        User {
            name: "Ryden".to_string(),
            wins: 0,
            losses: 0
        }
    ];
    collection.insert_many(docs, None).unwrap();

    let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("User.txt").unwrap();

    let cursor = collection.find(doc! { "wins": 0 }, None).unwrap();
    for result in cursor {
        let user = &result.unwrap();
        println!("title: {}", user.name);
        println!("title: {}", user.wins);
        println!("title: {}", user.losses);
        // bincode::serialize_into(&mut file, &result?).unwrap();
        // bincode::serialize_into(&mut file, &User {
        //     name: "Ryden".to_string(),
        //     wins: 0,
        //     losses: 0
        // }).unwrap();
        file.write_all(serde_json::to_string(&user).unwrap().as_bytes()).unwrap();
    }
    println!("waiting");
    // wait_until_file_created();
    loop {
        match File::open("User.txt") {
            Ok(user_file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                println!("{:#?}", contents);
                fs::remove_file("User.txt").unwrap();
                break;
            },
            _ => {}
        }
    }
    println!("done!");
}
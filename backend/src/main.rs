// #[macro_use] extern crate rocket;
use mongodb::{options::ClientOptions, sync::Client};
use mongodb::bson::{doc, Document};
// use serde::{Serialize, Deserialize};

use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io::prelude::*;

extern crate notify;

use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;

use rocket::{post, response::content, routes, serde::{Deserialize, Serialize}};
use rocket::serde::json::Json;
#[macro_use] extern crate rocket;
// use rocket_contrib::json::Json;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    name: String,
    wins: u32,
    losses: u32
}

#[get("/")]
fn index() -> Json<Vec<User>> {

    // loop {
    //     match File::open("User.txt") {
    //         Ok(user_file) => {
    //             let mut contents = String::new();
    //             file.read_to_string(&mut contents).unwrap();
    //             println!("{:#?}", contents);
    //             fs::remove_file("User.txt").unwrap();
    //             break;
    //         },
    //         _ => {}
    //     }
    // }
    // "Hello, world!"

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
    // standardize for testing for now REMOVE LATER
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

    //****************** NOTE: PUT WHATEVER QUERY YOU WANT HERE ******************* */
    let cursor = collection.find(doc! { "wins": 0 }, None).unwrap();

    let mut users = Vec::<User>::new();

    for result in cursor {
        let user = result.unwrap();
        println!("title: {}", user.name);
        println!("title: {}", user.wins);
        println!("title: {}", user.losses);
        // file.write_all(serde_json::to_string(&user).unwrap().as_bytes())?;
        users.push(user);
    }

    rocket::serde::json::Json(users)
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

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn rocket() -> Result<rocket::Rocket<rocket::Build>, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse(
        "mongodb+srv://myUser:myPassword@mycluster.zvnqo.mongodb.net/MyCluster?retryWrites=true&w=majority",
    )?;
    let client = Client::with_options(client_options)?;
    for db_name in client.list_database_names(None, None)? {
        println!("{}", db_name);
    }
    println!("");
    let database = client.database("Connect4DB");
    for collection_name in database.list_collection_names(None)? {
        println!("{}", collection_name);
    }
    let collection = database.collection::<User>("test");
    println!("connected");
    collection.delete_many(doc! { "losses": 0 }, None)?;
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
    collection.insert_many(docs, None)?;

    let mut file = File::create("User.txt")?;

    let cursor = collection.find(doc! { "wins": 0 }, None)?;
    for result in cursor {
        let user = &result?;
        println!("title: {}", user.name);
        println!("title: {}", user.wins);
        println!("title: {}", user.losses);
        file.write_all(serde_json::to_string(&user).unwrap().as_bytes())?;
    }

    Ok(rocket::build().mount("/", routes![index]))
}

#[rocket::main]
async fn main() {
    // launch server or report error
    match rocket() {
        Ok(rocket) => {
            let error = rocket
            .attach(CORS)
            .launch().await;
        }
        Err(error) => eprintln!("Failed to create server: {}", error),
    }
}

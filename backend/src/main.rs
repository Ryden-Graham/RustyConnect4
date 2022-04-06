#[macro_use] extern crate rocket;
use mongodb::{options::ClientOptions, sync::Client};
use mongodb::bson::{doc, Document};
use serde::{Serialize, Deserialize};

use std::fs::File;
use std::io::prelude::*;

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

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
    collection.delete_many(doc! { "wins": 0 }, None)?;
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
        // bincode::serialize_into(&mut file, &result?).unwrap();
        // bincode::serialize_into(&mut file, &User {
        //     name: "Ryden".to_string(),
        //     wins: 0,
        //     losses: 0
        // }).unwrap();
        file.write_all(serde_json::to_string(&user).unwrap().as_bytes())?;
    }

    Ok(rocket::build().mount("/", routes![index]))
}

#[rocket::main]
async fn main() {
    // launch server or report error
    match rocket() {
        Ok(rocket) => {
            let error = rocket.launch().await;
            // eprintln!("Failed to launch server: {}", error);
        }
        Err(error) => eprintln!("Failed to create server: {}", error),
    }
}
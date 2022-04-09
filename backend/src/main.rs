use mongodb::{options::ClientOptions, sync::Client};
use mongodb::bson::doc;

use rocket::{post, response::content, routes, serde::{Deserialize, Serialize}};
use rocket::serde::json::Json;
#[macro_use] extern crate rocket;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    name: String,
    wins: u32,
    losses: u32
}

#[post("/client", format = "json", data = "<user>")]
fn get_json(user: Json<User>) -> Json<User> {
    println!("title: {}", user.name);
    println!("title: {}", user.wins);
    println!("title: {}", user.losses);
    user
}

#[options("/client")]
fn confirm_options() {
    // we need this to accept options before the client posts
}

#[get("/")]
fn index() -> Json<Vec<User>> {
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

// prepare rocket for launch
fn rocket() -> Result<rocket::Rocket<rocket::Build>, mongodb::error::Error> {
    Ok(rocket::build().mount("/", routes![index, get_json, confirm_options]))
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

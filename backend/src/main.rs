use mongodb::{options::ClientOptions, sync::Client};
use mongodb::bson::doc;
use chrono::{DateTime, Utc};
use rand::prelude::*;

use rocket::{post, response::content, routes, serde::{Deserialize, Serialize}};
use rocket::serde::json::Json;
#[macro_use] extern crate rocket;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Game {
    id: u32,
    game_type_is_c4: bool, 
    player_1_name: String,
    player_2_name: String,
    player_2_is_computer: bool,
    player1_won: u32,
    date: DateTime<Utc>
}

// send database
fn get_game_database() -> mongodb::sync::Collection<Game> {
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
    database.collection::<Game>("games")
}

// used for the nuke (and possibly more) commands
// add more ifs for more commands
#[post("/command", data = "<command>")]
fn get_command(command: String) -> String {
    // nuke database
    if command == "nuke the world" {
        let collection = get_game_database();
        collection.delete_many(doc! { "player1_won": 2 }, None).unwrap();
        collection.delete_many(doc! { "player1_won": 1 }, None).unwrap();
        collection.delete_many(doc! { "player1_won": 0 }, None).unwrap();
    }
    command
}

// receive new game
#[post("/client", format = "json", data = "<game>")]
fn send_json(game: Json<Game>) -> Json<Game> {
    // println!("id: {}", game.id);
    println!("type: {}", game.game_type_is_c4);
    println!("p1: {}", game.player_1_name);
    println!("p2: {}", game.player_2_name);
    println!("p2isCPU: {}", game.player_2_is_computer);
    println!("player1_won: {}", game.player1_won);
    println!("date: {}", game.date);
    let mut rng = rand::thread_rng();

    let collection = get_game_database();

    let mut id = 0;
    loop {
        id = rng.gen_range(0..u32::MAX) as u32;
        if collection.find(doc! { "id": id }, None).unwrap().count() == 0 {
            break;
        }
    }

    let mut games = vec![Game {
        id: id,
        game_type_is_c4: game.game_type_is_c4,
        player_1_name: game.player_1_name.clone(),
        player_2_name: game.player_2_name.clone(),
        player_2_is_computer: game.player_2_is_computer,
        player1_won: game.player1_won,
        date: game.date
    }];

    collection.insert_many(games, None).unwrap();

    game
}

#[options("/client")]
fn confirm_client_options() {
    // we need this to accept options before the client posts
}

#[get("/")]
fn index() -> Json<Vec<Game>> {
    let collection = get_game_database();
    println!("connected");

    //****************** NOTE: PUT WHATEVER QUERY YOU WANT HERE ******************* */
    // We just retrun everything to the client
    let cursor = collection.find(doc! { }, None).unwrap();

    let mut games = Vec::<Game>::new();

    // debug print in backend console to show it's working
    for result in cursor {
        let game = result.unwrap();
        println!("id: {}", game.id);
        games.push(game);
    }

    rocket::serde::json::Json(games)
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
    Ok(rocket::build().mount("/", routes![index, send_json, confirm_client_options, get_command]))
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

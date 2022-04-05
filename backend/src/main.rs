#[macro_use] extern crate rocket;
use mongodb::{options::ClientOptions, sync::Client};
use mongodb::bson::{doc, Document};

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

fn rocket() -> Result<rocket::Rocket<rocket::Build>, mongodb::error::Error> {
    // match ClientOptions::parse(
    //     "mongodb+srv://myUser:myPassword@mycluster.zvnqo.mongodb.net/MyCluster?retryWrites=true&w=majority",
    // ){
    //     Some(client_options) => {
    //         let client = Client::with_options(client_options)?;
    //         let database = client.database("Connect4DB");
        
    //         Ok(rocket::build().mount("/", routes![index]))
    //     }
    //     Err(error) => {
    //         Err(error)
    //     }
    // };

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
    let collection = database.collection::<Document>("test");
    println!("");
    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];
    collection.insert_many(docs, None)?;
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
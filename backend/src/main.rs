#[macro_use] extern crate rocket;
use mongodb::{options::ClientOptions, sync::Client};

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

    let client_options = ClientOptions::parse(
        "mongodb+srv://myUser:myPassword@mycluster.zvnqo.mongodb.net/MyCluster?retryWrites=true&w=majority",
    )?;
    let client = Client::with_options(client_options)?;
    for db_name in client.list_database_names(None, None)? {
        println!("{}", db_name);
    }
    // let database = client.database("Connect4DB");
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
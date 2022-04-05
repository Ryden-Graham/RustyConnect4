// use rocket_contrib::json::Json;
// use serde::Deserialize;
// use rocket::response::{self, Responder};
use rocket::{post, response::content, routes, serde::{Deserialize, Serialize}};
use rocket::serde::json::Json;
// use rocket::Request;
#[macro_use] extern crate rocket;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct TestStruct {
    attr1: String,
    attr2: bool
}

// #[get("/hello")]
// #[post("/hello", data = "<data>")]
// #[post("/hello")]
// fn index() -> std::io::Result<()> {
//     let my_struct = TestStruct {
//         attr1: "working".to_string(),
//         attr2: true
//     };
//     Json(my_struct)
// }

// #[get("/hello", format = "json")]
// fn index() -> Json<TestStruct> {
//     let my_struct = TestStruct {
//         attr1: "working".to_string(),
//         attr2: true
//     };
//     Json(my_struct)
// }

// #[get("/hello")]
// fn receive() -> &'static str {
//     "Hello, world!"
// }

// #[post("/hello", format = "json", data = "<user>")]
// fn new_user(user: Json<TestStruct>){
//     // let out = user.attr1.clone().to_string();
//     println!("adwdwdwadwad {:?}", user);
// }

#[post("/<name>")]
fn hi(name: String) -> String {
    println!("adwdwdwadwad {:?}", name);
    name
}

#[launch]
fn rocket() -> _ {
    // rocket::build().mount("/", routes![index, new_user, hi])
    rocket::build().mount("/hello", routes![hi])
}
#[macro_use]
extern crate rocket;
extern crate serde;


use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
  message: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Gender {
  Male,
  Female,
  NonBinary,
  Transgender,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
  id: String,
  name: String,
  last_name: String,
  user_name: String,
  age: u8,
  email: String,
  deleted: bool,
  gender: Gender,
}


#[get("/<name>/<age>")]
fn user(name: &str, age: u8) -> Json<Message> {
  let my_message: String = format!("Hello {}, you are {} yo", name, age);
  Json(Message {
    message: my_message,
  })
}

#[get("/<name>")]
fn user_by_name(name: &str) -> Json<User> {
  let name: String = name.to_string();
  let my_uuid = Uuid::new_v4();

  Json(User {
    id: my_uuid.to_string(),
    name,
    last_name: "Arroyo".to_string(),
    user_name: "m11arroyo".to_string(),
    age: 18,
    email: "m11_arroyo@gmail.com".to_string(),
    deleted: false,
    gender: Gender::Female,
  })
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/users", routes![user, user_by_name])
}

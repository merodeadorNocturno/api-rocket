use rocket::serde::json::Json;
use uuid::Uuid;

mod definitions;
use definitions::{ Message, Gender, User };

#[get("/<name>/<age>")]
pub fn user(name: &str, age: u8) -> Json<Message> {
  let my_message: String = format!("Hello {}, you are {} yo", name, age);
  Json(Message {
    message: my_message,
  })
}

#[get("/<name>")]
pub fn user_by_name(name: &str) -> Json<User> {
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
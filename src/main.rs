#[macro_use]
extern crate rocket;

mod routes {
  pub mod users;
  pub mod classrooms;
}
use routes::users::{ user, user_by_name };
use routes::classrooms::{ classrooms };

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/users", routes![user, user_by_name])
    .mount("/classrooms", routes![classrooms])
}

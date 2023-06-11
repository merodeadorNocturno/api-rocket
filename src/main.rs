#[macro_use]
extern crate rocket;

mod routes;
use routes::{ user, user_by_name };

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/users", routes![user, user_by_name])
}

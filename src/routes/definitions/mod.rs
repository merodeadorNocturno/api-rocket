use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
  pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Gender {
  Male,
  Female,
  NonBinary,
  Transgender,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub id: String,
  pub name: String,
  pub last_name: String,
  pub user_name: String,
  pub age: u8,
  pub email: String,
  pub deleted: bool,
  pub gender: Gender,
}
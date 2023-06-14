use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ClassroomName {
  Beethoven,
  Yamaha,
  SteinwayAndSons,
  Casio,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Classroom {
  pub id: String,
  pub name: ClassroomName,
  pub capacity: u8,
  pub deleted: bool,
}
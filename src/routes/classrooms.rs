use rocket::serde::json::Json;
use uuid::Uuid;

mod definitions;
use definitions::{ Classroom, ClassroomName};

#[get("/")]
pub fn classrooms() -> Json<Classroom> {
  let my_uuid = Uuid::new_v4();

  Json(Classroom {
    id: my_uuid.to_string(),
    name: ClassroomName::Casio,
    capacity: 3,
    deleted: false,
  })
}
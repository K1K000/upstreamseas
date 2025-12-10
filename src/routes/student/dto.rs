use serde::{Deserialize, Serialize};

use crate::entities::student;

#[derive(Serialize)]
pub struct StudentResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct StudentCreate {
    // pub id: i32,
    pub name: String,
    pub email: String,
    pub book_ids: Vec<i32>,
}

pub fn student_to_dto(student: &student::Model) -> StudentResponse {
    StudentResponse {
        id: student.id,
        name: student.name.clone(),
        email: student.email.clone(),
    }
}

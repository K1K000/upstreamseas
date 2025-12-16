use serde::{Deserialize, Serialize};

use crate::entities::borrow;

#[derive(Serialize)]
pub struct BorrowResponse {
    pub id: i32,
    pub book_id: i32,
    pub student_id: i32,
}

#[derive(Deserialize)]
pub struct BorrowCreate {
    pub book_id: i32,
    pub student_id: i32,
}

pub fn borrow_to_dto(borrow: &borrow::Model) -> BorrowResponse {
    BorrowResponse {
        id: borrow.id,
        book_id: borrow.book_id,
        student_id: borrow.student_id,
    }
}

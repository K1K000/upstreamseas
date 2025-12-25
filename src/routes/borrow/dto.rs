use chrono::NaiveDate;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

use crate::entities::borrow;

#[derive(Serialize)]
pub struct BorrowResponse {
    pub id: i32,
    pub book_id: i32,
    pub student_id: i32,
    pub date: NaiveDate,
    pub limit: NaiveDate,
    pub active: bool
}


//TODO: DO SOMETHING HERE RN WE CANT CHANGE IF THIS IS ACTIVE OR NOT, also date and limit still not
//change able so grind on
#[derive(Deserialize)]
pub struct BorrowCreate {
    pub book_id: i32,
    pub student_id: i32,
    pub borrow_lenght: u64,
}

#[derive(Deserialize)]
pub struct BorrowChange {
    pub book_id: i32,
    pub student_id: i32,
    pub extension: u64,
    pub active: bool
}


pub fn to_dto(borrow: &borrow::Model) -> BorrowResponse {
    BorrowResponse {
        id: borrow.id,
        book_id: borrow.book_id,
        student_id: borrow.student_id,
        date: borrow.date,
        limit: borrow.limit,
        active: borrow.active,
    }
}

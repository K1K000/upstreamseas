use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::entities::book;

#[derive(Serialize, Debug)]
pub struct BookResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub available: u32,
    pub all_available: u32,
    pub release: NaiveDate,
    pub deleted: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BookUpdate {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub available: u32,
    pub all_available: u32,
    pub release: NaiveDate,
    pub deleted: bool,
}

#[derive(Deserialize)]
pub struct BookCreate {
    pub name: String,
    pub all_available: u32,
    pub description: String,
    pub release: NaiveDate,
}

pub fn book_to_dto(book: &book::Model) -> BookResponse {
    BookResponse {
        id: book.id,
        name: book.name.clone(),
        description: book.description.clone(),
        available: book.available,
        all_available: book.all_available,
        release: book.release,
        deleted: book.deleted,
    }
}

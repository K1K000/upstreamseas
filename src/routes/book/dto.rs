use serde::{Deserialize, Serialize};

use crate::entities::book;

#[derive(Serialize, Debug)]
pub struct BookResponse {
    pub id: i32,
    pub name: String,
    pub available: u32,
}

#[derive(Deserialize)]
pub struct BookCreate {
    pub name: String,
    pub available: u32,
}

pub fn book_to_dto(book: &book::Model) -> BookResponse {
    BookResponse {
        id: book.id,
        name: book.name.clone(),
        available: book.available,
    }
}

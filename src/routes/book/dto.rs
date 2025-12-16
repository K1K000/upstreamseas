use serde::{Deserialize, Serialize};

use crate::entities::book;

#[derive(Serialize)]
pub struct BookResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct BookCreate {
    pub name: String,
}

pub fn book_to_dto(book: &book::Model) -> BookResponse {
    BookResponse {
        id: book.id,
        name: book.name.clone(),
    }
}

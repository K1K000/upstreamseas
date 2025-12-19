use serde::{Deserialize, Serialize};

use crate::entities::book_author;

#[derive(Serialize)]
pub struct BookAuthorResponse {
    pub id: i32,
    pub book_id: i32,
    pub author_id: i32,
}

#[derive(Deserialize)]
pub struct BookAuthorCreate {
    pub book_id: i32,
    pub author_id: i32,
}

pub fn to_dto(borrow: &book_author::Model) -> BookAuthorResponse {
    BookAuthorResponse {
        id: borrow.id,
        book_id: borrow.book_id,
        author_id: borrow.author_id,
    }
}

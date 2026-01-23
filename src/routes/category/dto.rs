use serde::{Deserialize, Serialize};

use crate::entities::{book, category};

#[derive(Serialize, Debug)]
pub struct CategoryResponse {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

#[derive(Deserialize)]
pub struct CategoryCreate {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CategoryUpdate {
    pub name: String,
    pub active: bool,
}

pub fn to_dto(book: &category::Model) -> CategoryResponse {
    CategoryResponse {
        id: book.id,
        name: book.name.clone(),
        active: book.active,
    }
}

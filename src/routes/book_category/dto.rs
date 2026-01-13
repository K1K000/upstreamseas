use serde::{Deserialize, Serialize};

use crate::entities::book_category;

#[derive(Serialize)]
pub struct BookCategoryResponse {
    pub id: i32,
    pub book_id: i32,
    pub category_id: i32,
}

#[derive(Deserialize)]
pub struct BookCategoryCreate {
    pub book_id: i32,
    pub category_id: i32,
}

pub fn to_dto(category: &book_category::Model) -> BookCategoryResponse {
    BookCategoryResponse {
        id: category.id,
        book_id: category.book_id,
        category_id: category.category_id,
    }
}

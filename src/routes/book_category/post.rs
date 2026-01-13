use rocket::{State, post, response::status::Created, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{
    entities::book_category,
    error_handling::ErrorResponder,
    routes::book_category::dto::{BookCategoryCreate, BookCategoryResponse},
};

#[post("/", format = "json", data = "<data>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    data: Json<BookCategoryCreate>,
) -> Result<Created<Json<BookCategoryResponse>>, ErrorResponder> {
    let db = db.inner();

    let book_category = book_category::ActiveModel::builder()
        .set_book_id(data.book_id)
        .set_category_id(data.book_id)
        .insert(db)
        .await?;

    Ok(
        Created::new("/book_category").body(Json(BookCategoryResponse {
            id: book_category.id,
            book_id: book_category.book_id,
            category_id: book_category.category_id,
        })),
    )
}

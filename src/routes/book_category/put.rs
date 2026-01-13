use rocket::{State, http::Status, put, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    entities::{book_category, prelude::*},
    error_handling::ErrorResponder,
    routes::book_category::dto::BookCategoryCreate,
};

#[put("/<id>", data = "<data>", format = "json")]
pub async fn put(
    id: i32,
    data: Json<BookCategoryCreate>,
    db: &State<DatabaseConnection>,
) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match Book_Category::find_by_id(id).one(db).await? {
        Some(_val) => {
            let model = book_category::ActiveModel {
                id: sea_orm::ActiveValue::set(id),
                book_id: Set(data.book_id),
                category_id: Set(data.category_id),
            };
            Book_Category::update(model).exec(db).await?;
            Ok(Status::NoContent)
        }
        None => Err(ErrorResponder::NotFound(())),
    }
}

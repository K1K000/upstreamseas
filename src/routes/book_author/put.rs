use rocket::{State, http::Status, put, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    entities::{book_author, prelude::Book_Author},
    error_handling::ErrorResponder,
    routes::book_author::dto::*,
};

#[put("/<id>", data = "<new_item>", format = "json")]
pub async fn put(
    id: i32,
    new_item: Json<BookAuthorCreate>,
    db: &State<DatabaseConnection>,
) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match Book_Author::find_by_id(id).one(db).await? {
        Some(_val) => {
            let model = book_author::ActiveModel {
                id: sea_orm::ActiveValue::set(id),
                book_id: Set(new_item.book_id),
                author_id: Set(new_item.author_id),
            };
            Book_Author::update(model).exec(db).await?;
            Ok(Status::NoContent)
        }
        None => Err(ErrorResponder::NotFound(())),
    }
}

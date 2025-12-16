use rocket::{State, http::Status, put, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    entities::{book, prelude::Book},
    error_handling::ErrorResponder,
    routes::book::dto::BookCreate,
};

#[put("/<id>", data = "<new_item>", format = "json")]
pub async fn put(
    id: i32,
    new_item: Json<BookCreate>,
    db: &State<DatabaseConnection>,
) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match Book::find_by_id(id).one(db).await? {
        Some(_val) => {
            let model = book::ActiveModel {
                id: sea_orm::ActiveValue::set(id),
                name: Set(new_item.name.clone()),
            };
            Book::update(model).exec(db).await?;
            Ok(Status::NoContent)
        }
        None => Err(ErrorResponder::NotFound(())),
    }
}

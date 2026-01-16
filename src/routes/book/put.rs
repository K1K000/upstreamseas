use rocket::{State, http::Status, put, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    entities::{book, prelude::Book},
    error_handling::ErrorResponder,
    routes::book::dto::BookUpdate,
};

#[put("/<id>", data = "<data>", format = "json")]
pub async fn put(
    id: i32,
    data: Json<BookUpdate>,
    db: &State<DatabaseConnection>,
) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match Book::find_by_id(id).one(db).await? {
        Some(_val) => {
            let model = book::ActiveModel {
                id: sea_orm::ActiveValue::set(id),
                name: Set(data.name.clone()),
                description: Set(data.description.clone()),
                available: Set(data.available),
                all_available: Set(data.all_available),
                release: Set(data.release),
                deleted: Set(data.deleted),
            };
            Book::update(model).exec(db).await?;
            Ok(Status::NoContent)
        }
        None => Err(ErrorResponder::NotFound(())),
    }
}

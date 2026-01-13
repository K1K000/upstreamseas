use rocket::{State, http::Status, put, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    entities::{book, category, prelude::Category},
    error_handling::ErrorResponder,
    routes::{book::dto::BookCreate, category::dto::CategoryCreate},
};

#[put("/<id>", data = "<new_item>", format = "json")]
pub async fn put(
    id: i32,
    new_item: Json<CategoryCreate>,
    db: &State<DatabaseConnection>,
) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match Category::find_by_id(id).one(db).await? {
        Some(_val) => {
            let model = category::ActiveModel {
                id: sea_orm::ActiveValue::set(id),
                name: Set(new_item.name.clone()),
            };
            Category::update(model).exec(db).await?;
            Ok(Status::NoContent)
        }
        None => Err(ErrorResponder::NotFound(())),
    }
}

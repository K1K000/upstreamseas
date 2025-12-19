use chrono::Utc;
use rocket::{State, http::Status, put, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    entities::{author, prelude::Author},
    error_handling::ErrorResponder,
    routes::author::dto::*,
};

#[put("/<id>", data = "<new_item>", format = "json")]
pub async fn put(
    id: i32,
    new_item: Json<AuthorCreate>,
    db: &State<DatabaseConnection>,
) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match Author::find_by_id(id).one(db).await? {
        Some(_val) => {
            let model = author::ActiveModel {
                id: sea_orm::ActiveValue::set(id),
                name: Set(new_item.name.clone()),
                birthplace: Set(new_item.birthplace.clone()),
                birthdate: Set(Utc::now().date_naive()),
            };
            Author::update(model).exec(db).await?;
            Ok(Status::NoContent)
        }
        None => Err(ErrorResponder::NotFound(())),
    }
}

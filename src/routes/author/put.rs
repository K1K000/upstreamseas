use rocket::{State, http::Status, put, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    entities::{author, prelude::Author},
    error_handling::ErrorResponder,
    routes::author::dto::*,
};

#[put("/<id>", data = "<data>", format = "json")]
pub async fn put(
    id: i32,
    data: Json<AuthorUpdate>,
    db: &State<DatabaseConnection>,
) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match Author::find_by_id(id).one(db).await? {
        Some(_val) => {
            let model = author::ActiveModel {
                id: sea_orm::ActiveValue::set(id),
                name: Set(data.name.clone()),
                birthplace: Set(data.birthplace.clone()),
                birthdate: Set(data.birthdate),
                description: Set(data.description.clone()),
                deleted: Set(data.deleted),
            };
            Author::update(model).exec(db).await?;
            Ok(Status::NoContent)
        }
        None => Err(ErrorResponder::NotFound(())),
    }
}

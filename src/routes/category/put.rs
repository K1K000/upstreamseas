use rocket::{State, http::Status, put, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    entities::{category, prelude::Category},
    error_handling::ErrorResponder,
    routes::category::dto::CategoryUpdate,
};

#[put("/<id>", data = "<new_item>", format = "json")]
pub async fn put(
    id: i32,
    new_item: Json<CategoryUpdate>,
    db: &State<DatabaseConnection>,
) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match Category::find_by_id(id).one(db).await? {
        Some(_val) => {
            let model = category::ActiveModel {
                id: sea_orm::ActiveValue::set(id),
                name: Set(new_item.name.clone()),
                active: Set(new_item.active),
            };
            Category::update(model).exec(db).await?;
            Ok(Status::NoContent)
        }
        None => Err(ErrorResponder::NotFound(())),
    }
}

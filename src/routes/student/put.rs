use rocket::{State, http::Status, put, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    entities::{prelude::Student, student},
    error_handling::ErrorResponder,
    routes::student::dto::StudentCreate,
};

#[put("/<id>", data = "<new_item>", format = "json")]
pub async fn put(
    id: i32,
    new_item: Json<StudentCreate>,
    db: &State<DatabaseConnection>,
) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match Student::find_by_id(id).one(db).await? {
        Some(_val) => {
            let model = student::ActiveModel {
                id: Set(id),
                name: Set(new_item.name.clone()),
                email: Set(new_item.email.clone()),
            };
            Student::update(model).exec(db).await?;
            Ok(Status::NoContent)
        }
        None => Err(ErrorResponder::NotFound(())),
    }
}

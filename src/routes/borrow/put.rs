use rocket::{State, http::Status, put, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    entities::{borrow, prelude::Borrow},
    error_handling::ErrorResponder,
    routes::borrow::dto::BorrowCreate,
};

#[put("/<id>", data = "<new_item>", format = "json")]
pub async fn put(
    id: i32,
    new_item: Json<BorrowCreate>,
    db: &State<DatabaseConnection>,
) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match Borrow::find_by_id(id).one(db).await? {
        Some(_val) => {
            let model = borrow::ActiveModel {
                id: sea_orm::ActiveValue::set(id),
                book_id: Set(new_item.book_id),
                student_id: Set(new_item.student_id),
            };
            Borrow::update(model).exec(db).await?;
            Ok(Status::NoContent)
        }
        None => Err(ErrorResponder::NotFound(())),
    }
}

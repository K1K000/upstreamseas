use chrono::Days;
use rocket::{State, http::Status, put, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    entities::{borrow, prelude::Borrow},
    error_handling::ErrorResponder,
    routes::borrow::dto::BorrowUpdate,
};

#[put("/<id>", data = "<change>", format = "json")]
pub async fn put(
    id: i32,
    change: Json<BorrowUpdate>,
    db: &State<DatabaseConnection>,
) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match Borrow::find_by_id(id).one(db).await? {
        Some(val) => {
            let model = borrow::ActiveModel {
                id: sea_orm::ActiveValue::set(id),
                book_id: Set(change.book_id),
                student_id: Set(change.student_id),
                date: Set(val.date),
                end: Set(match change.end {
                    Some(val) => Some(val),
                    None => val.end,
                }),
                limit: Set(match change.limit {
                    Some(val) => val,
                    None => val.limit,
                }),
            };
            Borrow::update(model).exec(db).await?;
            Ok(Status::NoContent)
        }
        None => Err(ErrorResponder::NotFound(())),
    }
}

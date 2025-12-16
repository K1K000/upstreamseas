use rocket::{State, post, response::status::Created, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{
    entities::borrow,
    error_handling::ErrorResponder,
    routes::{borrow::dto::BorrowResponse, borrow::dto::*},
};

// TODO: proper error handling
#[post("/", format = "json", data = "<data>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    data: Json<BorrowCreate>,
) -> Result<Created<Json<BorrowResponse>>, ErrorResponder> {
    let db = db.inner();

    let borrow = borrow::ActiveModel::builder()
        .set_book_id(data.book_id)
        .set_student_id(data.student_id)
        .insert(db)
        .await?;

    Ok(Created::new("/borrow").body(Json(BorrowResponse {
        id: borrow.id,
        book_id: borrow.book_id,
        student_id: borrow.student_id,
    })))
}

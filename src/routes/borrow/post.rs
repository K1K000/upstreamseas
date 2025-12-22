use chrono::{Days, Utc};
use rocket::{State, post, response::status::Created, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{
    entities::borrow,
    error_handling::ErrorResponder,
    routes::borrow::business::*,
    routes::borrow::dto::{BorrowResponse, *},
};

// make this a variable somehow if needed
const BORROWLIMITDAYS: u64 = 100000;

#[post("/", format = "json", data = "<data>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    data: Json<BorrowCreate>,
) -> Result<Created<Json<BorrowResponse>>, ErrorResponder> {
    book_verification(data.book_id, db).await?;
    student_verification(data.student_id, db).await?;

    let db = db.inner();
    let borrow = borrow::ActiveModel::builder()
        .set_book_id(data.book_id)
        .set_student_id(data.student_id)
        .set_date(Utc::now().date_naive())
        .set_limit(
            // TODO: make limit changeable and handle error
            Utc::now()
                .date_naive()
                .checked_add_days(Days::new(BORROWLIMITDAYS))
                .unwrap_or(Utc::now().date_naive()),
        )
        .insert(db)
        .await?;

    Ok(Created::new("/borrow").body(Json(BorrowResponse {
        id: borrow.id,
        book_id: borrow.book_id,
        student_id: borrow.student_id,
        date: borrow.date,
        limit: borrow.limit,
    })))
}

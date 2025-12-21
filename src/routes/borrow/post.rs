use chrono::{Days, Utc};
use rocket::{State, post, response::status::Created, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    entities::{book, borrow, prelude::*},
    error_handling::{ErrorMessage, ErrorResponder},
    routes::borrow::dto::{BorrowResponse, *},
};

const BORROWLIMITDAYS: u64 = 100000;

#[post("/", format = "json", data = "<data>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    data: Json<BorrowCreate>,
) -> Result<Created<Json<BorrowResponse>>, ErrorResponder> {
    let db = db.inner();

    match Book::find_by_id(data.book_id).one(db).await? {
        None => {
            return Err(ErrorResponder::BadRequest(Json(ErrorMessage {
                message: String::from("The book doesn't exists"),
            })));
        }
        Some(book) => {
            if book.available == 0 {
                return Err(ErrorResponder::BadRequest(Json(ErrorMessage {
                    message: String::from("The book doesn't exists"),
                })));
            }

            Book::update(book::ActiveModel {
                id: Set(book.id),
                name: Set(book.name.clone()),
                // we decrease it by one because one book has been borrowed
                available: Set(book.available - 1),
            })
            .exec(db)
            .await?;
        } // _ => {}
    }

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

use rocket::{State, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    entities::{book, prelude::*},
    error_handling::{ErrorMessage, ErrorResponder},
};

pub async fn student_verification(
    student_id: i32,
    db: &State<DatabaseConnection>,
) -> Result<(), ErrorResponder> {
    let db = db.inner();
    match Student::find_by_id(student_id).one(db).await? {
        None => {
            return Err(ErrorResponder::BadRequest(Json(ErrorMessage {
                message: String::from("The student doesn't exists"),
            })));
        }
        Some(student) => {
            if !student.has_card {
                return Err(ErrorResponder::BadRequest(Json(ErrorMessage {
                    message: String::from("No valid readers card"),
                })));
            }
        }
    }
    Ok(())
}

pub async fn book_verification(
    book_id: i32,
    db: &State<DatabaseConnection>,
) -> Result<(), ErrorResponder> {
    let db = db.inner();
    match Book::find_by_id(book_id).one(db).await? {
        None => {
            return Err(ErrorResponder::BadRequest(Json(ErrorMessage {
                message: String::from("The book doesn't exists"),
            })));
        }
        Some(book) => {
            if book.available == 0 {
                return Err(ErrorResponder::BadRequest(Json(ErrorMessage {
                    message: String::from("The book isn't available"),
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
    Ok(())
}

use rocket::{State, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{
    entities::{book, prelude::*},
    error_handling::{ErrorMessage, ErrorResponder},
};

pub enum Flow {
    FBorrow,
    FGiveBack,
}

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

pub async fn book_handling(
    // book_model: Option<book::Model>,
    book_id: i32,
    db: &State<DatabaseConnection>,
    flow: Flow,
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
                //not very elegant :(
                available: Set(match flow {
                    Flow::FBorrow => book.available - 1, //borrowing decreases books by 1
                    Flow::FGiveBack => book.available + 1, // giving back to the community
                }),
            })
            .exec(db)
            .await?;
        } // _ => {}
    }
    Ok(())
}

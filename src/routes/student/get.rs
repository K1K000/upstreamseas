use crate::entities::{book, borrow};
use crate::routes::book::dto::{BookResponse, book_to_dto};
use crate::routes::student::dto::*;
use crate::{
    entities::prelude::*, error_handling::ErrorResponder, routes::student::dto::StudentResponse,
};
use rocket::{State, get, serde::json::Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[get("/")]
pub async fn all(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<StudentResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Student::find()
            .all(db)
            .await?
            .iter()
            .map(student_to_dto)
            .collect(),
    ))
}

#[get("/<student_id>/all_books")]
pub async fn student_all_books(
    db: &State<DatabaseConnection>,
    student_id: u64,
) -> Result<Json<Vec<BookResponse>>, ErrorResponder> {
    let db = db.inner();

    Ok(Json(
        (Book::find()
            .right_join(Borrow)
            .filter(borrow::Column::StudentId.eq(student_id))
            .all(db)
            .await?)
            .iter()
            .map(book_to_dto)
            .collect::<Vec<_>>(),
    ))
}
#[get("/<student_id>/active_books")]
pub async fn student_books(
    db: &State<DatabaseConnection>,
    student_id: u64,
) -> Result<Json<Vec<BookResponse>>, ErrorResponder> {
    let db = db.inner();

    Ok(Json(
        (Book::find()
            .right_join(Borrow)
            .filter(borrow::Column::StudentId.eq(student_id))
            .filter(borrow::Column::End.is_null())
            .all(db)
            .await?)
            .iter()
            .map(book_to_dto)
            .collect::<Vec<_>>(),
    ))
}

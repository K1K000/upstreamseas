use crate::entities::{book, borrow};
use crate::error_handling::ErrorMessage;
use crate::routes::book::dto::{BookResponse, book_to_dto};
use crate::routes::student::dto::*;
use crate::{
    entities::prelude::*, error_handling::ErrorResponder, routes::student::dto::StudentResponse,
};
use rocket::{State, get, serde::json::Json};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder, QuerySelect,
};

#[get("/id/<id>")]
pub async fn id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<StudentResponse>, ErrorResponder> {
    let db = db.inner();
    match Student::find_by_id(id).one(db).await? {
        Some(val) => Ok(Json(student_to_dto(&val))),
        None => Err(ErrorResponder::InternalError(Json(ErrorMessage {
            message: String::from("This id doesn't exist"),
        }))),
    }
}

const DEFAULT_LIMIT: u64 = 20;
#[get("/<n>")]
pub async fn all(
    db: &State<DatabaseConnection>,
    n: Option<u64>,
) -> Result<Json<Vec<StudentResponse>>, ErrorResponder> {
    let db = db.inner();
    let n = n.unwrap_or(DEFAULT_LIMIT);
    Ok(Json(
        Student::find()
            .limit(n)
            .all(db)
            .await?
            .iter()
            .map(student_to_dto)
            .collect(),
    ))
}

#[get("/top/<n>")]
pub async fn top(
    db: &State<DatabaseConnection>,
    n: u64,
) -> Result<Json<Vec<StudentResponse>>, ErrorResponder> {
    let db = db.inner();

    Ok(Json(
        Student::find()
            .left_join(Borrow)
            .group_by(borrow::Column::StudentId)
            .order_by(borrow::Column::StudentId.count(), Order::Desc)
            .limit(n)
            .all(db)
            .await?
            .iter()
            .map(student_to_dto)
            .collect::<Vec<_>>(),
    ))
}

#[get("/id/<student_id>/all_books")]
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
#[get("/id/<student_id>/active_books")]
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

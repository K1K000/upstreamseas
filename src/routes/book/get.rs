use crate::entities::{book, book_category, borrow};
use crate::error_handling::ErrorMessage;
use crate::routes::book::dto::*;
use crate::routes::borrow::dto::{self, BorrowResponse};
use crate::routes::category::dto::CategoryResponse;
use crate::routes::{self};
use crate::{
    entities::prelude::{Book, Book_Category, Borrow, Category},
    error_handling::ErrorResponder,
};
use rocket::{State, get, serde::json::Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

#[get("/id/<id>")]
pub async fn id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<BookResponse>, ErrorResponder> {
    let db = db.inner();
    match Book::find_by_id(id).one(db).await? {
        Some(val) => Ok(Json(book_to_dto(&val))),
        None => Err(ErrorResponder::InternalError(Json(ErrorMessage {
            message: String::from("This id doesn't exist"),
        }))),
    }
}

const DEFAULT_LIMIT: u64 = 20;
#[get("/<n>")]
pub async fn limit(
    db: &State<DatabaseConnection>,
    n: Option<u64>,
) -> Result<Json<Vec<BookResponse>>, ErrorResponder> {
    let db = db.inner();
    let n = n.unwrap_or(DEFAULT_LIMIT);

    Ok(Json(
        Book::find()
            .limit(n)
            .all(db)
            .await?
            .iter()
            .map(book_to_dto)
            .collect(),
    ))
}

#[get("/id/<id>/borrows")]
pub async fn book_borrows(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<Vec<crate::routes::borrow::dto::BorrowResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Borrow::find()
            .filter(borrow::Column::BookId.eq(id))
            .all(db)
            .await?
            .iter()
            .map(dto::to_dto)
            .collect::<Vec<BorrowResponse>>(),
    ))
}

#[get("/id/<id>/categories")]
pub async fn book_categories(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<Vec<CategoryResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Category::find()
            .left_join(Book_Category)
            .filter(book_category::Column::BookId.eq(id))
            .all(db)
            .await?
            .iter()
            .map(routes::category::dto::to_dto)
            .collect(),
    ))
}

use crate::entities::{book, book_category};
use crate::error_handling::ErrorMessage;
use crate::routes;
use crate::routes::book::dto::*;
use crate::routes::category::dto::CategoryResponse;
use crate::{
    entities::prelude::{Book, Book_Category, Category},
    error_handling::ErrorResponder,
};
use rocket::{State, get, serde::json::Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

#[get("/")]
pub async fn all(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<BookResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Book::find()
            .all(db)
            .await?
            .iter()
            .map(book_to_dto)
            .collect(),
    ))
}

#[get("/id/<id>")]
pub async fn id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<BookResponse>, ErrorResponder> {
    let db = db.inner();
    match Book::find_by_id(id).one(db).await? {
        Some(val) => Ok(Json(book_to_dto(&val))),
        None => Err(ErrorResponder::InternalError(Json(ErrorMessage {
            message: String::from("dattebayo"),
        }))),
    }
}

#[get("/<n>")]
pub async fn limit(
    db: &State<DatabaseConnection>,
    n: u64,
) -> Result<Json<Vec<BookResponse>>, ErrorResponder> {
    let db = db.inner();
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

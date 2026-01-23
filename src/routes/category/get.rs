use crate::entities::borrow;
use crate::entities::prelude::Book;
use crate::error_handling::ErrorMessage;
use crate::routes::book::dto::{BookResponse, book_to_dto};
use crate::routes::category::dto::{CategoryResponse, to_dto};
use crate::{entities::prelude::Category, error_handling::ErrorResponder};
use rocket::{State, get, serde::json::Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

#[get("/id/<id>")]
pub async fn by_id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<CategoryResponse>, ErrorResponder> {
    let db = db.inner();

    match Category::find_by_id(id).one(db).await? {
        Some(val) => Ok(Json(to_dto(&val))),
        None => Err(ErrorResponder::InternalError(Json(ErrorMessage {
            message: String::from("This id doesn't exist"),
        }))),
    }
}

#[get("/<n>")]
pub async fn all(
    db: &State<DatabaseConnection>,
    n: Option<u64>,
) -> Result<Json<Vec<CategoryResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Category::find()
            .limit(n)
            .all(db)
            .await?
            .iter()
            .map(to_dto)
            .collect(),
    ))
}

#[get("/id/<id>/books")]
pub async fn category_books(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<Vec<BookResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Book::find()
            .filter(borrow::Column::BookId.eq(id))
            .all(db)
            .await?
            .iter()
            .map(book_to_dto)
            .collect::<Vec<BookResponse>>(),
    ))
}

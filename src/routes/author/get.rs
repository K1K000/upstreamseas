use crate::entities::book_author;
use crate::error_handling::ErrorMessage;
use crate::routes;
use crate::routes::author::dto::*;
use crate::routes::book::dto::BookResponse;
use crate::{entities::prelude::*, error_handling::ErrorResponder};
use rocket::{State, get, serde::json::Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

// #[get("/")]
// pub async fn all(
//     db: &State<DatabaseConnection>,
// ) -> Result<Json<Vec<AuthorResponse>>, ErrorResponder> {
//     let db = db.inner();
//     Ok(Json(
//         Author::find().all(db).await?.iter().map(to_dto).collect(),
//     ))
// }

#[get("/id/<id>")]
pub async fn id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<AuthorResponse>, ErrorResponder> {
    let db = db.inner();
    match Author::find_by_id(id).one(db).await? {
        Some(val) => Ok(Json(to_dto(&val))),
        None => Err(ErrorResponder::InternalError(Json(ErrorMessage {
            message: String::from("dattebayo"),
        }))),
    }
}

#[get("/<n>")]
pub async fn limit(
    db: &State<DatabaseConnection>,
    n: Option<u64>,
) -> Result<Json<Vec<AuthorResponse>>, ErrorResponder> {
    let db = db.inner();
    match n {
        Some(val) => {
            return Ok(Json(
                Author::find()
                    .limit(val)
                    .all(db)
                    .await?
                    .iter()
                    .map(to_dto)
                    .collect(),
            ));
        }
        None => {
            return Ok(Json(
                Author::find().all(db).await?.iter().map(to_dto).collect(),
            ));
        }
    }
}

#[get("/id/<id>/books")]
pub async fn authors_books(
    db: &State<DatabaseConnection>,
    id: u64,
) -> Result<Json<Vec<BookResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Book::find()
            .left_join(Book_Author)
            .filter(book_author::Column::AuthorId.eq(id))
            .all(db)
            .await?
            .iter()
            .map(routes::book::dto::book_to_dto)
            .collect(),
    ))
}

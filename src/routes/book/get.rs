use crate::routes::book::dto::*;
use crate::{entities::prelude::Book, error_handling::ErrorResponder};
use rocket::{State, get, serde::json::Json};
use sea_orm::{DatabaseConnection, EntityTrait};

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

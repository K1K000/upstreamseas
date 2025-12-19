use crate::routes::book_author::dto::*;
use crate::{entities::prelude::Book_Author, error_handling::ErrorResponder};
use rocket::{State, get, serde::json::Json};
use sea_orm::{DatabaseConnection, EntityTrait};

#[get("/")]
pub async fn all(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<BookAuthorResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Book_Author::find()
            .all(db)
            .await?
            .iter()
            .map(to_dto)
            .collect(),
    ))
}

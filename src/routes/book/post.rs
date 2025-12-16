use rocket::{State, post, response::status::Created, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{
    entities::book,
    error_handling::ErrorResponder,
    routes::{book::dto::BookResponse, student::dto::*},
};

#[post("/", format = "json", data = "<data>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    data: Json<StudentCreate>,
) -> Result<Created<Json<BookResponse>>, ErrorResponder> {
    let db = db.inner();

    let book = book::ActiveModel::builder()
        .set_name(data.name.clone())
        .insert(db)
        .await?;

    Ok(Created::new("/student").body(Json(BookResponse {
        id: book.id,
        name: book.name,
    })))
}

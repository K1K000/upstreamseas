use rocket::{State, post, response::status::Created, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{
    entities::book,
    error_handling::ErrorResponder,
    routes::book::dto::{BookCreate, BookResponse},
};

#[post("/", format = "json", data = "<data>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    data: Json<BookCreate>,
) -> Result<Created<Json<BookResponse>>, ErrorResponder> {
    let db = db.inner();

    let book = book::ActiveModel::builder()
        .set_name(data.name.clone())
        .set_available(data.available)
        .insert(db)
        .await?;
    Ok(Created::new("/book").body(Json(BookResponse {
        id: book.id,
        name: book.name,
        available: book.available,
    })))
}

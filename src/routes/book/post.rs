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
        .set_all_available(data.all_available)
        .set_available(data.all_available)
        .set_description(data.description.clone())
        .set_release(data.release)
        .set_max_borrow(data.max_borrow)
        .set_deleted(false)
        .insert(db)
        .await?;
    Ok(Created::new("/book").body(Json(BookResponse {
        id: book.id,
        name: book.name,
        description: book.description,
        all_available: book.all_available,
        release: book.release,
        available: book.available,
        max_borrow: book.max_borrow,
        deleted: book.deleted,
    })))
}

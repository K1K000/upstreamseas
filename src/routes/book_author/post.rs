use rocket::{State, post, response::status::Created, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::book_author, error_handling::ErrorResponder, routes::book_author::dto::*};

#[post("/", format = "json", data = "<data>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    data: Json<BookAuthorCreate>,
) -> Result<Created<Json<BookAuthorResponse>>, ErrorResponder> {
    let db = db.inner();

    let book_author = book_author::ActiveModel::builder()
        .set_book_id(data.book_id)
        .set_author_id(data.book_id)
        .insert(db)
        .await?;

    Ok(Created::new("/book_author").body(Json(BookAuthorResponse {
        id: book_author.id,
        book_id: book_author.book_id,
        author_id: book_author.author_id,
    })))
}

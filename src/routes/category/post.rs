use rocket::{State, post, response::status::Created, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{
    entities::{book, category},
    error_handling::ErrorResponder,
    routes::{
        book::dto::{BookCreate, BookResponse},
        category::dto::{CategoryCreate, CategoryResponse},
    },
};

#[post("/", format = "json", data = "<data>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    data: Json<CategoryCreate>,
) -> Result<Created<Json<CategoryResponse>>, ErrorResponder> {
    let db = db.inner();

    let category = category::ActiveModel::builder()
        .set_name(data.name.clone())
        .insert(db)
        .await?;
    Ok(Created::new("/category").body(Json(CategoryResponse {
        id: category.id,
        name: category.name,
    })))
}

use crate::entities::book_category::Entity;
use crate::routes::book_category::dto::*;
use crate::{entities::prelude::Book_Category, error_handling::ErrorResponder};
use rocket::{State, get, serde::json::Json};
use sea_orm::{DatabaseConnection, EntityTrait, QuerySelect};

#[get("/")]
pub async fn all(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<BookCategoryResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Book_Category::find()
            .all(db)
            .await?
            .iter()
            .map(to_dto)
            .collect(),
    ))
}

#[get("/<n>")]
pub async fn limit(
    db: &State<DatabaseConnection>,
    n: u64,
) -> Result<Json<Vec<BookCategoryResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Book_Category::find()
            .limit(n)
            .all(db)
            .await?
            .iter()
            .map(to_dto)
            .collect(),
    ))
}

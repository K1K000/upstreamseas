use crate::routes::author::dto::*;
use crate::{entities::prelude::Author, error_handling::ErrorResponder};
use rocket::{State, get, serde::json::Json};
use sea_orm::{DatabaseConnection, EntityTrait};

#[get("/")]
pub async fn all(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<AuthorResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Author::find().all(db).await?.iter().map(to_dto).collect(),
    ))
}

// #[get("/")]
// pub async fn author() ->

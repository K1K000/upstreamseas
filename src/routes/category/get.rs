use crate::routes::category::dto::{CategoryResponse, to_dto};
use crate::{entities::prelude::Category, error_handling::ErrorResponder};
use rocket::{State, get, serde::json::Json};
use sea_orm::{DatabaseConnection, EntityTrait};

#[get("/")]
pub async fn all(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<CategoryResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Category::find().all(db).await?.iter().map(to_dto).collect(),
    ))
}

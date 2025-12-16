use crate::routes::borrow::dto::*;
use crate::{entities::prelude::Borrow, error_handling::ErrorResponder};
use rocket::{State, get, serde::json::Json};
use sea_orm::{DatabaseConnection, EntityTrait};

#[get("/")]
pub async fn all(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<BorrowResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Borrow::find()
            .all(db)
            .await?
            .iter()
            .map(borrow_to_dto)
            .collect(),
    ))
}

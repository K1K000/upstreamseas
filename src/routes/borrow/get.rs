use crate::error_handling::ErrorMessage;
use crate::routes::borrow::dto::*;
use crate::{entities::prelude::Borrow, error_handling::ErrorResponder};
use rocket::{State, get, serde::json::Json};
use sea_orm::{DatabaseConnection, EntityTrait, QuerySelect};

#[get("/id/<id>")]
pub async fn id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<BorrowResponse>, ErrorResponder> {
    let db = db.inner();
    match Borrow::find_by_id(id).one(db).await? {
        Some(val) => Ok(Json(to_dto(&val))),
        None => Err(ErrorResponder::InternalError(Json(ErrorMessage {
            message: String::from("This id doesn't exist"),
        }))),
    }
}

const DEFAULT_LIMIT: u64 = 20;
#[get("/<n>")]
pub async fn all(
    db: &State<DatabaseConnection>,
    n: Option<u64>,
) -> Result<Json<Vec<BorrowResponse>>, ErrorResponder> {
    let db = db.inner();
    let n = n.unwrap_or(DEFAULT_LIMIT);
    Ok(Json(
        Borrow::find()
            .limit(n)
            .all(db)
            .await?
            .iter()
            .map(to_dto)
            .collect(),
    ))
}

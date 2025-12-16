use crate::entities::prelude::Borrow;
use crate::error_handling::*;
use rocket::http::Status;
use rocket::*;
use sea_orm::*;

#[delete("/<id>")]
pub async fn by_id(db: &State<DatabaseConnection>, id: i32) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match Borrow::delete_by_id(id).exec(db).await? {
        DeleteResult { rows_affected: 1 } => Ok(Status::NoContent),
        _ => Err(ErrorResponder::NotFound(())),
    }
}

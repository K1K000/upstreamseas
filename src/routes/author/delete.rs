use crate::entities::prelude::Author;
use crate::error_handling::*;
use rocket::http::Status;
use rocket::{State, delete};
use sea_orm::*;

#[delete("/<id>")]
pub async fn by_id(db: &State<DatabaseConnection>, id: i32) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    let mut author = Author::find_by_id(id)
        .one(db)
        .await?
        .ok_or(ErrorResponder::NotFound(()))?
        .into_active_model();

    author.deleted = Set(true);
    Author::update(author).exec(db).await?;
    Ok(Status::NoContent)
}

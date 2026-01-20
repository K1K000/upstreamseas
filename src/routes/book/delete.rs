use crate::entities::prelude::Book;
use crate::error_handling::*;
use rocket::http::Status;
use rocket::*;
use sea_orm::*;

#[delete("/<id>")]
pub async fn by_id(db: &State<DatabaseConnection>, id: i32) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    let mut boook = Book::find_by_id(id)
        .one(db)
        .await?
        .ok_or(ErrorResponder::NotFound(()))?
        .into_active_model();

    boook.deleted = Set(true);
    Book::update(boook).exec(db).await?;
    Ok(Status::NoContent)
}

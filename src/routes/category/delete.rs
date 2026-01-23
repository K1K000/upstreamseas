use crate::entities::prelude::Category;
use crate::error_handling::*;
use rocket::http::Status;
use rocket::*;
use sea_orm::*;

#[delete("/<id>")]
pub async fn by_id(db: &State<DatabaseConnection>, id: i32) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    let mut category = Category::find_by_id(id)
        .one(db)
        .await?
        .ok_or(ErrorResponder::NotFound(()))?
        .into_active_model();

    category.active = Set(false);
    Category::update(category).exec(db).await?;
    Ok(Status::NoContent)
}

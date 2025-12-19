use chrono::Utc;
use rocket::{State, post, response::status::Created, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::author, error_handling::ErrorResponder, routes::author::dto::*};

#[post("/", format = "json", data = "<data>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    data: Json<AuthorCreate>,
) -> Result<Created<Json<AuthorResponse>>, ErrorResponder> {
    let db = db.inner();

    let author = author::ActiveModel::builder()
        .set_name(&data.name)
        .set_birthplace(&data.birthplace)
        .set_birthdate(Utc::now().date_naive())
        .insert(db)
        .await?;

    Ok(Created::new("/author").body(Json(AuthorResponse {
        id: author.id,
        name: author.name,
        birthplace: author.birthplace,
        birthdate: author.birthdate,
    })))
}

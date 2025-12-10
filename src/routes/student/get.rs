use crate::routes::student::dto::*;
use crate::{
    entities::prelude::Student, error_handling::ErrorResponder,
    routes::student::dto::StudentResponse,
};
use rocket::{State, get, serde::json::Json};
use sea_orm::{DatabaseConnection, EntityTrait};

#[get("/")]
pub async fn all(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<StudentResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Student::find()
            .all(db)
            .await?
            .iter()
            .map(student_to_dto)
            .collect(),
    ))
}

use crate::entities::prelude::Student;
use crate::error_handling::*;
use rocket::http::Status;
use rocket::*;
use sea_orm::*;

// TODO:n't make this work
#[delete("/<id>")]
pub async fn by_id(db: &State<DatabaseConnection>, id: i32) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match Student::delete_by_id(id).exec(db).await? {
        DeleteResult { rows_affected: 1 } => Ok(Status::NoContent),
        _ => Err(ErrorResponder::NotFound(())),
    }
}

//all kinda stumped have no idea how to implement
// #[delete("/")]
// pub async fn all(db: &State<DatabaseConnection>) -> Result<Status, ErrorResponder> {
//     let db = db.inner();
//     Student::delete_many()
//         .filter(student::Column::Id.)
//         .exec(db)
//         .await?;
//     Ok(Status::NoContent)
// }

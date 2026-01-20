use crate::entities::borrow;
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
// let book_model = match Borrow::find_by_id(id).one(db.inner()).await? {
//     Some(val) => val,
//     None => {
//         return Err(ErrorResponder::InternalError(Json(ErrorMessage {
//             message: String::from(
//                 "yeah you shouldnt see this error, i have no idea why is this here",
//             ),
//         })));
//     }
// };
// book_handling(book_model.id, db, Flow::GiveBack).await?;
// let db = db.inner();
//
// match Borrow::find_by_id(id).one(db).await? {
//     Some(val) => {
//         let model = borrow::ActiveModel {
//             id: sea_orm::ActiveValue::set(id),
//             book_id: Set(val.book_id),
//             student_id: Set(val.student_id),
//             date: Set(val.date),
//             limit: Set(val.limit),
//             active: Set(false),
//         };
//         Borrow::update(model).exec(db).await?;
//         Ok(Status::NoContent)
//     }
//     None => Err(ErrorResponder::NotFound(())),
// }

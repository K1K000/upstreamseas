use crate::entities::prelude::Ticket;
use crate::routes::ticket::dto::TicketUpdate;
use crate::{entities::ticket, error_handling::ErrorResponder, routes::ticket::dto::TicketCreate};
use rocket::{State, http::Status, put, serde::json::Json};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};

#[put("/<id>", data = "<data>", format = "json")]
pub async fn put(
    id: i32,
    data: Json<TicketUpdate>,
    db: &State<DatabaseConnection>,
) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match Ticket::find_by_id(id).one(db).await? {
        Some(_val) => {
            let model = ticket::ActiveModel {
                id: Set(id),
                student_id: Set(data.student_id),
                creation_date: Set(data.creation_date),
                end_date: Set(data.creation_date),
            };
            Ticket::update(model).exec(db).await?;
            Ok(Status::NoContent)
        }
        None => Err(ErrorResponder::NotFound(())),
    }
}

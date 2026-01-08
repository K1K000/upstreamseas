use chrono::Utc;
use rocket::{State, post, response::status::Created, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{
    entities::ticket,
    error_handling::ErrorResponder,
    routes::ticket::dto::{TicketCreate, TicketResponse},
};

#[post("/", format = "json", data = "<data>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    data: Json<TicketCreate>,
) -> Result<Created<Json<TicketResponse>>, ErrorResponder> {
    let db = db.inner();
    let ticket = ticket::ActiveModel::builder()
        .set_student_id(data.student_id)
        .set_creation_date(Utc::now().date_naive())
        .set_end_date(data.end_date)
        .insert(db)
        .await?;

    Ok(Created::new("/ticket").body(Json(TicketResponse {
        id: ticket.id,
        student_id: ticket.student_id,
        creation_date: ticket.creation_date,
        end_date: ticket.end_date,
    })))
}

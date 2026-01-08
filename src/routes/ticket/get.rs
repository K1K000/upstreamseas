use crate::entities::prelude::Ticket;
use crate::entities::ticket;
use crate::error_handling::ErrorResponder;
use crate::routes::ticket::dto::{TicketResponse, ticket_to_dto};
use rocket::{State, get, serde::json::Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[get("/")]
pub async fn all(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<TicketResponse>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        Ticket::find()
            .all(db)
            .await?
            .iter()
            .map(ticket_to_dto)
            .collect(),
    ))
}

#[get("/<student_id>")]
pub async fn student_ticket(
    db: &State<DatabaseConnection>,
    student_id: u64,
) -> Result<Json<TicketResponse>, ErrorResponder> {
    let db = db.inner();
    let ticket = match Ticket::find()
        .filter(ticket::Column::StudentId.eq(student_id))
        .one(db)
        .await?
    {
        Some(val) => val,
        None => return Err(ErrorResponder::NotFound(())),
    };

    Ok(Json(ticket_to_dto(&ticket)))
}

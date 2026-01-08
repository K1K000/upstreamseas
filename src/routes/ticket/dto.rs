use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::entities::ticket;

// pub fn student_to_dto(student: &student::Model) -> StudentResponse {
//     StudentResponse {
//         id: student.id,
//         name: student.name.clone(),
//         email: student.email.clone(),
//         has_card: student.has_card,
//     }
// }
pub fn ticket_to_dto(ticket: &ticket::Model) -> TicketResponse {
    TicketResponse {
        id: ticket.id,
        student_id: ticket.student_id,
        creation_date: ticket.creation_date,
        end_date: ticket.end_date,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketCreate {
    pub student_id: i32,
    pub creation_date: NaiveDate,
    pub end_date: NaiveDate,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TicketResponse {
    pub id: i32,
    pub student_id: i32,
    pub creation_date: NaiveDate,
    pub end_date: NaiveDate,
}

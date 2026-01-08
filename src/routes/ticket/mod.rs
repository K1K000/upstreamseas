use rocket::{Build, Rocket, routes};

use crate::mounter::Mounter;

pub mod delete;
pub mod dto;
pub mod get;
pub mod post;
pub mod put;
//
pub struct TicketMounter;

impl Mounter for TicketMounter {
    fn mount(r: Rocket<Build>) -> Rocket<Build> {
        r.mount(
            "/ticket",
            routes![
                get::all,
                post::single,
                delete::by_id,
                put::put,
                get::student_ticket
            ],
        )
    }
}

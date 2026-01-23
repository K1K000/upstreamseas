use rocket::{Build, Rocket, routes};

use crate::mounter::Mounter;

pub mod business;
pub mod delete;
pub mod dto;
pub mod get;
pub mod post;
pub mod put;

pub struct BorrowMounter;

impl Mounter for BorrowMounter {
    fn mount(r: Rocket<Build>) -> Rocket<Build> {
        r.mount(
            "/borrow",
            routes![get::id, get::all, post::single, delete::by_id, put::put],
        )
    }
}

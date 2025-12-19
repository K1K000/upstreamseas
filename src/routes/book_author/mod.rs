use rocket::{Build, Rocket, routes};

use crate::mounter::Mounter;

pub mod delete;
pub mod dto;
pub mod get;
pub mod post;
pub mod put;
//
pub struct BookAuthorMounter;

impl Mounter for BookAuthorMounter {
    fn mount(r: Rocket<Build>) -> Rocket<Build> {
        r.mount(
            "/book_author",
            routes![get::all, post::single, delete::by_id, put::put],
        )
    }
}

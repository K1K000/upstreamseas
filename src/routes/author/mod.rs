use rocket::{Build, Rocket, routes};

use crate::mounter::Mounter;

pub mod delete;
pub mod dto;
pub mod get;
pub mod post;
pub mod put;
//
pub struct AuthorMounter;

impl Mounter for AuthorMounter {
    fn mount(r: Rocket<Build>) -> Rocket<Build> {
        r.mount(
            "/author",
            routes![
                // get::all,
                get::limit,
                get::authors_books,
                get::id,
                post::single,
                delete::by_id,
                put::put
            ],
        )
    }
}

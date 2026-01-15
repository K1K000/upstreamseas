use rocket::{Build, Rocket, routes};

use crate::mounter::Mounter;

pub mod delete;
pub mod dto;
pub mod get;
pub mod post;
pub mod put;
//
pub struct BookCategoryMounter;

impl Mounter for BookCategoryMounter {
    fn mount(r: Rocket<Build>) -> Rocket<Build> {
        r.mount(
            "/book_category",
            routes![get::all, get::limit, post::single, delete::by_id, put::put],
        )
    }
}

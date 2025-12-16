use rocket::{Build, Rocket, routes};

use crate::mounter::Mounter;

pub mod delete;
pub mod dto;
pub mod get;
pub mod post;
pub mod put;
//
pub struct BookMounter;

impl Mounter for BookMounter {
    fn mount(r: Rocket<Build>) -> Rocket<Build> {
        r.mount(
            "/book",
            routes![get::all, post::single, delete::by_id, put::put],
        )
    }
}

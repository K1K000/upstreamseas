use rocket::{Build, Rocket, routes};

use crate::mounter::Mounter;

pub mod delete;
pub mod dto;
pub mod get;
pub mod post;
pub mod put;
//
pub struct StudentMounter;

impl Mounter for StudentMounter {
    fn mount(r: Rocket<Build>) -> Rocket<Build> {
        r.mount(
            "/student",
            routes![
                get::all,
                get::id,
                get::top,
                post::single,
                delete::by_id,
                put::put,
                get::student_books,
                get::student_all_books
            ],
        )
    }
}

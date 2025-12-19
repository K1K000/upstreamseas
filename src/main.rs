mod entities;
mod error_handling;
mod mounter;
mod routes;
use crate::mounter::RocketMount;
use crate::routes::author::AuthorMounter;
use crate::routes::book::BookMounter;
use crate::routes::book_author::BookAuthorMounter;
use crate::routes::borrow::BorrowMounter;
use crate::routes::student::StudentMounter;
use sea_orm::Database;

#[rocket::launch]
async fn lauch() -> _ {
    // let db = Database::connect("sqlite:./sqlite.db?mode=rwc")
    let db = Database::connect("sqlite::memory:").await.unwrap();
    // synchronizes database schema with entity definitions
    db.get_schema_registry("upstreamseas::entities::*")
        .sync(&db)
        .await
        .unwrap();
    rocket::build()
        .manage(db)
        .mount_route::<StudentMounter>()
        .mount_route::<BookMounter>()
        .mount_route::<BorrowMounter>()
        .mount_route::<AuthorMounter>()
        .mount_route::<BookAuthorMounter>()
}
//
// #[get("/")]
// async fn greet() -> &'static str {
//     return "hello everynyan";
// }

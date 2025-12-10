use rocket::{State, post, response::status::Created, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{
    entities::student,
    error_handling::ErrorResponder,
    routes::student::dto::{StudentCreate, StudentResponse},
};

#[post("/", format = "json", data = "<data>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    data: Json<StudentCreate>,
) -> Result<Created<Json<StudentResponse>>, ErrorResponder> {
    let db = db.inner();
    let student = student::ActiveModel::builder()
        .set_name(data.name.clone())
        .set_email(data.email.clone())
        .insert(db)
        .await?;
    // .add_book()
    // let book = book::Entity::find_by_id(books[0]).one(db).await?;
    // for book in data.book_ids.iter() {
    //     let book = book::Entity::find_by_id(books[0]).one(db);
    //     student.add_book(book).await;
    // }

    // let student = student.insert(db).await?;

    Ok(Created::new("/student").body(Json(StudentResponse {
        id: student.id,
        name: student.name,
        email: student.email,
    })))
}

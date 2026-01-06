use crate::entities::book::Model;
use crate::entities::{book, borrow, prelude::*};
use crate::error_handling::ErrorResponder;
use crate::routes::book::dto::*;
use rocket::{State, get, serde::json::Json};
use sea_orm::{DatabaseConnection, EntityTrait, LoaderTrait, Order, QueryOrder, QuerySelect};
use sqlx::query;

#[get("/top/<n>")]
pub async fn top(
    db: &State<DatabaseConnection>,
    n: u64,
) -> Result<Json<Vec<BookResponse>>, ErrorResponder> {
    let db = db.inner();

    let books = Book::find().all(db).await?;
    let mut borrows = books.load_many(Borrow, db).await?;

    borrows.sort_by_key(|a| a.len());
    borrows.reverse();
    let mut sorted_books: Vec<BookResponse> = vec![];

    for borrow in borrows.iter().filter(|val| val.iter().len() > 0) {
        // RUNTIME ERROR !!!
        sorted_books.push(book_to_dto(
            &books
                .iter()
                .filter(|a| a.id == borrow[0].book_id)
                .collect::<Vec<_>>()[0]
                .clone(),
        ));
    }
    dbg!(&sorted_books);
    dbg!(&borrows);

    todo!();
}

#[get("/<order>?<n>&<key>")]
pub async fn filter(
    db: &State<DatabaseConnection>,
    order: Option<&str>,
    n: u64,
    key: Option<&str>,
) -> Result<Json<Vec<BookResponse>>, ErrorResponder> {
    let db = db.inner();

    let ord = match order {
        Some("desc") => Order::Desc,
        _ => Order::Asc,
        // Some("asc") | _ => Order::Asc, pretty but has warn
    };

    let key = match key {
        Some("available") => book::Column::Available,
        Some("name") => book::Column::Name,
        _ => book::Column::Id,
    };

    Ok(Json(
        Book::find()
            .order_by(key, ord)
            .limit(n)
            .all(db)
            .await?
            .iter()
            .map(book_to_dto)
            .collect(),
    ))
}

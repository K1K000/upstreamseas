use crate::entities::{book, borrow, prelude::*};
use crate::error_handling::ErrorResponder;
use crate::routes::book::dto::*;
use rocket::{State, get, serde::json::Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryOrder, QuerySelect};

#[get("/top/<n>")]
pub async fn top(
    db: &State<DatabaseConnection>,
    n: u64,
) -> Result<Json<Vec<BookResponse>>, ErrorResponder> {
    let db = db.inner();

    Ok(Json(
        Book::find()
            .left_join(Borrow)
            .group_by(borrow::Column::BookId)
            .order_by(borrow::Column::BookId.count(), Order::Desc)
            .limit(n)
            .all(db)
            .await?
            .iter()
            .map(book_to_dto)
            .collect::<Vec<_>>(),
    ))
}
// TODO: currently borrowed




const DEFAULT_LIMIT: u64 = 20;
#[get("/<order>?<key>&<n>")]
pub async fn filter(
    db: &State<DatabaseConnection>,
    order: Option<&str>,
    n: Option<u64>,
    key: Option<&str>,
) -> Result<Json<Vec<BookResponse>>, ErrorResponder> {
    let db = db.inner();
    n.unwrap_or(DEFAULT_LIMIT);

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

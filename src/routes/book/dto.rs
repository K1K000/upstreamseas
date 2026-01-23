use crate::entities::{author, book};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct BookResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub available: u32,
    pub all_available: u32,
    pub release: NaiveDate,
    pub max_borrow: u32,
    pub deleted: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BookUpdate {
    pub name: String,
    pub description: String,
    pub available: u32,
    pub all_available: u32,
    pub release: NaiveDate,
    pub max_borrow: u32,
    pub deleted: bool,
}

#[derive(Deserialize)]
pub struct BookCreate {
    pub name: String,
    pub all_available: u32,
    pub description: String,
    pub release: NaiveDate,
    pub max_borrow: u32,
}

pub fn book_to_dto(book: &book::Model) -> BookResponse {
    BookResponse {
        id: book.id,
        name: book.name.clone(),
        description: book.description.clone(),
        available: book.available,
        all_available: book.all_available,
        release: book.release,
        max_borrow: book.max_borrow,
        deleted: book.deleted,
    }
}

#[derive(Debug, Serialize)]
pub struct MiniAuthor {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct BookAuthStat {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub available: u32,
    pub all_available: u32,
    pub release: NaiveDate,
    pub max_borrow: u32,
    pub deleted: bool,
    pub authors: Vec<MiniAuthor>,
}

pub fn author_to_mini<I>(authors: I) -> impl Iterator<Item = MiniAuthor>
where
    I: Iterator<Item = author::Model>,
{
    authors.map(|a| MiniAuthor {
        id: a.id,
        name: a.name,
    })
}

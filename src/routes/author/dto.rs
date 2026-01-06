use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::entities::author;

#[derive(Serialize)]
pub struct AuthorResponse {
    pub id: i32,
    pub name: String,
    pub birthplace: String,
    pub birthdate: NaiveDate,
}

#[derive(Deserialize)]
pub struct AuthorCreate {
    pub name: String,
    pub birthplace: String,
    // pub birthdate: NaiveDate,
}

pub fn to_dto(author: &author::Model) -> AuthorResponse {
    AuthorResponse {
        id: author.id,
        name: author.name.clone(),
        birthplace: author.birthplace.clone(),
        birthdate: author.birthdate,
    }
}

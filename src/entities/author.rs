use chrono::NaiveDate;
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "author")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub birthplace: String,
    pub birthdate: NaiveDate,
    pub description: String,
    pub deleted: bool,
    #[sea_orm(has_many, via = "book_author")]
    pub books: Vec<super::book::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

use chrono::NaiveDate;
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "book")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub all_available: u32,
    pub available: u32,
    pub description: String,
    pub release: NaiveDate,
    pub deleted: bool,
    #[sea_orm(has_many)]
    pub borrows: HasMany<super::borrow::Entity>,
    #[sea_orm(has_many)]
    pub book_categories: HasMany<super::book_category::Entity>,
    #[sea_orm(has_many)]
    pub book_author: HasMany<super::book_author::Entity>,
    #[sea_orm(has_many, via = "borrow")]
    pub students: HasMany<super::student::Entity>,
    #[sea_orm(has_many, via = "book_author")]
    pub author: HasMany<super::author::Entity>,
    #[sea_orm(has_many, via = "book_category")]
    pub categories: HasMany<super::category::Entity>,
    // pub email: String,
    // pub student_id: i32,
    // #[sea_orm(belongs_to, from = "student_id", to = "id")]
    // pub student: HasOne<super::student::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

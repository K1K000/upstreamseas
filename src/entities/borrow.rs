use chrono::NaiveDate;
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "borrow")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub date: NaiveDate,
    // #[sea_orm(nullable)]
    pub end: Option<NaiveDate>,
    pub limit: NaiveDate,
    pub book_id: i32,
    #[sea_orm(belongs_to, from = "book_id", to = "id")]
    pub books: HasOne<super::book::Entity>,
    pub student_id: i32,
    #[sea_orm(belongs_to, from = "student_id", to = "id")]
    pub student: HasOne<super::student::Entity>,
}
impl ActiveModelBehavior for ActiveModel {}

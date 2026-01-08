use chrono::NaiveDate;
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "ticket")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub student_id: i32,
    pub creation_date: NaiveDate,
    pub end_date: NaiveDate,
    #[sea_orm(belongs_to, from = "student_id", to = "id")]
    pub student: HasOne<super::student::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

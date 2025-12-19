use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "book")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(has_many, via = "borrow")]
    pub students: HasMany<super::student::Entity>,
    #[sea_orm(has_many, via = "book_author")]
    pub author: HasMany<super::author::Entity>,
    // pub email: String,
    // pub student_id: i32,
    // #[sea_orm(belongs_to, from = "student_id", to = "id")]
    // pub student: HasOne<super::student::Entity>,
}

// #[sea_orm(has_many, via = "cake_filling")]
// pub fillings: Vec<super::filling::Entity>,

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "student")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[sea_orm(has_many, via = "borrow")]
    pub books: Vec<super::book::Entity>,
    #[sea_orm(has_many)]
    pub borrow: Vec<super::borrow::Entity>,
    #[sea_orm(has_one)]
    pub ticket: HasOne<super::ticket::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(has_many, via = "book_category")]
    pub books: Vec<super::book::Entity>,
    #[sea_orm(has_many)]
    pub book_categories: HasMany<super::book_category::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

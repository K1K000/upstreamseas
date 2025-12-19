use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "book_author")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub book_id: i32,
    #[sea_orm(belongs_to, from = "book_id", to = "id")]
    pub book: HasOne<super::book::Entity>,
    pub author_id: i32,
    #[sea_orm(belongs_to, from = "author_id", to = "id")]
    pub author: HasOne<super::author::Entity>,
}
impl ActiveModelBehavior for ActiveModel {}

//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "recipe")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Text")]
    pub cid: String,
    pub ctime: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text", nullable)]
    pub mid: Option<String>,
    pub mtime: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text")]
    pub title: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub header: Option<String>,
    pub ingredients: Option<Vec<String>>,
    pub steps: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    #[sea_orm(column_type = "Text", nullable)]
    pub image_url: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub cook_time: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub prep_time: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub total_time: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::chef::Entity",
        from = "Column::Cid",
        to = "super::chef::Column::FirebaseId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Chef,
}

impl Related<super::chef::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chef.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

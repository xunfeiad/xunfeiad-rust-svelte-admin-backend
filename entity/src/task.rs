//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn schema_name(&self) -> Option<&str> {
        Some("redog")
    }
    fn table_name(&self) -> &str {
        "task"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub task_cate: String,
    pub status: String,
    pub create_by: String,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub is_delete: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    TaskCate,
    Status,
    CreateBy,
    CreateTime,
    UpdateTime,
    IsDelete,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::TaskCate => ColumnType::String(None).def(),
            Self::Status => ColumnType::String(None).def(),
            Self::CreateBy => ColumnType::String(None).def(),
            Self::CreateTime => ColumnType::DateTime.def().null(),
            Self::UpdateTime => ColumnType::DateTime.def().null(),
            Self::IsDelete => ColumnType::Boolean.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

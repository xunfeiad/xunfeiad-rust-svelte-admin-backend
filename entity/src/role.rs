//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};
use async_trait::async_trait;
pub use chrono::NaiveDateTime;
use macro_trait::Responder;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn schema_name(&self) -> Option<&str> {
        Some("redog")
    }
    fn table_name(&self) -> &str {
        "role"
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    DeriveModel,
    DeriveActiveModel,
    Eq,
    Serialize,
    Deserialize,
    Responder,
)]
pub struct Model {
    #[serde(skip_deserializing, skip_serializing)]
    pub id: i32,
    pub role_name: String,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    #[serde(skip)]
    pub is_delete: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    RoleName,
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
            Self::RoleName => ColumnType::String(None).def().unique(),
            Self::CreateTime => ColumnType::DateTime
                .def()
                .default(Expr::current_timestamp()),
            Self::UpdateTime => ColumnType::DateTime
                .def()
                .default(Expr::current_timestamp()),
            Self::IsDelete => ColumnType::Boolean.def().default(false).null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if self.is_delete.as_ref().is_none() {
            self.is_delete = ActiveValue::Set(Some(false))
        }
        Ok(self)
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_role::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_role::Relation::Role.def().rev())
    }
}

impl Related<super::access::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_access::Relation::Access.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::role_access::Relation::Access.def().rev())
    }
}

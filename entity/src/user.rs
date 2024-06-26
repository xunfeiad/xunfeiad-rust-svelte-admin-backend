//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use crate::{role, user, user_role};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};
use async_trait::async_trait;
use macro_trait::Responder;
use pkg::crypt::sha256_hash;
use pkg::get_current_datetime;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::Expr;
use sea_orm::{ActiveValue, LinkDef};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn schema_name(&self) -> Option<&str> {
        Some("redog")
    }
    fn table_name(&self) -> &str {
        "user"
    }
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveModel,
    DeriveActiveModel,
    Eq,
    Validate,
    Serialize,
    Deserialize,
    Responder,
)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[serde(skip_serializing)]
    pub id: Option<i32>,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub nick_name: Option<String>,
    #[validate(url)]
    pub avatar: Option<String>,
    #[validate(length(min = 8))]
    pub mobile: String,
    #[validate(email)]
    pub email: String,
    pub summary: Option<String>,
    #[serde(skip)]
    pub create_time: Option<DateTime>,
    #[serde(skip)]
    pub update_time: Option<DateTime>,
    #[serde(skip)]
    pub is_delete: Option<bool>,
}

impl Model {
    pub fn check_model(&self) -> bool {
        match self.validate() {
            Ok(_) => true,
            Err(e) => false,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Username,
    Password,
    NickName,
    Avatar,
    Mobile,
    Email,
    Summary,
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
            Self::Username => ColumnType::String(None).def().unique(),
            Self::Password => ColumnType::String(None).def(),
            Self::NickName => ColumnType::String(None).def().null(),
            Self::Avatar => ColumnType::String(None).def().null(),
            Self::Mobile => ColumnType::String(None).def().unique(),
            Self::Email => ColumnType::String(None).def().unique(),
            Self::Summary => ColumnType::String(None).def().null(),
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

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert == true {
            self.create_time = ActiveValue::Set(Some(get_current_datetime()));
            self.update_time = ActiveValue::Set(Some(get_current_datetime()));
            self.is_delete = ActiveValue::Set(Some(false))
        } else {
            self.update_time = ActiveValue::Set(Some(get_current_datetime()));
            if self.is_delete.is_not_set() {
                self.is_delete = ActiveValue::Set(Some(false));
            } else {
                self.is_delete = ActiveValue::Set(Some(true));
            }
        }
        self.password = ActiveValue::Set(sha256_hash(self.password.as_ref().clone()));
        Ok(self)
    }
}

impl Related<role::Entity> for Entity {
    // The final relation is Cake -> CakeFilling -> Filling
    fn to() -> RelationDef {
        user_role::Relation::Role.def()
    }

    fn via() -> Option<RelationDef> {
        // The original relation is CakeFilling -> Cake,
        // after `rev` it becomes Cake -> CakeFilling
        Some(user_role::Relation::User.def().rev())
    }
}

pub struct UserControl;

impl Linked for UserControl {
    type FromEntity = Entity;
    type ToEntity = role::Entity;

    fn link(&self) -> Vec<LinkDef> {
        vec![
            user_role::Relation::User
                .def()
                // .on_condition(|l, _r| Expr::tbl(l, Column::IsDelete.eq(false)))
                .rev(),
            user_role::Relation::Role.def(),
        ]
    }
}

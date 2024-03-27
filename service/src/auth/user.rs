use crate::WebResult;
use entity::user::{ActiveModel, Column, Entity as User, LoginUser, Model, UserControl};
use migration::{
    m20240310_171244_user::User as UserTable, m20240310_174955_role::Role as RoleTable,
    m20240310_180627_user_role::UserRole as UserRoleTable,
};
use pkg::{
    crypt::{jwt_encrypt, sha256_hash, validate_jwt},
    WebError,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, Linked,
    LoaderTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
    SelectColumns,
};
use sea_query::{Expr, Query};
use serde::{Deserialize, Serialize};

pub struct Service;

impl Service {
    pub async fn create(db: &DatabaseConnection, model: Model) -> WebResult<()> {
        if !model.check_model() {
            return Err(WebError::InvalidateDataError {
                msg: "Invalidate model.".to_string(),
            });
        };
        ActiveModel {
            username: ActiveValue::Set(model.username),
            password: ActiveValue::Set(model.password),
            nick_name: ActiveValue::Set(model.nick_name),
            avatar: ActiveValue::Set(model.avatar),
            mobile: ActiveValue::Set(model.mobile),
            email: ActiveValue::Set(model.email),
            summary: ActiveValue::Set(model.summary),
            ..Default::default()
        }
        .save(db)
        .await?;
        Ok(())
    }

    pub async fn get_multi(
        db: &DatabaseConnection,
        page: u64,
        page_size: u64,
    ) -> WebResult<Vec<Model>> {
        let user = User::find()
            .order_by_asc(Column::Id)
            .paginate(db, page_size)
            .fetch_page(page - 1)
            .await?;

        Ok(user)
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> WebResult<Model> {
        let user = User::find_by_id(id)
            .filter(Column::IsDelete.eq(false))
            .one(db)
            .await?
            .ok_or("User not found.".to_owned())
            .unwrap();
        Ok(user)
    }

    pub async fn update(db: &DatabaseConnection, id: i32, model: Model) -> WebResult<Model> {
        let user: ActiveModel = Self::get_by_id(db, id).await.map(Into::into)?;
        let user: Model = ActiveModel {
            id: user.id,
            username: ActiveValue::Set(model.username),
            password: ActiveValue::Set(model.password),
            nick_name: ActiveValue::Set(model.nick_name),
            avatar: ActiveValue::Set(model.avatar),
            mobile: ActiveValue::Set(model.mobile),
            email: ActiveValue::Set(model.email),
            summary: ActiveValue::Set(model.summary),
            ..Default::default()
        }
        .update(db)
        .await?;
        Ok(user)
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> WebResult<()> {
        let mut user: ActiveModel = Self::get_by_id(db, id).await.map(Into::into)?;
        user.is_delete = ActiveValue::Set(Some(true));
        user.save(db).await?;
        Ok(())
    }

    pub async fn validate_jwt(db: &DatabaseConnection, token: &str) -> WebResult<()> {
        let id = validate_jwt(token)?;
        let model = User::find_by_id(id as i32).one(db).await?;
        match model {
            Some(_) => Ok(()),
            None => Err(WebError::InvalidateJWTError {
                msg: "Invalidate jwt.".to_owned(),
            }),
        }
    }

    pub async fn login(
        db: &DatabaseConnection,
        login_user: LoginUser,
    ) -> WebResult<(String, Model)> {
        let model = User::find()
            .filter(Column::Username.eq(login_user.username))
            .one(db)
            .await?;
        match model {
            Some(user) => {
                if sha256_hash(login_user.password).ne(&user.password) {
                    Err(WebError::InternalError {
                        msg: "Incorrect username or password.".to_string(),
                    })
                } else {
                    let token = jwt_encrypt(user.id.unwrap() as usize)?;
                    Ok((token, user))
                }
            }
            None => Err(WebError::InternalError {
                msg: "Incorrect username or password.".to_string(),
            }),
        }
    }
    //
    // pub async fn get_role_by_id(db: &DatabaseConnection, id: i32) -> WebResult<Option<UserRoles>> {
    //     let query = Query::select()
    //         .columns([UserTable::Id,UserTable::Username,UserTable::NickName,UserTable::Avatar])
    //         .columns([RoleTable::Id,RoleTable::RoleName])
    //         .from(UserTable::Table)
    //         .from(RoleTable::Table)
    //         .from(UserRoleTable::Table)
    //         .left_join(UserRoleTable::Table, Expr::col(()))
    // }
}

#[derive(Deserialize, Default, Debug, Serialize)]
pub struct UserRoles {
    user_id: i32,
    username: String,
    roles: Vec<Role>,
}

#[derive(Deserialize, Default, Debug, Serialize)]
pub struct Role {
    role_id: i32,
    role_name: String,
}

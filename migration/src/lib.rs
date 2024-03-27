pub use sea_orm_migration::prelude::*;

pub mod m20240310_171244_user;
pub mod m20240310_174955_role;
pub mod m20240310_175000_access;
mod m20240310_175005_domain;
pub mod m20240310_180627_user_role;
mod m20240310_182026_role_access;
mod m20240310_182114_role_domain;
mod m20240310_184559_message;
mod m20240310_184603_group;
mod m20240310_184642_user_group;
mod m20240310_184750_task;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240310_171244_user::Migration),
            Box::new(m20240310_174955_role::Migration),
            Box::new(m20240310_175000_access::Migration),
            Box::new(m20240310_175005_domain::Migration),
            Box::new(m20240310_180627_user_role::Migration),
            Box::new(m20240310_182026_role_access::Migration),
            Box::new(m20240310_182114_role_domain::Migration),
            Box::new(m20240310_184559_message::Migration),
            Box::new(m20240310_184603_group::Migration),
            Box::new(m20240310_184642_user_group::Migration),
            Box::new(m20240310_184750_task::Migration),
        ]
    }
}

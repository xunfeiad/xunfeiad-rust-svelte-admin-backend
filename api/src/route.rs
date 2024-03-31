use actix_web::web;
use std::rc::Rc;

pub fn scoped_user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth/user")
            .service(crate::auth::user::create)
            .service(crate::auth::user::update)
            .service(crate::auth::user::get_multi)
            .service(crate::auth::user::get_by_name)
            .service(crate::auth::user::get_by_id)
            .service(crate::auth::user::delete)
            .service(crate::auth::user::login),
    )
    .service(
        web::scope("/auth/access")
            .service(crate::auth::access::create)
            .service(crate::auth::access::update)
            .service(crate::auth::access::get_multi)
            .service(crate::auth::access::get_by_id)
            .service(crate::auth::access::delete),
    )
    .service(
        web::scope("/auth/role")
            .service(crate::auth::role::create)
            .service(crate::auth::role::update)
            .service(crate::auth::role::get_by_name)
            .service(crate::auth::role::get_multi)
            .service(crate::auth::role::get_by_id)
            .service(crate::auth::role::delete),
    )
    .service(
        web::scope("/auth/domain")
            .service(crate::auth::domain::create)
            .service(crate::auth::domain::update)
            .service(crate::auth::domain::get_multi)
            .service(crate::auth::domain::get_by_id)
            .service(crate::auth::domain::delete),
    )
    .service(
        web::scope("/auth/user_role")
            .service(crate::auth::user_role::get_multi)
            .service(crate::auth::user_role::update_user_role),
    );
}

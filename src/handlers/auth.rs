use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie, SameSite},
    http::header::ContentType,
    Responder,
};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use serde_json::json;
use tera::Tera;

use crate::{
    auth::jwt_middleware::JwtMiddleware,
    config::ApplicationConfiguration,
    db::connection::{get_db_connection_from_pool, SqliteConnectionPool},
    models::user::User,
    utils::{password_helper::verify_hashes, token_helper::create_jwt_token},
    viewmodels::login::LoginVM,
};

#[derive(Deserialize)]
pub struct ReturnPath {
    pub return_url: String,
}

pub async fn login_get(
    tera: web::Data<Tera>,
    return_path: web::Query<ReturnPath>,
) -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("return_url", &return_path.return_url);
    let rendered = match tera.render("auth/login.html", &context) {
        Ok(t) => t,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Something went wrong: {}", e))
        }
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered)
}

pub async fn login_post(
    db_pool: web::Data<SqliteConnectionPool>,
    app_config: web::Data<ApplicationConfiguration>,
    login_vm: web::Form<LoginVM>,
    return_path: web::Query<ReturnPath>,
) -> impl Responder {
    use crate::schema::users::dsl::*;

    let requested_user: User = match users
        .filter(username.eq(login_vm.username.to_owned()))
        .select(User::as_select())
        .first(&mut get_db_connection_from_pool(&db_pool).unwrap())
        .optional()
    {
        Ok(user) => match user {
            Some(u) => u,
            None => {
                return HttpResponse::BadRequest()
                    .body("user not exist: Invalid username or password")
            }
        },
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Ops! something went wrong: {}", e))
        }
    };
    if !verify_hashes(requested_user.password, login_vm.password.to_owned()) {
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid email or password"}));
    }

    //create the jwt token
    let token = create_jwt_token(requested_user.id, app_config.into_inner());
    /* since app_config is available through web::Data<T>, I have no idea how
     * to extract T from it. Using into_inner() method converts it to Arc<T>.
     * So whatever works
     */

    //set a cookie
    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .same_site(SameSite::Lax)
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    println!("The return url is {}", return_path.return_url);

    if return_path.return_url.is_empty() {
        return HttpResponse::SeeOther()
            .append_header((actix_web::http::header::LOCATION, "/home"))
            .cookie(cookie)
            .finish();
    }

    HttpResponse::SeeOther()
        .append_header((
            actix_web::http::header::LOCATION,
            return_path.return_url.to_owned(),
        ))
        .cookie(cookie)
        .finish()
}

pub async fn logout(_: JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}

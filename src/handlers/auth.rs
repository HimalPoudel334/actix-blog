use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie, SameSite},
    http::header::ContentType,
    web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use diesel::prelude::*;
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

pub async fn login_get(tera: web::Data<Tera>) -> impl Responder {
    let context = tera::Context::new();
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
    req: HttpRequest,
    db_pool: web::Data<SqliteConnectionPool>,
    app_config: web::Data<ApplicationConfiguration>,
    login_vm: web::Form<LoginVM>,
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

    //check for return_url in request's extension
    let return_url: String = req
        .extensions()
        .get::<String>()
        .unwrap_or(&String::from(""))
        .to_owned();

    if return_url.is_empty() {
        return HttpResponse::Ok()
            .append_header((actix_web::http::header::LOCATION, "/home"))
            .cookie(cookie)
            .finish();
    }

    HttpResponse::Ok()
        .append_header((actix_web::http::header::LOCATION, return_url))
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

use actix_web::web;

use crate::handlers::{auth, home, posts, users};

pub fn app_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(home::index))
            .route("privacy", web::post().to(home::privacy)),
    )
    .service(
        web::scope("/auth")
            .route("/login", web::get().to(auth::login_get))
            .route("/login", web::post().to(auth::login_post))
            .route("/logout", web::get().to(auth::logout)),
    )
    .service(
        web::scope("/users")
            .route("", web::get().to(users::index))
            .route("/create", web::get().to(users::create_user_get))
            .route("/create", web::post().to(users::create_user_post))
            .route("/profile/{user_id}", web::get().to(users::user_profile_get)),
    )
    .service(
        web::scope("/posts")
            .route("", web::get().to(posts::index))
            .route("/create", web::get().to(posts::create_post_get))
            .route("/create", web::post().to(posts::create_post_post)),
    );
}

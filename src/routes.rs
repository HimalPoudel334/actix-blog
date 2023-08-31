use actix_web::web;

use crate::handlers::{auth, home, posts, users};

pub fn app_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
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
            .route("/profile/edit", web::get().to(users::edit_profile_get))
            .route("/profile/edit", web::post().to(users::edit_profile_post))
            .route("/profile/{user_id}", web::get().to(users::user_profile_get)),
        //this route is at last otherwise it will match /profile/edit as /profile/{user_id}
    )
    .service(
        web::scope("/posts")
            .route("", web::get().to(posts::index))
            .route("/create", web::get().to(posts::create_post_get))
            .route("/create", web::post().to(posts::create_post_post))
            .route("/edit", web::post().to(posts::edit_post_post))
            .route("/edit/{post_id}", web::get().to(posts::edit_post_get)),
    )
    .service(
        web::scope("/home")
            .route("/privacy", web::get().to(home::privacy))
            .route("/set-timezone", web::post().to(home::client_tz_set))
            .route("", web::get().to(home::index)),
    );
}

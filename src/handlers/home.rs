use actix_session::Session;
use actix_web::{http::header::ContentType, web, HttpResponse, Responder};
use diesel::prelude::*;
use tera::{Context, Tera};

use crate::{
    db::connection::{get_db_connection_from_pool, SqliteConnectionPool},
    utils::session_helper::set_client_timezone,
    viewmodels::post::UsersPostsVM,
};

pub async fn index(
    tera: web::Data<Tera>,
    db_pool: web::Data<SqliteConnectionPool>,
) -> impl Responder {
    use crate::schema::posts;
    use crate::schema::posts::dsl::*;
    use crate::schema::users;
    use crate::schema::users::dsl::*;

    //here we want to all the posts of our all users
    let users_posts: Vec<UsersPostsVM> = match posts
        .inner_join(users)
        .select((
            users::id,
            users::username,
            users::profile_image,
            posts::id,
            posts::title,
            posts::content,
            posts::created_on,
            posts::user_id,
        ))
        .load::<UsersPostsVM>(&mut get_db_connection_from_pool(&db_pool).unwrap())
        .optional()
    {
        Ok(user_posts) => match user_posts {
            Some(up) => up,
            None => Vec::<UsersPostsVM>::new(),
        },
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("Ops! something went wrong: {}", err))
        }
    };

    //insert the vec to context
    let mut context: Context = tera::Context::new();
    context.insert("users_posts", &users_posts);
    //render the template
    let rendered = match tera.render("home/index.html", &context) {
        Ok(t) => t,
        Err(e) => {
            println!("Error while loading template in handler: {}", e);
            std::process::exit(1);
        }
    };
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered)
}

pub async fn privacy(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn client_tz_set(client_tz: String, session: Session) -> impl Responder {
    match set_client_timezone(client_tz, session).await {
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Error while setting client timezone: {}", e)),
        Ok(_) => HttpResponse::Ok().finish(),
    }
}

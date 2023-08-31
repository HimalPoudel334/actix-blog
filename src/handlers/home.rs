use actix_session::Session;
use actix_web::{http::header::ContentType, web, HttpResponse, Responder};
use diesel::prelude::*;
use tera::{Context, Tera};

use crate::{
    auth::jwt_middleware::JwtMiddleware,
    db::connection::{get_db_connection_from_pool, SqliteConnectionPool},
    utils::session_helper::set_client_timezone,
    viewmodels::{
        post::UsersPostsVM,
        user::{UserTimeZone, UserVM},
    },
};

pub async fn index(
    tera: web::Data<Tera>,
    db_pool: web::Data<SqliteConnectionPool>,
    auth: JwtMiddleware,
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

    //get the user id of currently logged in user
    let logged_in_user_id: i32 = auth.user_id;

    //get the user from db
    let current_user: UserVM = match users
        .find(logged_in_user_id)
        .select((users::id, users::username, users::profile_image))
        .first(&mut get_db_connection_from_pool(&db_pool).unwrap())
        .optional()
    {
        Ok(usr) => match usr {
            Some(u) => u,
            None => return HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Ops! something went wrong: {}", e))
        }
    };

    //insert the logged in user to context
    let mut context: Context = tera::Context::new();
    context.insert("current_user", &current_user);
    context.insert("users_posts", &users_posts);
    //render the template
    let rendered = match tera.render("home/index.html", &context) {
        Ok(t) => t,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!(
                "Error! something went wrong: {} \nThe data is:\n {}",
                e,
                serde_json::to_string(&users_posts).unwrap()
            ))
        }
    };
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered)
}

pub async fn privacy() -> impl Responder {
    HttpResponse::Ok().body("Hello from privacy")
}

pub async fn client_tz_set(client_tz: web::Json<UserTimeZone>, session: Session) -> impl Responder {
    println!("Hit with timezone: {}", client_tz.timezone);
    match set_client_timezone(client_tz.timezone.to_owned(), session).await {
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Error while setting client timezone: {}", e)),
        Ok(_) => {
            println!("Insertion successfull");
            HttpResponse::Ok().finish()
        }
    }
}

pub async fn error_not_found(tera: web::Data<Tera>) -> impl Responder {
    //render the template
    let context = tera::Context::new();
    let rendered = match tera.render("home/404_not_found.html", &context) {
        Ok(t) => t,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Ops ! something went wrong: {}", e))
        }
    };
    HttpResponse::NotFound()
        .content_type(ContentType::html())
        .body(rendered)
}

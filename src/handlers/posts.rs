use actix_web::{http::header::ContentType, web, HttpResponse, Responder};
use tera::Tera;

use crate::{
    auth::jwt_middleware::JwtMiddleware,
    db::connection::{get_db_connection_from_pool, SqliteConnectionPool},
    models::post::{NewPost, Post},
    viewmodels::post::PostCreateVM,
};

pub async fn index(
    db_pool: web::Data<SqliteConnectionPool>,
    tera: web::Data<Tera>,
) -> impl Responder {
    use crate::schema::posts::dsl::*;
    use diesel::RunQueryDsl;

    //let all posts
    let posts_vec = posts
        .load::<Post>(&mut get_db_connection_from_pool(&db_pool).unwrap())
        .unwrap();

    let mut context = tera::Context::new();
    context.insert("posts", &posts_vec);

    let rendered = match tera.render("post/index.html", &context) {
        Ok(t) => t,
        Err(err) => {
            println!("Error while loading template: {}", err);
            std::process::exit(1);
        }
    };

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered)
}

pub async fn create_post_get(
    tera: web::Data<Tera>,
    _jwt_middleware: JwtMiddleware, // this parameter is for authentication middleware. This is how
                                    // to use if we have used struct instead of function for middleware
) -> impl Responder {
    let context = tera::Context::new();
    let rendered = match tera.render("post/create.html", &context) {
        Ok(t) => t,
        Err(e) => {
            println!("Error while rendering template: {}", e);
            // std::process::exit(1);
            return HttpResponse::InternalServerError().body("Ops! Something went wrong");
        }
    };
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered)
}

pub async fn create_post_post(
    db_pool: web::Data<SqliteConnectionPool>,
    _jwt_middleware: JwtMiddleware,
    post_vm: web::Form<PostCreateVM>,
) -> impl Responder {
    use crate::schema::posts::dsl::*;
    use diesel::RunQueryDsl;

    let logged_in_user_id: i32 = 1; //should get from auth
    let new_post = NewPost::new(
        post_vm.title.clone(),
        post_vm.content.clone(),
        logged_in_user_id,
    );

    let result = diesel::insert_into(posts)
        .values(&new_post)
        .execute(&mut get_db_connection_from_pool(&db_pool).unwrap());

    match result {
        Ok(_) => HttpResponse::Ok().body("Post created successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

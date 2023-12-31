use actix_web::{http::header::{ContentType, LOCATION, HeaderValue}, web, HttpResponse, Responder};
use diesel::prelude::*;
use tera::{Context, Tera};

use crate::{
    auth::jwt_middleware::JwtMiddleware,
    db::connection::{get_db_connection_from_pool, SqliteConnectionPool},
    models::post::{NewPost, Post},
    viewmodels::post::{PostCreateVM, PostVM, UsersPostsVM},
};

pub async fn index(
    db_pool: web::Data<SqliteConnectionPool>,
    tera: web::Data<Tera>,
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
        .order_by(posts::created_on.desc())
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

    //insert the logged in user to context
    let mut context: Context = tera::Context::new();
    context.insert("users_posts", &users_posts);
    //render the template
    let rendered = match tera.render("post/index.html", &context) {
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
    jwt_middleware: JwtMiddleware,
    post_vm: web::Form<PostCreateVM>,
) -> impl Responder {
    use crate::schema::posts::dsl::*;

    //get the user id of currently logged in user
    let logged_in_user_id: i32 = jwt_middleware.user_id;

    let new_post = NewPost::new(
        post_vm.title.clone(),
        post_vm.content.clone(),
        logged_in_user_id,
    );

    let result = diesel::insert_into(posts)
        .values(&new_post)
        .execute(&mut get_db_connection_from_pool(&db_pool).unwrap());
    match result {
        Ok(_) => {            
            let hv: String = format!("/users/profile/{}?view_type=full", logged_in_user_id);
            HttpResponse::SeeOther().append_header((LOCATION, HeaderValue::from_str(&hv).unwrap())).finish()
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

pub async fn edit_post_get(
    jwt_middleware: JwtMiddleware,
    db_pool: web::Data<SqliteConnectionPool>,
    tera: web::Data<Tera>,
    post_id: web::Path<(i32,)>,
) -> impl Responder {
    use crate::schema::posts::dsl::*;
    let (pid,) = post_id.into_inner();

    let uid: i32 = jwt_middleware.user_id;

    let post: Post = match posts
        .filter(id.eq(pid))
        .filter(user_id.eq(uid))
        .select(Post::as_select())
        .first(&mut get_db_connection_from_pool(&db_pool).unwrap())
        .optional()
    {
        Ok(pst) => match pst {
            Some(p) => p,
            None => {
                return HttpResponse::BadRequest()
                    .body("Sala url ma post id dekhe vandai ma arkako post edit garna khojxas")
            }
        },
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Ops! something went wrong: {}", e))
        }
    };

    //create the viewmodel
    let post_vm: PostVM = PostVM::from(&post);

    //set the context var
    let mut context: Context = tera::Context::new();
    context.insert("post_vm", &post_vm);

    //render the template
    let rendered = match tera.render("post/edit.html", &context) {
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

pub async fn edit_post_post(
    jwt_middleware: JwtMiddleware,
    db_pool: web::Data<SqliteConnectionPool>,
    post_vm: web::Form<PostVM>,
) -> impl Responder {
    use crate::schema::posts::dsl::*;

    let uid: i32 = jwt_middleware.user_id;

    //get the user from db
    let post: Post = match posts
        .find(post_vm.id)
        .select(Post::as_select())
        .first(&mut get_db_connection_from_pool(&db_pool).unwrap())
        .optional()
    {
        Ok(pst) => match pst {
            Some(p) => p,
            None => return HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Ops! something went wrong: {}", e))
        }
    };

    if uid != post_vm.user_id || uid != post.user_id {
        return HttpResponse::BadRequest().body("nonsese! trying to edit others data");
    }

    match diesel::update(&post)
        .set((
            title.eq(post_vm.title.to_owned()),
            content.eq(post_vm.content.to_owned()),
        ))
        .execute(&mut get_db_connection_from_pool(&db_pool).unwrap())
    {
        Ok(_) => return HttpResponse::Ok().body("Update successfull"),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Ops! something went wrong while updating: {}", e))
        }
    }
}

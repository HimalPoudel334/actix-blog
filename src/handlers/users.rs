use actix_web::{http::header::ContentType, web, HttpResponse, Responder};
use diesel::prelude::*;
use tera::Tera;

use crate::{
    db::connection::{get_db_connection_from_pool, SqliteConnectionPool},
    models::{
        post::Post,
        user::{NewUser, User},
    },
    utils::password_helper::hash_password,
    viewmodels::{
        post::PostVM,
        user::{UserCreateVM, UserProfileVM, UserVM},
    },
};

pub async fn index(
    db_pool: web::Data<SqliteConnectionPool>,
    tera: web::Data<Tera>,
) -> impl Responder {
    use crate::schema::users::dsl::*;

    let users_vec = users
        .load::<User>(&mut get_db_connection_from_pool(&db_pool).unwrap())
        .unwrap();

    let mut context = tera::Context::new();
    context.insert("users", &users_vec);

    let rendered = match tera.render("users/index.html", &context) {
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

pub async fn create_user_get(tera: web::Data<Tera>) -> impl Responder {
    // let user = user::UserCreateVM {
    //     username: String::from(""),
    //     password: String::from(""),
    //     confirm_password: String::from(""),
    //     profile_img: String::from(""),
    // };
    //
    // //serialize the struct to json
    // let form_json = serde_json::to_string(&user).unwrap();
    //
    //render the form using tera
    let context = tera::Context::new();
    // context.insert("form_json", &form_json);

    //render the html template
    let rendered = match tera.render("user/create.html", &context) {
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

pub async fn create_user_post(
    db_conn_pool: web::Data<SqliteConnectionPool>,
    user_vm: web::Form<UserCreateVM>,
) -> impl Responder {
    use crate::schema::users::dsl::*;

    //check if user already exists
    match users
        .filter(username.eq(user_vm.username.to_owned()))
        .select(username)
        .first::<String>(&mut get_db_connection_from_pool(&db_conn_pool).unwrap())
        .optional()
    {
        Ok(usrnm) => match usrnm {
            Some(_) => return HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "User with that email already exists"}),
        ),
            None => (),
        },
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Ops! something went wrong: {}", e))
        }
    };

    //if username is unique then create a salt and hash the user's password
    let hashed_password: String = hash_password(user_vm.password.to_owned());
    let new_user = NewUser::new(
        user_vm.username.to_owned(),
        hashed_password,
        user_vm.profile_img.to_owned(),
    );

    let result = diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut get_db_connection_from_pool(&db_conn_pool).unwrap());

    match result {
        Ok(_) => HttpResponse::Ok().body("User created successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

pub async fn user_profile_get(
    tera: web::Data<Tera>,
    db_pool: web::Data<SqliteConnectionPool>,
    user_id_path: web::Path<(i32,)>,
) -> impl Responder {
    use crate::schema::users::dsl::*;

    //destructure the user_id_path tuple to user_id
    let (user_id,) = user_id_path.into_inner();

    let user: User = match users
        .find(user_id)
        .select(User::as_select())
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

    // get all the posts of the user
    let user_posts: Vec<PostVM> = match Post::belonging_to(&user)
        .load::<PostVM>(&mut get_db_connection_from_pool(&db_pool).unwrap())
        .optional()
    {
        Ok(posts_vec) => posts_vec.unwrap(),
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error: Something went wrong. : {}", err))
        }
    };

    //create the vm
    let user_profile_vm: UserProfileVM = UserProfileVM {
        user: UserVM::from(&user),
        posts: user_posts,
    };

    //context
    let mut context = tera::Context::new();
    context.insert("user_profile", &user_profile_vm);

    //tera template
    let rendered = match tera.render("user/profile.html", &context) {
        Ok(t) => t,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!(
                "Opssss! something went wong!\n{} {}",
                e,
                serde_json::to_string(&user_profile_vm).unwrap()
            ));
        }
    };
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered)
}
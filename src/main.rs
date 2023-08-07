use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dotenvy::dotenv;
use tera::Tera;

mod auth;
mod config;
mod db;
mod handlers;
mod models;
mod responses;
mod routes;
mod schema;
mod utils;
mod viewmodels;

use crate::db::connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started on localhost:8000");

    //load the environenment variables
    dotenv().ok();

    //create the app config struct
    let app_config = config::ApplicationConfiguration::init();

    //loading the templating engine Tera
    let mut tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Template parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    //register a custom filter for tera to humanize datetime
    tera.register_filter("humanize", utils::tera_tags_helper::humanize_dt_filter);

    //initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    //setting up the sqlite database
    let sqlitedb_pool: connection::SqliteConnectionPool =
        connection::establish_connection(&app_config);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%r %s"))
            .app_data(Data::new(app_config.clone()))
            .app_data(Data::new(tera.clone()))
            .app_data(Data::new(sqlitedb_pool.clone()))
            .configure(routes::app_routes)
            .service(actix_files::Files::new("/static", "./static")) //serve the static files like
                                                                     //css, js and images
    })
    .bind(("localhost", 8000))
    .expect("Could not bind to port 8000")
    .run()
    .await
}

use actix_cors::Cors;
use actix_web::{ middleware::{from_fn, Logger}, web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use utils::app_state::AppState;

pub mod routes;
pub mod utils;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //Setting up the logger
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    dotenv::dotenv().ok();
    env_logger::init();


    //Getting the enviroment variables 
    let address = (*utils::constants::ADDRESS).clone();
    let port = (*utils::constants::PORT).clone();
    let database_url = (*utils::constants::DATABASE_URL).clone();


    //Connecting to the database
    let db = Database::connect(database_url).await.unwrap();
    println!("[FAGIA]   The database connection was sucessfull");

    //Running migrations
    Migrator::up(&db, None).await.unwrap();
    println!("[FAGIA]   The migrations were applied succesfully");

    println!("[FAGIA]   Starting service on: http://{}:{}", address,port);



    HttpServer::new(move || {
        App::new()
            //Loading the connection 
            .app_data(web::Data::new(AppState{ db: db.clone()}))

            // Setting up CORS permisive model
            .wrap(Cors::permissive())
            //Adding the logger
            .wrap(Logger::default())

            //Loading the account configurations
            .configure(routes::account::config)
            //Loading the routes (endpoints) and protecting them with autentification via JWT
            .service(web::scope("")
                .wrap(from_fn(routes::middlewares::auth_middleware::check_auth_middleware))
                .configure(routes::profile::config)
                .configure(routes::aliments::config) 
                .configure(routes::donation::config)
                .configure(routes::beneficiary_donation::config)
            )
        
    })
    .bind((address, port))?
    .run()
    .await
}

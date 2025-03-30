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


    //Connecting to the databaseç
    let db = Database::connect(database_url).await.unwrap();

    //Running migrations
    Migrator::up(&db, None).await.unwrap();


    HttpServer::new(move || {
        App::new()
            //Loading the connection 
            .app_data(web::Data::new(AppState{ db: db.clone()}))

            //Adding the logger
            .wrap(Logger::default())

            //Loading the account configurations
            .configure(routes::account::config)
            
            
            //Loading the user routes
            // .configure(routes::user_routes::config)

            //Loading the routes configurations
            // .configure(routes::blog_routes::config)
        
    })
    .bind((address, port))?
    .run()
    .await
}

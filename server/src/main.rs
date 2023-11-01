// mod handlers;
// mod db;
//
// use handlers::{hello, manual_hello};
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//    dotenv().ok();
//    env::set_var("RUST_LOG", "actix_web=debug");
//    let host = env::var("HOST").expect("Host not set");
//    let port = env::var("PORT").expect("Port not set");
//  
//    let pool = db::get_pool();
//
//     HttpServer::new(move || {
//         App::new()
//             .app_data(web::Data::new(pool.clone()))
//             .service(hello)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8000))?
//     .run()
//     .await
// }

mod handlers;
mod db;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

use handlers::{hello, manual_hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

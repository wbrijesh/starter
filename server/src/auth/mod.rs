use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use actix_web::{get,HttpResponse, Responder}; 

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

#[get("/v1/auth/register")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

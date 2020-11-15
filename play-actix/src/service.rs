use actix_web::{
    Scope, Route, Responder, HttpResponse, 
    web::{self, ServiceConfig},
    http::{Method}
};
use sqlx::postgres::PgPool;

mod user;
mod blog;
mod tag;
mod trash;
mod res;

pub fn service_config(cfg: &mut ServiceConfig) {
    cfg.service(
        Scope::new("/api")
        .service(
            Scope::new("/v1")
            .route("/login", Route::new().method(Method::GET).to(login))
            .route("/logout", Route::new().method(Method::GET).to(logout))
            .service(user::service("/user"))
            .service(blog::service("/blog"))
            .service(tag::service("/tag"))
            .service(trash::service("/trash"))
            .service(res::service("/res"))
        )
    );
}

async fn login(db: web::Data<PgPool>) -> impl Responder {

    let row: (i64,)= sqlx::query_as("SELECT $1")
        .bind(150i64)
        .fetch_one(&*db.into_inner()).await.unwrap();

    HttpResponse::Ok().body(format!("Hello {}", row.0))
}

async fn logout() -> impl Responder {
    HttpResponse::Ok().body("Bye-bye")
}

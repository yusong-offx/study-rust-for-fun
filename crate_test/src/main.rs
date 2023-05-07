use actix_web::{get, web, App, HttpServer, Responder, middleware::{Logger},};
use env_logger::Env;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let logger_format = r#"%a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %Dms"#;
    HttpServer::new(|| App::new()
        .wrap(Logger::new(logger_format))
        .service(index)
        .service(hello))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
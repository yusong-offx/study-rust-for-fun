use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    println!("{} start", name);
    for i in 0..10000000 {
        print!("{}\r", i);
    }
    println!("{} end", name);
    format!("Hello {}!", name)
}
//#[tokio::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(greet)
    })
    .workers(1)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

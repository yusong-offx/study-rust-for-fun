// use actix_web::{get, web, App, HttpServer, Responder};

// #[get("/hello/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     println!("{} start", name);
//     for i in 0..10000000 {
//         print!("{}\r", i);
//     }
//     println!("{} end", name);
//     format!("Hello {}!", name)
// }
// //#[tokio::main]
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//         .service(greet)
//     })
//     .workers(1)
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
fn myfn(x :usize)-> usize {
    let mut answer = 0;
    answer += x * 2;
    println!("{}", answer);
    if answer > 100 {
        return answer;
    }
    myfn(answer)
}
fn main() {

    myfn(10);
}

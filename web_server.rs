use actix_web::{web, App, HttpServer};

fn index(info: web::Path<(String,)>) -> String {
 format!(“Hello {}!”, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
 HttpServer::new(|| {
 App::new()
 .route(“/{name}”, web::get().to(index))
 })
 .bind(“127.0.0.1:8080”)?
 .run()
 .await
}

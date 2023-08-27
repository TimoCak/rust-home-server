use actix_files::Files;
use actix_web::{get, http::header::ContentType, App, HttpResponse, HttpServer};

#[get("/hello")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("../assets/hello.html"))
}

#[get("/")]
async fn index_page() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("../assets/index.html"))
}

#[get("/weather")]
async fn weather_page() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("../assets/weather.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(index_page)
            .service(weather_page)
            .service(Files::new("/assets", "./assets").prefer_utf8(true))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

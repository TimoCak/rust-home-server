use actix_web::{web, App, HttpResponse, HttpServer, get, http::header::ContentType};

#[get("/hello")]
async fn hello() -> HttpResponse{
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("../assets/hello.html"))
}

#[get("/")]
async fn home_page() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("../assets/home.html"))
}

#[get("/weather")]
async fn get_weather() -> HttpResponse {
    todo!()    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(home_page)
    })
        .bind(("192.168.178.34", 8080))?
        .run()
        .await
}

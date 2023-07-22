use actix_web::{App, HttpResponse, HttpServer, get, http::header::ContentType, post};

#[get("/hello")]
async fn hello() -> HttpResponse{
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

//posting temperatures
#[post("/postWeather")]
async fn post_weather() -> HttpResponse {
    todo!()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(index_page)
        .service(weather_page)
    })
        .bind(("192.168.178.22", 8080))?
        .run()
        .await
}

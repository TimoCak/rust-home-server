use actix_files::Files;
use actix_web::{
    get,
    http::header::ContentType,
    middleware::Logger,
    web::{self, resource},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use env_logger::Env;

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

#[get("/finance")]
async fn fincance_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("../assets/finance.html"))
}

#[get("/playlist")]
async fn playlist_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("../assets/playlist.html"))
}

async fn get_path(req: HttpRequest) -> HttpResponse {
    let url = req.resource_map();
    println!("{:?}", url);
    HttpResponse::Ok().finish()
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        resource("/users/{id}")
            .name("users_user_by_id")
            .route(web::get().to(get_path)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(config)
            .service(index_page)
            .service(weather_page)
            .service(fincance_page)
            .service(playlist_page)
            .service(Files::new("/assets", "./assets").prefer_utf8(true))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

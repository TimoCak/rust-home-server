use actix_web::HttpResponse;

/*
finance handlers
*/
pub(crate) async fn get_Balance() -> HttpResponse {
    HttpResponse::Ok().finish()
}

use actix_web::{HttpRequest, web, HttpResponse, Error};
use futures::Future;
use serde::{Serialize, Deserialize};
/*
#[derive(Serialize, Deserialize, Debug)]
struct WeatherResponse {
    date: String,
    lat: f32,
    lon: f32,
}

pub fn weather_request(req: HttpRequest, client: web::Data<WeatherForm>) 
-> impl Future<WeatherResponse = HttpResponse, Error> 
{

}
*/
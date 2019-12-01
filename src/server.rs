use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use crate::openweather::{WeatherRetriever};
use crate::fetch::{UrlFetcher};

pub trait Server {
    fn start(&self);
}

pub struct WeatherServer {
    pub ip: String,
    pub port: i64,
}

impl Server for WeatherServer {
    fn start(&self) {
        HttpServer::new(|| {
            App::new()
            .route("/current/{name}/{units}", web::get().to(current))
            .route("/forecast/{name}/{units}", web::get().to(forecast))
        })
        .bind(format!("{}:{}", self.ip, self.port))
        .expect("Can not bind to port 8000")
        .run()
        .unwrap()
    }
}

fn current(req: HttpRequest) -> impl Responder {
    let name = match req.match_info().get("name") {
        None => return "Error bad params".to_string(),
        Some(t) => t,
    };

    let units = req.match_info().get("units").unwrap_or("metric");
    
    let wthr = crate::openweather::OpenWeatherRetriever{
        ftch: Box::new(UrlFetcher{})
    };
    let current = match wthr.get_current(name.to_string(), units.to_string()) {
        Err(e) => format!("Error {}!", e),
        Ok(t) => format!("Temp {}!", t),
    };

    current
}

fn forecast(req: HttpRequest) -> impl Responder {
    let name = match req.match_info().get("name") {
        None => return "Error bad params".to_string(),
        Some(t) => t,
    };

    let units = req.match_info().get("units").unwrap_or("metric");
    
    let wthr = crate::openweather::OpenWeatherRetriever{
        ftch: Box::new(UrlFetcher{})
    };
    let current = match wthr.get_forecast(name.to_string(), units.to_string()) {
        Err(e) => format!("Error {}!", e),
        Ok(t) => format!("Temp {}!", t),
    };

    current
}
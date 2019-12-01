pub mod openweather;
mod server;
mod fetch;

use server::{Server};

fn main() {
    let srv = server::WeatherServer {
        ip: "localhost".to_string(),
        port: 8000,
    };

    srv.start();
}
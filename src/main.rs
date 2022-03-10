use std::{fmt, env};
use serde::Deserialize;

///
/// The wheather API.
/// 
const API_ENDPOINT: &str = "https://api.openweathermap.org/data/2.5/weather";

///
/// The program entry point.
/// 
#[tokio::main]
async fn main() {

    // Get API key from environment variable.
    let api_key: String = env::var("API_KEY").expect("API_KEY environment variable not set");

    // Get latitude and longitude.
    let lat: String = env::var("LAT").expect("LAT environment variable not set");
    let lon: String = env::var("LON").expect("LON environment variable not set");

    // Build API endpoint.
    let api_endpoint: String = format!("{}?appid={}&lat={}&lon={}", API_ENDPOINT, api_key, lat, lon);

    // Get the weather data.
    let weather: Weather = get_weather(api_endpoint).await.expect("Failed to get weather data");

    // Print.
    println!("{}", weather);

}

///
/// Get the weather.
/// 
async fn get_weather(url: String) -> Result<Weather, reqwest::Error> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    let weather: Weather = serde_json::from_str(&body).unwrap();
    Ok(weather)
}

///
/// The weather.
/// 
#[derive(Deserialize)]
struct Weather {
    name: String,
    main: Main,
}

impl fmt::Display for Weather {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Temperature for {}: {:.2}f ({:.2}c)", self.name, self.main.temp / 10.0, farenheit_to_celsius(self.main.temp / 10.0))
    }
}

///
/// The main weather data.
/// 
#[derive(Deserialize)]
struct Main {
    temp: f32,
}

///
/// Converts farhenheit to celcius.
/// 
fn farenheit_to_celsius(farenheit: f32) -> f32 {
    (farenheit - 32.0) * 5.0 / 9.0
}
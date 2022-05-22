use serde::{Serialize, Deserialize};
use std::error::Error;

static API_KEY: &str = "517585c5e642399d7a0246eb89e34877";
static API_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherStatus {
    pub id: u32, 
    pub main: String,
    pub description: String,
    pub icon: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherMainStats {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: f64,
    pub humidity: f64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindStats {
    pub speed: f64,
    pub deg: f64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherResponse {
    pub weather: Vec<WeatherStatus>,
    pub main: WeatherMainStats,
    pub visibility: f64,
    pub wind: WindStats,
    pub name: String
}

pub async fn get_weather(lat: f64, lon: f64) -> Result<WeatherResponse, Box<dyn Error>> {
    let params = [
        ("lat", lat.to_string()),
        ("lon", lon.to_string()),
        ("appId", String::from(API_KEY)),
        ("units", String::from("metric"))
    ];
    let url = reqwest::Url::parse_with_params(
        API_URL,
        &params
    )?;
    let raw_response = reqwest::get(url)
        .await?
        .text()
        .await?;
    let response: WeatherResponse = serde_json::from_str(&raw_response)?;
    Ok(response)
}
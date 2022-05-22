use std::error::Error;

use serde::{Serialize, Deserialize};

static API_KEY: &str = "517585c5e642399d7a0246eb89e34877";
static GEOCODING_API_URL: &str = "http://api.openweathermap.org/geo/1.0/direct";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IPLocationResponse {
    status: String,
    lon: f64,
    lat: f64
}

#[derive(Copy, Clone, Debug)]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
struct CoordinatesResponse {
    lat: f64,
    lon: f64
}

pub enum Location {
    Current,
    Coordinates { 
        lat: f64, 
        lon: f64 
    },
    City {
        city: String,
        state_code: Option<String>,
        country_code: Option<String>
    }
}

async fn get_city_coordinates(
    city: &str,
    state_code: Option<&str>, 
    country_code: Option<&str>
) -> Result<Coordinates, Box<dyn Error>> {
    let mut city_query = String::from("");
    city_query.push_str(city);
    if let Some(state_code_str) = state_code {
        city_query.push_str(&format!(",{}", state_code_str));
    }
    if let Some(country_code_str) = country_code {
        city_query.push_str(&format!(",{}", country_code_str));
    }
    let params = [
        ("q", city_query.as_str()),
        ("limit", "1"),
        ("appId", API_KEY)
    ];
    let url = reqwest::Url::parse_with_params(
        GEOCODING_API_URL,
        params
    )?;
    let text_response = reqwest::get(url)
        .await?
        .text()
        .await?;
    let response: Vec<CoordinatesResponse> = serde_json::from_str(&text_response)?;
    if let Some(&CoordinatesResponse { lat, lon }) = response.first() {
        Ok(Coordinates { lat, lon })
    } else {
        Err(Box::from("could not get coordinates"))
    }
}

pub async fn get_location_coordinates(location: &Location) -> Result<Coordinates, Box<dyn Error>> {
    match location {
        Location::Current => get_coordinates().await,
        Location::Coordinates { lat, lon } => Ok(Coordinates { lat: *lat, lon: *lon }),
        Location::City { city, state_code, country_code } => get_city_coordinates(city, state_code.as_deref(), country_code.as_deref()).await
    }
}

pub async fn get_coordinates() -> Result<Coordinates, Box<dyn Error>> {
    let raw_response = reqwest::get("http://ip-api.com/json")
            .await?
            .text()
            .await?;
    let ip_location_response: IPLocationResponse = serde_json::from_str(&raw_response)?;
    if ip_location_response.status != "success" {
        Err(Box::from("could not get current location coordinates"))
    } else {
        Ok(Coordinates { lat: ip_location_response.lat, lon: ip_location_response.lon })
    }
}
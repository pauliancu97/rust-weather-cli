use serde::{Serialize, Deserialize};


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

pub async fn get_coordinates() -> Option<Coordinates> {
    let raw_response = reqwest::get("http://ip-api.com/json")
            .await.ok()?
            .text().await.ok()?;
    let ip_location_response: IPLocationResponse = serde_json::from_str(&raw_response).ok()?;
    if ip_location_response.status != "success" {
        None
    } else {
        Some(Coordinates { lat: ip_location_response.lat, lon: ip_location_response.lon })
    }
}
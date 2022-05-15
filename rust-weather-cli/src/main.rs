mod location;
mod weather;

use std::error::Error;
use crate::location::get_coordinates;
use crate::weather::get_weather;


async fn test() -> Option<()> {
    let coordinates = get_coordinates().await?;
    println!("{:?}", coordinates);
    let weather = get_weather(coordinates.lat, coordinates.lon).await?;
    println!("{:?}", weather);
    Some(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    test().await;
    Ok(())
}

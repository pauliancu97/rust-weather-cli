mod location;

use std::error::Error;
use crate::location::get_coordinates;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Some(coordinates) = get_coordinates().await {
        println!("{:?}", coordinates);
    }
    Ok(())
}

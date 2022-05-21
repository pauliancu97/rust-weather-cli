mod location;
mod weather;

use std::error::Error;
use std::io;
use crossterm::terminal::enable_raw_mode;
use tui::{Terminal, Frame};
use tui::backend::{CrosstermBackend, Backend};
use tui::layout::{Layout, Constraint};
use tui::style::Style;
use tui::text::{Spans, Span};
use tui::widgets::{Block, Paragraph, Borders};
use weather::WeatherResponse;

use crate::location::get_coordinates;
use crate::weather::get_weather;

#[derive(Clone)]
struct WeatherMainStatsUi {
    description: String,
    temperatures: String,
    feels_like: String,
    wind: String,
    visibility: String,
    humidity: String
}

async fn test() -> Option<()> {
    let coordinates = get_coordinates().await?;
    println!("{:?}", coordinates);
    let weather = get_weather(coordinates.lat, coordinates.lon).await?;
    println!("{:?}", weather);
    Some(())
}

async fn get_weather_current_location() -> Option<WeatherResponse> {
    let coordinates = get_coordinates().await?;
    let response = get_weather(coordinates.lat, coordinates.lon).await?;
    Some(response)
}

fn get_weather_ui(weather_response: &WeatherResponse) -> Option<WeatherMainStatsUi> {
    let weather_desc = weather_response.weather.first()?.description.clone();
    let description = format!("Description: {}", weather_desc);
    let temperatures = format!("Temperatures: {:.1} - {:.1} °C", weather_response.main.temp_min, weather_response.main.temp_min);
    let feels_like = format!("Feels like: {:.1} °C", weather_response.main.feels_like);
    let wind = format!("Wind: {:.1} km/h", weather_response.wind.speed);
    let visibility = format!("Visibility: {:.1} km", weather_response.visibility);
    let humidity = format!("Humidity: {:.1} %", weather_response.main.humidity);
    Some(WeatherMainStatsUi {
        description,
        temperatures,
        feels_like,
        wind,
        visibility,
        humidity
    })
}

fn ui<B: Backend>(rect: &mut Frame<B>, weather_response: &WeatherMainStatsUi) {
    let size = rect.size();
        let chunks = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(30)
                ].as_ref()
            )
            .split(
                tui::layout::Rect { x: size.x, y: size.y, width: size.width, height: 5 }
            );
        let spans = vec![
            Spans::from(vec![Span::raw(&weather_response.description)]),
            Spans::from(vec![Span::raw(&weather_response.temperatures)]),
            Spans::from(vec![Span::raw(&weather_response.feels_like)]),
        ];
        let paragraph = Paragraph::new(spans)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(tui::style::Color::White))
                    .border_type(tui::widgets::BorderType::Plain)
            );
        rect.render_widget(paragraph, chunks[0]);
        let spans = vec![
            Spans::from(vec![Span::raw(&weather_response.wind)]),
            Spans::from(vec![Span::raw(&weather_response.visibility)]),
            Spans::from(vec![Span::raw(&weather_response.humidity)]),
        ];
        let paragraph = Paragraph::new(spans)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(tui::style::Color::White))
                    .border_type(tui::widgets::BorderType::Plain)
            );
        rect.render_widget(paragraph, chunks[1]);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let weather_response = get_weather_current_location().await.expect("could get weather for current location");
    let weather_ui = get_weather_ui(&weather_response).expect("all weather data available");
    terminal.clear()?;
    terminal.draw(|rect| ui(rect, &weather_ui))?;
    Ok(())
}

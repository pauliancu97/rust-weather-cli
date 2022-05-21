mod location;
mod weather;

use std::error::Error;
use std::io;
use std::collections::HashMap;
use crossterm::terminal::enable_raw_mode;
use lazy_static::lazy_static;
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
    humidity: String,
    icon: Vec<String>
}

async fn get_weather_current_location() -> Option<WeatherResponse> {
    let coordinates = get_coordinates().await?;
    let response = get_weather(coordinates.lat, coordinates.lon).await?;
    Some(response)
}

fn get_wind_direction(degree: f64) -> String {
    let wind_directions = ["↓", "↙", "←", "↖", "↑", "↗", "→", "↘"];
    let degree = degree + 22.5;
    let degree = if degree < 0.0 {
        360.0 + (degree % 360.0)
    } else {
        degree % 360.0
    };
    let index = ((degree as u64) / 45) as usize;
    String::from(wind_directions[index])
}

fn get_weather_ui(weather_response: &WeatherResponse) -> Option<WeatherMainStatsUi> {
    let weather_desc = weather_response.weather.first()?.description.clone();
    let weather_icon = weather_response.weather.first()?.icon.clone();
    let description = format!("Description: {}", weather_desc);
    let temperatures = format!("Temperatures: {:.1} - {:.1} °C", weather_response.main.temp_min, weather_response.main.temp_min);
    let feels_like = format!("Feels like: {:.1} °C", weather_response.main.feels_like);
    let wind_direction = get_wind_direction(weather_response.wind.deg);
    let wind = format!("Wind: {} {:.1} km/h", wind_direction, weather_response.wind.speed);
    let visibility = format!("Visibility: {:.1} km", weather_response.visibility);
    let humidity = format!("Humidity: {:.1} %", weather_response.main.humidity);
    let icon = get_weather_icon(&weather_icon)?;
    Some(WeatherMainStatsUi {
        description,
        temperatures,
        feels_like,
        wind,
        visibility,
        humidity,
        icon
    })
}

fn get_weather_icon(icon_id: &str) -> Option<Vec<String>> {
    lazy_static! {
        static ref ICON_MAP: HashMap<u32, Vec<String>> = {
            let mut map = HashMap::new();
            map.insert(1u32,
                [
                    "\033[38;5;226m    \\   /    \033[0m",
			        "\033[38;5;226m     .-.     \033[0m",
			        "\033[38;5;226m  ‒ (   ) ‒  \033[0m",
			        "\033[38;5;226m     `-᾿     \033[0m",
			        "\033[38;5;226m    /   \\    \033[0m",
                ]
                    .into_iter()
                    .map(|string| String::from(string))
                    .collect()
            );
            map.insert(2u32,
                [
                    "\033[38;5;226m   \\  /\033[0m      ",
			        "\033[38;5;226m _ /\"\"\033[38;5;250m.-.    \033[0m",
			        "\033[38;5;226m   \\_\033[38;5;250m(   ).  \033[0m",
			        "\033[38;5;226m   /\033[38;5;250m(___(__) \033[0m",
			        "             ",
                ]
                    .into_iter()
                    .map(|string| String::from(string))
                    .collect()
            );
            map.insert(3u32,
                [
                    "             ",
			        "\033[38;5;250m     .--.    \033[0m",
			        "\033[38;5;250m  .-(    ).  \033[0m",
			        "\033[38;5;250m (___.__)__) \033[0m",
                    "             ",
                ]
                    .into_iter()
                    .map(|string| String::from(string))
                    .collect()
            );
            map.insert(4u32,
                [
                    "             ",
			        "\x1b[38;5;240;1m     .--.    \x1b[0m",
			        "\x1b[38;5;240;1m  .-(    ).  \x1b[0m",
			        "\x1b[38;5;240;1m (___.__)__) \x1b[0m",
			        "             ",
                ]
                    .into_iter()
                    .map(|string| String::from(string))
                    .collect()
            );
            map.insert(9u32,
                [
                    "\033[38;5;226m _`/\"\"\033[38;5;250m.-.    \033[0m",
			        "\033[38;5;226m  ,\\_\033[38;5;250m(   ).  \033[0m",
			        "\033[38;5;226m   /\033[38;5;250m(___(__) \033[0m",
			        "\033[38;5;111m     ʻ ʻ ʻ ʻ \033[0m",
			        "\033[38;5;111m    ʻ ʻ ʻ ʻ  \033[0m",
                ]
                    .into_iter()
                    .map(|string| String::from(string))
                    .collect()
            );
            map.insert(10u32,
                [
                    "\033[38;5;240;1m     .-.     \033[0m",
			        "\033[38;5;240;1m    (   ).   \033[0m",
			        "\033[38;5;240;1m   (___(__)  \033[0m",
			        "\033[38;5;21;1m  ‚ʻ‚ʻ‚ʻ‚ʻ   \033[0m",
			        "\033[38;5;21;1m  ‚ʻ‚ʻ‚ʻ‚ʻ   \033[0m",
                ]
                    .into_iter()
                    .map(|string| String::from(string))
                    .collect()
            );
            map.insert(11u32,
                [
                    "\033[38;5;240;1m     .-.     \033[0m",
                    "\033[38;5;240;1m    (   ).   \033[0m",
			        "\033[38;5;240;1m   (___(__)  \033[0m",
			        "\033[38;5;21;1m  ‚ʻ\033[38;5;228;5m⚡\033[38;5;21;25mʻ‚\033[38;5;228;5m⚡\033[38;5;21;25m‚ʻ   \033[0m",
			        "\033[38;5;21;1m  ‚ʻ‚ʻ\033[38;5;228;5m⚡\033[38;5;21;25mʻ‚ʻ   \033[0m",
                ]
                    .into_iter()
                    .map(|string| String::from(string))
                    .collect()
            );
            map.insert(13u32,
                [
                    "\033[38;5;240;1m     .-.     \033[0m",
			        "\033[38;5;240;1m    (   ).   \033[0m",
			        "\033[38;5;240;1m   (___(__)  \033[0m",
			        "\033[38;5;255;1m   * * * *   \033[0m",
			        "\033[38;5;255;1m  * * * *    \033[0m",
                ]
                    .into_iter()
                    .map(|string| String::from(string))
                    .collect()
            );
            map.insert(50u32,
                [
                    "             ",
			        "\033[38;5;251m _ - _ - _ - \033[0m",
			        "\033[38;5;251m  _ - _ - _  \033[0m",
			        "\033[38;5;251m _ - _ - _ - \033[0m",
                    "             ",
                ]
                    .into_iter()
                    .map(|string| String::from(string))
                    .collect()
            );
            map
        };
    }
    let id = icon_id[..2].parse::<u32>().ok()?;
    ICON_MAP.get(&id).map(|strings| strings.clone())
}

fn ui<B: Backend>(rect: &mut Frame<B>, weather_response: &WeatherMainStatsUi) {
    let size = rect.size();
        let chunks = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                    Constraint::Percentage(30)
                ].as_ref()
            )
            .split(
                tui::layout::Rect { x: size.x, y: size.y, width: size.width, height: 7 }
            );
        let spans: Vec<_> = weather_response.icon.iter()
            .map(|string| Spans::from(vec![Span::raw(string)]))
            .collect();
        let paragraph = Paragraph::new(spans)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(tui::style::Color::White))
                    .border_type(tui::widgets::BorderType::Plain)
            );
        rect.render_widget(paragraph, chunks[0]);
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
        rect.render_widget(paragraph, chunks[1]);
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
        rect.render_widget(paragraph, chunks[2]);
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

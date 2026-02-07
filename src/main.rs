mod config;
mod display;

use config::Config;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use display::AsciiDisplay;
use std::io;
use std::time::{Duration, Instant};

const REFRESH_INTERVAL: Duration = Duration::from_secs(60);

fn main() -> io::Result<()> {
    let config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("\nPlease create a config file at:");
            eprintln!("  $XDG_CONFIG_HOME/weathr/config.toml");
            eprintln!("  or ~/.config/weathr/config.toml");
            eprintln!("\nExample config.toml:");
            eprintln!("  [location]");
            eprintln!("  latitude = 52.52");
            eprintln!("  longitude = 13.41");
            std::process::exit(1);
        }
    };

    enable_raw_mode()?;

    let result = run_app(&config);

    disable_raw_mode()?;
    AsciiDisplay::clear_screen()?;

    result
}

fn run_app(config: &Config) -> io::Result<()> {
    let house = AsciiDisplay::render_house();
    let mut last_update = Instant::now();

    loop {
        let weather_info =
            AsciiDisplay::format_weather_info(config.location.latitude, config.location.longitude);

        AsciiDisplay::render_frame(&house, &weather_info)?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        break
                    }
                    _ => {}
                }
            }
        }

        if last_update.elapsed() >= REFRESH_INTERVAL {
            last_update = Instant::now();
        }
    }

    Ok(())
}

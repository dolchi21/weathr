use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

pub struct AsciiDisplay;

impl AsciiDisplay {
    pub fn render_house() -> Vec<&'static str> {
        vec![
            "        `'::.",
            "     _________H ,%%&%,",
            "    /\\     _   \\%&&%%&%",
            "   /  \\___/^\\___\\%&%%&&",
            "   |  | []   [] |%\\Y&%'",
            "   |  |   .-.   | ||",
            " ~~@._|@@_|||_@@|~||~~~~~~~~~~~~~",
            "      `\"\"\") )\"\"\"`",
        ]
    }

    pub fn clear_screen() -> io::Result<()> {
        execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))
    }

    pub fn render_frame(house: &[&str], weather_info: &str) -> io::Result<()> {
        let mut stdout = io::stdout();
        let (term_width, _term_height) = crossterm::terminal::size()?;

        // Calculate starting position to center the content
        let house_width = house.iter().map(|line| line.len()).max().unwrap_or(0);
        let start_col = if term_width as usize > house_width {
            (term_width as usize - house_width) / 2
        } else {
            0
        };

        queue!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            SetForegroundColor(Color::Cyan),
            Print(weather_info),
            ResetColor,
        )?;

        // Render house line by line with proper positioning
        queue!(stdout, cursor::MoveTo(0, 3))?;
        for (idx, line) in house.iter().enumerate() {
            queue!(
                stdout,
                cursor::MoveTo(start_col as u16, 3 + idx as u16),
                SetForegroundColor(Color::Yellow),
                Print(line),
                ResetColor,
            )?;
        }

        stdout.flush()
    }

    pub fn format_weather_info(latitude: f64, longitude: f64) -> String {
        format!(
            "Weather for: {:.2}°N, {:.2}°E | Press 'q' to quit",
            latitude, longitude
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_house_not_empty() {
        let house = AsciiDisplay::render_house();
        assert!(!house.is_empty());
    }

    #[test]
    fn test_render_house_contains_structure() {
        let house = AsciiDisplay::render_house();
        let house_str = house.join("\n");
        assert!(house_str.contains("_________H"));
        assert!(house_str.contains("/\\"));
        assert!(house_str.contains("[]"));
        assert!(house_str.contains("~~~"));
    }

    #[test]
    fn test_render_house_is_multiline() {
        let house = AsciiDisplay::render_house();
        assert!(house.len() > 5, "House should have multiple lines");
    }

    #[test]
    fn test_format_weather_info_positive_coordinates() {
        let info = AsciiDisplay::format_weather_info(52.52, 13.41);
        assert!(info.contains("52.52°N"));
        assert!(info.contains("13.41°E"));
        assert!(info.contains("Press 'q' to quit"));
    }

    #[test]
    fn test_format_weather_info_negative_coordinates() {
        let info = AsciiDisplay::format_weather_info(-33.87, -74.01);
        assert!(info.contains("-33.87°N"));
        assert!(info.contains("-74.01°E"));
    }

    #[test]
    fn test_format_weather_info_zero_coordinates() {
        let info = AsciiDisplay::format_weather_info(0.0, 0.0);
        assert!(info.contains("0.00°N"));
        assert!(info.contains("0.00°E"));
    }

    #[test]
    fn test_format_weather_info_precision() {
        let info = AsciiDisplay::format_weather_info(52.5234567, 13.4134567);
        assert!(info.contains("52.52°N"));
        assert!(info.contains("13.41°E"));
    }

    #[test]
    fn test_format_weather_info_boundary_values() {
        let info = AsciiDisplay::format_weather_info(90.0, 180.0);
        assert!(info.contains("90.00°N"));
        assert!(info.contains("180.00°E"));
    }
}

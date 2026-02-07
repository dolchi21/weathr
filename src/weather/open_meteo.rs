use crate::weather::provider::{WeatherProvider, WeatherProviderResponse};
use crate::weather::types::{
    PrecipitationUnit, TemperatureUnit, WeatherLocation, WeatherUnits, WindSpeedUnit,
};
use async_trait::async_trait;
use serde::Deserialize;

pub struct OpenMeteoProvider {
    client: reqwest::Client,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct OpenMeteoResponse {
    current: CurrentWeather,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    time: String,
    temperature_2m: f64,
    relative_humidity_2m: f64,
    apparent_temperature: f64,
    is_day: i32,
    precipitation: f64,
    weather_code: i32,
    cloud_cover: f64,
    surface_pressure: f64,
    wind_speed_10m: f64,
    wind_direction_10m: f64,
    #[serde(default)]
    visibility: Option<f64>,
}

impl OpenMeteoProvider {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.open-meteo.com/v1/forecast".to_string(),
        }
    }

    fn temperature_unit_param(unit: &TemperatureUnit) -> &'static str {
        match unit {
            TemperatureUnit::Celsius => "celsius",
            TemperatureUnit::Fahrenheit => "fahrenheit",
        }
    }

    fn wind_speed_unit_param(unit: &WindSpeedUnit) -> &'static str {
        match unit {
            WindSpeedUnit::Kmh => "kmh",
            WindSpeedUnit::Ms => "ms",
            WindSpeedUnit::Mph => "mph",
            WindSpeedUnit::Kn => "kn",
        }
    }

    fn precipitation_unit_param(unit: &PrecipitationUnit) -> &'static str {
        match unit {
            PrecipitationUnit::Mm => "mm",
            PrecipitationUnit::Inch => "inch",
        }
    }
}

impl Default for OpenMeteoProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WeatherProvider for OpenMeteoProvider {
    async fn fetch_current_weather(
        &self,
        location: &WeatherLocation,
        units: &WeatherUnits,
    ) -> Result<WeatherProviderResponse, Box<dyn std::error::Error>> {
        let current_params = vec![
            "temperature_2m",
            "relative_humidity_2m",
            "apparent_temperature",
            "is_day",
            "precipitation",
            "weather_code",
            "cloud_cover",
            "surface_pressure",
            "wind_speed_10m",
            "wind_direction_10m",
            "visibility",
        ];

        let mut url = reqwest::Url::parse(&self.base_url)?;
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("latitude", &location.latitude.to_string());
            query.append_pair("longitude", &location.longitude.to_string());
            query.append_pair("current", &current_params.join(","));
            query.append_pair(
                "temperature_unit",
                Self::temperature_unit_param(&units.temperature),
            );
            query.append_pair(
                "wind_speed_unit",
                Self::wind_speed_unit_param(&units.wind_speed),
            );
            query.append_pair(
                "precipitation_unit",
                Self::precipitation_unit_param(&units.precipitation),
            );

            if let Some(elevation) = location.elevation {
                query.append_pair("elevation", &elevation.to_string());
            }
        }

        let response = self.client.get(url).send().await?;
        let data: OpenMeteoResponse = response.json().await?;

        Ok(WeatherProviderResponse {
            weather_code: data.current.weather_code,
            temperature: data.current.temperature_2m,
            apparent_temperature: data.current.apparent_temperature,
            humidity: data.current.relative_humidity_2m,
            precipitation: data.current.precipitation,
            wind_speed: data.current.wind_speed_10m,
            wind_direction: data.current.wind_direction_10m,
            cloud_cover: data.current.cloud_cover,
            pressure: data.current.surface_pressure,
            visibility: data.current.visibility,
            is_day: data.current.is_day,
            timestamp: data.current.time,
        })
    }

    fn get_name(&self) -> &'static str {
        "Open-Meteo"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_conversion_params() {
        assert_eq!(
            OpenMeteoProvider::temperature_unit_param(&TemperatureUnit::Celsius),
            "celsius"
        );
        assert_eq!(
            OpenMeteoProvider::temperature_unit_param(&TemperatureUnit::Fahrenheit),
            "fahrenheit"
        );
        assert_eq!(
            OpenMeteoProvider::wind_speed_unit_param(&WindSpeedUnit::Kmh),
            "kmh"
        );
        assert_eq!(
            OpenMeteoProvider::wind_speed_unit_param(&WindSpeedUnit::Ms),
            "ms"
        );
        assert_eq!(
            OpenMeteoProvider::precipitation_unit_param(&PrecipitationUnit::Mm),
            "mm"
        );
    }
}

use crate::weather::types::{WeatherLocation, WeatherUnits};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherProviderResponse {
    pub weather_code: i32,
    pub temperature: f64,
    pub apparent_temperature: f64,
    pub humidity: f64,
    pub precipitation: f64,
    pub wind_speed: f64,
    pub wind_direction: f64,
    pub cloud_cover: f64,
    pub pressure: f64,
    pub visibility: Option<f64>,
    pub is_day: i32,
    pub timestamp: String,
}

#[async_trait]
pub trait WeatherProvider: Send + Sync {
    async fn fetch_current_weather(
        &self,
        location: &WeatherLocation,
        units: &WeatherUnits,
    ) -> Result<WeatherProviderResponse, Box<dyn std::error::Error>>;

    fn get_name(&self) -> &'static str;
}

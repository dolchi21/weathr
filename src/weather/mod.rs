pub mod client;
pub mod normalizer;
pub mod open_meteo;
pub mod provider;
pub mod types;

pub use client::WeatherClient;
pub use normalizer::WeatherNormalizer;
pub use open_meteo::OpenMeteoProvider;
pub use provider::{WeatherProvider, WeatherProviderResponse};
pub use types::{
    PrecipitationUnit, TemperatureUnit, WeatherCondition, WeatherData, WeatherLocation,
    WeatherUnits, WindSpeedUnit,
};

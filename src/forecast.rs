use crate::WeatherResult;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ForecastsAtPoint {
    #[serde(alias = "properties")]
    forecast_links: ForecastLinks,
}
#[derive(Deserialize, Debug)]
struct ForecastLinks {
    #[serde(alias = "forecast")]
    _all: String,
    #[serde(alias = "forecastHourly")]
    hourly: String,
}
#[derive(Deserialize, Debug)]
pub struct HourlyForecast {
    #[serde(alias = "properties")]
    pub forecast: ForecastPeriods,
}
#[derive(Deserialize, Debug)]
pub struct ForecastPeriods {
    pub periods: Vec<ForecastPeriod>,
}

#[derive(Deserialize, Debug)]
pub struct ForecastPeriod {
    #[serde(alias = "startTime")]
    pub time: String,
    pub temperature: f32,
    #[serde(alias = "temperatureUnit")]
    pub unit: String,
    #[serde(alias = "windSpeed")]
    pub wind_speed: String,
    #[serde(alias = "windDirection")]
    pub wind_direction: String,
    #[serde(alias = "shortForecast")]
    pub short_forecast: String,
}

pub fn get_hourly_forecast(latitude: &String, longitude: &String) -> WeatherResult<HourlyForecast> {
    // Extract links to forecasts from location given
    let response = reqwest::blocking::Client::new()
        .get(format!(
            "https://api.weather.gov/points/{},{}",
            latitude, longitude
        ))
        .header(reqwest::header::USER_AGENT, "Weather APP")
        .send()?;
    let forecasts_at_point = response.json::<ForecastsAtPoint>()?;

    // Extract hourly forecast from links extracted
    let response = reqwest::blocking::Client::new()
        .get(forecasts_at_point.forecast_links.hourly)
        .header(reqwest::header::USER_AGENT, "Weather APP")
        .send()?;
    let hourly_forecast = response.json::<HourlyForecast>()?;

    Ok(hourly_forecast)
}

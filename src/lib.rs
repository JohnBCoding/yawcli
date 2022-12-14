use chrono::{DateTime, Datelike, Timelike};
use clap::{App, Arg};
use reqwest;
use scraper;
use serde::Deserialize;
use std::cmp::min;
use std::error::Error;

type WeatherResult<T> = Result<T, Box<dyn Error>>;
pub struct Config {
    celsius: bool,
    hours: usize,
    color: bool,
}
struct Location {
    latitude: String,
    longitude: String,
    country: String,
    region: String,
    city: String,
}

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
struct HourlyForecast {
    #[serde(alias = "properties")]
    forecast: ForecastPeriods,
}
#[derive(Deserialize, Debug)]
struct ForecastPeriods {
    periods: Vec<ForecastPeriod>,
}

#[derive(Deserialize, Debug)]
struct ForecastPeriod {
    #[serde(alias = "startTime")]
    time: String,
    temperature: f32,
    #[serde(alias = "temperatureUnit")]
    unit: String,
    #[serde(alias = "windSpeed")]
    wind_speed: String,
    #[serde(alias = "windDirection")]
    wind_direction: String,
    #[serde(alias = "shortForecast")]
    short_forecast: String,
}

pub fn run(config: Config) -> WeatherResult<()> {
    let location = get_data_from_ip()?;
    let hourly_forecast = get_hourly_forecast(&location.latitude, &location.longitude)?;
    if config.color {
        print_hourly_forecast_colored(location, hourly_forecast, config)?;
    } else {
        print_hourly_forecast(location, hourly_forecast, config)?;
    }

    Ok(())
}

pub fn get_args() -> WeatherResult<Config> {
    let matches = App::new("yawcli")
        .version("0.2")
        .author("John Bullard <johnbcooding@gmail.com>")
        .about("Uses your IP to get the local forecast, only works in USA.")
        .arg(
            Arg::with_name("celsius")
                .short("c")
                .long("celsius")
                .help("Converts temperature to celsius.")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("hours")
                .long("hours")
                .help("How many hours to show in hourly forecast, max 24.")
                .default_value("1")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("color")
                .long("color")
                .help("Prints out the forecast in color")
                .takes_value(false),
        )
        .get_matches();

    let hours = matches.value_of_lossy("hours").unwrap();

    Ok(Config {
        celsius: matches.is_present("celsius"),
        hours: hours.parse::<usize>()?,
        color: matches.is_present("color"),
    })
}

fn get_data_from_ip() -> WeatherResult<Location> {
    // Get location data from web
    let response = reqwest::blocking::get("https://iplocation.com/")?;
    let body = response.text()?;
    let document = scraper::Html::parse_document(&body);

    // Extract location information
    let mut lat: String = format!("");
    let mut lng: String = format!("");
    let mut country: String = format!("");
    let mut region: String = format!("");
    let mut city: String = format!("");
    if let Ok(lat_selector) = scraper::Selector::parse("td.lat") {
        document.select(&lat_selector).for_each(|lt| {
            lat = lt.inner_html();
        });
    }
    if let Ok(lng_selector) = scraper::Selector::parse("td.lng") {
        document.select(&lng_selector).for_each(|lg| {
            lng = lg.inner_html();
        });
    }
    if let Ok(country_selector) = scraper::Selector::parse("span.country_name") {
        document.select(&country_selector).for_each(|ctry| {
            country = ctry.inner_html();
        });
    }
    if let Ok(region_selector) = scraper::Selector::parse("span.region_name") {
        document.select(&region_selector).for_each(|rgn| {
            region = rgn.inner_html();
        });
    }
    if let Ok(city_selector) = scraper::Selector::parse("td.city") {
        document.select(&city_selector).for_each(|cy| {
            city = cy.inner_html();
        });
    }

    Ok(Location {
        latitude: lat,
        longitude: lng,
        country,
        region,
        city,
    })
}

fn get_hourly_forecast(latitude: &String, longitude: &String) -> WeatherResult<HourlyForecast> {
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

fn print_location(location: Location) {
    println!(
        "\n{}, {} | {} ({}, {})",
        location.city,
        location.region,
        location.country,
        location.latitude.split(".").next().unwrap_or(""),
        location.longitude.split(".").next().unwrap_or("")
    );
}

fn print_hourly_forecast(
    location: Location,
    hourly_forecast: HourlyForecast,
    config: Config,
) -> WeatherResult<()> {
    print_location(location);
    println!("\nWeather for the next {} hour(s):", config.hours);

    let mut temp_unit = hourly_forecast.forecast.periods[0].unit.to_uppercase();
    for period in 0..min(24, config.hours) {
        let mut temp = hourly_forecast.forecast.periods[period].temperature;

        // Convert to celsius if needed
        if config.celsius {
            temp = (temp - 32.0) * 0.5556;
            temp_unit = "C".to_string();
        }

        // Print retrieved info
        let time = DateTime::parse_from_rfc3339(&hourly_forecast.forecast.periods[period].time)?;
        let hour = time.hour12();
        println!(
            "  {} {}{}:  [ Temp: {:.0}°{} ]  [ Conditions: {} ]  [ Wind: {} {} ]",
            time.weekday(),
            hour.1,
            if hour.0 { "pm" } else { "am" },
            temp,
            temp_unit,
            hourly_forecast.forecast.periods[period].short_forecast,
            hourly_forecast.forecast.periods[period].wind_speed,
            hourly_forecast.forecast.periods[period].wind_direction
        );
    }

    Ok(())
}

/// Prints forecast with foreground and background colors
fn print_hourly_forecast_colored(
    location: Location,
    hourly_forecast: HourlyForecast,
    config: Config,
) -> WeatherResult<()> {
    print_location(location);
    println!("\nWeather for the next {} hour(s):", config.hours);

    let mut temp_unit = hourly_forecast.forecast.periods[0].unit.to_uppercase();
    for period in 0..min(24, config.hours) {
        let mut temp = hourly_forecast.forecast.periods[period].temperature;

        // Convert to celsius if needed
        if config.celsius {
            temp = (temp - 32.0) * 0.5556;
            temp_unit = "C".to_string();
        }

        // Print retrieved info, alternate background color every odd period
        let time = DateTime::parse_from_rfc3339(&hourly_forecast.forecast.periods[period].time)?;
        let hour = time.hour12();
        if period % 2 == 0 {
            println!(
            "  {} {}{}:  \x1b[47;30m Temp: {:.0}°{} \x1b[0m\x1b[47;34m Conditions: {}\x1b[0m\x1b[47;35m Wind: {} {} \x1b[0m",
            time.weekday(),
            hour.1,
            if hour.0 { "pm" } else { "am" },
            temp,
            temp_unit,
            hourly_forecast.forecast.periods[period].short_forecast,
            hourly_forecast.forecast.periods[period].wind_speed,
            hourly_forecast.forecast.periods[period].wind_direction
            );
        } else {
            println!(
                "  {} {}{}:  \x1b[100;30m Temp: {:.0}°{} \x1b[0m\x1b[100;34m Conditions: {}\x1b[0m\x1b[100;35m Wind: {} {} \x1b[0m",
                time.weekday(),
                hour.1,
                if hour.0 { "pm" } else { "am" },
                temp,
                temp_unit,
                hourly_forecast.forecast.periods[period].short_forecast,
                hourly_forecast.forecast.periods[period].wind_speed,
                hourly_forecast.forecast.periods[period].wind_direction
                );
        }
    }

    Ok(())
}

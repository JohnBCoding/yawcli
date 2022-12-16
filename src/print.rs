use crate::{forecast::HourlyForecast, location::Location, Config, WeatherResult};
use chrono::{DateTime, Datelike, Timelike};
use std::cmp::min;

/// Prints location information(city, region, country, latitude/longitude)
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

/// Prints hourly forecast with no colors
pub fn print_hourly_forecast(
    location: Location,
    hourly_forecast: HourlyForecast,
    config: Config,
) -> WeatherResult<()> {
    // Printe header
    print_location(location);
    println!("\nWeather for the next {} hour(s):", config.hours);

    // Print forecast for each hour given
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

/// Prints hourly forecast with foreground and background colors
pub fn print_hourly_forecast_colored(
    location: Location,
    hourly_forecast: HourlyForecast,
    config: Config,
) -> WeatherResult<()> {
    // Enables color in windows cmd
    #[cfg(windows)]
    enable_virtual_terminal_processing();

    // Print header
    print_location(location);
    println!("\nWeather for the next {} hour(s):", config.hours);

    // Print forecast for each hour given
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
            "  {} {}{}:  \x1b[47;30m Temp: {:.0}°{} \x1b[0m\x1b[47;34m Conditions: {} \x1b[0m\x1b[47;35m Wind: {} {} \x1b[0m",
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
                "  {} {}{}:  \x1b[100;30m Temp: {:.0}°{} \x1b[0m\x1b[100;34m Conditions: {} \x1b[0m\x1b[100;35m Wind: {} {} \x1b[0m",
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

/// Enables color in windows cmd
/// taken from https://stackoverflow.com/questions/63526130/why-do-ansi-escape-codes-sometimes-work-in-cmd
#[cfg(windows)]
fn enable_virtual_terminal_processing() {
    use winapi_util::console::Console;

    if let Ok(mut term) = Console::stdout() {
        let _ = term.set_virtual_terminal_processing(true);
    }

    if let Ok(mut term) = Console::stderr() {
        let _ = term.set_virtual_terminal_processing(true);
    }
}

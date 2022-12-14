use crate::{forecast, location, print};
use clap::{App, Arg};
use std::error::Error;

pub type WeatherResult<T> = Result<T, Box<dyn Error>>;
pub struct Config {
    pub celsius: bool,
    pub hours: usize,
    pub color: bool,
}

pub fn run(config: Config) -> WeatherResult<()> {
    let location_data = location::get_data_from_ip()?;
    let hourly_forecast =
        forecast::get_hourly_forecast(&location_data.latitude, &location_data.longitude)?;
    if config.color {
        print::print_hourly_forecast_colored(location_data, hourly_forecast, config)?;
    } else {
        print::print_hourly_forecast(location_data, hourly_forecast, config)?;
    }

    Ok(())
}

pub fn get_args() -> WeatherResult<Config> {
    let matches = App::new("yawcli")
        .version("0.2.3")
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

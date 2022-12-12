use assert_cmd::Command;
use reqwest;
use std::error::Error;

type TestResult<T> = Result<T, Box<dyn Error>>;

const PROGRAM: &str = "yawcli";

// ----- Connection Tests -----
#[test]
fn can_connect_to_ip() -> TestResult<()> {
    // Make sure we get a succesful response from the IP to location website.
    let response = reqwest::blocking::get("https://iplocation.com/")?;
    let _body = response.text()?;

    Ok(())
}

#[test]
fn can_connect_to_weather_api() -> TestResult<()> {
    // Make sure we get a succesful response from the IP to location website.
    let response = reqwest::blocking::get("https://api.weather.gov/points/39.7456,-97.0892")?;
    let _body = response.text()?;

    Ok(())
}
// ----------------------------

// -------- CLI Tests ---------
#[test]
fn run_empty() -> TestResult<()> {
    // Checks if 7 numbers are present, which should be the latitude/longitude, wind speed and current temp.
    Command::cargo_bin(PROGRAM)?
        .assert()
        .success()
        .stdout(predicates::str::is_match(r"(\D*\d){8,}").unwrap());
    Ok(())
}
// ----------------------------

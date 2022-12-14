use crate::WeatherResult;

pub struct Location {
    pub latitude: String,
    pub longitude: String,
    pub country: String,
    pub region: String,
    pub city: String,
}

pub fn get_data_from_ip() -> WeatherResult<Location> {
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

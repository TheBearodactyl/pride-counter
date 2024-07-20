use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

/// Fetches the HTML content of the Wikipedia page for the given country's list of cities.
fn fetch_html(country: &str) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://en.wikipedia.org/wiki/List_of_cities_in_{}",
        country
    );
    let response = get(&url)?.text()?;
    Ok(response)
}

/// Parses the HTML content and counts the number of official cities.
fn count_cities(html: &str) -> Result<i32, Box<dyn Error>> {
    let document = Html::parse_document(html);
    let selector =
        Selector::parse("table.wikitable tbody tr").map_err(|_| "Failed to parse selector")?;

    let count = document.select(&selector).count() - 1; // Subtract 1 to exclude the header row
    Ok(count as i32)
}

/// Finds the number of official cities for a given country by scraping its Wikipedia page.
pub fn find_number_of_cities(country: &str) -> i32 {
    let country = country.replace(" ", "_"); // Replace spaces with underscores
    let html = fetch_html(&country).expect("fuck.");
    let city_count = count_cities(&html).expect("fuck.");

    city_count
}

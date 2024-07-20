use crate::city_counter;

include!(concat!(env!("OUT_DIR"), "/current_year.rs"));

#[derive(Debug)]
pub struct PrideCountry {
    name: String,
    city_count: i32,
    years_with_pride: i32,
}

pub fn get_country_stats(name: &str, year_pride_started: i32) -> PrideCountry {
    PrideCountry {
        name: name.to_string(),
        city_count: city_counter::find_number_of_cities(name),
        years_with_pride: year_pride_started,
    }
}

pub(crate) fn sum_vector(entries: Vec<i32>) -> i32 {
    entries.iter().sum()
}

fn calc_pride_parade_count(country: PrideCountry) -> i32 {
    (country.city_count * country.years_with_pride) * 3
}

pub fn return_count_vec() -> Vec<i32> {
    // Define a list of tuples with country names and their respective years of pride
    let countries_info = vec![
        ("Angola", CURRENT_YEAR - 2019),
        ("Argentina", CURRENT_YEAR - 2023),
        ("Bahamas", 17),
        ("Belize", CURRENT_YEAR - 2021),
        ("Bosnia and Herzegovina", CURRENT_YEAR - 2019),
        ("Botswana", CURRENT_YEAR - 2019),
        ("Georgia", CURRENT_YEAR - 2012),
        ("Eswatini", CURRENT_YEAR - 2018),
        ("Guyana", CURRENT_YEAR - 2018),
        ("Hong Kong", CURRENT_YEAR - 2008),
        ("Hungary", CURRENT_YEAR - 2009),
        ("India", CURRENT_YEAR - 2014),
        ("Italy", CURRENT_YEAR - 1972),
        ("Jamaica", CURRENT_YEAR - 2015),
        ("Kosovo", CURRENT_YEAR - 2017),
        ("Lithuania", CURRENT_YEAR - 2009),
        ("Malawi", CURRENT_YEAR - 2021),
        ("Malta", CURRENT_YEAR - 2004),
        ("Mauritius", CURRENT_YEAR - 2005),
        ("Micronesia", CURRENT_YEAR - 2018),
        ("Namibia", CURRENT_YEAR - 2013),
        ("North Macedonia", CURRENT_YEAR - 2018),
        ("Peru", CURRENT_YEAR - 1995),
        ("Philippines", CURRENT_YEAR - 1990),
        ("Poland", CURRENT_YEAR - 2001),
        ("Rwanda", CURRENT_YEAR - 2021),
        ("Saint Lucia", CURRENT_YEAR - 2019),
        ("Singapore", CURRENT_YEAR - 2005),
        ("South Africa", CURRENT_YEAR - 1990),
        ("South Korea", CURRENT_YEAR - 2000),
        ("Sri Lanka", CURRENT_YEAR - 2005),
        ("Taiwan", CURRENT_YEAR - 2003),
        ("Trinidad and Tobago", CURRENT_YEAR - 2018),
        ("Türkiye", 9),
        ("Uganda", 3),
        ("Ukraine", CURRENT_YEAR - 2013),
        ("United States", CURRENT_YEAR - 1970),
    ];

    // Use map to transform each tuple into PrideCountry and then calculate the parade counts
    countries_info
        .iter()
        .map(|&(name, years_with_pride)| get_country_stats(name, years_with_pride))
        .map(calc_pride_parade_count)
        .collect()
}

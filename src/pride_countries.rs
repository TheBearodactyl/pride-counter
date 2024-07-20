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
    let mut pride_parade_counts: Vec<i32> = Vec::new();
    let pride_countries: Vec<PrideCountry> = vec![
        get_country_stats("Angola", CURRENT_YEAR - 2019),
        get_country_stats("Argentina", CURRENT_YEAR - 2023),
        get_country_stats("Bahamas", 17),
        get_country_stats("Belize", CURRENT_YEAR - 2021),
        get_country_stats("Bosnia and Herzegovina", CURRENT_YEAR - 2019),
        get_country_stats("Botswana", CURRENT_YEAR - 2019),
        get_country_stats("Georgia", CURRENT_YEAR - 2012),
        get_country_stats("Eswatini", CURRENT_YEAR - 2018),
        get_country_stats("Guyana", CURRENT_YEAR - 2018),
        get_country_stats("Hong Kong", CURRENT_YEAR - 2008),
        get_country_stats("Hungary", CURRENT_YEAR - 2009),
        get_country_stats("India", CURRENT_YEAR - 2014),
        get_country_stats("Italy", CURRENT_YEAR - 1972),
        get_country_stats("Jamaica", CURRENT_YEAR - 2015),
        get_country_stats("Kosovo", CURRENT_YEAR - 2017),
        get_country_stats("Lithuania", CURRENT_YEAR - 2009),
        get_country_stats("Malawi", CURRENT_YEAR - 2021),
        get_country_stats("Malta", CURRENT_YEAR - 2004),
        get_country_stats("Mauritius", CURRENT_YEAR - 2005),
        get_country_stats("Micronesia", CURRENT_YEAR - 2018),
        get_country_stats("Namibia", CURRENT_YEAR - 2013),
        get_country_stats("North Macedonia", CURRENT_YEAR - 2018),
        get_country_stats("Peru", CURRENT_YEAR - 1995),
        get_country_stats("Philippines", CURRENT_YEAR - 1990),
        get_country_stats("Poland", CURRENT_YEAR - 2001),
        get_country_stats("Rwanda", CURRENT_YEAR - 2021),
        get_country_stats("Saint Lucia", CURRENT_YEAR - 2019),
        get_country_stats("Singapore", CURRENT_YEAR - 2005),
        get_country_stats("South Africa", CURRENT_YEAR - 1990),
        get_country_stats("South Korea", CURRENT_YEAR - 2000),
        get_country_stats("Sri Lanka", CURRENT_YEAR - 2005),
        get_country_stats("Taiwan", CURRENT_YEAR - 2003),
        get_country_stats("Trinidad and Tobago", CURRENT_YEAR - 2018),
        get_country_stats("Türkiye", 9),
        get_country_stats("Uganda", 3),
        get_country_stats("Ukraine", CURRENT_YEAR - 2013),
        get_country_stats("United States", CURRENT_YEAR - 1970),
    ];

    for country in pride_countries {
        pride_parade_counts.push(calc_pride_parade_count(country));
    }

    pride_parade_counts
}

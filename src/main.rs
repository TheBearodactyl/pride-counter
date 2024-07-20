#![allow(unused)]

mod city_counter;
mod pride_countries;

use pride_countries::{return_count_vec, sum_vector, CURRENT_YEAR};

fn main() {
    println!(
        "How many pride parades have been held as of {}: {}",
        CURRENT_YEAR,
        sum_vector(return_count_vec())
    );
}

#![allow(unused)]

mod city_counter;
mod donut;
mod loading_screen;
mod pride_countries;

use loading_screen::with_loading_screen;
use pride_countries::{return_count_vec, sum_vector, CURRENT_YEAR};
use std::{
    io::{self, Write},
    process::{exit, Command},
};
use termion::{clear, cursor};

fn clear_screen() -> io::Result<()> {
    let mut stdout = io::stdout();
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1))?;
    stdout.flush()
}

fn actually_count_them() {
    println!(
        "\n\n\nHow many pride parades have been held as of {}: {}",
        CURRENT_YEAR,
        sum_vector(return_count_vec())
    );

    exit(0);
}

#[tokio::main]
async fn main() {
    clear_screen().expect("fuck.");

    with_loading_screen(donut::dnut, actually_count_them).await;
}

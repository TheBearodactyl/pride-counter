use std::env;
use std::fs::File;
use std::io::Write;
use chrono::{Datelike, Utc};

fn main() {
    let current_year = Utc::now().year();
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("current_year.rs");
    let mut f = File::create(&dest_path).unwrap();
    write!(f, "pub const CURRENT_YEAR: i32 = {};", current_year).unwrap();
}

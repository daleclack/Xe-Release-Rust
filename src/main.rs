use chrono::*;
use std::{fs, io};

fn main() {
    // Pointer to functions
    let funcs = [about, longterm, stable, develop, config];

    // Information
    println!("Input Mode:");
    println!("1.longterm;2.stable;3.develop;4.set config");

    // Input mode selection
    let mut string1 = String::new();
    io::stdin().read_line(&mut string1).expect("Read Error!");
    let index_string = string1.trim();
    if !index_string.is_empty()
        && (index_string == "0"
            || index_string == "1"
            || index_string == "2"
            || index_string == "3"
            || index_string == "4")
    {
        // Get index for mode5
        let index: usize = index_string.parse::<usize>().unwrap();
        funcs[index]();
    } else {
        println!("Please input a vaild mode index!");
    }
}

fn about() {
    // Print Help Information
    println!("XeRelease Rust Edition by daleclack");
}

fn longterm() {
    // Get current time
    let now = Utc::now();
    let local = Local::now();
    let naive_time = NaiveDate::from_ymd_opt(2019, 1, 11).unwrap().and_hms_opt(0, 0, 0).unwrap();
    let other_dt = DateTime::<Utc>::from_utc(naive_time, Utc);

    // Calculate the duration time
    let diff = now.signed_duration_since(other_dt);

    // Just print it on terminal
    println!(
        "5.4.{} {}-{}-{}",
        diff.num_days(),
        local.year(),
        local.month(),
        local.day()
    );
}

fn stable() {
    // Get current time
    let now = Utc::now();
    let local = Local::now();
    let naive_time = NaiveDate::from_ymd_opt(2017, 5, 19).unwrap().and_hms_opt(0, 0, 0).unwrap();
    let other_dt = DateTime::<Utc>::from_utc(naive_time, Utc);

    // Calculate the duration time
    let diff = now.signed_duration_since(other_dt);

    // Just print it on terminal
    println!(
        "7.2.{} {}-{}-{}",
        diff.num_days(),
        local.year(),
        local.month(),
        local.day()
    );
}

fn develop() {
    // Get current time
    let now = Utc::now();
    let local = Local::now();
    let naive_time = NaiveDate::from_ymd_opt(2017, 5, 19).unwrap().and_hms_opt(0, 0, 0).unwrap();
    let other_dt = DateTime::<Utc>::from_utc(naive_time, Utc);

    // Calculate the duration time
    let diff = now.signed_duration_since(other_dt);

    // Just print it on terminal
    println!(
        "8.0.{} {}-{}-{}",
        diff.num_days(),
        local.year(),
        local.month(),
        local.day()
    );
}

fn config() {
    println!("Input Config of XeRelease");
    // Strings to store version numbers
    let mut config_longterm = String::new();
    let mut config_stable = String::new();
    let mut config_devel = String::new();

    // Input version numbers
    println!("Input version config of longterm");
    io::stdin()
        .read_line(&mut config_longterm)
        .expect("Read Line Error!");
    println!("Input version config of stable");
    io::stdin()
        .read_line(&mut config_stable)
        .expect("Read Line Error!");
    println!("Input version config of development");
    io::stdin()
        .read_line(&mut config_devel)
        .expect("Read Line Error!");

    let contents = "longterm=".to_string()
        + &config_longterm
        + "stable="
        + &config_stable
        + "development="
        + &config_devel;

    // Save Configs to file
    fs::write("config", contents).expect("File Write Error!");
}

fn get_config() -> String{
    let str = String::new();

    return str;
}

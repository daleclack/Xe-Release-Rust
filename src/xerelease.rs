use chrono::*;
use std::{fs, io};
use crate::cfgfile::read_cfg_file;

pub fn longterm() {
    // Get current time
    let now = Utc::now();
    let local = Local::now();
    let naive_time = NaiveDate::from_ymd_opt(2019, 1, 11)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let other_dt = DateTime::<Utc>::from_utc(naive_time, Utc);

    // Calculate the duration time
    let diff = now.signed_duration_since(other_dt);

    // Get Version Config
    let mut version = String::new();
    read_cfg_file("xe_config", "Longterm", &mut version);

    // Just print it on terminal
    println!(
        "{}.{} {}-{}-{}",
        version,
        diff.num_days(),
        local.year(),
        local.month(),
        local.day()
    );
}

pub fn stable() {
    // Get current time
    let now = Utc::now();
    let local = Local::now();
    let naive_time = NaiveDate::from_ymd_opt(2017, 5, 19)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let other_dt = DateTime::<Utc>::from_utc(naive_time, Utc);

    // Calculate the duration time
    let diff = now.signed_duration_since(other_dt);

    // Get Version Config
    let mut version = String::new();
    read_cfg_file("xe_config", "Stable", &mut version);

    // Just print it on terminal
    println!(
        "{}.{} {}-{}-{}",
        version,
        diff.num_days(),
        local.year(),
        local.month(),
        local.day()
    );
}

pub fn develop() {
    // Get current time
    let now = Utc::now();
    let local = Local::now();
    let naive_time = NaiveDate::from_ymd_opt(2017, 5, 19)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let other_dt = DateTime::<Utc>::from_utc(naive_time, Utc);

    // Calculate the duration time
    let diff = now.signed_duration_since(other_dt);

    // Get Version Config
    let mut version = String::new();
    read_cfg_file("xe_config", "Develop", &mut version);

    // Just print it on terminal
    println!(
        "{}.{} {}-{}-{}",
        version,
        diff.num_days(),
        local.year(),
        local.month(),
        local.day()
    );
}

pub fn config() {
    println!("Input Config of XeRelease");
    // Strings to store version numbers
    let mut config_longterm = String::new();
    let mut config_stable = String::new();
    let mut config_devel = String::new();

    // Input version numbers
    println!("Input version config of longterm:");
    io::stdin()
        .read_line(&mut config_longterm)
        .expect("Read Line Error!");
    println!("Input version config of stable:");
    io::stdin()
        .read_line(&mut config_stable)
        .expect("Read Line Error!");
    println!("Input version config of development:");
    io::stdin()
        .read_line(&mut config_devel)
        .expect("Read Line Error!");

    let contents = "Longterm=".to_string()
        + &config_longterm
        + "Stable="
        + &config_stable
        + "Develop="
        + &config_devel;

    // Save Configs to file
    fs::write("xe_config", contents).expect("File Write Error!");
}

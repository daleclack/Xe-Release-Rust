use chrono::*;
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    process::exit,
};

fn main() {
    // Pointer to functions
    let funcs = [about, longterm, stable, develop, config, exit_app];

    loop {
        // Information
        println!("Input Mode:");
        println!("1.longterm;2.stable;3.develop;4.set config;5.exit");

        // Input mode selection
        let mut string1 = String::new();
        io::stdin().read_line(&mut string1).expect("Read Error!");
        let index_string = string1.trim();
        if !index_string.is_empty()
            && (index_string == "0"
                || index_string == "1"
                || index_string == "2"
                || index_string == "3"
                || index_string == "4"
                || index_string == "5")
        {
            // Get index for modes
            let index: usize = index_string.parse::<usize>().unwrap();
            funcs[index]();
        } else {
            println!("Please input a vaild mode index!");
        }
    }
}

fn about() {
    // Print Help Information
    println!("XeRelease Rust Edition by daleclack\nVersion 2.0");
}

fn longterm() {
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

fn stable() {
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

fn develop() {
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

fn exit_app() {
    exit(0);
}

fn config() {
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

fn read_cfg_file(filename: &str, key: &str, value: &mut String) {
    // let mut value = String::new();
    let key = key.trim();
    // Open the file
    // If the file not exist, create it
    let filename = filename.trim();
    let file1 = match File::open(filename) {
        Ok(file1) => file1,
        Err(_) => File::create(filename).unwrap(),
    };

    // Create a BufReader and read the file line by line
    let reader = BufReader::new(file1);
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        // Check the vaild config line
        let result = line.find("=");
        if result != None {
            // split the content line to key and value
            let key_value: Vec<&str> = line.split("=").collect();
            let key1 = key_value[0].clone();
            if key1 == key {
                // Push the value
                value.push_str(key_value[1]);
                return;
            }
        }
    }
}

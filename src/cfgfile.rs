/*

*/

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_cfg_file(filename: &str, key: &str, value: &mut String) {
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

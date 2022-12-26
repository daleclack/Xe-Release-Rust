// Import mods
mod xerelease;
mod cfgfile;

use std::{
    io,
    process::exit,
};
use xerelease::{longterm, stable, develop, config};

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
    println!("XeRelease Rust Edition by daleclack\nVersion 3.0");
}

fn exit_app() {
    exit(0);
}

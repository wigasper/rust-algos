mod lib;
pub use crate::lib::kmedoids::*;

use std::env;
use std::fs::File;
use std::io::prelude::*;

// main function for binary
fn main() {
    // Get command line args
    let args: Vec<String> = env::args().collect();
    let fp = &args[1];
    let k: u64 = args[2].parse().unwrap_or_else(|why| {
        panic!("could not parse '{}' for k: {}", args[2], why);
    });

    // Read input file
    let mut file = File::open(fp).unwrap_or_else(|why| {
        panic!("could not open {}: {}", fp, why);
    });

    let mut str_in = String::new();
    file.read_to_string(&mut str_in).expect("could not read file");

    let mut data: Vec<Vec<String>> = Vec::new();

    // Convert to String
    for line in str_in.split("\n") {
        if line.len() > 0 {
            let vals: Vec<&str> = line.split(",").collect();
            let mut string_vals: Vec<String> = Vec::new();
            for val in vals.iter() {
                string_vals.push(val.to_string());
            }

            data.push(string_vals);
        }
    }

    // Run KMedoids, print output
    let mut model = KMedoids::new();
    model.init(&data);
    model.fit(k);
    model.print_labels();
}


mod read_from_csv;
mod person;

use crate::person::PrintInfo;
use std::path::PathBuf;

fn main() {
    // Accept an optional first CLI argument as CSV path. If none provided, use `people.csv`.
    let csv_path: PathBuf = match std::env::args().nth(1) {
        Some(p) => PathBuf::from(p),
        None => PathBuf::from("people.csv"),
    };

    let people = match read_from_csv::read_people_from_csv(&csv_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "Failed to read CSV from '{}' ({}), falling back to hardcoded list",
                csv_path.display(),
                e
            );
            vec![
                person::Person { name: String::from("Sai"), age: 46, city: String::from("Singapore") },
                person::Person { name: String::from("Alex"), age: 30, city: String::from("New York") },
            ]
        }
    };

    for person in people {
        person.print_info();
        println!("---");
    }
}

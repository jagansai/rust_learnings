use rust_learnings::validation_impl;
use rust_learnings::prompt::prompt;
use rust_learnings::person::PrintInfo;
use rust_learnings::read_from_csv::read_people_from_csv;

fn main() {
    println!("Choose an option:");
    println!("1) Read from csv");
    println!("2) Account Validation Demo");

    let choice = prompt("Enter choice");

    match choice.as_str() {
        "1" => {
            // Use default CSV path like before
            let csv_path = std::path::PathBuf::from("resources/data/people.csv");
            match read_people_from_csv(&csv_path) {
                Ok(people) => {
                    for p in people {
                        p.print_info();
                        println!("---");
                    }
                }
                Err(e) => eprintln!("Failed to read CSV: {}", e),
            }
        }
        "2" => {
            validation_impl::run_account_validation_demo();
        }
        _ => println!("Unknown choice"),
    }
}

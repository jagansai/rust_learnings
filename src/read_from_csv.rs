use std::error::Error;
use std::fs::File;
use std::path::Path;

use crate::person::Person;

pub fn read_people_from_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Person>, Box<dyn Error>> {
	let file = File::open(path)?;
	let mut rdr = csv::Reader::from_reader(file);
	let mut people = Vec::new();

	for result in rdr.deserialize() {
		let person: Person = result?;
		people.push(person);
	}

	Ok(people)
}


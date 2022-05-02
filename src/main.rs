use std::fs::File;
use std::path::Path;

mod intro;
mod structs;
mod lifetimes;

fn main() -> std::io::Result<()> {
    // intro::intro();
    // structs::structs();
    // lifetimes::lifetimes();

    let persons = read_persons(Path::new("persons.json"))?;
    println!("{:?}", persons);

    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct Person {
    id: usize,
    name: String,
}

fn read_persons(path: &Path) -> std::io::Result<Vec<Person>> {
    let f = File::open(path)?;
    let persons = serde_json::from_reader(&f)?;
    Ok(persons)
}

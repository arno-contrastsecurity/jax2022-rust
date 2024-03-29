use std::fs::File;
use std::path::Path;

pub fn io() -> std::io::Result<()> {
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
    // let f = match File::open(path) {
    //     Ok(f) => f,
    //     Err(e) => return Err(e),
    // };
    let f = File::open(path)?;
    let persons = serde_json::from_reader(&f)?;
    Ok(persons)
}

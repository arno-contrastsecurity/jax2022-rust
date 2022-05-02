
mod intro;
mod structs;
mod lifetimes;
mod io;

fn main() -> std::io::Result<()> {
    intro::intro();
    structs::structs();
    lifetimes::lifetimes();
    io::io()?;

    Ok(())
}

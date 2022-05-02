
mod intro;
mod structs;
mod lifetimes;
mod io;

use clap::Parser;


fn main() -> std::io::Result<()> {

    let args = Args::parse();

    // intro::intro();
    // structs::structs();
    // lifetimes::lifetimes();
    // io::io()?;

    Ok(())
}


#[derive(Parser)]
#[clap(name = "movies", long_about = Some("This CLI tool provides query access to the movie database."))]
/// movies is an application for finding movies
struct Args {
    #[clap(arg_enum, long="The genre to look for")]
    genre: Genre,
}




#[derive(Clone, clap::ArgEnum)]
enum Genre {
    Action,
    Comedy,
    Romance,
}

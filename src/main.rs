use aoc2021::{aoc2021::AocRunner2021, runner::{AocRunner}};
use clap::{Arg, App};
pub mod runner;

fn main() {
    let matches = App::new("AocRunner")
                        .version("0.1")
                        .author("uniqueNullptr2")
                        .about("Runs Aoc code")
                        .arg(Arg::with_name("day")
                            .short("d")
                            .help("choose day to execute")
                            .takes_value(true))
                        .arg(Arg::with_name("part")
                            .short("p")
                            .help("choose part to execute")
                            .takes_value(true)
                            .requires("day"))
                        .arg(Arg::with_name("year")
                            .short("y")
                            .help("choose year to execute")
                            .takes_value(true)
                            .required(true))
                        .get_matches();
    let day: Option<usize> = matches.value_of("day").map(|s|str::parse(s).unwrap());
    let part= matches.value_of("part").map(|s|str::parse(s).unwrap()).unwrap_or(0);
    let year: usize = matches.value_of("year").map(|s|str::parse(s).unwrap()).unwrap();
    match year {
        2021 => {
            AocRunner2021::new().execute(day, part);
        },
        _=> {
            println!("Year not implemented.");
        }
    }
}
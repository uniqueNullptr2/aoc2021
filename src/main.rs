use aoc2021::{aoc2021::AocRunner2021, runner::{AocRunner, default_fn}};
use clap::{Arg, App};

fn main() {
    let matches = App::new("AocRunner")
                        .version("0.1")
                        .author("uniqueNullptr2")
                        .about("Runs Aoc code")
                        .arg(Arg::with_name("day")
                            .short("d")
                            .help("choose day to execute")
                            .takes_value(true)
                            .requires("year"))
                        .arg(Arg::with_name("year")
                            .short("y")
                            .help("choose year to execute")
                            .takes_value(true))
                        .get_matches();
    let day: Option<usize> = matches.value_of("day").map(|s|str::parse(s).unwrap());
    let year: Option<usize> = matches.value_of("year").map(|s|str::parse(s).unwrap());
    if let Some(year) = year {
        match year {
            2021 => {
                AocRunner2021::new().execute(day);
            },
            _=> {
                println!("Year not implemented.");
            }
        }
    } else {
        default_fn()
    }
}


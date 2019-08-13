extern crate clap;
extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod settings;

use clap::{App, Arg};
use settings::Settings;

fn main() {
    let matches = App::new("G Utils")
    .version("0.0.1")
    .author("Gurbakhshish Singh <gurbakhshish.s@gmail.com>")
    .about("Common command line utilities")
    .arg(Arg::with_name("config")
        .short("c")
        .long("config")
        .default_value("config.json")
        .help("config file")
    )
    .get_matches();


    let config_file = matches.value_of("config").unwrap();

    let s = Settings::new(config_file);

    println!("{:?}", s)

}

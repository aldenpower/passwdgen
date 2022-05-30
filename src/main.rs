mod args;

use passwords::PasswordGenerator;
use args::MyArgs;
use clap::Parser;
use clap::{App, Arg};
use std::str::FromStr;

fn main() {

    let passwdgen = App::new("passwdgen")
        .version("0.1.0")
        .author("aldenpower <eng.andersonfsl@gmail.com")
        .about("rust password generator")
        .arg(
            Arg::with_name("length")
            .help("Password length")
            .short('l')
            .takes_value(false)
            .long("length")
            .default_value("5")
        )
        .get_matches();

    // let length = passwdgen.value_of_lossy("length").unwrap();

    println!("{:#?}", passwdgen);

    // let args = MyArgs::parse();

    // let pg = PasswordGenerator {
    //     length: args.size,
    //     numbers: bool::from_str(&args.n).unwrap(),
    //     lowercase_letters: bool::from_str(&args.lcl).unwrap(),
    //     uppercase_letters: bool::from_str(&args.ucl).unwrap(),
    //     symbols :bool::from_str(&args.y).unwrap(),
    //     spaces: bool::from_str(&args.k).unwrap(),
    //     exclude_similar_characters: bool::from_str(&args.c).unwrap(), 
    //     strict: bool::from_str(&args.t).unwrap()
    // };
    // println!("{}", pg.generate_one().unwrap());
}
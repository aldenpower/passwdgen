mod args;

use passwords::PasswordGenerator;
use args::MyArgs;
use clap::Parser;
use std::str::FromStr;

fn main() {
    let args = MyArgs::parse();

    let pg = PasswordGenerator {
        length: args.size,
        numbers: bool::from_str(&args.n).unwrap(),
        lowercase_letters: bool::from_str(&args.lcl).unwrap(),
        uppercase_letters: bool::from_str(&args.ucl).unwrap(),
        symbols :bool::from_str(&args.y).unwrap(),
        spaces: bool::from_str(&args.k).unwrap(),
        exclude_similar_characters: bool::from_str(&args.c).unwrap(), 
        strict: bool::from_str(&args.t).unwrap()
    };
    println!("{}", pg.generate_one().unwrap());
}
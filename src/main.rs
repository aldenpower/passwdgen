use passwords::PasswordGenerator;
use clap::{Arg, Command};

fn main() {

    let passwdgen = Command::new("passwdgen")
        .version("0.1.0")
        .author("aldenpower <eng.andersonfsl@gmail.com")
        .about("rust password generator")
        .arg(
            Arg::new("length")
            .value_name("size")
            .long("size")
            .help("Password length")
            .short('s')
            .takes_value(true)
            .default_value("5")
        )
        .arg(
            Arg::new("numbers")
            .long("numbers")
            .help("Contain numbers")
            .short('n')
            .default_missing_value("true")
            .default_value("false")
        )
        .arg(
            Arg::new("lcl")
            .long("lowercaseletters")
            .help("Contain lower case letters")
            .short('c')
            .default_missing_value("true")
            .default_value("false")
        )
        .arg(
            Arg::new("ucl")
            .long("uppercaseletters")
            .help("Contain upper case letters")
            .short('u')
            .default_missing_value("true")
            .default_value("false")
        )
        .arg(
            Arg::new("symbol")
            .long("symbol")
            .help("Contain symbols")
            .short('o')
            .default_missing_value("true")
            .default_value("false")
        )
        .arg(
            Arg::new("spaces")
            .long("spaces")
            .help("Contain spaces")
            .short('k')
            .default_missing_value("true")
            .default_value("false")
        )
        .arg(
            Arg::new("similar")
            .long("similar")
            .help("Contain similar characters")
            .short('t')
            .default_missing_value("true")
            .default_value("false")
        )
        .get_matches();

    let pg = PasswordGenerator {
        length: passwdgen.value_of("length").unwrap().parse::<usize>().unwrap(),
        numbers: passwdgen.value_of("numbers").unwrap().parse::<bool>().unwrap(),
        lowercase_letters: passwdgen.value_of("lcl").unwrap().parse::<bool>().unwrap(),
        uppercase_letters: passwdgen.value_of("ucl").unwrap().parse::<bool>().unwrap(),
        symbols : passwdgen.value_of("symbol").unwrap().parse::<bool>().unwrap(),
        spaces: passwdgen.value_of("spaces").unwrap().parse::<bool>().unwrap(),
        exclude_similar_characters: passwdgen.value_of("similar").unwrap().parse::<bool>().unwrap(), 
        strict: true
    };

    println!("{}", pg.generate_one().unwrap());
}
use clap::{Command, Arg};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Alphanumeric;

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    let description = env!("CARGO_PKG_DESCRIPTION");
    let authors = env!("CARGO_PKG_AUTHORS");

    let matches = Command::new(name)
        .version(version)
        .author(authors)
        .about(description)
        .subcommand(
            Command::new("PIN")
                .about("Generiert eine PIN")
                .arg(
                    Arg::new("length")
                        .long("length")
                        .short('l')
                        .value_parser(clap::value_parser!(u32))
                        .default_value("6")
                        .help("Setzt die Länge des PINs"),
                )
                .arg(
                    Arg::new("seed")
                        .long("seed")
                        .short('s')
                        .value_parser(clap::value_parser!(String))
                        .help("Setzt den Seed für die Generierung des PINs"),
                ),
        )
        .subcommand(
            Command::new("Password")
                .about("Generiert ein Password")
                .arg(
                    Arg::new("length")
                        .long("length")
                        .short('l')
                        .value_parser(clap::value_parser!(u32))
                        .default_value("16")
                        .help("Setzt die Länge des Passworts"),
                )
                .arg(
                    Arg::new("seed")
                        .long("seed")
                        .short('s')
                        .value_parser(clap::value_parser!(String))
                        .help("Setzt den Seed für die Generierung des Passworts"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("PIN", sub_matches)) => {
            let pin_length: u32 = sub_matches.get_one::<u32>("length").copied().unwrap_or(6);
            let pin_seed: Option<&String> = sub_matches.get_one::<String>("seed");

            let pin = generate_pin(pin_length, pin_seed);
            println!("Generierte PIN: {}", pin);
        }
        Some(("Password", sub_matches)) => {
            let pw_length: u32 = sub_matches.get_one::<u32>("length").copied().unwrap_or(16);
            let pw_seed: Option<&String> = sub_matches.get_one::<String>("seed");

            let password = generate_password(pw_length, pw_seed);
            println!("Generiertes Passwort: {}", password);
        }
        _ => {
            eprintln!("Bitte gebe entweder 'PIN' oder 'Password' als ein Subcommand an.");
        }
    }
}

fn generate_pin(length: u32, seed: Option<&String>) -> String {
    let mut rng = create_rng(seed);  // rng als mutable deklarieren
    (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect::<String>()
}

fn generate_password(length: u32, seed: Option<&String>) -> String {
    let mut rng = create_rng(seed);  // rng als mutable deklarieren
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect::<String>()
}

fn create_rng(seed: Option<&String>) -> StdRng {
    match seed {
        Some(seed_value) => {
            let seed_bytes = seed_value.as_bytes();
            let mut seed_array = [0u8; 32];
            for (i, &byte) in seed_bytes.iter().enumerate() {
                if i < 32 {
                    seed_array[i] = byte;
                }
            }
            StdRng::from_seed(seed_array)
        }
        None => StdRng::from_entropy(),
    }
}
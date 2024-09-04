use clap::{Command, Arg};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Alphanumeric;

fn main() {
    // Retrieve environment variables set by the build script
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    let description = env!("CARGO_PKG_DESCRIPTION");
    let authors = env!("CARGO_PKG_AUTHORS");

    // Set up the CLI application using Clap
    let matches = Command::new(name)
        .version(version)
        .author(authors)
        .about(description)
        .subcommand(
            Command::new("PIN")
                .about("Generates a PIN")
                .arg(
                    Arg::new("length")
                        .long("length")
                        .short('l')
                        .value_parser(clap::value_parser!(u32))
                        .default_value("6")
                        .help("Sets the length of the PIN"),
                )
                .arg(
                    Arg::new("seed")
                        .long("seed")
                        .short('s')
                        .value_parser(clap::value_parser!(String))
                        .help("Sets the seed for generating the PIN"),
                ),
        )
        .subcommand(
            Command::new("Password")
                .about("Generates a Password")
                .arg(
                    Arg::new("length")
                        .long("length")
                        .short('l')
                        .value_parser(clap::value_parser!(u32))
                        .default_value("16")
                        .help("Sets the length of the password"),
                )
                .arg(
                    Arg::new("seed")
                        .long("seed")
                        .short('s')
                        .value_parser(clap::value_parser!(String))
                        .help("Sets the seed for generating the password"),
                ),
        )
        .get_matches();

    // Match the subcommand provided by the user
    match matches.subcommand() {
        Some(("PIN", sub_matches)) => {
            let pin_length: u32 = sub_matches.get_one::<u32>("length").copied().unwrap_or(6);
            let pin_seed: Option<&String> = sub_matches.get_one::<String>("seed");

            let pin = generate_pin(pin_length, pin_seed);
            println!("Generated PIN: {}", pin);
        }
        Some(("Password", sub_matches)) => {
            let pw_length: u32 = sub_matches.get_one::<u32>("length").copied().unwrap_or(16);
            let pw_seed: Option<&String> = sub_matches.get_one::<String>("seed");

            let password = generate_password(pw_length, pw_seed);
            println!("Generated Password: {}", password);
        }
        _ => {
            eprintln!("Please specify either 'PIN' or 'Password' as a subcommand.");
        }
    }
}

/// Generates a PIN of a specified length, optionally seeded
fn generate_pin(length: u32, seed: Option<&String>) -> String {
    let mut rng = create_rng(seed);  // Declare rng as mutable
    (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect::<String>()
}

/// Generates a password of a specified length, optionally seeded
fn generate_password(length: u32, seed: Option<&String>) -> String {
    let mut rng = create_rng(seed);  // Declare rng as mutable
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect::<String>()
}

/// Creates an RNG, optionally using a provided seed
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
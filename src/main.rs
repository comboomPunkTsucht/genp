use clap::{Command, Arg};

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let command = Command::new("Dino CLI Game")
        .version(version)
        .author("comboom.sucht")
        .about("Google's Dino Game in the Terminal in the language of Rust")
        .arg(
            Arg::new("fps")
                .long("fps")
                .value_parser(clap::value_parser!(u32))
                .default_value("120")
                .help("Setzt die Frames pro Sekunde"),
        )
        .arg(
            Arg::new("max_obstacles")
                .long("max_obstacles")
                .value_parser(clap::value_parser!(u32))
                .default_value("1")
                .help("Setzt die maximale Anzahl der Hindernisse"),
        )
        .arg(
            Arg::new("cheats")
                .long("cheats")
                .action(clap::ArgAction::SetTrue)
                .help("Aktiviert Cheats"),
        )
        .get_matches();

    let fps: u32 = command.get_one::<u32>("fps").copied().unwrap_or(120);
    let max_obstacles: u32 = command.get_one::<u32>("max_obstacles").copied().unwrap_or(1);
    let cheats: bool = command.get_one::<bool>("cheats").copied().unwrap_or(false);

    println!("cheats are currently set to {}", cheats);
    println!("max_obstacles are currently set to {}", max_obstacles);
    println!("fps is currently set to {}", fps);
}
use std::fs;

fn main() {
    // Read Cargo.toml at build time
    let content = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");

    // Extract the version number
    let version_line = content
        .lines()
        .find(|line| line.starts_with("version = "))
        .expect("Version not found");

    let version = version_line.split('=')
        .nth(1)
        .expect("No version found")
        .trim()
        .trim_matches('"');

    // Extract the name
    let name_line = content
        .lines()
        .find(|line| line.starts_with("name = "))
        .expect("Name not found");

    let name = name_line.split('=')
        .nth(1)
        .expect("No name found")
        .trim()
        .trim_matches('"');

    // Extract the description
    let description_line = content
        .lines()
        .find(|line| line.starts_with("description = "))
        .expect("Description not found");

    let description = description_line.split('=')
        .nth(1)
        .expect("No description found")
        .trim()
        .trim_matches('"');

    // Extract the authors (list of authors)
    let authors_line = content
        .lines()
        .find(|line| line.starts_with("authors = "))
        .expect("Authors not found");

    let authors = authors_line.split('=')
        .nth(1)
        .expect("No authors found")
        .trim()
        .trim_matches(|c| c == '[' || c == ']' || c == '"')
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<&str>>()
        .join(", ");

    // Print commands to cargo to rerun the script if Cargo.toml changes
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:version={}", version);
    println!("cargo:name={}", name);
    println!("cargo:description={}", description);
    println!("cargo:author={}", authors);
}
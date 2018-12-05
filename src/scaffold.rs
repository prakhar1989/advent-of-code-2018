/// helper code to generate a scaffolding for solution
/// call with 4

#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
struct CargoTargets {
    bin: Vec<Bin>,
}

#[derive(Debug, Deserialize)]
struct Bin {
    name: String,
    path: String,
}

fn main() -> std::io::Result<()> {
    let day = env::args()
        .skip(1)
        .next()
        .expect("Usage: cargo run --scaffold 05");

    create_input_file(&day)?;
    create_src_file(&day)?;
    check_target(&day)?;

    Ok(())
}

fn create_input_file(day: &str) -> std::io::Result<()> {
    let input_file_path = format!("input/day{}.txt", day);
    match File::open(&input_file_path) {
        Ok(_) => Ok(()), // file already exists
        Err(_) => {
            File::create(&input_file_path)?;
            Ok(())
        }
    }
}

fn create_src_file(day: &str) -> std::io::Result<()> {
    let input_file_path = format!("src/day{}.rs", day);
    let src_program = format!(
        "
const INPUT: &str = include_str!(\"../input/day{}.txt\");
fn main() {{}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn some_test() {{}}
}}
    ",
        day
    );
    match File::open(&input_file_path) {
        Ok(_) => Ok(()),
        Err(_) => {
            let mut f = File::create(&input_file_path)?;
            f.write_all(&src_program.into_bytes())?;

            Ok(())
        }
    }
}

fn check_target(day: &str) -> std::io::Result<()> {
    let mut f = File::open("Cargo.toml")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let commands: CargoTargets = toml::from_str(&contents).unwrap();

    let target_name = format!("day{}", day);

    let existing_target = commands
        .bin
        .iter()
        .find(|target| target.name.eq(&target_name));

    if existing_target.is_none() {
        println!("Please add a binary target for {}", target_name);
    }

    Ok(())
}

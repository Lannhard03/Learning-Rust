use std::env;
use std::process;
use std::time::Instant;

pub mod data;
pub mod med;
pub mod spellchecker;

fn main() {
    let now = Instant::now();
    let config = data::Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = data::run(&config) {
        println!("Problem running program: {e}");
        process::exit(1);
    }
    println!("{}", now.elapsed().as_secs_f32());
}

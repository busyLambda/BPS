use clap::Command;

pub mod models;

const LOGO: &str = ".
                    ____   ____   ____
                   | ()_\\ | ()_} (__  )
                   |_()_/ |_|    (____)";

fn main() {
    println!("Hello, world!");
    let matches = Command::new("Binary Packaging System")
        .version("0.1")
        .author("Susa Milán")
        .about("A simple binary packing system written in Rust.")
        .before_help(LOGO)
        .get_matches();
}

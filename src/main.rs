use clap::{Arg, Command};

pub mod models;

const LOGO: &str = ".
                    ____   ____   ____
                   | ()_\\ | ()_} (__  )
                   |_()_/ |_|    (____)";

fn main() {
    println!("csa, vilag!!");
    let matches = Command::new("Binary Packaging System")
        .version("0.1")
        .author("Susa Milán")
        .about("A simple binary packing system written in Rust.")
        .before_help(LOGO)
        .subcommand(
            Command::new("install").arg(
                // Force Reinstall
                Arg::new("force-reinstall")
                    .long("force-reinstall")
                    .short('r')
                    .required(false)
                    .help("Force BPS to reinstall already present packages on the system."),
            ),
        )
        .subcommand(
            Command::new("remove")
            .arg(Arg::new("packages")
                .required(true)
            )
            .arg(Arg::new("purge")
                .long("purge")
                .help("Remove all files and dependencies associated with the specified packages. IF ANOTHER PROGRAM REQUIRES THE DEPENDENCIES, THEN IT WILL NOT REMOVE THEM!")
            )
    )
        .get_matches();
}

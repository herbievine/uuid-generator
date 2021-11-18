use clap::{App, Arg};
use std::io::{Error, ErrorKind};
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

#[derive(Debug)]
enum UuidVersion {
    Version1,
    Version4,
}

fn main() {
    let matches = App::new("UUID Generator")
        .version("0.1.0")
        .author("Herbie VINE <vineherbie@gmail.com>")
        .about("Generates a UUID based on user input")
        .arg(
            Arg::with_name("version")
                .short("v")
                .long("version")
                .takes_value(true)
                .possible_values(&["1", "4"])
                .default_value("4")
                .help("UUID version"),
        )
        .get_matches();

    let version_str = matches.value_of("version").unwrap();

    let version: Result<UuidVersion, Error> = match version_str {
        "1" => Ok(UuidVersion::Version1),
        "4" => Ok(UuidVersion::Version4),
        &_ => Ok(UuidVersion::Version4),
    };

    let uuid = generate_uuid(version.unwrap());

    match uuid {
        Ok(res) => println!("Your unique identifier: {}", res),
        Err(_) => println!("There was an error generating the UUID"),
    }
}

fn generate_uuid(version: UuidVersion) -> Result<Uuid, Error> {
    match version {
        UuidVersion::Version1 => {
            println!("Generating UUID v1...");

            let context = Context::new(42);
            let ts = Timestamp::from_unix(&context, 1497624119, 1234);

            return Ok(Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6]).unwrap());
        }
        UuidVersion::Version4 => {
            println!("Generating UUID v4...");

            return Ok(Uuid::new_v4());
        }
    }
}

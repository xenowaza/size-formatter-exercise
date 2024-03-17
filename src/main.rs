use std::num::ParseIntError;
use std::{env, fmt};

struct Sizes {
    bytes: u64,
    kilobytes: u64,
    megabytes: u64,
    gigabytes: u64,
}

impl Sizes {
    fn new(digits: Result<u64, ParseIntError>, unit: &str) -> Self {
        let digits = match digits {
            Ok(digits) => digits,
            Err(e) => panic!(
                "Are you sure you entered your arguments correctly? Invalid number: {}",
                e
            ),
        };

        match unit {
            "b" => Self {
                bytes: digits,
                kilobytes: digits / 1000,
                megabytes: digits / 1_000_000,
                gigabytes: digits / 1_000_000_000,
            },
            "kb" => Self {
                bytes: digits * 1000,
                kilobytes: digits,
                megabytes: digits / 1000,
                gigabytes: digits / 1_000_000,
            },
            "mb" => Self {
                bytes: digits * 1_000_000,
                kilobytes: digits * 1000,
                megabytes: digits,
                gigabytes: digits / 1000,
            },
            "gb" => Self {
                bytes: digits * 1_000_000_000,
                kilobytes: digits * 1_000_000,
                megabytes: digits * 1000,
                gigabytes: digits,
            },
            _ => panic!("Invalid unit: {}", unit),
        }
    }
}

impl fmt::Debug for Sizes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sizes {{ bytes: \"{} bytes\", kilobytes: \"{} kilobytes\", megabytes: \"{} megabytes\"\
        , gigabytes: \"{} gigabytes\" }}", &self.bytes, &self.kilobytes, &self.megabytes, &self.gigabytes)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let size = args[1].split_whitespace().collect::<Vec<&str>>();
    let digits: Result<u64, ParseIntError> = size[0].parse();
    let units: &str = size[1];
    let sizes = Sizes::new(digits, units);
    println!("{:?}", sizes);
}

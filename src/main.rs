use clap::Parser;
use std::fmt;
use std::num::ParseIntError;
use std::process;

const B: u64 = 1;
const KIB: u64 = 1024 * B;
const MIB: u64 = 1024 * KIB;
const GIB: u64 = 1024 * MIB;
const TIB: u64 = 1024 * GIB;
const PIB: u64 = 1024 * TIB;

const KB: u64 = 1000 * B;
const MB: u64 = 1000 * KB;
const GB: u64 = 1000 * MB;
const TB: u64 = 1000 * GB;
const PB: u64 = 1000 * TB;

const SUFFIXES: &[(&str, u64)] = &[
    ("kib", KIB),
    ("mib", MIB),
    ("gib", GIB),
    ("tib", TIB),
    ("pib", PIB),
    ("kb", KB),
    ("mb", MB),
    ("gb", GB),
    ("tb", TB),
    ("pb", PB),
];

const SUFFIX_TABLE: &[(u64, &str)] = &[
    (PIB, "PiB"),
    (TIB, "TiB"),
    (GIB, "GiB"),
    (MIB, "MiB"),
    (KIB, "KiB"),
    (B, "B"),
];

const PREFIXES: &[(&str, u32)] = &[("0x", 16), ("0b", 2), ("0", 8)];

#[derive(Parser)]
struct Cli {
    value: String,
}

#[derive(Debug)]
enum ConvError {
    StringStripError(String),
    ParseError(ParseIntError),
}

impl From<ParseIntError> for ConvError {
    fn from(err: ParseIntError) -> ConvError {
        ConvError::ParseError(err)
    }
}

impl fmt::Display for ConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ConvError::StringStripError(ref err) => {
                write!(f, "Failed to strip string prefix/suffix: {}", err)
            }
            ConvError::ParseError(ref err) => write!(f, "Failed to parse input: {}", err),
        }
    }
}

impl PartialEq for ConvError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ConvError::StringStripError(_a), ConvError::StringStripError(_b)) => true,
            (ConvError::ParseError(_a), ConvError::ParseError(_b)) => true,
            _ => false,
        }
    }
}

fn split_multiplier(value: &String) -> Result<(u64, u64), ConvError> {
    let mut unit_multiplier: u64 = 1;
    let mut base_10_result: u64 = 0;
    let mut local_value: String = value.to_lowercase();

    for &suffix in SUFFIXES {
        if local_value.ends_with(suffix.0) {
            unit_multiplier = suffix.1;
            local_value = local_value
                .strip_suffix(suffix.0)
                .ok_or(ConvError::StringStripError(String::from(
                    "Failed to strip suffix",
                )))?
                .to_string();
            break;
        }
    }

    let mut prefix_matched: bool = false;
    for &prefix in PREFIXES {
        if local_value.starts_with(prefix.0) && local_value != "0" {
            local_value = local_value
                .strip_prefix(prefix.0)
                .ok_or(ConvError::StringStripError(String::from(
                    "Failed to strip prefix",
                )))?
                .to_string();

            base_10_result = u64::from_str_radix(local_value.as_str(), prefix.1)?;
            prefix_matched = true;
            break;
        }
    }

    if !prefix_matched {
        base_10_result = local_value.parse::<u64>()?;
    }

    return Ok((base_10_result, unit_multiplier));
}

fn human_readable(value: u64) -> String {
    let mut v: u64 = value;
    let mut result: String = "".to_owned();

    if value == 0 {
        return "0B".to_string();
    }

    for &suffix in SUFFIX_TABLE {
        if v >= suffix.0 {
            let val = v / suffix.0;
            v = v % suffix.0;
            let entry = format!("{}{} ", val, suffix.1);
            result.push_str(&entry);
        }
    }
    return result.trim().to_string();
}

fn main() {
    let args = Cli::parse();

    let (base, multiplier) = match split_multiplier(&args.value) {
        Ok((base, multiplier)) => (base, multiplier),
        Err(e) => {
            eprintln!("Failed to parse argument, Error: {:?}", e);
            process::exit(-1);
        }
    };

    println!("conversions:");
    let value = base * multiplier;
    println!("\tdec: {}", value);
    println!("\thex: {value:#x}");
    println!("\toct: {value:#o}");
    println!("\tbin: {value:#b}");
    println!("");
    println!("{}", human_readable(value));
}

#[test]
fn split_test() {
    // Parse Trivial
    assert_eq!(split_multiplier(&"123".to_string()), Ok((123, 1)));
    assert_eq!(split_multiplier(&"0".to_string()), Ok((0, 1)));
    assert_eq!(split_multiplier(&"0x0".to_string()), Ok((0, 1)));
    assert_eq!(split_multiplier(&"00".to_string()), Ok((0, 1)));
    assert_eq!(split_multiplier(&"0b0".to_string()), Ok((0, 1)));

    // Parse Suffixes Trivial
    assert_eq!(split_multiplier(&"1KIB".to_string()), Ok((1, 1024)));
    assert_eq!(split_multiplier(&"1gib".to_string()), Ok((1, GIB)));
    assert_eq!(split_multiplier(&"1mib".to_string()), Ok((1, MIB)));
    assert_eq!(split_multiplier(&"1tib".to_string()), Ok((1, TIB)));
    assert_eq!(split_multiplier(&"1pib".to_string()), Ok((1, PIB)));

    // Parse Suffixes nontrivial
    assert_eq!(split_multiplier(&"11KIB".to_string()), Ok((11, 1024)));
    assert_eq!(split_multiplier(&"12gib".to_string()), Ok((12, GIB)));
    assert_eq!(split_multiplier(&"13miB".to_string()), Ok((13, MIB)));
    assert_eq!(split_multiplier(&"14TiB".to_string()), Ok((14, TIB)));
    assert_eq!(split_multiplier(&"15pib".to_string()), Ok((15, PIB)));

    // Base conversion.
    assert_eq!(split_multiplier(&"0x10".to_string()), Ok((16, 1)));
    assert_eq!(split_multiplier(&"010".to_string()), Ok((8, 1)));
    assert_eq!(split_multiplier(&"0b10".to_string()), Ok((2, 1)));

    // Base conversion with suffixes.
    assert_eq!(split_multiplier(&"0x10mib".to_string()), Ok((16, MIB)));
    assert_eq!(split_multiplier(&"010kib".to_string()), Ok((8, KIB)));
    assert_eq!(split_multiplier(&"0b10gib".to_string()), Ok((2, GIB)));

    // Bad parses and parse failure.
    assert!(split_multiplier(&"15a".to_string()).is_err());
    assert!(split_multiplier(&"".to_string()).is_err());
    assert!(split_multiplier(&"0x15z".to_string()).is_err());
    assert!(split_multiplier(&"0x15kbkb".to_string()).is_err());
}

#[test]
fn human_readable_test() {
    assert_eq!(human_readable(KIB), "1KiB");
    assert_eq!(human_readable(MIB), "1MiB");
    assert_eq!(human_readable(GIB), "1GiB");
    assert_eq!(human_readable(TIB), "1TiB");
    assert_eq!(human_readable(PIB), "1PiB");

    assert_eq!(human_readable(3 * KIB + 5 * MIB + 7), "5MiB 3KiB 7B");
    assert_eq!(human_readable(7), "7B");
    assert_eq!(human_readable(0), "0B");
}

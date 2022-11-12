use clap::Parser;
use rand::Rng;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Uppercase
    #[arg(short, default_value_t = false)]
    pub upper: bool,
    /// Lowercase
    #[arg(short, default_value_t = false)]
    pub lower: bool,
    /// Number
    #[arg(short, default_value_t = false)]
    pub number: bool,
    /// Special character
    #[arg(short, default_value_t = false)]
    pub special: bool,
    /// Extended special character
    #[arg(short, default_value_t = false)]
    pub extend: bool,
    /// Password length
    #[arg(long, default_value_t = 32, value_parser = clap::value_parser!(u32).range(1..))]
    pub length: u32,
    /// password count
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..))]
    pub count: u32,
}

const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBER: &[u8] = b"0123456789";
const SPECIAL: &[u8] = b"*&^%$#@!";
const EXTENDED: &[u8] = b"~`()_-+={[}]|\\:;\"'<,>.?/";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args::parse();
    if !args.lower && !args.upper && !args.number && !args.special && !args.extend {
        args.lower = true;
        args.upper = true;
        args.number = true;
    }
    let mut charset: Vec<u8> = Vec::new();
    if args.lower {
        charset.extend(LOWER);
    }
    if args.upper {
        charset.extend(UPPER);
    }
    if args.number {
        charset.extend(NUMBER);
    }
    if args.special {
        charset.extend(SPECIAL);
    }
    if args.extend {
        charset.extend(EXTENDED);
    }
    while args.count > 0 {
        let mut rng = rand::thread_rng();
        println!(
            "{}",
            (0..args.length)
                .map(|_| charset[rng.gen_range(0..charset.len())] as char)
                .collect::<String>()
        );
        args.count -= 1;
    }
    Ok(())
}

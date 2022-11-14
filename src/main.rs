use std::{error, fmt::Write, fs::File, io, path::PathBuf, process::exit};

use clap::Parser;
use digest::Digest;
use colored::*;

#[derive(Parser)]
#[command(
    about = "A simple file hash calculator\n\
            Current supported algorithms: md2 md4 md5 sha1 sha256 sha384 sha512 blake2", 
    long_about = None
)]
struct Cli {
    /// The hash algorithm.
    algorithm: String,

    /// The file path.
    filepath: PathBuf,
}

fn main() {
    let Cli {
        algorithm,
        filepath,
    } = Cli::parse();
    let result = match algorithm.to_lowercase().as_str() {
        // hash
        "md2" => hash::<md2::Md2>(filepath),
        "md4" => hash::<md4::Md4>(filepath),
        "md5" => hash::<md5::Md5>(filepath),
        "sha1" => hash::<sha1::Sha1>(filepath),
        "sha256" => hash::<sha2::Sha256>(filepath),
        "sha384" => hash::<sha2::Sha384>(filepath),
        "sha512" => hash::<sha2::Sha512>(filepath),
        "blake2" => hash::<blake2::Blake2b512>(filepath),
        _ => {
            eprintln!(
                "{}: unknown or unsupported algorithm\n\
                Current supported algorithms: md2 md4 md5 sha1 sha256 sha384 sha512 blake2",
                "error".red().bold()
            );
            exit(1)
        }
    };

    match result {
        Ok(s) => {
            println!("{}", s);
        },
        Err(e) => {
            eprintln!("{}: {}", "error".red().bold(), e);
            exit(1);
        }
    }
}

fn hash<T: Digest + io::Write>(filepath: PathBuf) -> Result<String, Box<dyn error::Error>> {
    let mut file = File::open(filepath)?;
    let mut hasher = T::new();
    io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();

    let mut result = String::new();
    for i in hash {
        write!(result, "{:02x}", i)?;
    }
    Ok(result)
}

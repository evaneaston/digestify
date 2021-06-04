use clap::{App, Arg};
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;
mod algos;
use crate::algos::{Algorithm, CalculatedDigest, CRC32, SHA1, MD5, SHA256, SHA512};

fn is_valid_hex_string(hash: String) -> Result<(), String> {
    match hex::decode(hash) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from(
            "The provided has does not seem to be a valid hexadecimal string.",
        )),
    }
}

fn is_readable_file(file_name: String) -> Result<(), String> {
    if Path::new(&file_name).exists() {
        return Ok(());
    }
    Err(String::from(format!("File {} cannot be found", file_name)))
}

fn hex_len_to_size_description(hex_len: usize) -> String {
    format!("{} hex chars / {} bits", hex_len, hex_len_to_bit_len(hex_len))
}

fn hex_len_to_bit_len(hex_len: usize) -> usize {
    hex_len * 4
}

fn to_algorithm_name_list(algorithms: &Vec<Algorithm>) -> String {
    algorithms.into_iter().map(|a| a.name).collect::<Vec<&str>>().join(", ")
}

fn config_app<'a>() -> App<'a, 'a> {
    const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    App::new(APP_NAME)
        .version(VERSION)
        .about("Verify a file against a digest/checksum/hash.")
        .arg(
            Arg::with_name("FILE")
                .help("File to verify")
                .required(true)
                .validator(is_readable_file),
        )
        .arg(
            Arg::with_name("DIGEST")
                .help("Digest to compare to.  Must be specified as a hexadecimal string.  Case does not matter.")
                .required(true)
                .validator(is_valid_hex_string),
        )
}

fn find_candidates_based_on_digest_length<'a>(
    supported_algorithms: &'a Vec<Algorithm<'a>>,
    provided: &str,
) -> Vec<Algorithm<'a>> {
    let candidate_algorithms: Vec<Algorithm> = supported_algorithms
        .iter()
        .filter(|a| hex_len_to_bit_len(provided.len()) == a.digest_bit_size.into())
        .map(|a| *a)
        .collect();
    if candidate_algorithms.len() == 0 {
        eprintln!(
            "No supported algorithms for digest of size {}",
            hex_len_to_size_description(provided.len())
        );
        std::process::exit(-1);
    }
    candidate_algorithms
}

fn calculate_digests(file_name: &str, candidate_algorithms: &Vec<Algorithm>) -> Vec<CalculatedDigest> {
    let calculated_digests_results: Result<Vec<CalculatedDigest>, Error> = candidate_algorithms
        .into_iter()
        .map(|a| calculate_digest(file_name, &a))
        .collect();
    match calculated_digests_results {
        Ok(ds) => ds,
        Err(err) => {
            eprintln!("Unexpected error: {}", err);
            std::process::exit(-2);
        }
    }
}

struct DigestComparison<'a> {
    calculated: &'a CalculatedDigest,
    matches: bool,
}

fn compare_digests<'a>(provided: &str, calculated: &'a CalculatedDigest) -> DigestComparison<'a> {
    DigestComparison {
        calculated: &calculated,
        matches: provided.eq_ignore_ascii_case(&calculated.digest),
    }
}

fn calculate_digest<'a>(file_name: &str, algorithm: &'a Algorithm) -> Result<CalculatedDigest, Error> {
    let mut file = File::open(file_name)?;
    let metadata = file.metadata()?;

    let dresult = algorithm.digest(&mut file);
    let d = match dresult {
        Ok(digest) => digest,
        Err(err) => return Err(err),
    };

    if metadata.len() != d.bytes_read.into() {
        let msg = format!(
            "Wasn't able to read full file length of {} bytes.  Only read {}.",
            metadata.len(),
            d.bytes_read
        );
        return Err(Error::new(ErrorKind::Other, msg));
    }

    Ok(d)
}

fn main() -> Result<(), Error> {
    let supported_algorithms: Vec<Algorithm> = vec![CRC32::new(), SHA1::new(), MD5::new(), SHA256::new(), SHA512::new()];

    let app = config_app();
    let matches = app.get_matches();
    let file_name = matches.value_of("FILE").unwrap();
    let provided = matches.value_of("DIGEST").unwrap();

    let candidate_algorithms = find_candidates_based_on_digest_length(&supported_algorithms, &provided);

    println!(
        "\nVerifying '{}' against provided digest of size {}.  Candidate digest(s): {}.",
        file_name,
        hex_len_to_size_description(provided.len()),
        to_algorithm_name_list(&candidate_algorithms)
    );

    let calculated_digests = calculate_digests(&file_name, &candidate_algorithms);

    let mut has_match = false;
    for calculated in calculated_digests {
        let comparison = compare_digests(provided, &calculated);

        if comparison.matches {
            has_match = true;
        }

        println!(
            "\n {}: {}    ",
            comparison.calculated.algorithm_name,
            if comparison.matches { "MATCHES" } else { "FAIL" }
        );
        println!("\tExpected={}\n\t  Actual={}", provided.to_ascii_lowercase(), comparison.calculated.digest);
    }
    if !has_match {
        eprintln!("\nFAIL: Provided digest doesn't match any of the candidate digest results.");
        std::process::exit(-2);
    }

    Ok(())
}

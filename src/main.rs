// SPDX-License-Identifier: Apache-2.0 OR MIT-0

use clap::Parser;
use digestify::crc32::Crc32;
use digestify::md5::Md5;
use digestify::sha1::Sha1;
use digestify::sha256::Sha256;
use digestify::sha512::Sha512;
use digestify::{Algorithm, CalculatedDigest};
use std::fs::File;
use std::io::{Error, ErrorKind};

mod args;
use args::DigestifyArgs;

fn hex_len_to_size_description(hex_len: usize) -> String {
    format!("{} hex chars / {} bits", hex_len, hex_len_to_bit_len(hex_len))
}

fn hex_len_to_bit_len(hex_len: usize) -> usize {
    hex_len * 4
}

fn to_algorithm_name_list(algorithms: &Vec<Algorithm>) -> String {
    algorithms.into_iter().map(|a| a.name).collect::<Vec<&str>>().join(", ")
}

fn find_candidates_based_on_digest_length<'a>(
    supported_algorithms: &'a Vec<Algorithm<'a>>,
    provided: &str,
) -> Result<Vec<Algorithm<'a>>, Error> {
    let candidate_algorithms: Vec<Algorithm> = supported_algorithms
        .iter()
        .filter(|a| hex_len_to_bit_len(provided.len()) == a.digest_bit_size.into())
        .map(|a| *a)
        .collect();
    if candidate_algorithms.len() == 0 {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "No supported algorithms for digest of size {}",
                hex_len_to_size_description(provided.len())
            ),
        ));
    }
    Ok(candidate_algorithms)
}

fn calculate_digests(file_name: &str, candidate_algorithms: &Vec<Algorithm>) -> Result<Vec<CalculatedDigest>, Error> {
    candidate_algorithms
        .into_iter()
        .map(|a| calculate_file_digest(file_name, &a))
        .collect()
}

struct DigestComparison<'a> {
    provided: &'a str,
    calculated: &'a CalculatedDigest,
    matches: bool,
}

fn compare_digests<'a>(provided: &'a str, calculated: &'a CalculatedDigest) -> DigestComparison<'a> {
    DigestComparison {
        provided: provided,
        calculated: &calculated,
        matches: provided.eq_ignore_ascii_case(&calculated.digest),
    }
}

fn calculate_file_digest<'a>(file_name: &str, algorithm: &'a Algorithm) -> Result<CalculatedDigest, Error> {
    let mut file = File::open(file_name)?;
    let metadata = file.metadata()?;
    let d = algorithm.digest(&mut file)?;
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
    let args = DigestifyArgs::parse();

    let file_name = args.file_name;
    let provided = args.digest;

    let supported_algorithms: Vec<Algorithm> =
        vec![Crc32::new(), Md5::new(), Sha1::new(), Sha256::new(), Sha512::new()];

    let candidate_algorithms = find_candidates_based_on_digest_length(&supported_algorithms, &provided)?;

    println!(
        "\nVerifying '{}' against provided digest of size {}.  Candidate digest(s): {}.",
        file_name,
        hex_len_to_size_description(provided.len()),
        to_algorithm_name_list(&candidate_algorithms)
    );

    let mut match_count = 0;
    for calculated in calculate_digests(&file_name, &candidate_algorithms)? {
        let comparison = compare_digests(&provided, &calculated);

        let match_string = match comparison.matches {
            true => {
                match_count += 1;
                "PASS"
            }
            false => "FAIL",
        };

        println!("\n {}: {}    ", comparison.calculated.algorithm_name, match_string);
        println!(
            "\tExpected={}\n\t  Actual={}",
            comparison.provided.to_ascii_lowercase(),
            comparison.calculated.digest
        );
    }
    if match_count == 0 {
        eprintln!("\nFAIL: Provided digest doesn't match any of the candidate digest results.");
        std::process::exit(2);
    } else {
        eprintln!("\nPASS: Provided digest matches the content.");
    }
    Ok(())
}

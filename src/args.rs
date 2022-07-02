// SPDX-License-Identifier: Apache-2.0 OR MIT-0

use clap::Parser;
use std::path::Path;

pub fn valid_hex_string(hash: &str) -> Result<String, String> {
    match hex::decode(hash) {
        Ok(_) => Ok(hash.to_string()),
        Err(_) => Err(String::from(
            "The provided has does not seem to be a valid hexadecimal string.",
        )),
    }
}

pub fn readable_files_name(file_name: &str) -> Result<String, String> {
    if Path::new(&file_name).exists() {
        return Ok(file_name.to_string());
    }
    Err(String::from(format!("File '{}' cannot be found", file_name)))
}

#[derive(Parser)]
#[clap(version, about)]
pub struct DigestifyArgs {
    /// File to verify
    #[clap(name="file", value_parser=readable_files_name)]
    pub file_name: String,

    /// Digest to compare to, specified as a case-insensitive hexadecimal string.
    #[clap(name="digest", value_parser=valid_hex_string)]
    pub digest: String,
}

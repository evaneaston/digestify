//
// Copyright 2021 3nav3
// SPDX-License-Identifier: AGPL-3.0-only
//
use super::{Algorithm, CalculatedDigest};
use std::io::{Read, Result};
use digest::Digest;

fn sha512digest(a: &Algorithm, read: &mut dyn Read) -> Result<CalculatedDigest> {
    let mut d = sha2::Sha512::new();
    let bytes_read = std::io::copy(read, &mut d);
    bytes_read.map(|br| CalculatedDigest {
        algorithm_name: String::from(a.name).clone(),
        bytes_read: br,
        digest: hex::encode(d.finalize()),
    })
}

pub struct Sha512 {}

impl Sha512 {
    pub fn new() -> Algorithm<'static> {
        Algorithm {
            digest_bit_size: 512,
            name: "SHA-512",
            digest_fn: sha512digest,
        }
    }
}

#[cfg(test)]
#[test]
fn sha512_computes_correct_value() {
    let cd = Sha512::new().digest(&mut "bomb jail carve page".as_bytes()).unwrap();
    assert_eq!(cd.bytes_read, 20);
    assert_eq!(cd.algorithm_name, "SHA-512");
    assert_eq!(cd.digest, "0eab7d2bf7ca3de11946249dba2be8ddfd48dc77e976b0278b63b9a48ecd3db3a530a5c220157bba89d0bea2ed337905082575ead62ee9e4409921dbfad2ddf9");
}

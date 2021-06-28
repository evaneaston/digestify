//
// Copyright 2021 3nav3
// SPDX-License-Identifier: AGPL-3.0-only
//

use crate::{to_calculated_digest, Algorithm, CalculatedDigest};
use digest::Digest;
use std::io::{Read, Result};

fn sha256(a: &Algorithm, read: &mut dyn Read) -> Result<CalculatedDigest> {
    let mut d = sha2::Sha256::new();
    let bytes_read = std::io::copy(read, &mut d);
    to_calculated_digest(a, bytes_read, &d.finalize())
}

pub struct Sha256 {}

impl Sha256 {
    pub fn new() -> Algorithm<'static> {
        Algorithm {
            digest_bit_size: 256,
            name: "SHA-256",
            digest_fn: sha256,
        }
    }
}

#[cfg(test)]
#[test]
fn sha256_computes_correct_value() {
    let cd = Sha256::new().digest(&mut "data jump apple expose".as_bytes()).unwrap();
    assert_eq!(cd.bytes_read, 22);
    assert_eq!(cd.algorithm_name, "SHA-256");
    assert_eq!(
        cd.digest,
        "ce30b449ab38286971ea9edffb3e45891f137af0040d28463b35466a4329616d"
    );
}

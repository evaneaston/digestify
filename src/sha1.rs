//
// Copyright 2021 3nav3
// SPDX-License-Identifier: AGPL-3.0-only
//

use crate::{Algorithm, CalculatedDigest};
use std::io::{Read, Result};
use digest::Digest;

fn sha1digest(a: &Algorithm, read: &mut dyn Read) -> Result<CalculatedDigest> {
    let mut d = sha1::Sha1::new();
    let bytes_read = std::io::copy(read, &mut d);
    bytes_read.map(|br| CalculatedDigest {
        algorithm_name: String::from(a.name).clone(),
        bytes_read: br,
        digest: hex::encode(d.finalize()),
    })
}

pub struct Sha1 {}

impl Sha1 {
    pub fn new() -> Algorithm<'static> {
        Algorithm {
            digest_bit_size: 160,
            name: "SHA-1",
            digest_fn: sha1digest,
        }
    }
}

#[cfg(test)]
#[test]
fn sha1_computes_correct_value() {
    let cd = Sha1::new().digest(&mut "bottom oral strain dna".as_bytes()).unwrap();
    assert_eq!(cd.bytes_read, 22);
    assert_eq!(cd.algorithm_name, "SHA-1");
    assert_eq!(cd.digest, "449efd26253a16d5e35c41c99fcc606fad9006dd");
}
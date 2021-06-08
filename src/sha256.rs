//
// Copyright 2021 3nav3
// SPDX-License-Identifier: AGPL-3.0-only
//

use sha2::Sha256;
use crate::{Algorithm, CalculatedDigest};
use std::io::{Read, Result};
use digest::Digest;


fn sha256(a: &Algorithm, read: &mut dyn Read) -> Result<CalculatedDigest> {
    let mut d = Sha256::new();
    let bytes_read = std::io::copy(read, &mut d);
    bytes_read.map(|br| CalculatedDigest {
        algorithm_name: String::from(a.name).clone(),
        bytes_read: br,
        digest: hex::encode(d.finalize()),
    })
}

pub struct SHA256 {}

impl SHA256 {
    pub fn new() -> Algorithm<'static> {
        Algorithm {
            digest_bit_size: 256,
            name: "SHA-256",
            digest_fn: sha256,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sha256_works() {
        let cd = super::SHA256::new().digest(&mut "ABCDE".as_bytes()).unwrap();
        assert_eq!(cd.bytes_read, 5);
        assert_eq!(cd.algorithm_name, "SHA-256");
        assert_eq!(cd.digest, "f0393febe8baaa55e32f7be2a7cc180bf34e52137d99e056c817a9c07b8f239a");
    }
}

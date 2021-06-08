//
// Copyright 2021 3nav3
// SPDX-License-Identifier: AGPL-3.0-only
//

use sha2::Sha512;
use crate::{Algorithm, CalculatedDigest};
use std::io::{Read, Result};
use digest::Digest;

fn sha512digest(a: &Algorithm, read: &mut dyn Read) -> Result<CalculatedDigest> {
    let mut d = Sha512::new();
    let bytes_read = std::io::copy(read, &mut d);
    bytes_read.map(|br| CalculatedDigest {
        algorithm_name: String::from(a.name).clone(),
        bytes_read: br,
        digest: hex::encode(d.finalize()),
    })
}

pub struct SHA512 {}

impl SHA512 {
    pub fn new() -> Algorithm<'static> {
        Algorithm {
            digest_bit_size: 512,
            name: "SHA-512",
            digest_fn: sha512digest,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sha512_works() {
        let cd = super::SHA512::new().digest(&mut "ABCDE".as_bytes()).unwrap();
        assert_eq!(cd.bytes_read, 5);
        assert_eq!(cd.algorithm_name, "SHA-512");
        assert_eq!(cd.digest, "9989a8fcbc29044b5883a0a36c146fe7415b1439e995b4d806ea0af7da9ca4390eb92a604b3ecfa3d75f9911c768fbe2aecc59eff1e48dcaeca1957bdde01dfb");
    }
}
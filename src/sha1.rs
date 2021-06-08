//
// Copyright 2021 3nav3
// SPDX-License-Identifier: AGPL-3.0-only
//

use sha1::Sha1;
use crate::{Algorithm, CalculatedDigest};
use std::io::{Read, Result};
use digest::Digest;

fn sha1digest(a: &Algorithm, read: &mut dyn Read) -> Result<CalculatedDigest> {
    let mut d = Sha1::new();
    let bytes_read = std::io::copy(read, &mut d);
    bytes_read.map(|br| CalculatedDigest {
        algorithm_name: String::from(a.name).clone(),
        bytes_read: br,
        digest: hex::encode(d.finalize()),
    })
}

pub struct SHA1 {}

impl SHA1 {
    pub fn new() -> Algorithm<'static> {
        Algorithm {
            digest_bit_size: 160,
            name: "SHA-1",
            digest_fn: sha1digest,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sha1_works() {
        let cd = super::SHA1::new().digest(&mut "ABCDE".as_bytes()).unwrap();
        assert_eq!(cd.bytes_read, 5);
        assert_eq!(cd.algorithm_name, "SHA-1");
        assert_eq!(cd.digest, "7be07aaf460d593a323d0db33da05b64bfdcb3a5");
    }
}
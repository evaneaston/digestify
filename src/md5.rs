//
// Copyright 2021 3nav3
// SPDX-License-Identifier: AGPL-3.0-only
//

use md5::Md5;
use crate::{Algorithm, CalculatedDigest};
use std::io::{Read, Result};
use digest::Digest;

fn md5digest(a: &Algorithm, read: &mut dyn Read) -> Result<CalculatedDigest> {
    let mut d = Md5::new();
    let bytes_read = std::io::copy(read, &mut d);
    bytes_read.map(|br| CalculatedDigest {
        algorithm_name: String::from(a.name).clone(),
        bytes_read: br,
        digest: hex::encode(d.finalize()),
    })
}

pub struct MD5 {}

impl MD5 {
    pub fn new() -> Algorithm<'static> {
        Algorithm {
            digest_bit_size: 128,
            name: "MD5",
            digest_fn: md5digest,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn md5_works() {
        let cd = super::MD5::new().digest(&mut "ABCDE".as_bytes()).unwrap();
        assert_eq!(cd.bytes_read, 5);
        assert_eq!(cd.algorithm_name, "MD5");
        assert_eq!(cd.digest, "2ecdde3959051d913f61b14579ea136d");
    }
}
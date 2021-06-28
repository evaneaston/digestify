//
// Copyright 2021 3nav3
// SPDX-License-Identifier: AGPL-3.0-only
//

use crate::{to_calculated_digest, Algorithm, CalculatedDigest};
use digest::Digest;
use std::io::{Read, Result};

fn md5digest(a: &Algorithm, read: &mut dyn Read) -> Result<CalculatedDigest> {
    let mut d = md5::Md5::new();
    let bytes_read = std::io::copy(read, &mut d);
    to_calculated_digest(a, bytes_read, &d.finalize())
}

pub struct Md5 {}

impl Md5 {
    pub fn new() -> Algorithm<'static> {
        Algorithm {
            digest_bit_size: 128,
            name: "MD5",
            digest_fn: md5digest,
        }
    }
}

#[cfg(test)]
#[test]
fn md5_computes_correct_value() {
    let cd = Md5::new().digest(&mut "heal shake auto bar blast".as_bytes()).unwrap();
    assert_eq!(cd.bytes_read, 25);
    assert_eq!(cd.algorithm_name, "MD5");
    assert_eq!(cd.digest, "2722d38fcedba2f3277b736d5f41185e");
}

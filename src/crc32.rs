//
// Copyright 2021 3nav3
// SPDX-License-Identifier: AGPL-3.0-only
//

use crate::{to_calculated_digest, Algorithm, CalculatedDigest};
use digest::Update;
use std::io::{Read, Result, Write};

struct Crc32Digest {
    hasher: crc32fast::Hasher,
}

impl Update for Crc32Digest {
    fn update(&mut self, input: impl AsRef<[u8]>) {
        self.hasher.update(input.as_ref());
    }
}

impl Write for Crc32Digest {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        digest::Update::update(self, buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

fn crc32digest(a: &Algorithm, read: &mut dyn Read) -> Result<CalculatedDigest> {
    let mut copier = Crc32Digest {
        hasher: crc32fast::Hasher::new(),
    };
    let bytes_read = std::io::copy(read, &mut copier);
    to_calculated_digest(a, bytes_read, &copier.hasher.finalize().to_be_bytes())
}

pub struct Crc32 {}

impl Crc32 {
    pub fn new() -> super::Algorithm<'static> {
        super::Algorithm {
            digest_bit_size: 32,
            name: "CRC-32",
            digest_fn: crc32digest,
        }
    }
}

#[cfg(test)]
#[test]
fn crc32_works() {
    let cd = Crc32::new()
        .digest(&mut "home brick limb heal prefer".as_bytes())
        .unwrap();
    assert_eq!(cd.bytes_read, 27);
    assert_eq!(cd.algorithm_name, "CRC-32");
    assert_eq!(cd.digest, "974d7be7");
}

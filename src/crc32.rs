//
// Copyright 2021 3nav3
// SPDX-License-Identifier: AGPL-3.0-only
//

use crate::{Algorithm, CalculatedDigest};
use std::io::{Read, Result, Write};
use digest::{Update};

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
    let mut copier = Crc32Digest { hasher: crc32fast::Hasher::new() };
    let bytes_read = std::io::copy(read, &mut copier);

    bytes_read.map(|br| super::CalculatedDigest {
        algorithm_name: String::from(a.name).clone(),
        bytes_read: br,
        digest: hex::encode(copier.hasher.finalize().to_be_bytes()),
    })
}

pub struct CRC32 {}

impl CRC32 {
    pub fn new() -> super::Algorithm<'static> {
        super::Algorithm {
            digest_bit_size: 32,
            name: "CRC32",
            digest_fn: crc32digest,
        }
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn crc32_works() {
        let cd = super::CRC32::new().digest(&mut "ABCDE".as_bytes()).unwrap();
        assert_eq!(cd.bytes_read, 5);
        assert_eq!(cd.algorithm_name, "CRC32");
        assert_eq!(cd.digest, "72d31ad5");
    }
}
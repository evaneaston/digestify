//
// Copyright 2021 3nav3
// SPDX-License-Identifier: AGPL-3.0-only
//

use digest::Digest;
use md5::Md5;
use sha2::{Sha256, Sha512};
use sha1::{Sha1};
use std::io::{Read, Result, Write};

pub struct CalculatedDigest {
    pub bytes_read: u64,
    pub algorithm_name: String,
    pub digest: String,
}

#[derive(Copy, Clone)]
pub struct Algorithm<'a> {
    pub digest_bit_size: u16,
    pub name: &'a str,
    digest_fn: fn(&Algorithm, &mut dyn Read) -> Result<CalculatedDigest>,
}

impl<'a> Algorithm<'a> {
    pub fn digest(&'a self, read: &mut dyn Read) -> Result<CalculatedDigest> {
        (self.digest_fn)(&self, read)
    }
}

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

struct Copier<'a> {
    hasher: &'a mut crc32fast::Hasher,
}
impl<'a> Write for Copier<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.hasher.update(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

fn crc32digest(a: &Algorithm, read: &mut dyn Read) -> Result<CalculatedDigest> {
    let mut d = crc32fast::Hasher::new();
    let mut copier = Copier { hasher: &mut d };
    let bytes_read = std::io::copy(read, &mut copier);

    bytes_read.map(|br| CalculatedDigest {
        algorithm_name: String::from(a.name).clone(),
        bytes_read: br,
        digest: hex::encode(d.finalize().to_be_bytes()),
    })
}
pub struct CRC32 {}
impl CRC32 {
    pub fn new() -> Algorithm<'static> {
        Algorithm {
            digest_bit_size: 32,
            name: "CRC32",
            digest_fn: crc32digest,
        }
    }
}

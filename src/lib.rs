//
// Copyright 2021 3nav3
// SPDX-License-Identifier: AGPL-3.0-only
//

use std::io::{Read, Result};

#[derive(Clone)]
pub struct CalculatedDigest {
    pub bytes_read: u64,
    pub algorithm_name: String,
    pub digest: String,
}

//
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

fn to_calculated_digest(a: &Algorithm, bytes_read: Result<u64>, dr: &[u8]) -> Result<CalculatedDigest> {
    bytes_read.map(|br| CalculatedDigest {
        algorithm_name: String::from(a.name).clone(),
        bytes_read: br,
        digest: hex::encode(dr),
    })
}

pub mod crc32;
pub mod md5;
pub mod sha1;
pub mod sha256;
pub mod sha512;

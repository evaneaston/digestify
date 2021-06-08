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

pub mod crc32;
pub mod md5;
pub mod sha1;
pub mod sha256;
pub mod sha512;
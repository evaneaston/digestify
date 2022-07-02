// SPDX-License-Identifier: Apache-2.0 OR MIT-0

use std::io::{Read, Result, Write};

use digest::Digest;

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

fn calculated_digest<D: Digest + Write>(a: &Algorithm, read: &mut dyn Read) -> Result<CalculatedDigest> {
    let mut digest = D::new();
    let bytes_read = std::io::copy(read, &mut digest);
    bytes_read.map(|br| CalculatedDigest {
        algorithm_name: String::from(a.name).clone(),
        bytes_read: br,
        digest: hex::encode(digest.finalize().as_slice()),
    })
}

fn to_calculated_digest(a: &Algorithm, bytes_read: Result<u64>, dr: &[u8]) -> Result<CalculatedDigest> {
    bytes_read.map(|br| CalculatedDigest {
        algorithm_name: String::from(a.name).clone(),
        bytes_read: br,
        digest: hex::encode(dr),
    })
}

#[cfg(test)]
mod tests {
    use crate::Algorithm;

    pub fn test_algorithm(
        algorithm: &Algorithm,
        data: &mut dyn std::io::Read,
        expected_length: u64,
        expected_value: &str,
    ) {
        let calculated = algorithm.digest(data).unwrap();
        assert_eq!(calculated.bytes_read, expected_length);
        assert_eq!(calculated.algorithm_name, algorithm.name);
        assert_eq!(
            calculated.digest, expected_value,
            "Expected {}, got {}",
            expected_value, calculated.digest
        );
    }

    #[macro_export]
    macro_rules! test_algorithm {
        ($algorithm:ident, $name:ident, $input:expr, $expected_digest:expr) => {
            #[cfg(test)]
            #[test]
            fn $name() {
                println!("'{:?}'", $input);
                crate::tests::test_algorithm(
                    &(<$algorithm>::new()),
                    &mut std::io::Cursor::new($input),
                    $input.len() as u64,
                    $expected_digest,
                )
            }
        };
    }

    #[macro_export]
    macro_rules! test_algorithm_s {
        ($algorithm:ident, $name:ident, $str:expr, $expected_digest:expr) => {
            #[test]
            fn $name() {
                crate::tests::test_algorithm(
                    &(<$algorithm>::new()),
                    &mut ($str).as_bytes(),
                    $str.len() as u64,
                    $expected_digest,
                )
            }
        };
    }

    pub const U8_EMPTY: [u8; 0] = [0x00; 0];
    pub const U8_32_ALL_ZEROS: [u8; 32] = [0x00; 32];
    pub const U8_32_HALF_ONES: [u8; 32] = [0x0f; 32];
    pub const U8_32_ALL_ONES: [u8; 32] = [0xff; 32];
    pub const U8_32_ASCENDING: [u8;32] = *b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\x20";
    pub const U8_32_DESCENDING: [u8;32] = *b"\x20\x1F\x1E\x1D\x1C\x1B\x1A\x19\x18\x17\x16\x15\x14\x13\x12\x11\x10\x0F\x0E\x0D\x0C\x0B\x0A\x09\x08\x07\x06\x05\x04\x03\x02\x01";
    pub const U8_1_BYTE: [u8; 1] = [0x65];
    pub const U8_2_BYTES: [u8; 2] = [0xac, 0x23];
    pub const U8_3_BYTES: [u8; 3] = [0xbc, 0x33, 0xe2];
    pub const U8_4_BYTES: [u8; 4] = [0xe2, 0xf3, 0xa1, 0xdf];
    pub const S_HI: &str = "hi";
    pub const S_QUICK_BROWN_FOX: &str = "The quick brown fox jumps over 13 lazy dogs.";
    pub const S_QUICK_BROWN_FOX_W_EOL: &str = "The quick brown fox jumps over 13 lazy dogs.\n";
    pub const S_QEOIC_SMOUAU_POSWUC: &str = "Qeoic-Smouau-Poswuc";
}

pub mod crc32;
pub mod md5;
pub mod sha1;
pub mod sha256;
pub mod sha512;

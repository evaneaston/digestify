// SPDX-License-Identifier: Apache-2.0 OR MIT-0

use crate::calculated_digest;
use digest::{consts::U4, FixedOutput, HashMarker, Output, OutputSizeUser, Update};
use std::io::Write;

struct Crc32Digest {
    hasher: crc32fast::Hasher,
}

impl Default for Crc32Digest {
    fn default() -> Self {
        Self {
            hasher: crc32fast::Hasher::new(),
        }
    }
}
impl HashMarker for Crc32Digest {}

impl OutputSizeUser for Crc32Digest {
    type OutputSize = U4;
}

impl FixedOutput for Crc32Digest {
    fn finalize_into(self, out: &mut Output<Self>) {
        let beb = self.hasher.finalize().to_be_bytes();
        out.clone_from_slice(&beb);
    }
}

impl Update for Crc32Digest {
    fn update(&mut self, input: &[u8]) {
        self.hasher.update(input);
    }
}

impl Write for Crc32Digest {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        digest::Update::update(self, buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct Crc32 {}

impl Crc32 {
    pub fn new() -> super::Algorithm<'static> {
        super::Algorithm {
            digest_bit_size: 32,
            name: "CRC-32",
            digest_fn: calculated_digest::<Crc32Digest>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Crc32;
    use crate::{test_algorithm, test_algorithm_s, tests::*};
    test_algorithm!(Crc32, empty, U8_EMPTY, "00000000");
    test_algorithm!(Crc32, thirty_two_bytes_of_all_zeros, U8_32_ALL_ZEROS, "190a55ad");
    test_algorithm!(Crc32, thirty_two_bytes_of_half_ones, U8_32_HALF_ONES, "cce47b41");
    test_algorithm!(Crc32, thirty_two_bytes_of_all_ones, U8_32_ALL_ONES, "ff6cab0b");
    test_algorithm!(Crc32, one_byte, U8_1_BYTE, "efda7a5a");
    test_algorithm!(Crc32, two_bytes, U8_2_BYTES, "e10c9068");
    test_algorithm!(Crc32, three_bytes, U8_3_BYTES, "896a3802");
    test_algorithm!(Crc32, four_bytes, U8_4_BYTES, "4bc0d9cb");
    test_algorithm!(Crc32, ascending, U8_32_ASCENDING, "87e6ec25");
    test_algorithm!(Crc32, descending, U8_32_DESCENDING, "4a879068");
    test_algorithm_s!(Crc32, hi, S_HI, "d8932aac");
    test_algorithm_s!(Crc32, quick_brown_fox, S_QUICK_BROWN_FOX, "0f7add3d");
    test_algorithm_s!(Crc32, quick_brown_fox_with_eol, S_QUICK_BROWN_FOX_W_EOL, "6ab0305f");
    test_algorithm_s!(Crc32, qeoic_smouau_poswuc, S_QEOIC_SMOUAU_POSWUC, "41099b90");
}

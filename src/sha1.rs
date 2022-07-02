// SPDX-License-Identifier: Apache-2.0 OR MIT-0

use crate::{Algorithm, calculated_digest};

pub struct Sha1 {}

impl Sha1 {
    pub fn new() -> Algorithm<'static> {
        Algorithm {
            digest_bit_size: 160,
            name: "SHA-1",
            digest_fn: calculated_digest::<sha1::Sha1>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sha1;
    use crate::{test_algorithm, test_algorithm_s, tests::*};
    test_algorithm!(Sha1, empty, U8_EMPTY, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    test_algorithm!(Sha1, thirty_two_bytes_of_all_zeros, U8_32_ALL_ZEROS, "de8a847bff8c343d69b853a215e6ee775ef2ef96");
    test_algorithm!(Sha1, thirty_two_bytes_of_half_ones, U8_32_HALF_ONES, "9e1da9d4154cf5a448938421ccb7891adaa4d2fe");
    test_algorithm!(Sha1, thirty_two_bytes_of_all_ones, U8_32_ALL_ONES, "9e5175008751d08f361488c9927086b276b965fa");
    test_algorithm!(Sha1, one_byte, U8_1_BYTE, "58e6b3a414a1e090dfc6029add0f3555ccba127f");
    test_algorithm!(Sha1, two_bytes, U8_2_BYTES, "716db01d75f15b9face863c2e3284f706f4ecd1b");
    test_algorithm!(Sha1, three_bytes, U8_3_BYTES, "f5cd3bd472e85486a12bf10d9f6412e4a0f62e90");
    test_algorithm!(Sha1, four_bytes, U8_4_BYTES, "26cba705f7c0cec3fae689d7e289bc4469e32e89");
    test_algorithm!(Sha1, ascending, U8_32_ASCENDING, "4c9b68754cc4c6a52c2871b90740059372385833");
    test_algorithm!(Sha1, descending, U8_32_DESCENDING, "1113b9fc54134b43533dd685614e398ab8e8d663");
    test_algorithm_s!(Sha1, hi, S_HI, "c22b5f9178342609428d6f51b2c5af4c0bde6a42");
    test_algorithm_s!(Sha1, quick_brown_fox, S_QUICK_BROWN_FOX, "cb4a8948faa586ccc1694d80e25941a5227d8b9e");
    test_algorithm_s!(Sha1, quick_brown_fox_with_eol, S_QUICK_BROWN_FOX_W_EOL, "743c817468028042ef80b0d32242ec7171c67525");
    test_algorithm_s!(Sha1, qeoic_smouau_poswuc, S_QEOIC_SMOUAU_POSWUC, "a25ddb163a3bd72e2392c25f19388ec1d0351716");
}

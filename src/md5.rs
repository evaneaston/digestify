// SPDX-License-Identifier: Apache-2.0 OR MIT-0

use crate::{calculated_digest, Algorithm};

pub struct Md5 {}

impl Md5 {
    pub fn new() -> Algorithm<'static> {
        Algorithm {
            digest_bit_size: 128,
            name: "MD5",
            digest_fn: calculated_digest::<md5::Md5>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Md5;
    use crate::{test_algorithm, test_algorithm_s, tests::*};
    test_algorithm!(Md5, empty, U8_EMPTY, "d41d8cd98f00b204e9800998ecf8427e");
    test_algorithm!(Md5, thirty_two_bytes_of_all_zeros, U8_32_ALL_ZEROS, "70bc8f4b72a86921468bf8e8441dce51");
    test_algorithm!(Md5, thirty_two_bytes_of_half_ones, U8_32_HALF_ONES, "70a4f0549692bbb3bf9b54fb97b2c898");
    test_algorithm!(Md5, thirty_two_bytes_of_all_ones, U8_32_ALL_ONES, "0d7dc4266497100e4831f5b31b6b274f");
    test_algorithm!(Md5, one_byte, U8_1_BYTE, "e1671797c52e15f763380b45e841ec32");
    test_algorithm!(Md5, two_bytes, U8_2_BYTES, "0f6867de06b475d9ba77b88f36cd6656");
    test_algorithm!(Md5, three_bytes, U8_3_BYTES, "3632b24b06d4bbfbc77f836143908f5c");
    test_algorithm!(Md5, four_bytes, U8_4_BYTES, "cc5bdc23a51662a79d2436db99833964");
    test_algorithm!(Md5, ascending, U8_32_ASCENDING, "5985331bcec971b262122aa1ca5ad411");
    test_algorithm!(Md5, descending, U8_32_DESCENDING, "9f0622df1fcfad38e402e289c1a621d0");
    test_algorithm_s!(Md5, hi, S_HI, "49f68a5c8493ec2c0bf489821c21fc3b");
    test_algorithm_s!(Md5, quick_brown_fox, S_QUICK_BROWN_FOX, "6363fe744f74ee8f280958ab2f185dde");
    test_algorithm_s!(Md5, quick_brown_fox_with_eol, S_QUICK_BROWN_FOX_W_EOL, "26b694e5659c7caa44f040907ba82612");
    test_algorithm_s!(Md5, qeoic_smouau_poswuc, S_QEOIC_SMOUAU_POSWUC, "03a54624ecc2b05289ccac57b9c9ec58");
}

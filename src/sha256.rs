// SPDX-License-Identifier: Apache-2.0 OR MIT-0

use crate::{calculated_digest, Algorithm};

pub struct Sha256 {}

impl Sha256 {
    pub fn new() -> Algorithm<'static> {
        Algorithm {
            digest_bit_size: 256,
            name: "SHA-256",
            digest_fn: calculated_digest::<sha2::Sha256>,
        }
    }
}

#[cfg(test)]
mod tests {   
    use super::Sha256;
    use crate::{test_algorithm, test_algorithm_s, tests::*};
    test_algorithm!(Sha256, empty, U8_EMPTY, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    test_algorithm!(Sha256, thirty_two_bytes_of_all_zeros, U8_32_ALL_ZEROS, "66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925");
    test_algorithm!(Sha256, thirty_two_bytes_of_half_ones, U8_32_HALF_ONES, "9b68d49bb092f71292ad76ab8fb8750d710aae5af70e43b8ec0a901d048c0030");
    test_algorithm!(Sha256, thirty_two_bytes_of_all_ones, U8_32_ALL_ONES, "af9613760f72635fbdb44a5a0a63c39f12af30f950a6ee5c971be188e89c4051");
    test_algorithm!(Sha256, one_byte, U8_1_BYTE, "3f79bb7b435b05321651daefd374cdc681dc06faa65e374e38337b88ca046dea");
    test_algorithm!(Sha256, two_bytes, U8_2_BYTES, "cc62bd0e57c931e3e917c9b6ef75c79aedc05c1b12fb9cc709e301b8102ed0fe");
    test_algorithm!(Sha256, three_bytes, U8_3_BYTES, "2d99a256b34fc1a2446891228f1f7e084ebeb295bdf6b5d7e843ee03cb0fd77f");
    test_algorithm!(Sha256, four_bytes, U8_4_BYTES, "1fd681fb6e069e0fc7bdc905bafdf6154c2d75b81f8fa6aaff09f9ef33bc3a6a");
    test_algorithm!(Sha256, ascending, U8_32_ASCENDING, "ae216c2ef5247a3782c135efa279a3e4cdc61094270f5d2be58c6204b7a612c9");
    test_algorithm!(Sha256, descending, U8_32_DESCENDING, "4d9dd9451ec30afdac127017bab76d26d74f06efcfa8f27755b8b31aa3bfe4a8");
    test_algorithm_s!(Sha256, hi, S_HI, "8f434346648f6b96df89dda901c5176b10a6d83961dd3c1ac88b59b2dc327aa4");
    test_algorithm_s!(Sha256, quick_brown_fox, S_QUICK_BROWN_FOX, "531fd7e9f8f6d4b3836684a94b1b39e0966842dcf0ba251f8f8d774dad6e5ed9");
    test_algorithm_s!(Sha256, quick_brown_fox_with_eol, S_QUICK_BROWN_FOX_W_EOL, "f74fe80e1e1a01568a2695bdedafe64a11fe54bfb1729d888f9f66e6a8b873b8");
    test_algorithm_s!(Sha256, qeoic_smouau_poswuc, S_QEOIC_SMOUAU_POSWUC, "acc178e4a10e25274b6b05c47444e89528ea82199c70db9b1fe4c7e45df9013d");
}

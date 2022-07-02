// SPDX-License-Identifier: Apache-2.0 OR MIT-0

use super::{to_calculated_digest, Algorithm, CalculatedDigest};
use digest::Digest;
use std::io::{Read, Result};

fn sha512digest(a: &Algorithm, read: &mut dyn Read) -> Result<CalculatedDigest> {
    let mut d = sha2::Sha512::new();
    let bytes_read = std::io::copy(read, &mut d);
    to_calculated_digest(a, bytes_read, &d.finalize())
}

pub struct Sha512 {}

impl Sha512 {
    pub fn new() -> Algorithm<'static> {
        Algorithm {
            digest_bit_size: 512,
            name: "SHA-512",
            digest_fn: sha512digest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sha512;
    use crate::{test_algorithm, test_algorithm_s, tests::*};
    test_algorithm!(Sha512, empty, U8_EMPTY, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
    test_algorithm!(Sha512, thirty_two_bytes_of_all_zeros, U8_32_ALL_ZEROS, "5046adc1dba838867b2bbbfdd0c3423e58b57970b5267a90f57960924a87f1960a6a85eaa642dac835424b5d7c8d637c00408c7a73da672b7f498521420b6dd3");
    test_algorithm!(Sha512, thirty_two_bytes_of_half_ones, U8_32_HALF_ONES, "f0ba98521c6a724998b9dd342704c160b9b8dd2571b0aeed27b883c8b9c61c448545f3df33ebcdeefe8d6b7ab60faf4340755e1e6a8063e05e618408e5377c82");
    test_algorithm!(Sha512, thirty_two_bytes_of_all_ones, U8_32_ALL_ONES, "27cd6935864716a79d74dd5fabbd8964304051ca41a31c4659158ebb7c3d0b979b4522968797885508e64703de1cc0593063264b74b588f71b7de0161f21cf29");
    test_algorithm!(Sha512, one_byte, U8_1_BYTE, "87c568e037a5fa50b1bc911e8ee19a77c4dd3c22bce9932f86fdd8a216afe1681c89737fada6859e91047eece711ec16da62d6ccb9fd0de2c51f132347350d8c");
    test_algorithm!(Sha512, two_bytes, U8_2_BYTES, "1699d1e61cb0d5363ea9364fdd43399476ba02195e041d75656616cbafb40f9e84066b5c1362375858b0f33b435b10db9d96f1870a14c65afb102dd71b90aa1e");
    test_algorithm!(Sha512, three_bytes, U8_3_BYTES, "b4dea34fee8287a7daf44882d278bc26141f31a80eed866809b4369450116e0b743a4a02afc3bdcbb7a348a850c499b240223565cdd452ac38344af288566d7a");
    test_algorithm!(Sha512, four_bytes, U8_4_BYTES, "a37a979d7a1f4eff37e2c78cd008262bf6f519480026bae03a83bea5f9ed9b1dee23af9bd5b91d390eb5bacbf6452ecbcc0c95189b894d61c91df0ff80b68f15");
    test_algorithm!(Sha512, ascending, U8_32_ASCENDING, "77788f1a0cea001a2631dae5d05dbd062008d5b30f50b9e29beb2a7822289004573dfc9b6ffeb1c786a16349e70f9836876a743c31c0a7a2a70727a852eec372");
    test_algorithm!(Sha512, descending, U8_32_DESCENDING, "5618ac7db7f7fe035b983f4830ee5491ef4b6cfc8c667d9a23f4439f2088fbff5a1993410e022cd7c631f2490836fb9d098360bc9a4222544bf625f3df0c483d");
    test_algorithm_s!(Sha512, hi, S_HI, "150a14ed5bea6cc731cf86c41566ac427a8db48ef1b9fd626664b3bfbb99071fa4c922f33dde38719b8c8354e2b7ab9d77e0e67fc12843920a712e73d558e197");
    test_algorithm_s!(Sha512, quick_brown_fox, S_QUICK_BROWN_FOX, "e3cb25182765cbb590e0e47711e14179ddc2a6b29802fa25bb785042e351efeb6608af6c57d0db55e8cc6232be85ee61b4878604f783bd360757e41db16bc96e");
    test_algorithm_s!(Sha512, quick_brown_fox_with_eol, S_QUICK_BROWN_FOX_W_EOL, "f0033ace51f9eff5acd73409f22ab30144ec70487b1b38b613f5eee46d199b3565648237b8af31b6b2cb94d22605edcf5f7e88d7b4a241fae52e5979d6034b4f");
    test_algorithm_s!(Sha512, qeoic_smouau_poswuc, S_QEOIC_SMOUAU_POSWUC, "a1aa08726c71bf0d8d0b9f380fc4000806a8a0be52a5a665d62558861c96725b8f8b3c5453082d05327d2d580f6b586cef8f87a3c3d8ed1efca8e5793b166ebb");
}

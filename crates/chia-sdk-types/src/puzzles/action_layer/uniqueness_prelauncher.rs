use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::Mod;

pub const UNIQUENESS_PRELAUNCHER_PUZZLE: [u8; 49] = hex!(
    "
    ff04ffff04ffff0133ffff04ff02ffff04ffff0101ff80808080ffff04ffff04
    ffff013effff04ff05ff808080ff808080
    "
);

pub const UNIQUENESS_PRELAUNCHER_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    cdadfbef6a84ef68b04f18775d3b27a27f31bcd10d694e1532189dff0f861c96
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct UniquenessPrelauncher1stCurryArgs {
    pub launcher_puzzle_hash: Bytes32,
}

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct UniquenessPrelauncher2ndCurryArgs<V> {
    pub value: V,
}

impl Mod for UniquenessPrelauncher1stCurryArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&UNIQUENESS_PRELAUNCHER_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        UNIQUENESS_PRELAUNCHER_PUZZLE_HASH
    }
}

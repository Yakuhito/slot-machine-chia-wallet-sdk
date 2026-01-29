use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::Mod;

pub const REWARD_DISTRIBUTOR_ADD_ENTRY_PUZZLE: [u8; 489] = hex!(
    // Rue
    "
    ff02ffff01ff04ffff04ff819fffff04ff82015fffff04ffff10ff8202dfff82
    017f80ff8203df808080ffff04ffff04ffff0143ffff04ffff0112ffff04ffff
    0effff0161ffff0bffff0102ffff0bffff0101ff81bf80ffff0bffff0101ff82
    017f808080ffff04ffff02ff04ffff04ff06ffff04ff05ffff04ff0bffff04ff
    8201ffff808080808080ff8080808080ffff04ffff04ffff0133ffff04ffff02
    ff04ffff04ff06ffff04ff17ffff04ffff0bffff0101ffff0bffff0102ffff0b
    ffff0101ff81bf80ffff0bffff0102ffff0bffff0101ff8209df80ffff0bffff
    0101ff82017f80808080ff8080808080ffff04ff80ffff04ffff04ff81bfff80
    80ff8080808080ffff04ffff04ffff0155ffff04ffff10ff820bdfff2f80ff80
    8080ff8080808080ffff04ffff04ffff01ff0bffff0102ffff0bffff01820102
    80ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bff
    ff0102ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff01ff
    02ffff03ff03ffff01ff0bffff0102ffff0bffff0182010480ffff0bffff0102
    ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02
    ffff04ff02ff078080ffff0bffff010180808080ffff01ff0bffff0182010180
    80ff018080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_ADD_ENTRY_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    131c6e0e9bcb2071a281c6186403e51b5fa22635c90b406687ba99242a870823
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct RewardDistributorAddEntryActionArgs {
    pub singleton_mod_hash: Bytes32,
    pub manager_singleton_struct_hash: Bytes32,
    pub entry_slot_1st_curry_hash: Bytes32,
    pub max_second_offset: u64,
}

#[derive(FromClvm, ToClvm, Copy, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct RewardDistributorAddEntryActionSolution {
    pub entry_payout_puzzle_hash: Bytes32,
    pub entry_shares: u64,
    #[clvm(rest)]
    pub manager_singleton_inner_puzzle_hash: Bytes32,
}

impl Mod for RewardDistributorAddEntryActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&REWARD_DISTRIBUTOR_ADD_ENTRY_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        REWARD_DISTRIBUTOR_ADD_ENTRY_PUZZLE_HASH
    }
}

use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::{
    puzzles::{RewardDistributorEntryPayoutInfo, RewardDistributorEntrySlotValue},
    Mod,
};

pub const REWARD_DISTRIBUTOR_REMOVE_ENTRY_PUZZLE: [u8; 663] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff03ffff22ffff22ffff09ffff12ffff11ff8213bfff8205
    7f80ff82077f80ffff10ffff12ff8204ffff5f80ff8206ff8080ffff15ff8206
    ffffff0181ff8080ffff15ff5fff8206ff8080ffff01ff04ffff04ff82013fff
    ff04ffff11ff8202bfff8204ff80ffff04ffff11ff8205bfff82077f80ffff04
    ffff04ff8213bfffff10ff821bbfff8206ff8080ff820fbf80808080ffff04ff
    ff04ffff0143ffff04ffff0112ffff04ffff0effff0172ffff0bffff0102ffff
    0bffff0101ff82027f80ffff0bffff0101ff82077f808080ffff04ffff02ff04
    ffff04ff06ffff04ff05ffff04ff0bffff04ff8203ffff808080808080ff8080
    808080ffff04ffff04ffff0155ffff04ffff10ff8217bfff2f80ff808080ffff
    04ffff04ffff0142ffff04ffff0112ffff04ff80ffff04ffff02ff04ffff04ff
    06ffff04ff17ffff04ffff0bffff0101ffff0bffff0102ffff0bffff0101ff82
    027f80ffff0bffff0102ffff0bffff0101ff82057f80ffff0bffff0101ff8207
    7f80808080ff8080808080ff8080808080ffff04ffff04ffff0181d6ffff04ff
    ff0133ffff04ff82027fffff04ff8204ffffff04ffff04ff82027fff8080ff80
    8080808080ff808080808080ffff01ff088080ff0180ffff04ffff04ffff01ff
    0bffff0102ffff0bffff0182010280ffff0bffff0102ffff0bffff0102ffff0b
    ffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff078080ff
    ff0bffff010180808080ffff01ff02ffff03ff03ffff01ff0bffff0102ffff0b
    ffff0182010480ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff
    0580ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff01018080
    8080ffff01ff0bffff018201018080ff018080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_REMOVE_ENTRY_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    3007f9987445c9bd442b11231023627bf086b0f4410956d49e3874bcc85f9645
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct RewardDistributorRemoveEntryActionArgs {
    pub singleton_mod_hash: Bytes32,
    pub manager_singleton_struct_hash: Bytes32,
    pub entry_slot_1st_curry_hash: Bytes32,
    pub max_seconds_offset: u64,
    pub precision: u64,
}

#[derive(FromClvm, ToClvm, Copy, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct RewardDistributorRemoveEntryActionSolution {
    pub entry_slot: RewardDistributorEntrySlotValue,
    pub entry_payout_info: RewardDistributorEntryPayoutInfo,
    #[clvm(rest)]
    pub manager_singleton_inner_puzzle_hash: Bytes32,
}

impl Mod for RewardDistributorRemoveEntryActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&REWARD_DISTRIBUTOR_REMOVE_ENTRY_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        REWARD_DISTRIBUTOR_REMOVE_ENTRY_PUZZLE_HASH
    }
}

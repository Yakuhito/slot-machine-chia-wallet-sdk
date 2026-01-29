use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::Mod;

pub const REWARD_DISTRIBUTOR_NEW_EPOCH_PUZZLE: [u8; 723] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff03ffff22ffff22ffff09ff8217bfff821fbf80ffff09ff
    820fffffff13ffff12ff820bffff1780ffff01822710808080ffff21ffff22ff
    ff09ff82017fff821fbf80ffff09ff820bffff8205ff8080ffff22ffff22ffff
    15ff821fbfff82017f80ffff20ff8202ff8080ffff20ff820bff80808080ffff
    01ff02ffff01ff04ffff04ff82027fffff04ffff11ff82057fff821fff80ffff
    04ff820b7fffff04ffff04ff82277fffff10ff82377fffff12ffff11ff8217ff
    ff821fff80ff81bf808080ffff04ff823f7fffff10ff823f7fff5f8080808080
    80ffff04ffff04ffff013effff04ffff0effff0165ffff0bffff0101ff823f7f
    8080ff808080ffff04ffff04ffff0181d6ffff04ffff0133ffff04ff17ffff04
    ff821fffffff04ffff04ff17ff8080ff808080808080ffff04ffff04ffff0142
    ffff04ffff0112ffff04ff80ffff04ffff02ff09ffff04ff0dffff04ff0bffff
    04ffff0bffff0101ff0280ff8080808080ff8080808080ffff04ffff04ffff01
    33ffff04ffff02ff09ffff04ff0dffff04ff0bffff04ffff0bffff0101ff0280
    ff8080808080ffff04ff80ffff04ffff04ffff0bffff0101ff8202ff80ff8080
    ff8080808080ff808080808080ffff04ffff0bffff0102ffff0bffff0101ff82
    017f80ffff0bffff0102ffff0bffff0101ff8202ff80ffff0bffff0101ff8205
    ff808080ff018080ffff01ff088080ff0180ffff04ffff04ffff01ff0bffff01
    02ffff0bffff0182010280ffff0bffff0102ffff0bffff0102ffff0bffff0182
    010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff
    010180808080ffff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff0182
    010480ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff
    0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff
    01ff0bffff018201018080ff018080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_NEW_EPOCH_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    6dc911f4b4af06b922610ebd297bca8413ea7736d9c9abfcdca3422c08134684
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct RewardDistributorNewEpochActionArgs {
    pub reward_slot_1st_curry_hash: Bytes32,
    pub fee_payout_puzzle_hash: Bytes32,
    pub fee_bps: u64,
    pub epoch_seconds: u64,
    pub precision: u64,
}

#[derive(FromClvm, ToClvm, Copy, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct RewardDistributorNewEpochActionSolution {
    pub slot_epoch_time: u64,
    pub slot_next_epoch_initialized: bool,
    pub slot_total_rewards: u64,
    pub epoch_total_rewards: u64,
    #[clvm(rest)]
    pub fee: u64,
}

impl Mod for RewardDistributorNewEpochActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&REWARD_DISTRIBUTOR_NEW_EPOCH_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        REWARD_DISTRIBUTOR_NEW_EPOCH_PUZZLE_HASH
    }
}

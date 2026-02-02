use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::Mod;

pub const REWARD_DISTRIBUTOR_WITHDRAW_INCENTIVES_PUZZLE: [u8; 746] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff03ffff09ff81bfffff13ffff12ff8202ffff1780ffff01
    8227108080ffff01ff04ffff04ff4fffff04ffff11ff81afff81bf80ff81ef80
    80ffff04ffff04ffff0155ffff04ff5fff808080ffff04ffff02ff0effff04ff
    ff04ff0cff0a80ffff04ff05ffff02ff08ffff04ff5fffff04ff8207ffff8205
    ff808080808080ffff04ffff04ffff0133ffff04ffff02ff0cffff04ff0affff
    04ff05ffff04ffff0bffff0101ffff02ff08ffff04ff5fffff04ff8207ffffff
    11ff8205ffff81bf8080808080ff8080808080ffff04ff80ffff04ffff04ffff
    0bffff0101ff5f80ff8080ff8080808080ffff04ffff02ff0effff04ffff04ff
    0cff0a80ffff04ff0bffff02ff08ffff04ff5fffff04ff82017fff8202ff8080
    80808080ffff04ffff04ffff0143ffff04ffff0112ffff04ffff0effff0177ff
    ff0bffff0102ffff0bffff0101ff5f80ffff0bffff0101ff8202ff808080ffff
    04ff82017fff8080808080ffff04ffff04ffff0181d6ffff04ffff0133ffff04
    ff82017fffff04ff81bfffff04ffff04ff82017fff8080ff808080808080ff80
    80808080808080ffff01ff088080ff0180ffff04ffff04ffff04ffff01ff0bff
    ff0102ffff0bffff0101ff0280ffff0bffff0102ffff0bffff0101ff0580ffff
    0bffff0101ff07808080ffff01ff0bffff0102ffff0bffff0182010280ffff0b
    ffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ff
    ff02ff02ffff04ff02ff078080ffff0bffff01018080808080ffff04ffff01ff
    02ffff03ff03ffff01ff0bffff0102ffff0bffff0182010480ffff0bffff0102
    ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02
    ffff04ff02ff078080ffff0bffff010180808080ffff01ff0bffff0182010180
    80ff0180ffff01ff04ffff0142ffff04ffff0112ffff04ff80ffff04ffff02ff
    04ffff04ff06ffff04ff05ffff04ffff0bffff0101ff0780ff8080808080ff80
    808080808080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_WITHDRAW_INCENTIVES_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    2b452f6101b52670d8e6fc313865e95bfa60788dfa6e6911e6a1e26a7d6c51f9
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct RewardDistributorWithdrawIncentivesActionArgs {
    pub reward_slot_1st_curry_hash: Bytes32,
    pub commitment_slot_1st_curry_hash: Bytes32,
    pub withdrawal_share_bps: u64,
}

#[derive(FromClvm, ToClvm, Copy, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct RewardDistributorWithdrawIncentivesActionSolution {
    pub reward_slot_epoch_time: u64,
    pub withdrawal_share: u64,
    pub clawback_ph: Bytes32,
    pub committed_value: u64,
    pub reward_slot_total_rewards: u64,
    #[clvm(rest)]
    pub reward_slot_next_epoch_initialized: bool,
}

impl Mod for RewardDistributorWithdrawIncentivesActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&REWARD_DISTRIBUTOR_WITHDRAW_INCENTIVES_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        REWARD_DISTRIBUTOR_WITHDRAW_INCENTIVES_PUZZLE_HASH
    }
}

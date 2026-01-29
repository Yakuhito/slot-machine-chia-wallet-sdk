use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::Mod;

pub const REWARD_DISTRIBUTOR_WITHDRAW_INCENTIVES_PUZZLE: [u8; 772] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff03ffff09ff81bfffff13ffff12ff8202ffff1780ffff01
    8227108080ffff01ff04ffff04ff4fffff04ffff11ff81afff81bf80ff81ef80
    80ffff04ffff04ffff0155ffff04ff5fff808080ffff04ffff02ff0effff04ff
    ff04ff04ff0a80ffff04ff05ffff0bffff0102ffff0bffff0101ff5f80ffff0b
    ffff0102ffff0bffff0101ff8207ff80ffff0bffff0101ff8205ff8080808080
    80ffff04ffff04ffff0133ffff04ffff02ff04ffff04ff0affff04ff05ffff04
    ffff0bffff0101ffff0bffff0102ffff0bffff0101ff5f80ffff0bffff0102ff
    ff0bffff0101ff8207ff80ffff0bffff0101ffff11ff8205ffff81bf80808080
    80ff8080808080ffff04ff80ffff04ffff04ffff0bffff0101ff5f80ff8080ff
    8080808080ffff04ffff02ff0effff04ffff04ff04ff0a80ffff04ff0bffff0b
    ffff0102ffff0bffff0101ff5f80ffff0bffff0102ffff0bffff0101ff82017f
    80ffff0bffff0101ff8202ff808080808080ffff04ffff04ffff0143ffff04ff
    ff0112ffff04ffff0effff0177ffff0bffff0102ffff0bffff0101ff5f80ffff
    0bffff0101ff8202ff808080ffff04ff82017fff8080808080ffff04ffff04ff
    ff0181d6ffff04ffff0133ffff04ff82017fffff04ff81bfffff04ffff04ff82
    017fff8080ff808080808080ff8080808080808080ffff01ff088080ff0180ff
    ff04ffff04ffff01ff0bffff0102ffff0bffff0182010280ffff0bffff0102ff
    ff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ff
    ff04ff02ff078080ffff0bffff010180808080ffff04ffff01ff02ffff03ff03
    ffff01ff0bffff0102ffff0bffff0182010480ffff0bffff0102ffff0bffff01
    02ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff
    078080ffff0bffff010180808080ffff01ff0bffff018201018080ff0180ffff
    01ff04ffff0142ffff04ffff0112ffff04ff80ffff04ffff02ff04ffff04ff06
    ffff04ff05ffff04ffff0bffff0101ff0780ff8080808080ff80808080808080
    ff018080
    "
);

pub const REWARD_DISTRIBUTOR_WITHDRAW_INCENTIVES_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    9dd870db7822f3b8e3c76ef938a31bb35c722f90597c5972a30707f6b36de0c5
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

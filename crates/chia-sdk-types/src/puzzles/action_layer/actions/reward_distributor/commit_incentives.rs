use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::Mod;

pub const REWARD_DISTRIBUTOR_COMMIT_INCENTIVES_PUZZLE: [u8; 1093] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff03ffff22ffff21ffff15ff8202ffff8207ef80ffff09ff
    8202ffff8207ef8080ffff15ff8207ffff808080ffff01ff02ffff01ff04ffff
    04ff819fffff04ffff10ff82015fff820fff80ff8201df8080ffff04ffff04ff
    ff013effff04ffff0effff0163ff0280ff808080ffff04ffff02ff09ffff04ff
    1dffff04ff17ffff04ff02ff820bff80808080ffff04ffff04ffff0142ffff04
    ffff0112ffff04ff80ffff04ffff02ff2dffff04ff3dffff04ff0bffff04ffff
    0bffff0101ffff0bffff0102ffff0bffff0101ff81bf80ffff0bffff0102ffff
    0bffff0101ff82017f80ffff0bffff0101ff8202ff80808080ff8080808080ff
    8080808080ffff02ffff03ffff09ff8205ffff81bf80ffff01ff04ffff02ff09
    ffff04ff1dffff04ff0bffff04ffff0bffff0102ffff0bffff0101ff81bf80ff
    ff0bffff0102ffff0bffff0101ff82017f80ffff0bffff0101ffff10ff8202ff
    ff820fff80808080ffff0bffff0101ff81bf8080808080ff8080ffff01ff02ff
    ff03ff82017fffff01ff0880ffff01ff04ffff02ff09ffff04ff1dffff04ff0b
    ffff04ffff0bffff0102ffff0bffff0101ff81bf80ffff0bffff0102ffff0bff
    ff0182010180ffff0bffff0101ff8202ff808080ffff0bffff0101ff81bf8080
    808080ffff04ffff02ff09ffff04ff1dffff04ff0bffff04ffff0bffff0102ff
    ff0bffff0101ff8205ff80ffff0bffff0102ffff0bffff010180ffff0bffff01
    01ff820fff808080ffff0bffff0101ff8205ff8080808080ffff02ff15ffff04
    ff05ffff04ffff10ff81bfff2f80ffff04ff0bffff04ff2fff8205ff80808080
    80808080ff018080ff018080808080ffff04ffff0bffff0102ffff0bffff0101
    ff8202ff80ffff0bffff0102ffff0bffff0101ff8205ff80ffff0bffff0101ff
    8207ff808080ff018080ffff01ff088080ff0180ffff04ffff04ffff01ff04ff
    ff0133ffff04ffff02ff04ffff04ff06ffff04ff05ffff04ffff0bffff0101ff
    0b80ff8080808080ffff04ff80ffff04ffff04ff0fff8080ff8080808080ffff
    04ffff01ff02ffff03ffff09ff05ff1f80ffff0180ffff01ff04ffff02ff04ff
    ff04ff0effff04ff0bffff04ffff0bffff0102ffff0bffff0101ff0580ffff0b
    ffff0102ffff0bffff0182010180ffff0bffff0101808080ffff0bffff0101ff
    058080808080ffff02ff0affff04ff02ffff04ffff10ff05ff1780ff07808080
    8080ff0180ffff04ffff01ff0bffff0102ffff0bffff0182010280ffff0bffff
    0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02
    ff02ffff04ff02ff078080ffff0bffff010180808080ffff01ff02ffff03ff03
    ffff01ff0bffff0102ffff0bffff0182010480ffff0bffff0102ffff0bffff01
    02ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff
    078080ffff0bffff010180808080ffff01ff0bffff018201018080ff01808080
    80ff018080
    "
);

pub const REWARD_DISTRIBUTOR_COMMIT_INCENTIVES_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    b4d6325d63ce2b4b118d4871c04f24ae865b92b92038657dde556f281bf88a04
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct RewardDistributorCommitIncentivesActionArgs {
    pub reward_slot_1st_curry_hash: Bytes32,
    pub commitment_slot_1st_curry_hash: Bytes32,
    pub epoch_seconds: u64,
}

#[derive(FromClvm, ToClvm, Copy, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct RewardDistributorCommitIncentivesActionSolution {
    pub slot_epoch_time: u64,
    pub slot_next_epoch_initialized: bool,
    pub slot_total_rewards: u64,
    pub epoch_start: u64,
    pub clawback_ph: Bytes32,
    #[clvm(rest)]
    pub rewards_to_add: u64,
}

impl Mod for RewardDistributorCommitIncentivesActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&REWARD_DISTRIBUTOR_COMMIT_INCENTIVES_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        REWARD_DISTRIBUTOR_COMMIT_INCENTIVES_PUZZLE_HASH
    }
}

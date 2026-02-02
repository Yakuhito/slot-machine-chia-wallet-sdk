use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::Mod;

pub const REWARD_DISTRIBUTOR_COMMIT_INCENTIVES_PUZZLE: [u8; 981] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff03ffff22ffff21ffff15ff8202ffff8207ef80ffff09ff
    8202ffff8207ef8080ffff15ff8207ffff808080ffff01ff02ffff01ff04ffff
    04ff819fffff04ffff10ff82015fff820fff80ff8201df8080ffff04ffff04ff
    ff013effff04ffff0effff0163ff0280ff808080ffff04ffff02ff15ffff04ff
    3dffff04ff17ffff04ff02ff820bff80808080ffff04ffff04ffff0142ffff04
    ffff0112ffff04ff80ffff04ffff02ff5dffff04ff7dffff04ff0bffff04ffff
    0bffff0101ffff02ff09ffff04ff81bfffff04ff82017fff8202ff80808080ff
    8080808080ff8080808080ffff02ffff03ffff09ff8205ffff81bf80ffff01ff
    04ffff02ff15ffff04ff3dffff04ff0bffff04ffff02ff09ffff04ff81bfffff
    04ff82017fffff10ff8202ffff820fff80808080ffff0bffff0101ff81bf8080
    808080ff8080ffff01ff02ffff03ff82017fffff01ff0880ffff01ff04ffff02
    ff15ffff04ff3dffff04ff0bffff04ffff02ff09ffff04ff81bfffff04ffff01
    01ff8202ff808080ffff0bffff0101ff81bf8080808080ffff04ffff02ff15ff
    ff04ff3dffff04ff0bffff04ffff02ff09ffff04ff8205ffffff04ff80ff820f
    ff808080ffff0bffff0101ff8205ff8080808080ffff02ff2dffff04ff05ffff
    04ffff10ff81bfff2f80ffff04ff0bffff04ff2fff8205ff8080808080808080
    ff018080ff018080808080ffff04ffff02ff04ff8201ff80ff018080ffff01ff
    088080ff0180ffff04ffff04ffff01ff0bffff0102ffff0bffff0101ff0280ff
    ff0bffff0102ffff0bffff0101ff0580ffff0bffff0101ff07808080ffff04ff
    ff01ff04ffff0133ffff04ffff02ff04ffff04ff06ffff04ff05ffff04ffff0b
    ffff0101ff0b80ff8080808080ffff04ff80ffff04ffff04ff0fff8080ff8080
    808080ffff04ffff01ff02ffff03ffff09ff05ff1f80ffff0180ffff01ff04ff
    ff02ff0affff04ff1effff04ff0bffff04ffff02ff04ffff04ff05ffff04ffff
    0101ff80808080ffff0bffff0101ff058080808080ffff02ff16ffff04ff02ff
    ff04ffff10ff05ff1780ff078080808080ff0180ffff04ffff01ff0bffff0102
    ffff0bffff0182010280ffff0bffff0102ffff0bffff0102ffff0bffff018201
    0180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff01
    0180808080ffff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff018201
    0480ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0b
    ffff0102ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff01
    ff0bffff018201018080ff018080808080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_COMMIT_INCENTIVES_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    c31e384e34b6905da6bca3c7bb7e920d21b647f1d1d3cb7ec22436c878b7fc86
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

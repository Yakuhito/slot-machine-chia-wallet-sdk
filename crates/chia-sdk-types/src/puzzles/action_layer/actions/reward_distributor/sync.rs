use std::borrow::Cow;

use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::Mod;

pub const REWARD_DISTRIBUTOR_SYNC_PUZZLE: [u8; 257] = hex!(
    // Rue
    "
    ff02ffff03ffff22ffff21ffff15ff7eff0380ffff09ff03ff7e8080ffff15ff
    03ff5e8080ffff01ff02ffff01ff04ffff04ff09ffff04ff15ffff04ff2dffff
    04ffff04ffff10ff819dff0280ffff11ff81ddffff12ff02ff2d808080ffff04
    ff07ff81fd8080808080ffff04ffff04ffff0151ffff04ff07ff808080ffff04
    ffff04ffff013effff04ffff0effff0173ffff0bffff0102ffff0bffff0101ff
    0780ffff0bffff0101ff81fd808080ff808080ff80808080ffff04ffff02ffff
    03ffff15ff16ff8080ffff01ff13ffff12ff6effff11ff03ff5e8080ffff12ff
    16ffff11ff7eff5e808080ffff018080ff0180ff018080ffff01ff088080ff01
    80
    "
);

pub const REWARD_DISTRIBUTOR_SYNC_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    ae887c11d52e1a53d6b670d224d560c9a18a237ea751ee97be13e2b99edb836e
    "
));

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RewardDistributorSyncActionArgs {}

impl RewardDistributorSyncActionArgs {
    pub fn curry_tree_hash() -> TreeHash {
        REWARD_DISTRIBUTOR_SYNC_PUZZLE_HASH
    }
}

#[derive(FromClvm, ToClvm, Copy, Debug, Clone, PartialEq, Eq)]
#[clvm(transparent)]
pub struct RewardDistributorSyncActionSolution {
    pub update_time: u64,
}

impl Mod for RewardDistributorSyncActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&REWARD_DISTRIBUTOR_SYNC_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        REWARD_DISTRIBUTOR_SYNC_PUZZLE_HASH
    }
}

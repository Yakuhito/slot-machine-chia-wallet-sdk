use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::Mod;

pub const REWARD_DISTRIBUTOR_STAKE_PUZZLE: [u8; 757] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff01ff02ffff01ff04ffff04ffff10ff82013fffff010180
    ffff04ff8202bfffff04ffff10ff8205bfff1180ffff04ffff04ff8213bfffff
    10ff821bbfff0d8080ff820fbf80808080ffff04ffff04ffff0133ffff04ffff
    02ff2bffff04ff3bffff04ff17ffff04ffff0bffff0101ff0280ff8080808080
    ffff04ff80ffff04ffff04ff8202ffff8080ff8080808080ffff04ffff04ffff
    013effff04ffff0effff0174ff0280ff808080ffff04ffff04ffff0155ffff04
    ffff10ff8217bfff2f80ff808080ffff02ffff03ffff15ff8205ffffff0181ff
    80ffff01ff04ffff04ffff0143ffff04ffff0112ffff04ffff0effff0173ffff
    0bffff0101ff0d8080ffff04ff8202ffff8080808080ffff04ffff04ffff0142
    ffff04ffff0112ffff04ff80ffff04ffff02ff2bffff04ff3bffff04ff17ffff
    04ffff0bffff0101ffff02ff13ffff04ff13ff8201ff808080ff8080808080ff
    8080808080ff198080ffff01ff02ffff03ff8207ffffff01ff0880ffff011980
    ff018080ff018080808080ffff04ffff02ff09ffff04ff09ffff04ff82017fff
    ff04ff8209dfffff10ff8203ffff088080808080ff018080ffff04ffff04ffff
    02ff17ffff04ff4fffff04ff81bfff5f808080ffff12ff8201ffffff11ff8204
    efff82017f808080ff018080ffff04ffff04ffff01ff02ffff03ffff07ff0380
    ffff01ff0bffff0102ffff02ff02ffff04ff02ff058080ffff02ff02ffff04ff
    02ff07808080ffff01ff0bffff0101ff038080ff0180ffff04ffff01ff0bffff
    0102ffff0bffff0182010280ffff0bffff0102ffff0bffff0102ffff0bffff01
    82010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bff
    ff010180808080ffff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff01
    82010480ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ff
    ff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ff
    ff01ff0bffff018201018080ff01808080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_STAKE_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    e2813a39963d5a7656447e07a45a3400898dc14e3e9a4baec3f0b5ce2d30c93c
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct RewardDistributorStakeActionArgs<LP> {
    pub entry_slot_1st_curry_hash: Bytes32,
    pub max_second_offset: u64,
    pub lock_puzzle: LP,
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct RewardDistributorStakeActionSolution<LPS> {
    pub lock_puzzle_solution: LPS,
    pub entry_custody_puzzle_hash: Bytes32,
    pub existing_slot_cumulative_payout: i128,
    #[clvm(rest)]
    pub existing_slot_shares: u64,
}

impl<LP> Mod for RewardDistributorStakeActionArgs<LP> {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&REWARD_DISTRIBUTOR_STAKE_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        REWARD_DISTRIBUTOR_STAKE_PUZZLE_HASH
    }
}

// run '(mod (NONCE INNER_PUZZLE . inner_solution) (a INNER_PUZZLE inner_solution))' -d
pub const NONCE_WRAPPER_PUZZLE: [u8; 7] = hex!("ff02ff05ff0780");
pub const NONCE_WRAPPER_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "847d971ef523417d555ea9854b1612837155d34d453298defcd310774305f657"
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct NonceWrapperArgs<N, I> {
    pub nonce: N,
    pub inner_puzzle: I,
}

impl<N, I> Mod for NonceWrapperArgs<N, I> {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&NONCE_WRAPPER_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        NONCE_WRAPPER_PUZZLE_HASH
    }
}

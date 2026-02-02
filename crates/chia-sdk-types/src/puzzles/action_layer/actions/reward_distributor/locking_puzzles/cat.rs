use std::borrow::Cow;

use chia_protocol::Bytes32;
use chia_puzzles::SETTLEMENT_PAYMENT_HASH;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::{puzzles::NONCE_WRAPPER_PUZZLE_HASH, Mod};

pub const REWARD_DISTRIBUTOR_CAT_LOCKING_PUZZLE: [u8; 493] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff01ff02ffff01ff04ff820bffffff04ffff04ffff013fff
    ff04ff02ff808080ffff04ffff04ffff013effff04ffff0effff016cff0280ff
    808080ffff04ffff04ffff0146ffff04ff8205ffff808080ff8080808080ffff
    04ffff0bffff02ff0bffff04ff17ff8207ff8080ffff02ff09ffff04ff09ffff
    04ffff02ff09ffff04ff09ffff04ff81bfff8202ff808080ffff04ffff04ff02
    ffff04ff8205ffffff04ffff04ff02ff8080ff80808080ff808080808080ff01
    8080ffff04ffff0bffff0102ffff0bffff0182010280ffff0bffff0102ffff0b
    ffff0102ffff0bffff0182010180ff1780ffff0bffff0102ffff02ff06ffff04
    ff06ffff04ffff02ff04ffff04ff04ffff04ff81bfff8202ff808080ffff04ff
    2fff8080808080ffff0bffff010180808080ff018080ffff04ffff04ffff01ff
    02ffff03ffff07ff0380ffff01ff0bffff0102ffff02ff02ffff04ff02ff0580
    80ffff02ff02ffff04ff02ff07808080ffff01ff0bffff0101ff038080ff0180
    ffff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff0182010480ffff0b
    ffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ff
    ff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff01ff0bffff01
    8201018080ff018080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_CAT_LOCKING_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    e44f5a224b24daea738a0380bab1149cee38707dc1d385af6dc9d26b8dc7c28e
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct RewardDistributorCatLockingPuzzleArgs<CM> {
    pub cat_maker: CM,
    pub offer_mod_hash: Bytes32,
    pub nonce_mod_hash: Bytes32,
    pub my_p2_puzzle_hash: Bytes32,
}

impl<CM> RewardDistributorCatLockingPuzzleArgs<CM> {
    pub fn new(cat_maker: CM, my_p2_puzzle_hash: Bytes32) -> Self {
        Self {
            cat_maker,
            offer_mod_hash: SETTLEMENT_PAYMENT_HASH.into(),
            nonce_mod_hash: NONCE_WRAPPER_PUZZLE_HASH.into(),
            my_p2_puzzle_hash,
        }
    }
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct RewardDistributorCatLockingPuzzleSolution<CMS> {
    pub my_id: Bytes32,
    pub cat_amount: u64,
    #[clvm(rest)]
    pub cat_maker_solution_rest: CMS,
}

impl<CM> Mod for RewardDistributorCatLockingPuzzleArgs<CM> {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&REWARD_DISTRIBUTOR_CAT_LOCKING_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        REWARD_DISTRIBUTOR_CAT_LOCKING_PUZZLE_HASH
    }
}

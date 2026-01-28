use std::borrow::Cow;

use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::Mod;

pub const XCHANDLES_FACTOR_PRICING_PUZZLE: [u8; 465] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff03ffff15ff7fff8080ffff01ff04ffff12ff7fff05ffff
    02ff06ffff04ffff0dff5f80ffff02ff04ffff04ff04ff5f8080808080ffff12
    ff7fff0b8080ffff01ff088080ff0180ffff04ffff04ffff01ff02ffff03ff03
    ffff01ff02ffff01ff02ffff03ffff02ffff03ffff15ff04ffff016080ffff01
    ff02ffff03ffff15ffff017bff0480ffff01ff0101ffff018080ff0180ffff01
    8080ff0180ffff01ff02ff05ffff04ff05ff068080ffff01ff02ffff03ffff02
    ffff03ffff15ff04ffff012f80ffff01ff02ffff03ffff15ffff013aff0480ff
    ff01ff0101ffff018080ff0180ffff018080ff0180ffff01ff10ffff02ff05ff
    ff04ff05ff068080ffff010180ffff01ff088080ff018080ff0180ffff04ffff
    04ffff0cff03ff80ffff010180ffff0cff03ffff01018080ff018080ffff0180
    80ff0180ffff01ff13ffff02ffff03ffff15ff02ffff010280ffff01ff03ffff
    15ff02ffff010480ffff03ffff09ff02ffff010580ffff0110ffff02ffff03ff
    ff15ff02ffff011f80ffff01ff0880ffff01ff010280ff018080ffff03ffff09
    ff02ffff010380ffff01820080ffff01408080ffff01ff088080ff0180ffff03
    ff03ffff0102ffff0101808080ff018080
    "
);

pub const XCHANDLES_FACTOR_PRICING_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    f2cd9e13fffd0f0a53f4d23e8fc3733a4e713ba8dc54f3ce7f32586b8a0ae256
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct XchandlesFactorPricingPuzzleArgs {
    pub base_price: u64,
    pub registration_period: u64,
}

impl XchandlesFactorPricingPuzzleArgs {
    pub fn get_price(base_price: u64, handle: &str, num_periods: u64) -> u64 {
        base_price
            * match handle.len() {
                3 => 128,
                4 => 64,
                5 => 16,
                _ => 2,
            }
            / if handle.contains(|c: char| c.is_numeric()) {
                2
            } else {
                1
            }
            * num_periods
    }
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct XchandlesPricingSolution {
    pub buy_time: u64,
    pub current_expiration: u64,
    pub handle: String,
    #[clvm(rest)]
    pub num_periods: u64,
}

impl Mod for XchandlesFactorPricingPuzzleArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&XCHANDLES_FACTOR_PRICING_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        XCHANDLES_FACTOR_PRICING_PUZZLE_HASH
    }
}

use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::Mod;

pub const REWARD_DISTRIBUTOR_INITIATE_PAYOUT_WITHOUT_APPROVAL_PUZZLE: [u8; 710] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff03ffff22ffff22ffff22ffff09ffff12ffff11ff8204ef
    ff8203ff80ff8202ff80ffff10ffff12ff5fff1780ff82017f8080ffff15ff82
    017fffff0181ff8080ffff15ff17ff82017f8080ffff21ffff15ff5fff0b80ff
    ff09ff5fff0b808080ffff01ff04ffff04ff4fffff04ffff11ff81afff5f80ff
    ff04ff82016fffff04ffff04ff8204efffff10ff8206efff82017f8080ff8203
    ef80808080ffff04ffff04ffff0142ffff04ffff0112ffff04ff80ffff04ffff
    02ff04ffff04ff06ffff04ff05ffff04ffff0bffff0101ffff0bffff0102ffff
    0bffff0101ff81bf80ffff0bffff0102ffff0bffff0101ff8203ff80ffff0bff
    ff0101ff8202ff80808080ff8080808080ff8080808080ffff04ffff04ffff01
    33ffff04ffff02ff04ffff04ff06ffff04ff05ffff04ffff0bffff0101ffff0b
    ffff0102ffff0bffff0101ff81bf80ffff0bffff0102ffff0bffff0101ff8204
    ef80ffff0bffff0101ff8202ff80808080ff8080808080ffff04ff80ffff04ff
    ff04ff81bfff8080ff8080808080ffff04ffff04ffff013effff04ffff0effff
    0170ffff0bffff0102ffff0bffff0101ff81bf80ffff0bffff0101ff5f808080
    ff808080ffff04ffff04ffff0181d6ffff04ffff0133ffff04ff81bfffff04ff
    5fffff04ffff04ff81bfff8080ff808080808080ff808080808080ffff01ff08
    8080ff0180ffff04ffff04ffff01ff0bffff0102ffff0bffff0182010280ffff
    0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102
    ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff01ff02ffff
    03ff03ffff01ff0bffff0102ffff0bffff0182010480ffff0bffff0102ffff0b
    ffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04
    ff02ff078080ffff0bffff010180808080ffff01ff0bffff018201018080ff01
    8080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_INITIATE_PAYOUT_WITHOUT_APPROVAL_PUZZLE_HASH: TreeHash =
    TreeHash::new(hex!(
        "
    504b496d9e36ebcdae99c2dfdfbc0a7e6f3df4d3104eddc0debe8bcd8c6f96fc
    "
    ));

pub const REWARD_DISTRIBUTOR_INITIATE_PAYOUT_WITH_APPROVAL_PUZZLE: [u8; 781] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff03ffff22ffff22ffff22ffff09ffff12ffff11ff8204ef
    ff8203ff80ff8202ff80ffff10ffff12ff5fff1780ff82017f8080ffff15ff82
    017fffff0181ff8080ffff15ff17ff82017f8080ffff21ffff15ff5fff0b80ff
    ff09ff5fff0b808080ffff01ff04ffff04ff4fffff04ffff11ff81afff5f80ff
    ff04ff82016fffff04ffff04ff8204efffff10ff8206efff82017f8080ff8203
    ef80808080ffff04ffff04ffff0142ffff04ffff0112ffff04ff80ffff04ffff
    02ff04ffff04ff06ffff04ff05ffff04ffff0bffff0101ffff0bffff0102ffff
    0bffff0101ff81bf80ffff0bffff0102ffff0bffff0101ff8203ff80ffff0bff
    ff0101ff8202ff80808080ff8080808080ff8080808080ffff04ffff04ffff01
    33ffff04ffff02ff04ffff04ff06ffff04ff05ffff04ffff0bffff0101ffff0b
    ffff0102ffff0bffff0101ff81bf80ffff0bffff0102ffff0bffff0101ff8204
    ef80ffff0bffff0101ff8202ff80808080ff8080808080ffff04ff80ffff04ff
    ff04ff81bfff8080ff8080808080ffff04ffff04ffff013effff04ffff0effff
    0170ffff0bffff0102ffff0bffff0101ff81bf80ffff0bffff0101ff5f808080
    ff808080ffff04ffff04ffff0181d6ffff04ffff0133ffff04ff81bfffff04ff
    5fffff04ffff04ff81bfff8080ff808080808080ffff04ffff04ffff0143ffff
    04ffff0112ffff04ffff0effff0170ffff0bffff0102ffff0bffff0101ff5f80
    ffff0bffff0101ff82017f808080ffff04ff81bfff8080808080ff8080808080
    8080ffff01ff088080ff0180ffff04ffff04ffff01ff0bffff0102ffff0bffff
    0182010280ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580
    ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff010180808080
    ffff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff0182010480ffff0b
    ffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ff
    ff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff01ff0bffff01
    8201018080ff018080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_INITIATE_PAYOUT_WITH_APPROVAL_PUZZLE_HASH: TreeHash =
    TreeHash::new(hex!(
        "
        03a98b9392f21d751043a880245558f06c348eeed8265bf09b94d6f2da693445
        "
    ));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct RewardDistributorInitiatePayoutWithoutApprovalActionArgs {
    pub entry_slot_1st_curry_hash: Bytes32,
    pub payout_threshold: u64,
    pub precision: u64,
}

impl Mod for RewardDistributorInitiatePayoutWithoutApprovalActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&REWARD_DISTRIBUTOR_INITIATE_PAYOUT_WITHOUT_APPROVAL_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        REWARD_DISTRIBUTOR_INITIATE_PAYOUT_WITHOUT_APPROVAL_PUZZLE_HASH
    }
}

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct RewardDistributorInitiatePayoutWithApprovalActionArgs {
    pub entry_slot_1st_curry_hash: Bytes32,
    pub payout_threshold: u64,
    pub precision: u64,
}

impl Mod for RewardDistributorInitiatePayoutWithApprovalActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&REWARD_DISTRIBUTOR_INITIATE_PAYOUT_WITH_APPROVAL_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        REWARD_DISTRIBUTOR_INITIATE_PAYOUT_WITH_APPROVAL_PUZZLE_HASH
    }
}

#[derive(FromClvm, ToClvm, Copy, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct RewardDistributorInitiatePayoutActionSolution {
    pub entry_payout_amount: u64,
    pub entry_payout_puzzle_hash: Bytes32,
    pub payout_rounding_error: u128,
    pub entry_shares: u64,
    #[clvm(rest)]
    pub entry_initial_cumulative_payout: u128,
}

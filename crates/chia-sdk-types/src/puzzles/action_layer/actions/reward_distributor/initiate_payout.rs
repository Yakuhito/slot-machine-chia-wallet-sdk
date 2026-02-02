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
    02ff04ffff04ff0affff04ff05ffff04ffff0bffff0101ffff02ff0effff04ff
    81bfffff04ff8203ffff8202ff80808080ff8080808080ff8080808080ffff04
    ffff04ffff0133ffff04ffff02ff04ffff04ff0affff04ff05ffff04ffff0bff
    ff0101ffff02ff0effff04ff81bfffff04ff8204efff8202ff80808080ff8080
    808080ffff04ff80ffff04ffff04ff81bfff8080ff8080808080ffff04ffff04
    ffff013effff04ffff0effff0170ffff0bffff0102ffff0bffff0101ff81bf80
    ffff0bffff0101ff5f808080ff808080ffff04ffff04ffff0181d6ffff04ffff
    0133ffff04ff81bfffff04ff5fffff04ffff04ff81bfff8080ff808080808080
    ff808080808080ffff01ff088080ff0180ffff04ffff04ffff01ff0bffff0102
    ffff0bffff0182010280ffff0bffff0102ffff0bffff0102ffff0bffff018201
    0180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff01
    0180808080ffff04ffff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff
    0182010480ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580
    ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff010180808080
    ffff01ff0bffff018201018080ff0180ffff01ff0bffff0102ffff0bffff0101
    ff0280ffff0bffff0102ffff0bffff0101ff0580ffff0bffff0101ff07808080
    8080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_INITIATE_PAYOUT_WITHOUT_APPROVAL_PUZZLE_HASH: TreeHash =
    TreeHash::new(hex!(
        "
    0ef50c990e1f4fb4faf6f22af91555ce5889eef155e1f8f0f8a3e4c66718b640
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
    02ff04ffff04ff0affff04ff05ffff04ffff0bffff0101ffff02ff0effff04ff
    81bfffff04ff8203ffff8202ff80808080ff8080808080ff8080808080ffff04
    ffff04ffff0133ffff04ffff02ff04ffff04ff0affff04ff05ffff04ffff0bff
    ff0101ffff02ff0effff04ff81bfffff04ff8204efff8202ff80808080ff8080
    808080ffff04ff80ffff04ffff04ff81bfff8080ff8080808080ffff04ffff04
    ffff013effff04ffff0effff0170ffff0bffff0102ffff0bffff0101ff81bf80
    ffff0bffff0101ff5f808080ff808080ffff04ffff04ffff0181d6ffff04ffff
    0133ffff04ff81bfffff04ff5fffff04ffff04ff81bfff8080ff808080808080
    ffff04ffff04ffff0143ffff04ffff0112ffff04ffff0effff0170ffff0bffff
    0102ffff0bffff0101ff5f80ffff0bffff0101ff82017f808080ffff04ff81bf
    ff8080808080ff80808080808080ffff01ff088080ff0180ffff04ffff04ffff
    01ff0bffff0102ffff0bffff0182010280ffff0bffff0102ffff0bffff0102ff
    ff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff0780
    80ffff0bffff010180808080ffff04ffff01ff02ffff03ff03ffff01ff0bffff
    0102ffff0bffff0182010480ffff0bffff0102ffff0bffff0102ffff0bffff01
    82010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bff
    ff010180808080ffff01ff0bffff018201018080ff0180ffff01ff0bffff0102
    ffff0bffff0101ff0280ffff0bffff0102ffff0bffff0101ff0580ffff0bffff
    0101ff078080808080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_INITIATE_PAYOUT_WITH_APPROVAL_PUZZLE_HASH: TreeHash =
    TreeHash::new(hex!(
        "
        a424af712119ead264299c7859db494764edf94815a648f98fb5d4c3ac38e38b
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

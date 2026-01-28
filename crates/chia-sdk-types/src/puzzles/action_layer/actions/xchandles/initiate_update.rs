use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::{
    puzzles::{CompactCoinProof, XchandlesDataValue, XchandlesHandleSlotValue},
    Mod,
};

pub const XCHANDLES_INITIATE_UPDATE_PUZZLE: [u8; 783] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff01ff02ffff01ff04ff8202ffffff04ffff04ffff0155ff
    ff04ff8215ffff808080ffff04ffff04ffff0157ffff04ff821fffff808080ff
    ff04ffff04ffff0142ffff04ffff0112ffff04ff80ffff04ffff02ff2bffff04
    ff5bffff04ff81bfffff04ffff0bffff0101ff0d80ff8080808080ff80808080
    80ffff04ffff02ff7bffff04ffff04ff2bff5b80ffff04ff81bfff0d808080ff
    ff04ffff04ffff0143ffff04ffff013affff04ffff0effff0169ff0980ffff04
    ff02ff8080808080ffff04ffff04ffff0133ffff04ffff02ff2bffff04ff5bff
    ff04ff82017fffff04ffff0bffff0101ffff0bffff0102ffff02ff13ffff04ff
    13ffff04ff02ffff10ff821fffff5f80808080ff098080ff8080808080ffff04
    ff80ffff04ffff04ff02ff8080ff8080808080ff8080808080808080ffff04ff
    ff30ff8213ffffff02ff15ffff04ff2dffff04ff0bffff04ffff02ff09ffff04
    ff09ffff04ff0bffff04ff8216ffff1780808080ffff04ff822bffff80808080
    8080ff823bff80ff018080ffff04ffff04ffff02ff04ffff04ff04ffff04ff82
    047fff8202ff808080ffff02ff04ffff04ff04ff82017f808080ff018080ffff
    04ffff04ffff01ff02ffff03ffff07ff0380ffff01ff0bffff0102ffff02ff02
    ffff04ff02ff058080ffff02ff02ffff04ff02ff07808080ffff01ff0bffff01
    01ff038080ff0180ffff04ffff01ff0bffff0102ffff0bffff0182010280ffff
    0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102
    ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff04ffff01ff
    02ffff03ff03ffff01ff0bffff0102ffff0bffff0182010480ffff0bffff0102
    ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02
    ffff04ff02ff078080ffff0bffff010180808080ffff01ff0bffff0182010180
    80ff0180ffff01ff04ffff0133ffff04ffff02ff04ffff04ff06ffff04ff05ff
    ff04ffff0bffff0101ff0780ff8080808080ffff04ff80ffff04ffff04ff05ff
    8080ff8080808080808080ff018080
    "
);

pub const XCHANDLES_INITIATE_UPDATE_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    c1e16228a1b4c8874dbe23b097ce8318f9eca184035a4869324294cef1b8cf88
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct XchandlesInitiateUpdateActionArgs {
    pub singleton_mod_hash: Bytes32,
    pub singleton_launcher_mod_hash: Bytes32,
    pub relative_block_height: u32,
    pub handle_slot_1st_curry_hash: Bytes32,
    pub update_slot_1st_curry_hash: Bytes32,
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct XchandlesInitiateUpdateActionSolution {
    pub current_slot_value: XchandlesHandleSlotValue,
    pub new_data: XchandlesDataValue,
    pub current_owner: CompactCoinProof,
    #[clvm(rest)]
    pub min_height: u32,
}

impl Mod for XchandlesInitiateUpdateActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&XCHANDLES_INITIATE_UPDATE_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        XCHANDLES_INITIATE_UPDATE_PUZZLE_HASH
    }
}

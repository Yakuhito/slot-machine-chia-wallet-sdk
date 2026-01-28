use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::{
    puzzles::{CompactCoinProof, XchandlesDataValue, XchandlesHandleSlotValue},
    Mod,
};

pub const XCHANDLES_EXECUTE_UPDATE_PUZZLE: [u8; 976] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff01ff04ff81bfffff04ffff04ffff0155ffff04ff820aff
    ff808080ffff04ffff04ffff0153ffff04ff82017fff808080ffff04ffff02ff
    5dffff04ffff04ff15ff2d80ffff04ff2fffff02ff09ffff04ff09ff8202ff80
    80808080ffff04ffff02ff7dffff04ffff04ff15ff2d80ffff04ff2fffff02ff
    09ffff04ff09ffff04ff8204ffffff04ff820affff8205ff80808080808080ff
    ff04ffff04ffff0143ffff04ffff013affff04ffff0effff0175ff0280ffff04
    ffff30ff8213ffffff02ff15ffff04ff2dffff04ff0bffff04ffff02ff09ffff
    04ff09ffff04ff0bffff04ff8216ffff1780808080ffff04ff822bffff808080
    808080ff823bff80ff8080808080ffff04ffff04ffff0143ffff04ffff0112ff
    ff04ffff0effff016fff0280ffff04ffff02ff15ffff04ff2dffff04ff0bffff
    04ffff02ff09ffff04ff09ffff04ff0bffff04ff8209ffff1780808080ffff04
    ff8217ffff808080808080ff8080808080ffff04ffff04ffff0143ffff04ffff
    0112ffff04ffff0effff0172ff0280ffff04ffff02ff15ffff04ff2dffff04ff
    0bffff04ffff02ff09ffff04ff09ffff04ff0bffff04ff820dffff1780808080
    ffff04ff821fffff808080808080ff8080808080ffff04ffff02ff5dffff04ff
    ff04ff15ff2d80ffff04ff5fffff0bffff0102ffff02ff09ffff04ff09ffff04
    ff8213ffff82017f808080ff0280808080ff80808080808080808080ffff04ff
    ff02ff04ffff04ff04ffff04ff82047fff8202ff808080ff018080ffff04ffff
    04ffff01ff02ffff03ffff07ff0380ffff01ff0bffff0102ffff02ff02ffff04
    ff02ff058080ffff02ff02ffff04ff02ff07808080ffff01ff0bffff0101ff03
    8080ff0180ffff04ffff01ff0bffff0102ffff0bffff0182010280ffff0bffff
    0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02
    ff02ffff04ff02ff078080ffff0bffff010180808080ffff04ffff01ff02ffff
    03ff03ffff01ff0bffff0102ffff0bffff0182010480ffff0bffff0102ffff0b
    ffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04
    ff02ff078080ffff0bffff010180808080ffff01ff0bffff018201018080ff01
    80ffff04ffff01ff04ffff0142ffff04ffff0112ffff04ff80ffff04ffff02ff
    04ffff04ff06ffff04ff05ffff04ffff0bffff0101ff0780ff8080808080ff80
    80808080ffff01ff04ffff0133ffff04ffff02ff04ffff04ff06ffff04ff05ff
    ff04ffff0bffff0101ff0780ff8080808080ffff04ff80ffff04ffff04ff05ff
    8080ff808080808080808080ff018080
    "
);

pub const XCHANDLES_EXECUTE_UPDATE_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    d6534ea206347423685790f911d091513b9d5deb6205ed8a5a8b41018239f078
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct XchandlesExecuteUpdateActionArgs {
    pub singleton_mod_hash: Bytes32,
    pub singleton_launcher_mod_hash: Bytes32,
    pub handle_slot_1st_curry_hash: Bytes32,
    pub update_slot_1st_curry_hash: Bytes32,
}

#[derive(FromClvm, ToClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(list)]
pub struct XchandlesNewDataPuzzleHashes {
    pub new_owner_inner_puzzle_hash: Bytes32,
    #[clvm(rest)]
    pub new_resolved_inner_puzzle_hash: Bytes32,
}

impl XchandlesNewDataPuzzleHashes {
    pub fn new(
        new_owner_inner_puzzle_hash: Bytes32,
        new_resolved_inner_puzzle_hash: Bytes32,
    ) -> Self {
        Self {
            new_owner_inner_puzzle_hash,
            new_resolved_inner_puzzle_hash,
        }
    }
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct XchandlesExecuteUpdateActionSolution {
    pub min_execution_height: u32,
    pub current_slot_value: XchandlesHandleSlotValue,
    pub new_data: XchandlesDataValue,
    pub current_owner: CompactCoinProof,
    #[clvm(rest)]
    pub new_data_puzzle_hashes: XchandlesNewDataPuzzleHashes,
}

impl Mod for XchandlesExecuteUpdateActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&XCHANDLES_EXECUTE_UPDATE_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        XCHANDLES_EXECUTE_UPDATE_PUZZLE_HASH
    }
}

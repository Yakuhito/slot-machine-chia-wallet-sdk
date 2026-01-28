use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::Mod;

pub const XCHANDLES_ORACLE_PUZZLE: [u8; 499] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff01ff04ff17ffff04ffff04ffff0142ffff04ffff0112ff
    ff04ff80ffff04ffff02ff15ffff04ff2dffff04ff0bffff04ffff0bffff0101
    ff0280ff8080808080ff8080808080ffff04ffff02ff3dffff04ffff04ff15ff
    2d80ffff04ff0bff02808080ffff04ffff04ffff013effff04ffff0effff016f
    ff0280ff808080ff8080808080ffff04ffff02ff04ffff04ff04ff0f8080ff01
    8080ffff04ffff04ffff01ff02ffff03ffff07ff0380ffff01ff0bffff0102ff
    ff02ff02ffff04ff02ff058080ffff02ff02ffff04ff02ff07808080ffff01ff
    0bffff0101ff038080ff0180ffff04ffff01ff0bffff0102ffff0bffff018201
    0280ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0b
    ffff0102ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff04
    ffff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff0182010480ffff0b
    ffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ff
    ff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff01ff0bffff01
    8201018080ff0180ffff01ff04ffff0133ffff04ffff02ff04ffff04ff06ffff
    04ff05ffff04ffff0bffff0101ff0780ff8080808080ffff04ff80ffff04ffff
    04ff05ff8080ff8080808080808080ff018080
    "
);

pub const XCHANDLES_ORACLE_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    2647db165fc60f83c7afb267fb7ec780302f7fa2138ad54140e50bfdb8a19388
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct XchandlesOracleActionArgs {
    pub handle_slot_1st_curry_hash: Bytes32,
}

impl Mod for XchandlesOracleActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&XCHANDLES_ORACLE_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        XCHANDLES_ORACLE_PUZZLE_HASH
    }
}

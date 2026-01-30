use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::{
    puzzles::{PuzzleAndSolution, SlotNeigborsInfo, XchandlesDataValue},
    Mod,
};

pub const XCHANDLES_EXTEND_PUZZLE: [u8; 863] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff01ff02ffff03ffff22ffff09ffff02ff09ffff04ff09ff
    8205ff8080ff82015f80ffff09ffff02ff09ffff04ff09ff82013f8080ff8202
    df8080ffff01ff04ff5fffff04ffff04ffff0142ffff04ffff0112ffff04ff80
    ffff04ffff0bffff0102ffff0bffff0182010280ffff0bffff0102ffff0bffff
    0102ffff0bffff0182010180ff2f80ffff0bffff0102ffff02ff15ffff04ff15
    ffff04ffff0bffff0101ffff02ff09ffff04ff09ffff04ffff04ffff0bffff01
    01ff820bbf80ff82017f80ffff04ff8205bfff8202ff8080808080ff80808080
    ffff0bffff010180808080ff8080808080ffff04ffff04ffff013effff04ffff
    0effff0165ffff02ff09ffff04ff09ffff04ff04ff820bbf80808080ff808080
    ffff04ffff04ffff0155ffff04ff8205bfff808080ffff04ffff04ffff0151ff
    ff04ff8202bfff808080ffff04ffff02ff1dffff04ff15ffff04ff2fffff02ff
    09ffff04ff09ffff04ffff04ffff0bffff0101ff820bbf80ff82017f80ffff04
    ffff10ff8205bfff0680ff8202ff80808080808080ffff04ffff04ffff013fff
    ff04ffff0bffff02ff8205ffffff04ff0bff8207ff8080ffff02ff09ffff04ff
    09ffff04ffff02ff09ffff04ff09ffff04ff820bbfff8205bf808080ffff04ff
    ff04ff17ffff04ff04ffff04ffff04ff17ff8080ff80808080ff808080808080
    ff808080ff8080808080808080ffff01ff088080ff0180ffff04ffff02ff819f
    ff81df80ff018080ffff04ffff04ffff01ff02ffff03ffff07ff0380ffff01ff
    0bffff0102ffff02ff02ffff04ff02ff058080ffff02ff02ffff04ff02ff0780
    8080ffff01ff0bffff0101ff038080ff0180ffff04ffff01ff02ffff03ff03ff
    ff01ff0bffff0102ffff0bffff0182010480ffff0bffff0102ffff0bffff0102
    ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff07
    8080ffff0bffff010180808080ffff01ff0bffff018201018080ff0180ffff01
    ff04ffff0133ffff04ffff0bffff0102ffff0bffff0182010280ffff0bffff01
    02ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff
    02ffff04ff02ffff04ffff0bffff0101ff0780ff80808080ffff0bffff010180
    808080ffff04ff80ffff04ffff04ff05ff8080ff80808080808080ff018080
    "
);

pub const XCHANDLES_EXTEND_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    731eb553801bd78234cebeee9e15102f829fd5ccc27030eb56bced0a756a6e93
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct XchandlesExtendActionArgs {
    pub offer_mod_hash: Bytes32,
    pub payout_puzzle_hash: Bytes32,
    pub handle_slot_1st_curry_hash: Bytes32,
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct XchandlesExtendActionSolution<PP, PS, CMP, CMS> {
    pub pricing_puzzle_and_solution: PuzzleAndSolution<PP, PS>,
    pub neighbors: SlotNeigborsInfo,
    pub rest: XchandlesDataValue,
    #[clvm(rest)]
    pub cat_maker_and_solution: PuzzleAndSolution<CMP, CMS>,
}

impl Mod for XchandlesExtendActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&XCHANDLES_EXTEND_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        XCHANDLES_EXTEND_PUZZLE_HASH
    }
}

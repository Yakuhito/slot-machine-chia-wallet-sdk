use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::{
    puzzles::{PuzzleAndSolution, SlotNeigborsInfo, XchandlesDataValue},
    Mod,
};

pub const XCHANDLES_EXTEND_PUZZLE: [u8; 871] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff01ff02ffff03ffff02ffff03ffff09ffff02ff09ffff04
    ff09ff8205ff8080ff82015f80ffff01ff02ffff03ffff09ffff02ff09ffff04
    ff09ff82013f8080ff8202df80ffff01ff0101ffff018080ff0180ffff018080
    ff0180ffff01ff04ff5fffff04ffff04ffff0142ffff04ffff0112ffff04ff80
    ffff04ffff02ff15ffff04ff2dffff04ff2fffff04ffff0bffff0101ffff02ff
    09ffff04ff09ffff04ffff04ffff0bffff0101ff820bbf80ff82017f80ffff04
    ff8205bfff8202ff8080808080ff8080808080ff8080808080ffff04ffff04ff
    ff013effff04ffff0effff0165ffff02ff09ffff04ff09ffff04ff04ff820bbf
    80808080ff808080ffff04ffff04ffff0155ffff04ff8205bfff808080ffff04
    ffff04ffff0151ffff04ff8202bfff808080ffff04ffff02ff3dffff04ffff04
    ff15ff2d80ffff04ff2fffff02ff09ffff04ff09ffff04ffff04ffff0bffff01
    01ff820bbf80ff82017f80ffff04ffff10ff8205bfff0680ff8202ff80808080
    808080ffff04ffff04ffff013fffff04ffff0bffff02ff8205ffffff04ff0bff
    8207ff8080ffff02ff09ffff04ff09ffff04ffff02ff09ffff04ff09ffff04ff
    820bbfff8205bf808080ffff04ffff04ff17ffff04ff04ffff04ffff04ff17ff
    8080ff80808080ff808080808080ff808080ff8080808080808080ffff01ff08
    8080ff0180ffff04ffff02ff819fff81df80ff018080ffff04ffff04ffff01ff
    02ffff03ffff07ff0380ffff01ff0bffff0102ffff02ff02ffff04ff02ff0580
    80ffff02ff02ffff04ff02ff07808080ffff01ff0bffff0101ff038080ff0180
    ffff04ffff01ff0bffff0102ffff0bffff0182010280ffff0bffff0102ffff0b
    ffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04
    ff02ff078080ffff0bffff010180808080ffff04ffff01ff02ffff03ff03ffff
    01ff0bffff0102ffff0bffff0182010480ffff0bffff0102ffff0bffff0102ff
    ff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff0780
    80ffff0bffff010180808080ffff01ff0bffff018201018080ff0180ffff01ff
    04ffff0133ffff04ffff02ff04ffff04ff06ffff04ff05ffff04ffff0bffff01
    01ff0780ff8080808080ffff04ff80ffff04ffff04ff05ff8080ff8080808080
    808080ff018080
    "
);

pub const XCHANDLES_EXTEND_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    01bd41d86abb6ef70f33ab3bb5dd53c9bfad730a6691904c000e0b69911f2811
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

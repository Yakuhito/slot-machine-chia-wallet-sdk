use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::{
    puzzles::{PuzzleHashPuzzleAndSolution, XchandlesHandleSlotValue, XchandlesOtherPrecommitData},
    Mod,
};

pub const XCHANDLES_REFUND_PUZZLE: [u8; 904] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff03ffff22ffff22ffff09ff819fffff02ff04ffff04ff04
    ff82015f808080ffff09ff4fffff02ff04ffff04ff04ff81af80808080ffff02
    ffff03ff8202ffffff01ff09ffff0bffff0101ff81bf80ff8208ff80ffff01ff
    010180ff018080ffff01ff02ffff01ff04ff2fffff04ffff04ffff0142ffff04
    ffff0113ffff04ff80ffff04ff02ffff04ff8202ffff808080808080ffff04ff
    ff04ffff013effff04ffff0effff0124ff0280ff808080ffff02ffff03ffff22
    ffff22ffff22ffff09ff81afff82013f80ffff09ff82017fff820bdf8080ffff
    21ffff09ff819fff82016f80ffff09ff819fff8201ef808080ffff09ff8202ff
    ffff05ffff02ff82015fff8201df80808080ffff01ff02ffff01ff04ffff04ff
    ff0155ffff04ff822bffff808080ffff04ffff04ffff0142ffff04ffff0112ff
    ff04ff80ffff04ffff02ff2bffff04ff5bffff04ff2fffff04ffff0bffff0101
    ff0280ff8080808080ff8080808080ffff04ffff02ff7bffff04ffff04ff2bff
    5b80ffff04ff2fff02808080ff80808080ffff04ffff02ff09ffff04ff09ff82
    05ff8080ff018080ffff018080ff0180808080ffff04ffff02ff82015fffff04
    ffff02ff0affff04ff16ffff04ff05ffff04ff820bffffff04ffff0bffff0101
    ffff02ff04ffff04ff04ffff04ffff04ffff04ff819fff8201df80ffff04ff4f
    ff81ef8080ffff04ffff04ff81bfff820fff80ff8205ff8080808080ff808080
    808080ff8201df8080ff018080ffff01ff088080ff0180ffff04ffff04ffff01
    ff02ffff03ffff07ff0380ffff01ff0bffff0102ffff02ff02ffff04ff02ff05
    8080ffff02ff02ffff04ff02ff07808080ffff01ff0bffff0101ff038080ff01
    80ffff04ffff01ff0bffff0102ffff0bffff0182010280ffff0bffff0102ffff
    0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff
    04ff02ff078080ffff0bffff010180808080ffff04ffff01ff02ffff03ff03ff
    ff01ff0bffff0102ffff0bffff0182010480ffff0bffff0102ffff0bffff0102
    ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff07
    8080ffff0bffff010180808080ffff01ff0bffff018201018080ff0180ffff01
    ff04ffff0133ffff04ffff02ff04ffff04ff06ffff04ff05ffff04ffff0bffff
    0101ff0780ff8080808080ffff04ff80ffff04ffff04ff05ff8080ff80808080
    80808080ff018080
    "
);

pub const XCHANDLES_REFUND_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    b3d4f7cb4ca59aa2e4ae428e1a0547589364420b375e16153688dcc0988fbe8b
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct XchandlesRefundActionArgs {
    pub precommit_1st_curry_hash: Bytes32,
    pub handle_slot_1st_curry_hash: Bytes32,
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct XchandlesRefundActionSolution<CMP, CMS, PP, PS, S> {
    pub precommited_pricing_puzzle_and_solution: PuzzleHashPuzzleAndSolution<PP, PS>,
    pub precommited_cat_maker_and_solution: PuzzleHashPuzzleAndSolution<CMP, CMS>,
    pub handle: String,
    pub precommit_amount: u64,
    pub slot_value: Option<XchandlesHandleSlotValue>,
    #[clvm(rest)]
    pub other_precommit_data: XchandlesOtherPrecommitData<S>,
}

impl Mod for XchandlesRefundActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&XCHANDLES_REFUND_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        XCHANDLES_REFUND_PUZZLE_HASH
    }
}

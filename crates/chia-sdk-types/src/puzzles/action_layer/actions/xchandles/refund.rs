use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::{
    puzzles::{PuzzleHashPuzzleAndSolution, XchandlesHandleSlotValue, XchandlesOtherPrecommitData},
    Mod,
};

pub const XCHANDLES_REFUND_PUZZLE: [u8; 1098] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff03ffff02ffff03ffff02ffff03ffff09ff819fffff02ff04ffff04ff04ff82015f808080ffff01ff02ffff03ffff09ff4fffff02ff04ffff04ff04ff81af808080ffff01ff0101ffff018080ff0180ffff018080ff0180ffff01ff02ffff03ffff02ffff03ff8202ffffff01ff09ffff0bffff0101ff81bf80ff8208ff80ffff01ff010180ff0180ffff01ff0101ffff018080ff0180ffff018080ff0180ffff01ff02ffff01ff04ff2fffff04ffff04ffff0142ffff04ffff0113ffff04ff80ffff04ff02ffff04ff8202ffff808080808080ffff04ffff04ffff013effff04ffff0effff0124ff0280ff808080ffff02ffff03ffff02ffff03ffff02ffff03ffff02ffff03ffff09ff81afff82013f80ffff01ff02ffff03ffff09ff82017fff820bdf80ffff01ff0101ffff018080ff0180ffff018080ff0180ffff01ff02ffff03ffff02ffff03ffff09ff819fff82016f80ffff01ff0101ffff01ff02ffff03ffff09ff819fff8201ef80ffff01ff0101ffff018080ff018080ff0180ffff01ff0101ffff018080ff0180ffff018080ff0180ffff01ff02ffff03ffff09ff8202ffffff05ffff02ff82015fff8201df808080ffff01ff0101ffff018080ff0180ffff018080ff0180ffff01ff02ffff01ff04ffff04ffff0155ffff04ff822bffff808080ffff04ffff04ffff0142ffff04ffff0112ffff04ff80ffff04ffff02ff2bffff04ff5bffff04ff2fffff04ffff0bffff0101ff0280ff8080808080ff8080808080ffff04ffff02ff7bffff04ffff04ff2bff5b80ffff04ff2fff02808080ff80808080ffff04ffff02ff09ffff04ff09ff8205ff8080ff018080ffff018080ff0180808080ffff04ffff02ff82015fffff04ffff02ff0affff04ff16ffff04ff05ffff04ff820bffffff04ffff0bffff0101ffff02ff04ffff04ff04ffff04ffff04ffff04ff819fff8201df80ffff04ff4fff81ef8080ffff04ffff04ff81bfff820fff80ff8205ff8080808080ff808080808080ff8201df8080ff018080ffff01ff088080ff0180ffff04ffff04ffff01ff02ffff03ffff07ff0380ffff01ff0bffff0102ffff02ff02ffff04ff02ff058080ffff02ff02ffff04ff02ff07808080ffff01ff0bffff0101ff038080ff0180ffff04ffff01ff0bffff0102ffff0bffff0182010280ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff04ffff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff0182010480ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff01ff0bffff018201018080ff0180ffff01ff04ffff0133ffff04ffff02ff04ffff04ff06ffff04ff05ffff04ffff0bffff0101ff0780ff8080808080ffff04ff80ffff04ffff04ff05ff8080ff8080808080808080ff018080
    "
);

pub const XCHANDLES_REFUND_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    84f98ad725b2bef6ad06cd4519feadb0d7a938f6a4988c26a4ed4a45791fab98
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

use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::{
    puzzles::{
        PuzzleAndSolution, SlotNeigborsInfo, XchandlesDataValue, XchandlesNewDataPuzzleHashes,
        XchandlesOtherPrecommitData,
    },
    Mod,
};

pub const XCHANDLES_REGISTER_PUZZLE: [u8; 1628] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff01ff02ffff03ffff02ffff03ffff02ffff03ffff02ffff03ffff02ffff03ffff02ffff03ffff09ff82017fffff0bffff0101ff82bbff8080ffff01ff02ffff03ff825bffffff0180ffff01ff010180ff0180ffff018080ff0180ffff01ff02ffff03ffff0aff82017fff8204ff80ffff01ff0101ffff018080ff0180ffff018080ff0180ffff01ff02ffff03ffff0aff8206ffff82017f80ffff01ff0101ffff018080ff0180ffff018080ff0180ffff01ff02ffff03ffff09ff8202bfffff02ff09ffff04ff09ff8209ff808080ffff01ff0101ffff018080ff0180ffff018080ff0180ffff01ff02ffff03ffff09ff8205bfffff02ff09ffff04ff09ff8213ff808080ffff01ff0101ffff018080ff0180ffff018080ff0180ffff01ff02ffff01ff02ffff01ff04ff8202ffffff04ffff04ffff0151ffff04ff82afffff808080ffff04ffff02ff8201f7ffff04ffff04ff57ff82017780ffff04ff82017fffff02ff27ffff04ff27ffff04ffff04ff8213ffffff04ff829fffff821bff8080ffff04ff83015fffff8301dfff80808080808080ffff04ffff02ff8201f7ffff04ffff04ff57ff82017780ffff04ff82017fffff02ff27ffff04ff27ffff04ffff04ff821bffffff04ff8213ffff83013fff8080ffff04ff8302bfffff8303bfff80808080808080ffff04ffff02ff81b7ffff04ffff04ff57ff82017780ffff04ff82017fffff02ff27ffff04ff27ffff04ffff04ff8205ffff820bff80ffff04ffff10ff82afffff1b80ff8302ffff80808080808080ffff04ffff02ff81b7ffff04ffff04ff57ff82017780ffff04ff82017fffff02ff27ffff04ff27ffff04ffff04ff8213ffffff04ff829fffff8205ff8080ffff04ff83015fffff8301dfff80808080808080ffff04ffff02ff81b7ffff04ffff04ff57ff82017780ffff04ff82017fffff02ff27ffff04ff27ffff04ffff04ff821bffffff04ff8205ffff83013fff8080ffff04ff8302bfffff8303bfff80808080808080ffff04ffff04ffff0142ffff04ffff0113ffff04ffff0101ffff04ff02ffff04ff13ff808080808080ffff04ffff04ffff013effff04ffff0effff0172ff0280ff808080ffff04ffff04ffff0143ffff04ffff0112ffff04ffff0effff0161ff0280ffff04ff09ff8080808080ffff02ffff03ffff09ff09ff0d80ffff0180ffff01ff04ffff04ffff0143ffff04ffff0112ffff04ffff0effff0162ff0280ffff04ff0dff8080808080ff808080ff018080808080808080808080ffff04ffff02ff8213ffffff04ffff02ff2bffff04ff81bbffff04ff5fffff04ff8302ffffffff04ffff0bffff0101ffff02ff13ffff04ff13ffff04ffff04ffff04ff82057fff821bff80ffff04ff820b7fff8237ff8080ffff04ffff04ff830177ffff8303ffff80ff83017fff8080808080ff808080808080ff821bff8080ff018080ffff04ffff04ffff02ff15ffff04ff5dffff04ff0bffff04ffff02ff09ffff04ff09ffff04ff0bffff04ff83013fffff1780808080ffff04ff829fffff808080808080ffff02ff15ffff04ff5dffff04ff0bffff04ffff02ff09ffff04ff09ffff04ff0bffff04ff8301bfffff1780808080ffff04ff82dfffff80808080808080ff018080ffff01ff088080ff0180ffff04ffff02ff8209ffff820dff80ff018080ffff04ffff04ffff01ff02ffff03ffff07ff0380ffff01ff0bffff0102ffff02ff02ffff04ff02ff058080ffff02ff02ffff04ff02ff07808080ffff01ff0bffff0101ff038080ff0180ffff04ffff01ff0bffff0102ffff0bffff0182010280ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff04ffff01ff04ffff0133ffff04ffff02ff04ffff04ff06ffff04ff05ffff04ffff0bffff0101ff0780ff8080808080ffff04ff80ffff04ffff04ff05ff8080ff8080808080ffff04ffff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff0182010480ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff01ff0bffff018201018080ff0180ffff01ff04ffff0142ffff04ffff0112ffff04ff80ffff04ffff02ff04ffff04ff06ffff04ff05ffff04ffff0bffff0101ff0780ff8080808080ff808080808080808080ff018080
    "
);

pub const XCHANDLES_REGISTER_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    89b65548e87b1188baa427f1d1b5d7cf6d38cd37c5810a864f60ad794373b3f4
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct XchandlesRegisterActionArgs {
    pub singleton_mod_hash: Bytes32,
    pub singleton_launcher_puzzle_hash: Bytes32,
    pub precommit_1st_curry_hash: Bytes32,
    pub handle_slot_1st_curry_hash: Bytes32,
}

#[derive(FromClvm, ToClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(list)]
pub struct XchandlesRestOfSlot {
    pub this_this_value: Bytes32, // left_left_value or right_right_value
    pub this_expiration: u64,     // left_expiration or right_expiration
    #[clvm(rest)]
    pub this_data: XchandlesDataValue, // left_data or right_data
}

impl XchandlesRestOfSlot {
    pub fn new(
        this_this_value: Bytes32,
        this_expiration: u64,
        this_data: XchandlesDataValue,
    ) -> Self {
        Self {
            this_this_value,
            this_expiration,
            this_data,
        }
    }
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct XchandlesRegisterActionSolution<PP, PS, CMP, CMS, S> {
    pub handle_hash: Bytes32,
    pub neighbors: SlotNeigborsInfo,
    pub cat_maker_puzzle_and_solution: PuzzleAndSolution<CMP, CMS>,
    pub pricing_puzzle_and_solution: PuzzleAndSolution<PP, PS>,
    pub left_rest_of_slot: XchandlesRestOfSlot,
    pub right_rest_of_slot: XchandlesRestOfSlot,
    pub data_puzzle_hashes: XchandlesNewDataPuzzleHashes,
    #[clvm(rest)]
    pub other_precommit_data: XchandlesOtherPrecommitData<S>,
}

impl Mod for XchandlesRegisterActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&XCHANDLES_REGISTER_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        XCHANDLES_REGISTER_PUZZLE_HASH
    }
}

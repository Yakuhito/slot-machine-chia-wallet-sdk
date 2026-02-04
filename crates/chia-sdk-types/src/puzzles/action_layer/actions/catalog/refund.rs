use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::{
    puzzles::{PuzzleAndSolution, SlotNeigborsInfo},
    Mod,
};

pub const CATALOG_REFUND_PUZZLE: [u8; 751] = hex!(
    "
    ff02ffff01ff02ffff01ff02ffff01ff04ff5fffff02ffff03ffff22ffff09ff
    8201dfff8202ff80ffff09ff82015fff058080ffff01ff02ffff01ff04ffff04
    ffff0142ffff04ffff0112ffff04ff80ffff04ffff02ff57ffff04ff81b7ffff
    04ff5fffff04ffff0bffff0101ff0280ff8080808080ff8080808080ffff04ff
    ff02ff81f7ffff04ffff04ff57ff81b780ffff04ff5fff02808080ff058080ff
    ff04ffff02ff13ffff04ff13ffff04ff82027fff8203ff808080ff018080ffff
    010280ff018080ffff04ffff04ffff04ffff013effff04ffff0effff0124ffff
    02ff09ffff04ff09ffff04ff82013fff8202bf80808080ff808080ffff04ffff
    04ffff0142ffff04ffff0113ffff04ff80ffff04ffff02ff819fffff04ffff02
    ff15ffff04ff2dffff04ff0bffff04ff8203bfffff04ffff0bffff0102ff8201
    3fffff0bffff0101ffff02ff09ffff04ff09ffff04ff8202bfffff04ff02ff81
    df808080808080ff808080808080ff81df8080ffff04ff82017fff8080808080
    80ff808080ff018080ffff04ffff02ff04ffff04ff04ff4f8080ff018080ffff
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

pub const CATALOG_REFUND_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    61a62089bada309b624437d7aa8b5112f077c10a82e48968d93504b7b69bed6f
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct CatalogRefundActionArgs {
    pub precommit_1st_curry_hash: Bytes32,
    pub slot_1st_curry_hash: Bytes32,
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct PuzzleHashPuzzleAndSolution<P, S> {
    pub puzzle_hash: Bytes32,
    pub puzzle: P,
    #[clvm(rest)]
    pub solution: S,
}

impl<P, S> PuzzleHashPuzzleAndSolution<P, S> {
    pub fn new(puzzle_hash: Bytes32, puzzle: P, solution: S) -> Self {
        Self {
            puzzle_hash,
            puzzle,
            solution,
        }
    }
}

#[derive(FromClvm, ToClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(list)]
pub struct CatalogOtherPrecommitData {
    pub tail_hash: Bytes32,
    pub initial_nft_owner_ph: Bytes32,
    #[clvm(rest)]
    pub refund_puzzle_hash_hash: Bytes32,
}

impl CatalogOtherPrecommitData {
    pub fn new(
        tail_hash: Bytes32,
        initial_nft_owner_ph: Bytes32,
        refund_puzzle_hash_hash: Bytes32,
    ) -> Self {
        Self {
            tail_hash,
            initial_nft_owner_ph,
            refund_puzzle_hash_hash,
        }
    }
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct CatalogRefundActionSolution<P, S> {
    // pub precommited_cat_maker_and_solution: PuzzleHashPuzzleAndSolution<P, S>,
    pub precommited_cat_maker_and_solution: PuzzleAndSolution<P, S>,
    pub other_precommit_data: CatalogOtherPrecommitData,
    pub precommit_amount: u64,
    #[clvm(rest)]
    pub neighbors: Option<SlotNeigborsInfo>,
}

impl Mod for CatalogRefundActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&CATALOG_REFUND_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        CATALOG_REFUND_PUZZLE_HASH
    }
}

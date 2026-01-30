use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::{
    puzzles::{
        PuzzleAndSolution, SlotNeigborsInfo, XchandlesDataValue, XchandlesNewDataPuzzleHashes,
    },
    Mod,
};

pub const XCHANDLES_EXPIRE_PUZZLE: [u8; 1126] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff03ffff22ffff09ffff02ff04ffff04ff04ff82027f8080
    ff82015f80ffff09ffff02ff04ffff04ff04ff82013f8080ff8203df8080ffff
    01ff02ffff01ff02ffff01ff02ffff01ff04ff8202ffffff02ffff03ffff09ff
    8247ffff8267ff80ffff0102ffff01ff04ffff04ffff0143ffff04ffff0112ff
    ff04ffff0effff0166ff0580ffff04ffff02ff57ffff04ff81b7ffff04ff2fff
    ff04ffff02ff27ffff04ff27ffff04ff2fffff04ff8267ffff5f80808080ffff
    04ff82ffffff808080808080ff8080808080ff028080ff018080ffff04ffff04
    ffff04ffff0151ffff04ff820affff808080ffff04ffff04ffff0151ffff04ff
    8216ffff808080ffff04ffff04ffff0142ffff04ffff0112ffff04ff80ffff04
    ffff02ff2bffff04ff5bffff04ff81bfffff04ffff0bffff0101ffff02ff13ff
    ff04ff13ffff04ffff04ff09ff8217ff80ffff04ff8216ffff822fff80808080
    80ff8080808080ff8080808080ffff04ffff02ff7bffff04ffff04ff2bff5b80
    ffff04ff81bfffff02ff13ffff04ff13ffff04ffff04ff09ff8217ff80ffff04
    ffff10ff1dff820aff80ff8213ff80808080808080ffff04ffff04ffff0142ff
    ff04ffff0113ffff04ffff0101ffff04ff02ffff04ff15ff808080808080ffff
    04ffff04ffff013effff04ffff0effff0178ff0280ff808080ffff04ffff04ff
    ff0143ffff04ffff0112ffff04ffff0effff0165ff0280ffff04ffff02ff2bff
    ff04ff5bffff04ff17ffff04ffff02ff13ffff04ff13ffff04ff17ffff04ff82
    23ffff2f80808080ffff04ff825fffff808080808080ff8080808080ff808080
    8080808080ff018080ffff04ffff02ff8204ffffff04ffff02ff15ffff04ff2d
    ffff04ff2fffff04ff8215ffffff04ffff0bffff0101ffff02ff09ffff04ff09
    ffff04ffff04ffff04ff8202bfff8206ff80ffff04ff8207bfff82037f8080ff
    ff04ffff04ff82177fff821dff80ff8209ff8080808080ff808080808080ff82
    06ff8080ff018080ffff04ffff04ffff0bffff0101ff820bbf80ffff02ff8201
    3fff8201bf8080ff018080ffff01ff088080ff0180ffff04ffff04ffff01ff02
    ffff03ffff07ff0380ffff01ff0bffff0102ffff02ff02ffff04ff02ff058080
    ffff02ff02ffff04ff02ff07808080ffff01ff0bffff0101ff038080ff0180ff
    ff04ffff01ff0bffff0102ffff0bffff0182010280ffff0bffff0102ffff0bff
    ff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff
    02ff078080ffff0bffff010180808080ffff04ffff01ff02ffff03ff03ffff01
    ff0bffff0102ffff0bffff0182010480ffff0bffff0102ffff0bffff0102ffff
    0bffff0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff078080
    ffff0bffff010180808080ffff01ff0bffff018201018080ff0180ffff01ff04
    ffff0133ffff04ffff02ff04ffff04ff06ffff04ff05ffff04ffff0bffff0101
    ff0780ff8080808080ffff04ff80ffff04ffff04ff05ff8080ff808080808080
    8080ff018080
    "
);

pub const XCHANDLES_EXPIRE_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    bf2addcadd521d2e0a3708dfdc90ad870141c90d4329064b6011dfd72e20ae02
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct XchandlesExpireActionArgs {
    pub singleton_mod_hash: Bytes32,
    pub singleton_launcher_mod_hash: Bytes32,
    pub precommit_1st_curry_hash: Bytes32,
    pub handle_slot_1st_curry_hash: Bytes32,
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct XchandlesRefundAndSecret<S> {
    pub refund_puzzle_hash_hash: Bytes32,
    #[clvm(rest)]
    pub secret: S,
}

impl<S> XchandlesRefundAndSecret<S> {
    pub fn new(refund_puzzle_hash_hash: Bytes32, secret: S) -> Self {
        Self {
            refund_puzzle_hash_hash,
            secret,
        }
    }
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct XchandlesOtherPrecommitData<S> {
    pub launcher_ids: XchandlesDataValue,
    #[clvm(rest)]
    pub refund_and_secret: XchandlesRefundAndSecret<S>,
}

impl<S> XchandlesOtherPrecommitData<S> {
    pub fn new(
        owner_launcher_id: Bytes32,
        resolved_launcher_id: Bytes32,
        refund_puzzle_hash_hash: Bytes32,
        secret: S,
    ) -> Self {
        Self {
            launcher_ids: XchandlesDataValue {
                owner_launcher_id,
                resolved_launcher_id,
            },
            refund_and_secret: XchandlesRefundAndSecret::new(refund_puzzle_hash_hash, secret),
        }
    }
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct XchandlesExpireActionSolution<CMP, CMS, EP, ES, S> {
    pub expired_handle_pricing_puzzle_and_solution: PuzzleAndSolution<EP, ES>,
    pub cat_maker_and_solution: PuzzleAndSolution<CMP, CMS>,
    pub other_precommit_data: XchandlesOtherPrecommitData<S>,
    pub neighbors: SlotNeigborsInfo,
    pub old_rest: XchandlesDataValue,
    #[clvm(rest)]
    pub new_inner_puzzle_hashes: XchandlesNewDataPuzzleHashes,
}

impl Mod for XchandlesExpireActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&XCHANDLES_EXPIRE_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        XCHANDLES_EXPIRE_PUZZLE_HASH
    }
}

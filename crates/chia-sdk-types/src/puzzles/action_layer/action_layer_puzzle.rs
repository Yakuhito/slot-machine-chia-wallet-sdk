use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::{CurriedProgram, ToTreeHash, TreeHash};
use hex_literal::hex;

use crate::{MerkleProof, Mod};

pub const ACTION_LAYER_PUZZLE: [u8; 640] = hex!(
    // Rue
    "
    ff02ffff01ff02ff05ffff04ffff04ff0bff1780ffff04ffff02ff1effff04ff
    1effff04ffff04ffff02ff0cffff04ffff04ffff04ff08ff0c80ffff04ff0aff
    168080ffff04ffff04ff80ff5f80ffff04ff0bff2f80808080ff2f80ffff04ff
    80ffff04ffff04ffff04ff80ff1780ff8080ff81bf8080808080ff81ff808080
    ffff04ffff04ffff04ffff01ff02ffff03ffff07ff0380ffff01ff0bffff0102
    ffff02ff02ffff04ff02ff058080ffff02ff02ffff04ff02ff07808080ffff01
    ff0bffff0101ff038080ff0180ffff01ff02ffff03ff0dffff01ff02ffff03ff
    ff02ffff03ff35ffff01ff09ff0bffff02ff0affff04ff0affff04ff35ffff0b
    ffff0101ffff02ff08ffff04ff08ffff02ff25ff0f8080808080808080ffff01
    ff02ff0effff04ff0effff04ff25ff0980808080ff0180ffff01ff02ff0cffff
    04ffff04ffff04ff08ff0c80ffff04ff0aff0e8080ffff04ffff04ffff04ff25
    ff0980ff1d80ffff04ff0bff0f80808080ffff01ff088080ff0180ffff010980
    ff018080ffff04ffff01ff02ffff03ff0dffff01ff02ff02ffff04ff02ffff04
    ffff04ffff17ff09ffff0181ff80ff1d80ffff0bffff0102ffff03ffff18ff09
    ffff010180ff15ff0780ffff03ffff18ff09ffff010180ff07ff158080808080
    ffff010780ff0180ffff04ffff01ff02ffff03ffff09ff0bff0580ffff01ff01
    01ffff01ff02ff02ffff04ff02ffff04ff05ff0f80808080ff0180ffff01ff02
    ffff03ff09ffff01ff02ff02ffff04ff02ffff04ffff04ff19ff0d80ffff04ff
    ff04ff37ff0b80ffff04ffff02ffff02ff11ff0d80ffff04ff27ff2f8080ff3f
    8080808080ffff01ff04ff27ffff04ff37ff0b808080ff0180808080ff018080
    "
);

pub const ACTION_LAYER_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    ddce67597b2f47fa9a005527e19b637f7779da2f195e1a89d9907670863e4fa7
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct ActionLayerArgs<F, S> {
    pub finalizer: F,
    pub merkle_root: Bytes32,
    pub state: S,
}

impl<F, S> ActionLayerArgs<F, S> {
    pub fn new(finalizer: F, merkle_root: Bytes32, state: S) -> Self {
        Self {
            finalizer,
            merkle_root,
            state,
        }
    }
}

impl ActionLayerArgs<TreeHash, TreeHash> {
    pub fn curry_tree_hash(
        finalizer: TreeHash,
        merkle_root: Bytes32,
        state_hash: TreeHash,
    ) -> TreeHash {
        CurriedProgram {
            program: ACTION_LAYER_PUZZLE_HASH,
            args: ActionLayerArgs::<TreeHash, TreeHash>::new(finalizer, merkle_root, state_hash),
        }
        .tree_hash()
    }
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct RawActionLayerSolution<P, S, F> {
    pub puzzles: Vec<P>,
    pub selectors_and_proofs: Vec<(u32, Option<MerkleProof>)>,
    pub solutions: Vec<S>,
    #[clvm(rest)]
    pub finalizer_solution: F,
}

impl<P, S, F> Mod for RawActionLayerSolution<P, S, F> {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&ACTION_LAYER_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        ACTION_LAYER_PUZZLE_HASH
    }
}

impl<P, S> Mod for ActionLayerArgs<P, S> {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&ACTION_LAYER_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        ACTION_LAYER_PUZZLE_HASH
    }
}

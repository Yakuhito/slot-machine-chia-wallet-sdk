use std::borrow::Cow;

use chia_protocol::Bytes32;
use chia_puzzles::{
    NFT_OWNERSHIP_LAYER_HASH, NFT_OWNERSHIP_TRANSFER_PROGRAM_ONE_WAY_CLAIM_WITH_ROYALTIES_HASH,
    NFT_STATE_LAYER_HASH, SINGLETON_LAUNCHER_HASH, SINGLETON_TOP_LAYER_V1_1_HASH,
};
use clvm_traits::{
    clvm_tuple, ClvmDecoder, ClvmEncoder, FromClvm, FromClvmError, ToClvm, ToClvmError,
};
use clvm_utils::{ToTreeHash, TreeHash};
use hex_literal::hex;

use crate::{
    puzzles::{CatalogOtherPrecommitData, ANY_METADATA_UPDATER_HASH},
    Mod,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NftPack {
    pub launcher_hash: Bytes32,
    pub singleton_mod_hash: Bytes32,
    pub state_layer_mod_hash: Bytes32,
    pub metadata_updater_hash_hash: Bytes32,
    pub nft_ownership_layer_mod_hash: Bytes32,
    pub transfer_program_mod_hash: Bytes32,
    pub royalty_puzzle_hash_hash: Bytes32,
    pub trade_price_percentage: u16,
}

impl<N, D: ClvmDecoder<Node = N>> FromClvm<D> for NftPack {
    fn from_clvm(decoder: &D, node: N) -> Result<Self, FromClvmError> {
        #[allow(clippy::type_complexity)]
        let (
            (
                (launcher_hash, singleton_mod_hash),
                (state_layer_mod_hash, metadata_updater_hash_hash),
            ),
            (
                (nft_ownership_layer_mod_hash, transfer_program_mod_hash),
                (royalty_puzzle_hash_hash, trade_price_percentage),
            ),
        ): (
            ((Bytes32, Bytes32), (Bytes32, Bytes32)),
            ((Bytes32, Bytes32), (Bytes32, u16)),
        ) = FromClvm::from_clvm(decoder, node)?;

        Ok(Self {
            launcher_hash,
            singleton_mod_hash,
            state_layer_mod_hash,
            metadata_updater_hash_hash,
            nft_ownership_layer_mod_hash,
            transfer_program_mod_hash,
            royalty_puzzle_hash_hash,
            trade_price_percentage,
        })
    }
}

impl<N, E: ClvmEncoder<Node = N>> ToClvm<E> for NftPack {
    fn to_clvm(&self, encoder: &mut E) -> Result<N, ToClvmError> {
        let obj = clvm_tuple!(
            clvm_tuple!(
                clvm_tuple!(self.launcher_hash, self.singleton_mod_hash,),
                clvm_tuple!(self.state_layer_mod_hash, self.metadata_updater_hash_hash),
            ),
            clvm_tuple!(
                clvm_tuple!(
                    self.nft_ownership_layer_mod_hash,
                    self.transfer_program_mod_hash
                ),
                clvm_tuple!(self.royalty_puzzle_hash_hash, self.trade_price_percentage)
            )
        );

        obj.to_clvm(encoder)
    }
}

impl NftPack {
    pub fn new(royalty_puzzle_hash_hash: Bytes32, trade_price_percentage: u16) -> Self {
        let meta_updater_hash: Bytes32 = ANY_METADATA_UPDATER_HASH.into();

        Self {
            launcher_hash: SINGLETON_LAUNCHER_HASH.into(),
            singleton_mod_hash: SINGLETON_TOP_LAYER_V1_1_HASH.into(),
            state_layer_mod_hash: NFT_STATE_LAYER_HASH.into(),
            metadata_updater_hash_hash: meta_updater_hash.tree_hash().into(),
            nft_ownership_layer_mod_hash: NFT_OWNERSHIP_LAYER_HASH.into(),
            transfer_program_mod_hash:
                NFT_OWNERSHIP_TRANSFER_PROGRAM_ONE_WAY_CLAIM_WITH_ROYALTIES_HASH.into(),
            royalty_puzzle_hash_hash,
            trade_price_percentage,
        }
    }
}

pub const CATALOG_REGISTER_PUZZLE: [u8; 1404] = hex!(
    "
    ff02ffff01ff02ffff03ffff22ffff22ffff0aff820bffff82027f80ffff0aff
    8204ffff820bff8080ffff09ffff02ff04ffff04ff04ff8209ff8080ff82015f
    8080ffff01ff02ffff01ff04ff81bfffff04ffff04ffff0133ffff04ff02ffff
    04ff80ff80808080ffff04ffff02ff7dffff04ffff04ff09ffff04ff15ff5d80
    80ffff04ff0bffff04ffff30ffff30ff82017fff02ff8080ff43ffff010180ff
    822fff80808080ffff04ffff04ffff0146ffff04ff82017fff808080ffff04ff
    ff04ffff013effff04ffff0effff0172ffff02ff09ffff04ff09ffff04ff8217
    ffff822fff80808080ff808080ffff04ffff04ffff0142ffff04ffff0112ffff
    04ff80ffff04ffff02ff15ffff04ff5dffff04ff5fffff04ffff0bffff0101ff
    ff02ff09ffff04ff09ffff04ff8204ffffff04ff8206ffff8209ff8080808080
    ff8080808080ff8080808080ffff04ffff04ffff0142ffff04ffff0112ffff04
    ff80ffff04ffff02ff15ffff04ff5dffff04ff5fffff04ffff0bffff0101ffff
    02ff09ffff04ff09ffff04ff8209ffffff04ff8204ffff820dff8080808080ff
    8080808080ff8080808080ffff04ffff02ff2dffff04ffff04ff15ff5d80ffff
    04ff5fffff02ff09ffff04ff09ffff04ff8217ffffff04ff8204ffff8209ff80
    808080808080ffff04ffff02ff2dffff04ffff04ff15ff5d80ffff04ff5fffff
    02ff09ffff04ff09ffff04ff8204ffffff04ff8206ffff8217ff808080808080
    80ffff04ffff02ff2dffff04ffff04ff15ff5d80ffff04ff5fffff02ff09ffff
    04ff09ffff04ff8209ffffff04ff8217ffff820dff80808080808080ffff04ff
    ff04ffff0142ffff04ffff0113ffff04ffff0101ffff04ffff02ff8213ffffff
    04ffff02ff15ffff04ff5dffff04ff2fffff04ff823fffffff04ffff0bffff01
    02ff8217ffffff0bffff0101ffff02ff09ffff04ff09ffff04ff822fffffff04
    ff8202bfff821bff808080808080ff808080808080ff821bff8080ffff04ff82
    03bfff808080808080ff808080808080808080808080ffff04ffff02ff0affff
    04ff2effff04ff0bffff04ffff0bffff0101ff820bff80ff8080808080ff0180
    80ffff01ff088080ff0180ffff04ffff04ffff01ff02ffff03ffff07ff0380ff
    ff01ff0bffff0102ffff02ff02ffff04ff02ff058080ffff02ff02ffff04ff02
    ff07808080ffff01ff0bffff0101ff038080ff0180ffff04ffff01ff0bffff01
    02ffff0bffff0182010280ffff0bffff0102ffff0bffff0102ffff0bffff0182
    010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff
    010180808080ffff04ffff01ff04ffff0133ffff04ffff02ff04ffff04ff06ff
    ff04ff05ffff04ffff0bffff0101ff0780ff8080808080ffff04ff80ffff04ff
    ff04ff05ff8080ff8080808080ffff04ffff01ff02ffff03ff03ffff01ff0bff
    ff0102ffff0bffff0182010480ffff0bffff0102ffff0bffff0102ffff0bffff
    0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0b
    ffff010180808080ffff01ff0bffff018201018080ff0180ffff01ff02ffff01
    ff04ffff0140ffff04ffff30ff17ffff02ff15ffff04ff1dffff04ff63ffff04
    ff02ffff04ffff02ff15ffff04ff1dffff04ff53ffff04ffff0bffff0101ff53
    80ffff04ffff0bffff010180ffff04ff73ffff04ffff02ff15ffff04ff1dffff
    04ff4bffff04ffff0bffff0101ff4b80ffff04ffff0bffff010180ffff04ffff
    02ff15ffff04ff1dffff04ff6bffff04ff02ffff04ff5bffff04ffff0bffff01
    01ff7b80ff80808080808080ffff04ff1fff8080808080808080ff8080808080
    808080ff808080808080ffff010180ff808080ffff04ffff02ff04ffff04ff04
    ffff04ff31ffff04ff0bff2180808080ff01808080808080ff018080
    "
);

pub const CATALOG_REGISTER_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    058016cb3d9c0e84d7e8b2f410420bf31f970afd079593bfd2bffbcf264992ff
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct CatalogRegisterActionArgs {
    pub nft_pack: NftPack,
    pub uniqueness_prelauncher_1st_curry_hash: Bytes32,
    pub precommit_1st_curry_hash: Bytes32,
    pub slot_1st_curry_hash: Bytes32,
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct PuzzleAndSolution<P, S> {
    pub puzzle: P,
    #[clvm(rest)]
    pub solution: S,
}

impl<P, S> PuzzleAndSolution<P, S> {
    pub fn new(puzzle: P, solution: S) -> Self {
        Self { puzzle, solution }
    }
}

#[derive(FromClvm, ToClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(list)]
pub struct CatalogDoubleTailHashData {
    pub this_tail_hash: Bytes32, // left_tail_hash or right_tail_hash
    #[clvm(rest)]
    pub this_this_tail_hash: Bytes32, // left_left_tail_hash or right_right_tail_hash
}

impl CatalogDoubleTailHashData {
    pub fn new(this_tail_hash: Bytes32, this_this_tail_hash: Bytes32) -> Self {
        Self {
            this_tail_hash,
            this_this_tail_hash,
        }
    }
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct CatalogRegisterActionSolution<P, S> {
    pub my_id: Bytes32,
    pub left_data: CatalogDoubleTailHashData,
    pub right_data: CatalogDoubleTailHashData,
    pub precommitted_cat_maker_data: PuzzleAndSolution<P, S>,
    #[clvm(rest)]
    pub other_precommit_data: CatalogOtherPrecommitData,
}

impl Mod for CatalogRegisterActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&CATALOG_REGISTER_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        CATALOG_REGISTER_PUZZLE_HASH
    }
}

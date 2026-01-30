use std::borrow::Cow;

use chia_protocol::Bytes32;
use chia_puzzle_types::singleton::SingletonStruct;
use chia_puzzles::{NFT_OWNERSHIP_LAYER_HASH, NFT_STATE_LAYER_HASH, SETTLEMENT_PAYMENT_HASH};
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::{
    puzzles::{CompactLineageProof, NONCE_WRAPPER_PUZZLE_HASH},
    Mod,
};

pub const REWARD_DISTRIBUTOR_NFTS_FROM_DID_LOCKING_PUZZLE: [u8; 1070] = hex!(
    // Rue
    "
    ff02ffff01ff02ff16ffff04ffff04ff04ffff04ff0affff04ff2effff04ff3e
    ff1680808080ffff04ffff04ffff04ff05ff0b80ffff04ff17ff8203ff8080ff
    ff04ffff04ff80ffff04ff2fff5f8080ffff04ffff04ff81bfff82017f80ffff
    04ff8202ffff80808080808080ffff04ffff04ffff01ff02ffff03ffff07ff03
    80ffff01ff0bffff0102ffff02ff02ffff04ff02ff058080ffff02ff02ffff04
    ff02ff07808080ffff01ff0bffff0101ff038080ff0180ffff04ffff01ff0bff
    ff0102ffff0bffff0182010280ffff0bffff0102ffff0bffff0102ffff0bffff
    0182010180ff0580ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0b
    ffff010180808080ffff04ffff01ff02ffff03ff3dffff01ff02ffff01ff02ff
    ff01ff02ff81fbffff04ff0bffff04ffff04ff27ffff04ff57ffff04ff81b7ff
    8201f7808080ffff04ffff04ffff10ff4fffff010180ff6f80ffff04ff5fffff
    04ff81bfffff04ffff04ffff013fffff04ff02ff808080ffff04ffff04ffff01
    3effff04ffff0effff016cff0280ff808080ff81ff8080808080808080ffff04
    ffff0bffff02ff15ffff04ff2dffff04ff43ffff04ffff02ff09ffff04ff09ff
    ff04ff43ffff04ffff02ffff01ff02ff81bbffff04ff81bbffff04ff06ffff30
    ff08ffff02ff2bffff04ff5bffff04ff8187ffff04ffff02ff13ffff04ff13ff
    478080ffff04ff14ff808080808080ff1c80808080ffff04ff3eff018080ff81
    e380808080ffff04ffff02ff15ffff04ff2dffff04ff33ffff04ffff0bffff01
    01ff3380ffff04ff04ffff04ff0affff04ffff02ff15ffff04ff2dffff04ff2b
    ffff04ffff0bffff0101ff2b80ffff04ffff0bffff0101ff1680ffff04ff2eff
    ff04ff57ff8080808080808080ff8080808080808080ff808080808080ffff02
    ff09ffff04ff09ffff04ffff02ff09ffff04ff09ffff04ff27ffff04ff6fff5b
    80808080ffff04ffff04ffff02ff15ffff04ff2dffff04ff77ffff04ffff02ff
    09ffff04ff09ffff04ff5fffff0101808080ffff04ff4fff808080808080ffff
    04ffff0101ffff04ffff04ffff02ff15ffff04ff2dffff04ff77ffff04ffff02
    ff09ffff04ff09ffff04ff5fffff0101808080ffff04ff4fff808080808080ff
    8080ff80808080ff808080808080ff018080ffff04ff5dff018080ffff01ff04
    ff13ffff04ffff04ffff0146ffff04ff2dff808080ff3f808080ff0180ffff04
    ffff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff0182010480ffff0b
    ffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ff
    ff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff01ff0bffff01
    8201018080ff0180ffff01ff02ffff03ff05ffff01ff02ffff01ff30ffff02ff
    05ffff04ff05ffff04ff1bff0f808080ff04ff0680ffff04ff09ff018080ffff
    010780ff018080808080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_NFTS_FROM_DID_LOCKING_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    ff77c82d24a17e7b7824e452553cb057a5e4ea16c49a3c863b3e9eff54f757c2
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct RewardDistributorNftsFromDidLockingPuzzleArgs {
    pub did_singleton_struct: SingletonStruct,
    pub nft_state_layer_mod_hash: Bytes32,
    pub nft_ownership_layer_mod_hash: Bytes32,
    pub offer_mod_hash: Bytes32,
    pub nonce_mod_hash: Bytes32,
    pub my_p2_puzzle_hash: Bytes32,
}

impl RewardDistributorNftsFromDidLockingPuzzleArgs {
    pub fn new(did_launcher_id: Bytes32, my_p2_puzzle_hash: Bytes32) -> Self {
        Self {
            did_singleton_struct: SingletonStruct::new(did_launcher_id),
            nft_state_layer_mod_hash: NFT_STATE_LAYER_HASH.into(),
            nft_ownership_layer_mod_hash: NFT_OWNERSHIP_LAYER_HASH.into(),
            offer_mod_hash: SETTLEMENT_PAYMENT_HASH.into(),
            nonce_mod_hash: NONCE_WRAPPER_PUZZLE_HASH.into(),
            my_p2_puzzle_hash,
        }
    }
}

#[derive(FromClvm, ToClvm, Copy, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct IntermediaryCoinProof {
    pub full_puzzle_hash: Bytes32,
    #[clvm(rest)]
    pub amount: u64,
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct NftLauncherProof {
    pub did_proof: CompactLineageProof,
    #[clvm(rest)]
    pub intermediary_coin_proofs: Vec<IntermediaryCoinProof>,
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct StakeNftFromDidInfo {
    pub nft_metadata_hash: Bytes32,
    pub nft_metadata_updater_hash_hash: Bytes32,
    pub nft_owner: Option<Bytes32>,
    pub nft_transfer_porgram_hash: Bytes32,
    #[clvm(rest)]
    pub nft_launcher_proof: NftLauncherProof,
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct RewardDistributorNftsFromDidLockingPuzzleSolution {
    pub my_id: Bytes32,
    #[clvm(rest)]
    pub nft_infos: Vec<StakeNftFromDidInfo>,
}

impl Mod for RewardDistributorNftsFromDidLockingPuzzleArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&REWARD_DISTRIBUTOR_NFTS_FROM_DID_LOCKING_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        REWARD_DISTRIBUTOR_NFTS_FROM_DID_LOCKING_PUZZLE_HASH
    }
}

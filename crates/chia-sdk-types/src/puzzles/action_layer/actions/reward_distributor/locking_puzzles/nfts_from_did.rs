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

pub const REWARD_DISTRIBUTOR_NFTS_FROM_DID_LOCKING_PUZZLE: [u8; 931] = hex!(
    // Rue
    "
    ff02ffff01ff02ff16ffff04ffff04ff04ffff04ff0affff04ff2effff04ff3e
    ff1680808080ffff04ff03ffff04ff8207ffffff04ff80ff808080808080ffff
    04ffff04ffff01ff02ffff03ffff07ff0380ffff01ff0bffff0102ffff02ff02
    ffff04ff02ff058080ffff02ff02ffff04ff02ff07808080ffff01ff0bffff01
    01ff038080ff0180ffff04ffff01ff0bffff0102ffff0bffff0182010280ffff
    0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102
    ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff04ffff01ff
    02ffff03ff0bffff01ff02ffff01ff02ffff01ff02ff81fbffff04ff0bffff04
    ff17ffff04ff6fffff04ffff10ff5fffff010180ffff04ffff04ffff013fffff
    04ff02ff808080ffff04ffff04ffff013effff04ffff0effff016cff0280ff80
    8080ff7f80808080808080ffff04ffff0bffff02ff15ffff04ff2dffff04ff23
    ffff04ffff02ff09ffff04ff09ffff04ff23ffff04ffff02ff5dffff04ff5dff
    ff04ff8207e7ffff30ff8209e7ffff02ff15ffff04ff2dffff04ff23ffff04ff
    ff02ff09ffff04ff09ff138080ffff04ff8215e7ff808080808080ff821de780
    808080ff7380808080ffff04ffff02ff15ffff04ff2dffff04ff2bffff04ffff
    0bffff0101ff2b80ffff04ff47ffff04ff81a7ffff04ffff02ff15ffff04ff2d
    ffff04ff5bffff04ffff0bffff0101ff5b80ffff04ffff0bffff0101ff820167
    80ffff04ff8202e7ffff04ff81bbff8080808080808080ff8080808080808080
    ff808080808080ffff02ff09ffff04ff09ffff04ffff02ff09ffff04ff09ffff
    04ff2fffff04ff8205fbff8217fb80808080ffff04ffff04ff02ffff04ffff01
    01ffff04ffff04ff02ff8080ff80808080ff808080808080ff018080ffff04ff
    ff02ff0affff04ff16ffff04ff81bdffff04ffff02ff04ffff04ff04ffff04ff
    8205fdffff0101808080ffff04ff82017dff808080808080ff018080ffff01ff
    04ff17ffff04ffff04ffff0146ffff04ff820bfdff808080ff1f808080ff0180
    ffff04ffff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff0182010480
    ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff
    0102ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff01ff0b
    ffff018201018080ff0180ffff01ff02ffff03ff05ffff01ff30ffff02ff02ff
    ff04ff02ffff04ff0dff07808080ff11ff1980ffff010780ff018080808080ff
    018080
    "
);

pub const REWARD_DISTRIBUTOR_NFTS_FROM_DID_LOCKING_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    2b39fa5f7dd59c3244dd406238a0d5f4a054d8ede2244d1dc7ee849a5859c416
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

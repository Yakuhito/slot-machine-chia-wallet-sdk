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

pub const REWARD_DISTRIBUTOR_NFTS_FROM_DID_LOCKING_PUZZLE: [u8; 1005] = hex!(
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
    ffff04ffff02ff09ffff04ff09ffff04ff23ffff04ffff02ffff01ff02ff81bb
    ffff04ff81bbffff04ff06ffff30ff08ffff02ff2bffff04ff5bffff04ff47ff
    ff04ffff02ff13ffff04ff13ff278080ffff04ff14ff808080808080ff1c8080
    8080ffff04ff3eff018080ff7380808080ffff04ffff02ff15ffff04ff2dffff
    04ff2bffff04ffff0bffff0101ff2b80ffff04ff04ffff04ff0affff04ffff02
    ff15ffff04ff2dffff04ff5bffff04ffff0bffff0101ff5b80ffff04ffff0bff
    ff0101ff1680ffff04ff2effff04ff81bbff8080808080808080ff8080808080
    808080ff808080808080ffff02ff09ffff04ff09ffff04ffff02ff09ffff04ff
    09ffff04ff2fffff04ff8205fbff8217fb80808080ffff04ffff04ffff02ff15
    ffff04ff2dffff04ff82017bffff04ffff02ff09ffff04ff09ffff04ff820bfb
    ffff0101808080ffff04ff8202fbff808080808080ffff04ffff0101ffff04ff
    ff04ffff02ff15ffff04ff2dffff04ff82017bffff04ffff02ff09ffff04ff09
    ffff04ff820bfbffff0101808080ffff04ff8202fbff808080808080ff8080ff
    80808080ff808080808080ff018080ffff04ff13ff018080ffff01ff04ff17ff
    ff04ffff04ffff0146ffff04ff820bfdff808080ff1f808080ff0180ffff04ff
    ff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff0182010480ffff0bff
    ff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff
    02ff02ffff04ff02ff078080ffff0bffff010180808080ffff01ff0bffff0182
    01018080ff0180ffff01ff02ffff03ff05ffff01ff02ffff01ff30ffff02ff05
    ffff04ff05ffff04ff1bff0f808080ff04ff0680ffff04ff09ff018080ffff01
    0780ff018080808080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_NFTS_FROM_DID_LOCKING_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    7d513ecf5745baef359d5a28a6f869a2819307d84fae0cd14ac420c2d99648e1
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

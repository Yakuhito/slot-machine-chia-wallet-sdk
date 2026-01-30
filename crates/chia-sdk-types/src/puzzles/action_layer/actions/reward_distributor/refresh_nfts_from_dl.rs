use std::borrow::Cow;

use chia_protocol::Bytes32;
use chia_puzzle_types::singleton::SingletonStruct;
use chia_puzzles::{NFT_OWNERSHIP_LAYER_HASH, NFT_STATE_LAYER_HASH};
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::TreeHash;
use hex_literal::hex;

use crate::{
    puzzles::{RewardDistributorEntrySlotValue, NONCE_WRAPPER_PUZZLE_HASH},
    MerkleProof, Mod,
};

pub const REWARD_DISTRIBUTOR_REFRESH_NFTS_FROM_DL_PUZZLE: [u8; 2010] = hex!(
    // Rue
    "
    ff02ffff01ff04ffff04ff8209ffffff04ffff11ff8215ffff824fff80ffff04
    ffff10ff822dffff82afff80ffff04ffff04ff829dffffff10ff82ddffff82ef
    ff8080ff827dff80808080ffff04ffff04ffff0155ffff04ffff10ff82bdffff
    82017f80ff808080ffff04ffff04ffff013fffff04ffff0bffff02ff0affff04
    ff2effff04ff09ffff04ffff02ff04ffff04ff04ff058080ffff04ffff02ff0a
    ffff04ff2effff04ff0bffff04ffff0bffff0101ff0b80ffff04ffff0bffff01
    02ffff0bffff0101ff820bff80ffff02ffff03ff8227ffffff018227ffffff01
    ff01a04bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c77
    85459a80ff018080ffff04ff8257ffffff04ff8277ffff8080808080808080ff
    808080808080ffff012480ff808080ffff02ff26ffff04ff02ffff04ffff04ff
    ff04ff81bfff8202ff80ffff04ff8205ffffff04ff823fffff05808080ffff04
    ffff04ffff04ff0bff1780ffff04ff2fff5f8080ffff04ffff04ff82017fff82
    0bff80ffff04ff824fffffff04ff82efffff82afff80808080808080808080ff
    ff04ffff04ffff01ff02ffff03ffff07ff0380ffff01ff0bffff0102ffff02ff
    02ffff04ff02ff058080ffff02ff02ffff04ff02ff07808080ffff01ff0bffff
    0101ff038080ff0180ffff04ffff01ff0bffff0102ffff0bffff0182010280ff
    ff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff01
    02ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff04ffff04
    ffff01ff02ffff03ff2dffff01ff02ffff03ffff22ffff22ffff22ffff09ffff
    12ffff11ff820275ff82028d80ff82038d80ffff10ffff12ff82024dff1980ff
    82034d8080ffff15ff82034dffff0181ff8080ffff15ff19ff82034d8080ffff
    21ffff15ff82038dff8202cd80ffff09ff8202cdff82038d808080ffff01ff04
    ffff04ffff0142ffff04ffff0112ffff04ff80ffff04ffff02ff0affff04ff2e
    ffff04ff11ffff04ffff0bffff0101ffff02ff04ffff04ff04ff818d808080ff
    8080808080ff8080808080ffff04ffff04ffff0133ffff04ffff02ff0affff04
    ff2effff04ff11ffff04ffff0bffff0101ffff02ff04ffff04ff04ffff04ff82
    010dffff04ff820275ffff10ff82038dff8202cd808080808080ff8080808080
    ffff04ff80ffff04ffff04ff82010dff8080ff8080808080ffff04ffff04ffff
    0181d6ffff04ffff0133ffff04ff82010dffff04ff82024dffff04ffff04ff82
    010dff8080ff808080808080ffff02ff36ffff04ffff04ff04ffff04ff0affff
    04ffff04ff36ff2e80ff3e808080ffff04ffff04ffff04ff3dff2380ffff04ff
    33ff1b8080ffff04ffff04ff818dffff04ff8203cdff378080ffff04ffff04ff
    8202cdffff02ff26ffff04ff02ffff04ffff04ff09ffff04ff15ffff04ff6dff
    3d808080ffff04ff0bffff04ff17ffff04ffff11ff2fff82024d80ffff04ffff
    11ff5fff82034d80ffff11ff7fff8202cd808080808080808080ffff04ff11ff
    ff04ff27ff1980808080808080808080ffff01ff088080ff0180ffff01ff02ff
    ff03ffff22ffff20ff2f80ffff20ff5f8080ffff0180ffff01ff088080ff0180
    80ff0180ffff01ff02ffff03ff2bffff01ff02ffff03ffff22ff818bffff09ff
    3bffff02ff2effff04ff2effff04ff827fcbffff0bffff0101ffff02ff04ffff
    04ff04ffff04ff8205cbff82014b808080808080808080ffff01ff04ffff04ff
    ff0142ffff04ffff0117ffff04ffff02ff04ffff04ff04ffff04ffff0101ffff
    04ffff02ff3effff02ff0affff04ff36ffff04ff2dffff04ffff02ff04ffff04
    ff04ffff04ff23ff82014b808080ffff04ff3dff80808080808080ff80808080
    80ffff04ffff30ff8202cbffff02ff0affff04ff36ffff04ff21ffff04ffff02
    ff04ffff04ff04ffff04ff21ffff04ff8205cbff7180808080ffff04ffff02ff
    0affff04ff36ffff04ff19ffff04ffff0bffff0101ff1980ffff04ff820bcbff
    ff04ff8217cbffff04ffff02ff0affff04ff36ffff04ff15ffff04ffff0bffff
    0101ff1580ffff04ffff0bffff0101ff825fcb80ffff04ff822fcbffff04ffff
    02ff0affff04ff36ffff04ff2dffff04ffff02ff04ffff04ff04ffff04ff23ff
    ff11ff82014bff818b80808080ffff04ff3dff808080808080ff808080808080
    8080ff8080808080808080ff808080808080ffff010180ff8080808080ffff04
    ffff04ffff013effff04ffff0effff0172ff8205cb80ff808080ffff02ff26ff
    ff04ff02ffff04ff05ffff04ffff04ff13ffff04ff6bff3b8080ffff04ffff04
    ffff11ff27ff818b80ff3780ff1f80808080808080ffff01ff088080ff0180ff
    ff01ff02ffff03ff27ffff01ff0880ffff013780ff018080ff018080ffff04ff
    ff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff0182010480ffff0bff
    ff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ffff
    02ff02ffff04ff02ff078080ffff0bffff010180808080ffff01ff0bffff0182
    01018080ff0180ffff04ffff01ff02ffff03ff0dffff01ff02ff02ffff04ff02
    ffff04ffff04ffff17ff09ffff0181ff80ff1d80ffff0bffff0102ffff03ffff
    18ff09ffff010180ff15ff0780ffff03ffff18ff09ffff010180ff07ff158080
    808080ffff010780ff0180ffff01ff04ffff0133ffff04ff01ffff04ffff0101
    ffff04ffff04ff01ff8080ff80808080808080808080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_REFRESH_NFTS_FROM_DL_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    65fceaa374679b631aee813013b82e744dbfe09a51ae646e375c72732113d579
    "
));

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct RewardDistributorRefreshNftsFromDlActionArgs {
    pub dl_singleton_struct: SingletonStruct,
    pub nft_state_layer_mod_hash: Bytes32,
    pub nft_ownership_layer_mod_hash: Bytes32,
    pub nonce_mod_hash: Bytes32,
    pub my_p2_puzzle_hash: Bytes32,
    pub entry_slot_1st_curry_hash: Bytes32,
    pub max_second_offset: u64,
    pub precision: u64,
}

impl RewardDistributorRefreshNftsFromDlActionArgs {
    pub fn new(
        dl_launcher_id: Bytes32,
        my_p2_puzzle_hash: Bytes32,
        entry_slot_1st_curry_hash: Bytes32,
        max_second_offset: u64,
        precision: u64,
    ) -> Self {
        Self {
            dl_singleton_struct: SingletonStruct::new(dl_launcher_id),
            nft_state_layer_mod_hash: NFT_STATE_LAYER_HASH.into(),
            nft_ownership_layer_mod_hash: NFT_OWNERSHIP_LAYER_HASH.into(),
            nonce_mod_hash: NONCE_WRAPPER_PUZZLE_HASH.into(),
            my_p2_puzzle_hash,
            entry_slot_1st_curry_hash,
            max_second_offset,
            precision,
        }
    }
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct RefreshNftInfo {
    pub nft_shares_delta: i64,
    pub new_nft_shares: u64,
    pub nft_parent_id: Bytes32,
    pub nft_launcher_id: Bytes32,
    pub nft_metadata_hash: Bytes32,
    pub nft_metadata_updater_hash_hash: Bytes32,
    pub nft_transfer_porgram_hash: Bytes32,
    pub nft_owner: Option<Bytes32>,
    #[clvm(rest)]
    pub nft_inclusion_proof: MerkleProof,
}

#[derive(FromClvm, ToClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(list)]
pub struct RewardDistributorEntryPayoutInfo {
    pub payout_amount: u64,
    #[clvm(rest)]
    pub payout_rounding_error: u128,
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct SlotAndNfts {
    pub existing_slot_value: RewardDistributorEntrySlotValue,
    pub entry_payout_info: RewardDistributorEntryPayoutInfo,
    pub nfts_total_shares_delta: i64,
    #[clvm(rest)]
    pub nfts: Vec<RefreshNftInfo>,
}

#[derive(FromClvm, ToClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(list)]
pub struct RewardDistributorDlInfo {
    pub dl_metadata_rest_hash: Option<Bytes32>,
    pub dl_metadata_updater_hash_hash: Bytes32,
    #[clvm(rest)]
    pub dl_inner_puzzle_hash: Bytes32,
}

#[derive(FromClvm, ToClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(list)]
pub struct RewardDistributorRefreshNftsTotals {
    pub total_entry_payout_amount: u64,
    pub total_shares_delta: i128,
    #[clvm(rest)]
    pub total_payout_rounding_error: u128,
}

#[derive(FromClvm, ToClvm, Debug, Clone, PartialEq, Eq)]
#[clvm(list)]
pub struct RewardDistributorRefreshNftsFromDlActionSolution {
    pub dl_root_hash: Bytes32,
    pub dl_info: RewardDistributorDlInfo,
    pub totals: RewardDistributorRefreshNftsTotals,
    #[clvm(rest)]
    pub slots_and_nfts: Vec<SlotAndNfts>,
}

impl Mod for RewardDistributorRefreshNftsFromDlActionArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&REWARD_DISTRIBUTOR_REFRESH_NFTS_FROM_DL_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        REWARD_DISTRIBUTOR_REFRESH_NFTS_FROM_DL_PUZZLE_HASH
    }
}

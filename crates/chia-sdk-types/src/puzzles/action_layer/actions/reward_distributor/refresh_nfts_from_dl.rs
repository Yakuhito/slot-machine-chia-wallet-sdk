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

pub const REWARD_DISTRIBUTOR_REFRESH_NFTS_FROM_DL_PUZZLE: [u8; 1821] = hex!(
    // Rue
    "
    ff02ffff01ff04ffff04ff8209ffffff04ffff11ff8215ffff824fff80ffff04
    ffff10ff822dffff82afff80ffff04ffff04ff829dffffff10ff82ddffff82ef
    ff8080ff827dff80808080ffff04ffff04ffff0155ffff04ffff10ff82bdffff
    82017f80ff808080ffff04ffff04ffff013fffff04ffff0bffff02ff0affff04
    ff2effff04ff09ffff04ffff02ff04ffff04ff04ff058080ffff04ffff02ff0a
    ffff04ff2effff04ff0bffff04ffff0bffff0101ff0b80ffff04ffff0bffff01
    02ffff0bffff0101ff820bff80ffff02ffff03ff8227ffffff018227ffffff01
    ff0bffff01018080ff018080ffff04ff8257ffffff04ff8277ffff8080808080
    808080ff808080808080ffff012480ff808080ffff02ff26ffff04ff02ffff04
    ffff04ff03ff823fff80ffff04ff824fffffff04ff82efffff82afff80808080
    80808080ffff04ffff04ffff01ff02ffff03ffff07ff0380ffff01ff0bffff01
    02ffff02ff02ffff04ff02ff058080ffff02ff02ffff04ff02ff07808080ffff
    01ff0bffff0101ff038080ff0180ffff04ffff01ff0bffff0102ffff0bffff01
    82010280ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ff
    ff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ff
    ff04ffff04ffff01ff02ffff03ff0dffff01ff02ffff03ffff22ffff22ffff22
    ffff09ffff12ffff11ff830277f9ff81a580ff81e580ffff10ffff12ff8195ff
    820bf980ff81d58080ffff15ff81d5ffff0181ff8080ffff15ff820bf9ff81d5
    8080ffff21ffff15ff81e5ff81b580ffff09ff81b5ff81e5808080ffff01ff04
    ffff04ffff0142ffff04ffff0112ffff04ff80ffff04ffff02ff0affff04ff2e
    ffff04ff8202f9ffff04ffff0bffff0101ffff02ff04ffff04ff04ff25808080
    ff8080808080ff8080808080ffff04ffff04ffff0133ffff04ffff02ff0affff
    04ff2effff04ff8202f9ffff04ffff0bffff0101ffff02ff04ffff04ff04ffff
    04ff45ffff04ff830277f9ffff10ff81e5ff81b5808080808080ff8080808080
    ffff04ff80ffff04ffff04ff45ff8080ff8080808080ffff04ffff04ffff0181
    d6ffff04ffff0133ffff04ff45ffff04ff8195ffff04ffff04ff45ff8080ff80
    8080808080ffff02ff36ffff04ffff04ff04ffff04ff0affff04ffff04ff36ff
    2e80ff3e808080ffff04ff09ffff04ffff04ff25ff81f580ffff04ff81b5ffff
    02ff26ffff04ff02ffff04ffff04ff09ff1d80ffff04ffff11ff0bff819580ff
    ff04ffff11ff17ff81d580ffff11ff1fff81b580808080808080808080808080
    80ffff01ff088080ff0180ffff01ff02ffff03ffff22ffff20ff0b80ffff20ff
    178080ffff0180ffff01ff088080ff018080ff0180ffff01ff02ffff03ff1bff
    ff01ff02ffff03ffff22ff4bffff09ff8217fdffff02ff2effff04ff2effff04
    ff823febffff0bffff0101ffff02ff04ffff04ff04ffff04ff8202ebff81ab80
    8080808080808080ffff01ff04ffff04ffff0142ffff04ffff0117ffff04ffff
    02ff04ffff04ff04ffff04ffff0101ffff04ffff02ff3effff02ff0affff04ff
    36ffff04ff5dffff04ffff02ff04ffff04ff04ffff04ff23ff81ab808080ffff
    04ff81bdff80808080808080ff8080808080ffff04ffff30ff82016bffff02ff
    0affff04ff36ffff04ff11ffff04ffff02ff04ffff04ff04ffff04ff11ffff04
    ff8202ebff3980808080ffff04ffff02ff0affff04ff36ffff04ff15ffff04ff
    ff0bffff0101ff1580ffff04ff8205ebffff04ff820bebffff04ffff02ff0aff
    ff04ff36ffff04ff2dffff04ffff0bffff0101ff2d80ffff04ffff0bffff0101
    ff822feb80ffff04ff8217ebffff04ffff02ff0affff04ff36ffff04ff5dffff
    04ffff02ff04ffff04ff04ffff04ff23ffff11ff81abff4b80808080ffff04ff
    81bdff808080808080ff8080808080808080ff8080808080808080ff80808080
    8080ffff010180ff8080808080ffff04ffff04ffff013effff04ffff0effff01
    72ff8202eb80ff808080ffff02ff26ffff04ff02ffff04ff05ffff04ffff04ff
    13ff3b80ffff04ffff11ff17ff4b80ff1f80808080808080ffff01ff088080ff
    0180ffff01ff02ffff03ff17ffff01ff0880ffff011f80ff018080ff018080ff
    ff04ffff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff0182010480ff
    ff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff01
    02ffff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff01ff0bff
    ff018201018080ff0180ffff04ffff01ff02ffff03ff0dffff01ff02ff02ffff
    04ff02ffff04ffff04ffff17ff09ffff0181ff80ff1d80ffff0bffff0102ffff
    03ffff18ff09ffff010180ff15ff0780ffff03ffff18ff09ffff010180ff07ff
    158080808080ffff010780ff0180ffff01ff04ffff0133ffff04ff01ffff04ff
    ff0101ffff04ffff04ff01ff8080ff80808080808080808080ff018080
    "
);

pub const REWARD_DISTRIBUTOR_REFRESH_NFTS_FROM_DL_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    fb2b73b94771fbc4013d5e49e3016cf9082a4f122bf2fed4fb827196d375da59
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

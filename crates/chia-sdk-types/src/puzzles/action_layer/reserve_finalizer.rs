use std::borrow::Cow;

use chia_protocol::Bytes32;
use clvm_traits::{FromClvm, ToClvm};
use clvm_utils::{CurriedProgram, ToTreeHash, TreeHash};
use hex_literal::hex;

use crate::{puzzles::ACTION_LAYER_PUZZLE_HASH, Mod};

pub const RESERVE_FINALIZER_PUZZLE: [u8; 751] = hex!(
    // Rue
    "
    ff02ffff01ff02ffff01ff04ffff04ffff0133ffff04ffff02ff2dffff04ff3d
    ffff04ff0bffff04ffff02ff2dffff04ff3dffff04ff82017fffff04ffff0bff
    ff0101ff82017f80ff8080808080ffff04ffff0bffff0101ff8204ff80ffff04
    ffff02ff15ffff04ff15ff8219ff8080ff80808080808080ffff04ffff0101ff
    ff04ffff04ff81bfff8080ff8080808080ffff04ffff04ffff0142ffff04ffff
    0117ffff04ffff02ff15ffff04ff15ffff04ffff0101ffff04ffff04ffff0133
    ffff04ff2fffff04ffff02ff5fffff04ff8219ffff808080ffff04ffff04ff2f
    ff8080ff8080808080ff0680808080ffff04ffff30ff8207ffff17ffff02ff5f
    ffff04ff8206ffff80808080ff8080808080ff048080ffff04ffff02ff04ffff
    04ff04ffff04ff80ffff04ff80ff8206ff80808080ff018080ffff04ffff04ff
    ff01ff02ffff03ff17ffff01ff02ffff03ffff09ff47ffff0181d680ffff01ff
    02ff02ffff04ff02ffff04ff05ffff04ffff04ff67ff0b80ffff04ff37ff1f80
    80808080ffff01ff02ff02ffff04ff02ffff04ffff04ff67ff0580ffff04ff0b
    ffff04ff37ff1f808080808080ff0180ffff01ff02ffff03ff1fffff01ff02ff
    02ffff04ff02ffff04ff05ffff04ff0bff1f80808080ffff01ff04ff05ff0b80
    80ff018080ff0180ffff04ffff01ff02ffff03ffff07ff0380ffff01ff0bffff
    0102ffff02ff02ffff04ff02ff058080ffff02ff02ffff04ff02ff07808080ff
    ff01ff0bffff0101ff038080ff0180ffff04ffff01ff0bffff0102ffff0bffff
    0182010280ffff0bffff0102ffff0bffff0102ffff0bffff0182010180ff0580
    ffff0bffff0102ffff02ff02ffff04ff02ff078080ffff0bffff010180808080
    ffff01ff02ffff03ff03ffff01ff0bffff0102ffff0bffff0182010480ffff0b
    ffff0102ffff0bffff0102ffff0bffff0182010180ff0580ffff0bffff0102ff
    ff02ff02ffff04ff02ff078080ffff0bffff010180808080ffff01ff0bffff01
    8201018080ff0180808080ff018080
    "
);

pub const RESERVE_FINALIZER_PUZZLE_HASH: TreeHash = TreeHash::new(hex!(
    "
    c706430c3b133e70a78eb7dce5fd208c991840a0a88dd6f8aa69310008f39536
    "
));

// run '(mod state (f state))' -d
pub const RESERVE_FINALIZER_DEFAULT_RESERVE_AMOUNT_FROM_STATE_PROGRAM: [u8; 1] = hex!("02");
pub const RESERVE_FINALIZER_DEFAULT_RESERVE_AMOUNT_FROM_STATE_PROGRAM_HASH: TreeHash =
    TreeHash::new(hex!(
        "a12871fee210fb8619291eaea194581cbd2531e4b23759d225f6806923f63222"
    ));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultReserveAmountFromStateProgramArgs {}

impl Mod for DefaultReserveAmountFromStateProgramArgs {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&RESERVE_FINALIZER_DEFAULT_RESERVE_AMOUNT_FROM_STATE_PROGRAM)
    }

    fn mod_hash() -> TreeHash {
        RESERVE_FINALIZER_DEFAULT_RESERVE_AMOUNT_FROM_STATE_PROGRAM_HASH
    }
}

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct ReserveFinalizer1stCurryArgs<P> {
    pub action_layer_mod_hash: Bytes32,
    pub reserve_full_puzzle_hash: Bytes32,
    pub reserve_inner_puzzle_hash: Bytes32,
    pub reserve_amount_from_state_program: P,
    pub hint: Bytes32,
}

impl<P> ReserveFinalizer1stCurryArgs<P> {
    pub fn new(
        reserve_full_puzzle_hash: Bytes32,
        reserve_inner_puzzle_hash: Bytes32,
        reserve_amount_from_state_program: P,
        hint: Bytes32,
    ) -> Self {
        Self {
            action_layer_mod_hash: ACTION_LAYER_PUZZLE_HASH.into(),
            reserve_full_puzzle_hash,
            reserve_inner_puzzle_hash,
            reserve_amount_from_state_program,
            hint,
        }
    }

    pub fn curry_tree_hash(
        reserve_full_puzzle_hash: Bytes32,
        reserve_inner_puzzle_hash: Bytes32,
        reserve_amount_from_state_program: TreeHash,
        hint: Bytes32,
    ) -> TreeHash {
        CurriedProgram {
            program: RESERVE_FINALIZER_PUZZLE_HASH,
            args: ReserveFinalizer1stCurryArgs::new(
                reserve_full_puzzle_hash,
                reserve_inner_puzzle_hash,
                reserve_amount_from_state_program,
                hint,
            ),
        }
        .tree_hash()
    }
}

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(curry)]
pub struct ReserveFinalizer2ndCurryArgs {
    pub finalizer_self_hash: Bytes32,
}

impl ReserveFinalizer2ndCurryArgs {
    pub fn new<P>(
        reserve_full_puzzle_hash: Bytes32,
        reserve_inner_puzzle_hash: Bytes32,
        reserve_amount_from_state_program: &P,
        hint: Bytes32,
    ) -> Self
    where
        P: ToTreeHash,
    {
        Self {
            finalizer_self_hash: ReserveFinalizer1stCurryArgs::<TreeHash>::curry_tree_hash(
                reserve_full_puzzle_hash,
                reserve_inner_puzzle_hash,
                reserve_amount_from_state_program.tree_hash(),
                hint,
            )
            .into(),
        }
    }

    pub fn curry_tree_hash(
        reserve_full_puzzle_hash: Bytes32,
        reserve_inner_puzzle_hash: Bytes32,
        reserve_amount_from_state_program: TreeHash,
        hint: Bytes32,
    ) -> TreeHash {
        let self_hash = ReserveFinalizer1stCurryArgs::<TreeHash>::curry_tree_hash(
            reserve_full_puzzle_hash,
            reserve_inner_puzzle_hash,
            reserve_amount_from_state_program,
            hint,
        );

        CurriedProgram {
            program: self_hash,
            args: ReserveFinalizer2ndCurryArgs {
                finalizer_self_hash: self_hash.into(),
            },
        }
        .tree_hash()
    }
}

#[derive(ToClvm, FromClvm, Debug, Clone, Copy, PartialEq, Eq)]
#[clvm(transparent)]
pub struct ReserveFinalizerSolution {
    pub reserve_parent_id: Bytes32,
}

impl<P> Mod for ReserveFinalizer1stCurryArgs<P> {
    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Borrowed(&RESERVE_FINALIZER_PUZZLE)
    }

    fn mod_hash() -> TreeHash {
        RESERVE_FINALIZER_PUZZLE_HASH
    }
}

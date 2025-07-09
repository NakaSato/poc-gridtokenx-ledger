// This file is part of the energy trading token system pallet.
// Weight functions for token system operations.

use frame_support::{
    traits::Get,
    weights::{Weight, constants::RocksDbWeight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_token_system.
pub trait WeightInfo {
    fn initialize_tokens() -> Weight;
    fn mint_grid_tokens() -> Weight;
    fn mint_watt_tokens() -> Weight;
    fn burn_grid_tokens() -> Weight;
    fn burn_watt_tokens() -> Weight;
    fn transfer_grid_tokens() -> Weight;
    fn transfer_watt_tokens() -> Weight;
    fn stake_tokens() -> Weight;
    fn unstake_tokens() -> Weight;
    fn claim_rewards() -> Weight;
    fn create_proposal() -> Weight;
    fn vote_on_proposal() -> Weight;
    fn finalize_proposal() -> Weight;
    fn update_watt_price() -> Weight;
}

/// Weights for pallet_token_system using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn initialize_tokens() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 10_000 nanos.
        Weight::from_parts(10_000, 0)
            .saturating_add(T::DbWeight::get().writes(2))
    }

    fn mint_grid_tokens() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `174`
        //  Estimated: `3639`
        // Minimum execution time: 15_000 nanos.
        Weight::from_parts(15_000, 3639)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }

    fn mint_watt_tokens() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `174`
        //  Estimated: `3639`
        // Minimum execution time: 15_000 nanos.
        Weight::from_parts(15_000, 3639)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }

    fn burn_grid_tokens() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `207`
        //  Estimated: `3672`
        // Minimum execution time: 18_000 nanos.
        Weight::from_parts(18_000, 3672)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }

    fn burn_watt_tokens() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `207`
        //  Estimated: `3672`
        // Minimum execution time: 18_000 nanos.
        Weight::from_parts(18_000, 3672)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }

    fn transfer_grid_tokens() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `103`
        //  Estimated: `6196`
        // Minimum execution time: 20_000 nanos.
        Weight::from_parts(20_000, 6196)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }

    fn transfer_watt_tokens() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `103`
        //  Estimated: `6196`
        // Minimum execution time: 20_000 nanos.
        Weight::from_parts(20_000, 6196)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }

    fn stake_tokens() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `207`
        //  Estimated: `3672`
        // Minimum execution time: 25_000 nanos.
        Weight::from_parts(25_000, 3672)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
    }

    fn unstake_tokens() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3705`
        // Minimum execution time: 30_000 nanos.
        Weight::from_parts(30_000, 3705)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
    }

    fn claim_rewards() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `240`
        //  Estimated: `3705`
        // Minimum execution time: 35_000 nanos.
        Weight::from_parts(35_000, 3705)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }

    fn create_proposal() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `174`
        //  Estimated: `3639`
        // Minimum execution time: 20_000 nanos.
        Weight::from_parts(20_000, 3639)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }

    fn vote_on_proposal() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `340`
        //  Estimated: `3805`
        // Minimum execution time: 25_000 nanos.
        Weight::from_parts(25_000, 3805)
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(2))
    }

    fn finalize_proposal() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `273`
        //  Estimated: `3738`
        // Minimum execution time: 20_000 nanos.
        Weight::from_parts(20_000, 3738)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }

    fn update_watt_price() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `174`
        //  Estimated: `3639`
        // Minimum execution time: 15_000 nanos.
        Weight::from_parts(15_000, 3639)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn initialize_tokens() -> Weight {
        Weight::from_parts(10_000, 0)
    }

    fn mint_grid_tokens() -> Weight {
        Weight::from_parts(15_000, 0)
    }

    fn mint_watt_tokens() -> Weight {
        Weight::from_parts(15_000, 0)
    }

    fn burn_grid_tokens() -> Weight {
        Weight::from_parts(18_000, 0)
    }

    fn burn_watt_tokens() -> Weight {
        Weight::from_parts(18_000, 0)
    }

    fn transfer_grid_tokens() -> Weight {
        Weight::from_parts(20_000, 0)
    }

    fn transfer_watt_tokens() -> Weight {
        Weight::from_parts(20_000, 0)
    }

    fn stake_tokens() -> Weight {
        Weight::from_parts(25_000, 0)
    }

    fn unstake_tokens() -> Weight {
        Weight::from_parts(30_000, 0)
    }

    fn claim_rewards() -> Weight {
        Weight::from_parts(35_000, 0)
    }

    fn create_proposal() -> Weight {
        Weight::from_parts(20_000, 0)
    }

    fn vote_on_proposal() -> Weight {
        Weight::from_parts(25_000, 0)
    }

    fn finalize_proposal() -> Weight {
        Weight::from_parts(20_000, 0)
    }

    fn update_watt_price() -> Weight {
        Weight::from_parts(15_000, 0)
    }
}

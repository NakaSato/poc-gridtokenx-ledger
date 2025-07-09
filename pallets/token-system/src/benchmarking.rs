//! Benchmarking setup for pallet-token-system

use super::*;
use crate::Pallet as TokenSystem;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_std::vec;

benchmarks! {
    initialize_tokens {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Root, 1_000_000u64, 1_000_000u64)
    verify {
        assert!(TokenInfoStorage::<T>::contains_key(TokenType::Grid));
        assert!(TokenInfoStorage::<T>::contains_key(TokenType::Watt));
    }

    mint_grid_tokens {
        let caller: T::AccountId = whitelisted_caller();
        TokenSystem::<T>::initialize_tokens(RawOrigin::Root.into(), 1_000_000u64, 1_000_000u64)?;
    }: _(RawOrigin::Root, caller.clone(), 1000u64)
    verify {
        assert_eq!(TokenSystem::<T>::get_grid_balance(&caller), 1000u64);
    }

    mint_watt_tokens {
        let caller: T::AccountId = whitelisted_caller();
        TokenSystem::<T>::initialize_tokens(RawOrigin::Root.into(), 1_000_000u64, 1_000_000u64)?;
    }: _(RawOrigin::Root, caller.clone(), 2000u64)
    verify {
        assert_eq!(TokenSystem::<T>::get_watt_balance(&caller), 2000u64);
    }

    burn_grid_tokens {
        let caller: T::AccountId = whitelisted_caller();
        TokenSystem::<T>::initialize_tokens(RawOrigin::Root.into(), 1_000_000u64, 1_000_000u64)?;
        TokenSystem::<T>::mint_grid_tokens(RawOrigin::Root.into(), caller.clone(), 1000u64)?;
    }: _(RawOrigin::Root, caller.clone(), 500u64)
    verify {
        assert_eq!(TokenSystem::<T>::get_grid_balance(&caller), 500u64);
    }

    burn_watt_tokens {
        let caller: T::AccountId = whitelisted_caller();
        TokenSystem::<T>::initialize_tokens(RawOrigin::Root.into(), 1_000_000u64, 1_000_000u64)?;
        TokenSystem::<T>::mint_watt_tokens(RawOrigin::Root.into(), caller.clone(), 2000u64)?;
    }: _(RawOrigin::Root, caller.clone(), 1000u64)
    verify {
        assert_eq!(TokenSystem::<T>::get_watt_balance(&caller), 1000u64);
    }

    transfer_grid_tokens {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        TokenSystem::<T>::initialize_tokens(RawOrigin::Root.into(), 1_000_000u64, 1_000_000u64)?;
        TokenSystem::<T>::mint_grid_tokens(RawOrigin::Root.into(), caller.clone(), 1000u64)?;
    }: _(RawOrigin::Signed(caller.clone()), recipient.clone(), 300u64)
    verify {
        assert_eq!(TokenSystem::<T>::get_grid_balance(&caller), 700u64);
        assert_eq!(TokenSystem::<T>::get_grid_balance(&recipient), 300u64);
    }

    transfer_watt_tokens {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        TokenSystem::<T>::initialize_tokens(RawOrigin::Root.into(), 1_000_000u64, 1_000_000u64)?;
        TokenSystem::<T>::mint_watt_tokens(RawOrigin::Root.into(), caller.clone(), 2000u64)?;
    }: _(RawOrigin::Signed(caller.clone()), recipient.clone(), 500u64)
    verify {
        assert_eq!(TokenSystem::<T>::get_watt_balance(&caller), 1500u64);
        assert_eq!(TokenSystem::<T>::get_watt_balance(&recipient), 500u64);
    }

    stake_tokens {
        let caller: T::AccountId = whitelisted_caller();
        TokenSystem::<T>::initialize_tokens(RawOrigin::Root.into(), 1_000_000u64, 1_000_000u64)?;
        TokenSystem::<T>::mint_grid_tokens(RawOrigin::Root.into(), caller.clone(), 5000u64)?;
    }: _(RawOrigin::Signed(caller.clone()), 2000u64)
    verify {
        assert!(TokenSystem::<T>::get_stake_info(&caller).is_some());
        assert_eq!(TokenSystem::<T>::get_stake_info(&caller).unwrap().amount, 2000u64);
    }

    unstake_tokens {
        let caller: T::AccountId = whitelisted_caller();
        TokenSystem::<T>::initialize_tokens(RawOrigin::Root.into(), 1_000_000u64, 1_000_000u64)?;
        TokenSystem::<T>::mint_grid_tokens(RawOrigin::Root.into(), caller.clone(), 5000u64)?;
        TokenSystem::<T>::stake_tokens(RawOrigin::Signed(caller.clone()).into(), 3000u64)?;
    }: _(RawOrigin::Signed(caller.clone()), 1000u64)
    verify {
        assert_eq!(TokenSystem::<T>::get_stake_info(&caller).unwrap().amount, 2000u64);
    }

    claim_rewards {
        let caller: T::AccountId = whitelisted_caller();
        TokenSystem::<T>::initialize_tokens(RawOrigin::Root.into(), 1_000_000u64, 1_000_000u64)?;
        TokenSystem::<T>::mint_grid_tokens(RawOrigin::Root.into(), caller.clone(), 5000u64)?;
        TokenSystem::<T>::stake_tokens(RawOrigin::Signed(caller.clone()).into(), 2000u64)?;
        // Fast forward some blocks to accumulate rewards
        frame_system::Pallet::<T>::set_block_number(100u32.into());
    }: _(RawOrigin::Signed(caller.clone()))
    verify {
        let stake_info = TokenSystem::<T>::get_stake_info(&caller).unwrap();
        assert_eq!(stake_info.unclaimed_rewards, 0u64);
    }

    create_proposal {
        let caller: T::AccountId = whitelisted_caller();
        TokenSystem::<T>::initialize_tokens(RawOrigin::Root.into(), 1_000_000u64, 1_000_000u64)?;
        TokenSystem::<T>::mint_grid_tokens(RawOrigin::Root.into(), caller.clone(), 5000u64)?;
        TokenSystem::<T>::stake_tokens(RawOrigin::Signed(caller.clone()).into(), 2000u64)?;
    }: _(RawOrigin::Signed(caller.clone()), b"Test Proposal".to_vec(), b"Description".to_vec(), 100u32.into())
    verify {
        assert!(TokenSystem::<T>::get_proposal(0).is_some());
    }

    vote_on_proposal {
        let caller: T::AccountId = whitelisted_caller();
        let voter: T::AccountId = account("voter", 0, 0);
        TokenSystem::<T>::initialize_tokens(RawOrigin::Root.into(), 1_000_000u64, 1_000_000u64)?;
        TokenSystem::<T>::mint_grid_tokens(RawOrigin::Root.into(), caller.clone(), 5000u64)?;
        TokenSystem::<T>::mint_grid_tokens(RawOrigin::Root.into(), voter.clone(), 5000u64)?;
        TokenSystem::<T>::stake_tokens(RawOrigin::Signed(caller.clone()).into(), 2000u64)?;
        TokenSystem::<T>::stake_tokens(RawOrigin::Signed(voter.clone()).into(), 3000u64)?;
        TokenSystem::<T>::create_proposal(
            RawOrigin::Signed(caller.clone()).into(),
            b"Test Proposal".to_vec(),
            b"Description".to_vec(),
            100u32.into()
        )?;
    }: _(RawOrigin::Signed(voter.clone()), 0u32, true)
    verify {
        assert!(TokenSystem::<T>::has_voted(0u32, &voter));
    }

    finalize_proposal {
        let caller: T::AccountId = whitelisted_caller();
        let voter: T::AccountId = account("voter", 0, 0);
        TokenSystem::<T>::initialize_tokens(RawOrigin::Root.into(), 1_000_000u64, 1_000_000u64)?;
        TokenSystem::<T>::mint_grid_tokens(RawOrigin::Root.into(), caller.clone(), 5000u64)?;
        TokenSystem::<T>::mint_grid_tokens(RawOrigin::Root.into(), voter.clone(), 5000u64)?;
        TokenSystem::<T>::stake_tokens(RawOrigin::Signed(caller.clone()).into(), 2000u64)?;
        TokenSystem::<T>::stake_tokens(RawOrigin::Signed(voter.clone()).into(), 3000u64)?;
        TokenSystem::<T>::create_proposal(
            RawOrigin::Signed(caller.clone()).into(),
            b"Test Proposal".to_vec(),
            b"Description".to_vec(),
            50u32.into()
        )?;
        TokenSystem::<T>::vote_on_proposal(RawOrigin::Signed(voter.clone()).into(), 0u32, true)?;
        // Fast forward past voting deadline
        frame_system::Pallet::<T>::set_block_number(100u32.into());
    }: _(RawOrigin::Signed(caller.clone()), 0u32)
    verify {
        let proposal = TokenSystem::<T>::get_proposal(0).unwrap();
        assert_ne!(proposal.status, ProposalStatus::Active);
    }

    update_watt_price {
        TokenSystem::<T>::initialize_tokens(RawOrigin::Root.into(), 1_000_000u64, 1_000_000u64)?;
    }: _(RawOrigin::Root, 9500u32)
    verify {
        let watt_info = TokenSystem::<T>::get_token_info(TokenType::Watt).unwrap();
        assert_eq!(watt_info.price, 9500u32);
    }
}

impl_benchmark_test_suite!(TokenSystem, crate::mock::new_test_ext(), crate::mock::Test);

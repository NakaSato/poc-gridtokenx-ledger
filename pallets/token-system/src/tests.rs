use crate::{mock::*, Error, Event, TokenType, TokenInfo, StakeInfo, ProposalStatus};
use frame_support::{
    assert_noop, assert_ok,
    traits::Get,
};
use sp_arithmetic::Percent;

/// Test token initialization
#[test]
fn test_initialize_tokens() {
    new_test_ext().execute_with(|| {
        // Initialize tokens
        assert_ok!(TokenSystem::initialize_tokens(
            RuntimeOrigin::root(),
            1_000_000, // GRID supply
            1_000_000, // WATT supply
        ));

        // Check GRID token info
        let grid_info = TokenSystem::get_token_info(TokenType::Grid).unwrap();
        assert_eq!(grid_info.total_supply, 1_000_000);
        assert_eq!(grid_info.price, 10000);
        assert_eq!(grid_info.is_active, true);

        // Check WATT token info
        let watt_info = TokenSystem::get_token_info(TokenType::Watt).unwrap();
        assert_eq!(watt_info.total_supply, 1_000_000);
        assert_eq!(watt_info.price, 10000);
        assert_eq!(watt_info.is_active, true);
    });
}

/// Test GRID token minting and burning
#[test]
fn test_grid_token_operations() {
    new_test_ext().execute_with(|| {
        // Initialize tokens
        assert_ok!(TokenSystem::initialize_tokens(
            RuntimeOrigin::root(),
            1_000_000,
            1_000_000,
        ));

        // Mint GRID tokens
        assert_ok!(TokenSystem::mint_grid_tokens(
            RuntimeOrigin::root(),
            1, // account
            1000, // amount
        ));

        // Check balance
        assert_eq!(TokenSystem::get_grid_balance(&1), 1000);

        // Check event
        System::assert_has_event(
            Event::GridTokensMinted { account: 1, amount: 1000 }.into()
        );

        // Burn GRID tokens
        assert_ok!(TokenSystem::burn_grid_tokens(
            RuntimeOrigin::root(),
            1, // account
            500, // amount
        ));

        // Check balance after burn
        assert_eq!(TokenSystem::get_grid_balance(&1), 500);

        // Check event
        System::assert_has_event(
            Event::GridTokensBurned { account: 1, amount: 500 }.into()
        );

        // Try to burn more than available
        assert_noop!(
            TokenSystem::burn_grid_tokens(
                RuntimeOrigin::root(),
                1,
                1000,
            ),
            Error::<Test>::InsufficientBalance
        );
    });
}

/// Test WATT token operations
#[test]
fn test_watt_token_operations() {
    new_test_ext().execute_with(|| {
        // Initialize tokens
        assert_ok!(TokenSystem::initialize_tokens(
            RuntimeOrigin::root(),
            1_000_000,
            1_000_000,
        ));

        // Mint WATT tokens
        assert_ok!(TokenSystem::mint_watt_tokens(
            RuntimeOrigin::root(),
            1, // account
            2000, // amount
        ));

        // Check balance
        assert_eq!(TokenSystem::get_watt_balance(&1), 2000);

        // Burn WATT tokens
        assert_ok!(TokenSystem::burn_watt_tokens(
            RuntimeOrigin::root(),
            1, // account
            1000, // amount
        ));

        // Check balance after burn
        assert_eq!(TokenSystem::get_watt_balance(&1), 1000);
    });
}

/// Test token transfers
#[test]
fn test_token_transfers() {
    new_test_ext().execute_with(|| {
        // Initialize tokens
        assert_ok!(TokenSystem::initialize_tokens(
            RuntimeOrigin::root(),
            1_000_000,
            1_000_000,
        ));

        // Mint tokens to account 1
        assert_ok!(TokenSystem::mint_grid_tokens(
            RuntimeOrigin::root(),
            1,
            1000,
        ));
        assert_ok!(TokenSystem::mint_watt_tokens(
            RuntimeOrigin::root(),
            1,
            2000,
        ));

        // Transfer GRID tokens
        assert_ok!(TokenSystem::transfer_grid_tokens(
            RuntimeOrigin::signed(1),
            2, // to
            300, // amount
        ));

        // Check balances
        assert_eq!(TokenSystem::get_grid_balance(&1), 700);
        assert_eq!(TokenSystem::get_grid_balance(&2), 300);

        // Transfer WATT tokens
        assert_ok!(TokenSystem::transfer_watt_tokens(
            RuntimeOrigin::signed(1),
            2, // to
            500, // amount
        ));

        // Check balances
        assert_eq!(TokenSystem::get_watt_balance(&1), 1500);
        assert_eq!(TokenSystem::get_watt_balance(&2), 500);

        // Try to transfer more than available
        assert_noop!(
            TokenSystem::transfer_grid_tokens(
                RuntimeOrigin::signed(1),
                2,
                1000,
            ),
            Error::<Test>::InsufficientBalance
        );
    });
}

/// Test staking functionality
#[test]
fn test_staking() {
    new_test_ext().execute_with(|| {
        // Initialize tokens
        assert_ok!(TokenSystem::initialize_tokens(
            RuntimeOrigin::root(),
            1_000_000,
            1_000_000,
        ));

        // Mint GRID tokens for staking
        assert_ok!(TokenSystem::mint_grid_tokens(
            RuntimeOrigin::root(),
            1,
            5000,
        ));

        // Stake tokens
        assert_ok!(TokenSystem::stake_tokens(
            RuntimeOrigin::signed(1),
            2000, // amount
        ));

        // Check stake info
        let stake_info = TokenSystem::get_stake_info(&1).unwrap();
        assert_eq!(stake_info.amount, 2000);
        assert_eq!(stake_info.start_block, 1);

        // Check balance after staking
        assert_eq!(TokenSystem::get_grid_balance(&1), 3000);

        // Check total staked
        assert_eq!(TokenSystem::total_staked(), 2000);

        // Stake more tokens
        assert_ok!(TokenSystem::stake_tokens(
            RuntimeOrigin::signed(1),
            1000,
        ));

        // Check updated stake
        let stake_info = TokenSystem::get_stake_info(&1).unwrap();
        assert_eq!(stake_info.amount, 3000);

        // Unstake some tokens
        assert_ok!(TokenSystem::unstake_tokens(
            RuntimeOrigin::signed(1),
            1000,
        ));

        // Check updated stake
        let stake_info = TokenSystem::get_stake_info(&1).unwrap();
        assert_eq!(stake_info.amount, 2000);

        // Check balance after unstaking
        assert_eq!(TokenSystem::get_grid_balance(&1), 2000);

        // Try to stake below minimum
        assert_noop!(
            TokenSystem::stake_tokens(
                RuntimeOrigin::signed(2),
                500, // below minimum
            ),
            Error::<Test>::MinimumStakeNotMet
        );
    });
}

/// Test staking rewards
#[test]
fn test_staking_rewards() {
    new_test_ext().execute_with(|| {
        // Initialize tokens
        assert_ok!(TokenSystem::initialize_tokens(
            RuntimeOrigin::root(),
            1_000_000,
            1_000_000,
        ));

        // Mint GRID tokens for staking
        assert_ok!(TokenSystem::mint_grid_tokens(
            RuntimeOrigin::root(),
            1,
            5000,
        ));

        // Stake tokens
        assert_ok!(TokenSystem::stake_tokens(
            RuntimeOrigin::signed(1),
            2000,
        ));

        // Fast forward some blocks to accumulate rewards
        run_to_block(1000);

        // Claim rewards
        assert_ok!(TokenSystem::claim_rewards(
            RuntimeOrigin::signed(1),
        ));

        // Check that balance increased (rewards were added)
        assert!(TokenSystem::get_grid_balance(&1) > 3000);

        // Check that unclaimed rewards are reset
        let stake_info = TokenSystem::get_stake_info(&1).unwrap();
        assert_eq!(stake_info.unclaimed_rewards, 0);
    });
}

/// Test governance proposal creation
#[test]
fn test_governance_proposals() {
    new_test_ext().execute_with(|| {
        // Initialize tokens
        assert_ok!(TokenSystem::initialize_tokens(
            RuntimeOrigin::root(),
            1_000_000,
            1_000_000,
        ));

        // Mint and stake tokens for governance
        assert_ok!(TokenSystem::mint_grid_tokens(
            RuntimeOrigin::root(),
            1,
            5000,
        ));
        assert_ok!(TokenSystem::stake_tokens(
            RuntimeOrigin::signed(1),
            2000,
        ));

        // Create a proposal
        assert_ok!(TokenSystem::create_proposal(
            RuntimeOrigin::signed(1),
            b"Test Proposal".to_vec(),
            b"This is a test proposal".to_vec(),
            100, // voting period
        ));

        // Check proposal was created
        let proposal = TokenSystem::get_proposal(0).unwrap();
        assert_eq!(proposal.proposer, 1);
        assert_eq!(proposal.title, b"Test Proposal".to_vec());
        assert_eq!(proposal.status, ProposalStatus::Active);

        // Try to create proposal without staking
        assert_noop!(
            TokenSystem::create_proposal(
                RuntimeOrigin::signed(2),
                b"Invalid Proposal".to_vec(),
                b"Should fail".to_vec(),
                100,
            ),
            Error::<Test>::NotStaking
        );
    });
}

/// Test governance voting
#[test]
fn test_governance_voting() {
    new_test_ext().execute_with(|| {
        // Initialize tokens
        assert_ok!(TokenSystem::initialize_tokens(
            RuntimeOrigin::root(),
            1_000_000,
            1_000_000,
        ));

        // Setup two stakers
        assert_ok!(TokenSystem::mint_grid_tokens(RuntimeOrigin::root(), 1, 5000));
        assert_ok!(TokenSystem::mint_grid_tokens(RuntimeOrigin::root(), 2, 5000));
        
        assert_ok!(TokenSystem::stake_tokens(RuntimeOrigin::signed(1), 2000));
        assert_ok!(TokenSystem::stake_tokens(RuntimeOrigin::signed(2), 3000));

        // Create a proposal
        assert_ok!(TokenSystem::create_proposal(
            RuntimeOrigin::signed(1),
            b"Test Proposal".to_vec(),
            b"This is a test proposal".to_vec(),
            100,
        ));

        // Vote on the proposal
        assert_ok!(TokenSystem::vote_on_proposal(
            RuntimeOrigin::signed(2),
            0, // proposal_id
            true, // vote for
        ));

        // Check proposal votes
        let proposal = TokenSystem::get_proposal(0).unwrap();
        assert_eq!(proposal.votes_for, 3000);
        assert_eq!(proposal.votes_against, 0);

        // Try to vote again (should fail)
        assert_noop!(
            TokenSystem::vote_on_proposal(
                RuntimeOrigin::signed(2),
                0,
                false,
            ),
            Error::<Test>::AlreadyVoted
        );

        // Try to vote on own proposal (should fail)
        assert_noop!(
            TokenSystem::vote_on_proposal(
                RuntimeOrigin::signed(1),
                0,
                true,
            ),
            Error::<Test>::CannotVoteOnOwnProposal
        );
    });
}

/// Test proposal finalization
#[test]
fn test_proposal_finalization() {
    new_test_ext().execute_with(|| {
        // Initialize tokens
        assert_ok!(TokenSystem::initialize_tokens(
            RuntimeOrigin::root(),
            1_000_000,
            1_000_000,
        ));

        // Setup stakers
        assert_ok!(TokenSystem::mint_grid_tokens(RuntimeOrigin::root(), 1, 5000));
        assert_ok!(TokenSystem::mint_grid_tokens(RuntimeOrigin::root(), 2, 5000));
        assert_ok!(TokenSystem::mint_grid_tokens(RuntimeOrigin::root(), 3, 5000));
        
        assert_ok!(TokenSystem::stake_tokens(RuntimeOrigin::signed(1), 2000));
        assert_ok!(TokenSystem::stake_tokens(RuntimeOrigin::signed(2), 3000));
        assert_ok!(TokenSystem::stake_tokens(RuntimeOrigin::signed(3), 1000));

        // Create proposal with 100 block voting period
        assert_ok!(TokenSystem::create_proposal(
            RuntimeOrigin::signed(1),
            b"Test Proposal".to_vec(),
            b"This is a test proposal".to_vec(),
            100,
        ));

        // Vote on the proposal
        assert_ok!(TokenSystem::vote_on_proposal(RuntimeOrigin::signed(2), 0, true));
        assert_ok!(TokenSystem::vote_on_proposal(RuntimeOrigin::signed(3), 0, false));

        // Try to finalize before voting period ends (should fail)
        assert_noop!(
            TokenSystem::finalize_proposal(RuntimeOrigin::signed(1), 0),
            Error::<Test>::VotingPeriodEnded
        );

        // Fast forward past voting deadline
        run_to_block(102);

        // Finalize the proposal
        assert_ok!(TokenSystem::finalize_proposal(RuntimeOrigin::signed(1), 0));

        // Check proposal status
        let proposal = TokenSystem::get_proposal(0).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Passed); // 3000 for > 1000 against
    });
}

/// Test WATT price updates
#[test]
fn test_watt_price_updates() {
    new_test_ext().execute_with(|| {
        // Initialize tokens
        assert_ok!(TokenSystem::initialize_tokens(
            RuntimeOrigin::root(),
            1_000_000,
            1_000_000,
        ));

        // Update WATT price
        assert_ok!(TokenSystem::update_watt_price(
            RuntimeOrigin::root(),
            9500, // New price (5% decrease)
        ));

        // Check updated price
        let watt_info = TokenSystem::get_token_info(TokenType::Watt).unwrap();
        assert_eq!(watt_info.price, 9500);

        // Check event
        System::assert_has_event(
            Event::WattPriceUpdated { new_price: 9500 }.into()
        );
    });
}

/// Test error conditions
#[test]
fn test_error_conditions() {
    new_test_ext().execute_with(|| {
        // Initialize tokens
        assert_ok!(TokenSystem::initialize_tokens(
            RuntimeOrigin::root(),
            1_000_000,
            1_000_000,
        ));

        // Try to unstake without staking
        assert_noop!(
            TokenSystem::unstake_tokens(RuntimeOrigin::signed(1), 1000),
            Error::<Test>::NotStaking
        );

        // Try to claim rewards without staking
        assert_noop!(
            TokenSystem::claim_rewards(RuntimeOrigin::signed(1)),
            Error::<Test>::NotStaking
        );

        // Try to vote on non-existent proposal
        assert_noop!(
            TokenSystem::vote_on_proposal(RuntimeOrigin::signed(1), 999, true),
            Error::<Test>::ProposalNotFound
        );

        // Try to finalize non-existent proposal
        assert_noop!(
            TokenSystem::finalize_proposal(RuntimeOrigin::signed(1), 999),
            Error::<Test>::ProposalNotFound
        );
    });
}

/// Test comprehensive token system workflow
#[test]
fn test_comprehensive_workflow() {
    new_test_ext().execute_with(|| {
        // Initialize the token system
        assert_ok!(TokenSystem::initialize_tokens(
            RuntimeOrigin::root(),
            1_000_000,
            1_000_000,
        ));

        // Mint initial tokens to users
        assert_ok!(TokenSystem::mint_grid_tokens(RuntimeOrigin::root(), 1, 10000));
        assert_ok!(TokenSystem::mint_grid_tokens(RuntimeOrigin::root(), 2, 10000));
        assert_ok!(TokenSystem::mint_watt_tokens(RuntimeOrigin::root(), 1, 5000));
        assert_ok!(TokenSystem::mint_watt_tokens(RuntimeOrigin::root(), 2, 5000));

        // Users stake tokens for governance
        assert_ok!(TokenSystem::stake_tokens(RuntimeOrigin::signed(1), 5000));
        assert_ok!(TokenSystem::stake_tokens(RuntimeOrigin::signed(2), 7000));

        // Create governance proposal
        assert_ok!(TokenSystem::create_proposal(
            RuntimeOrigin::signed(1),
            b"Increase staking rewards".to_vec(),
            b"Proposal to increase staking rewards from 8% to 10%".to_vec(),
            50,
        ));

        // Vote on proposal
        assert_ok!(TokenSystem::vote_on_proposal(RuntimeOrigin::signed(2), 0, true));

        // Transfer tokens between users
        assert_ok!(TokenSystem::transfer_grid_tokens(RuntimeOrigin::signed(1), 2, 1000));
        assert_ok!(TokenSystem::transfer_watt_tokens(RuntimeOrigin::signed(2), 1, 2000));

        // Fast forward and finalize proposal
        run_to_block(52);
        assert_ok!(TokenSystem::finalize_proposal(RuntimeOrigin::signed(1), 0));

        // Check final proposal status
        let proposal = TokenSystem::get_proposal(0).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Passed);

        // Claim staking rewards
        assert_ok!(TokenSystem::claim_rewards(RuntimeOrigin::signed(1)));
        assert_ok!(TokenSystem::claim_rewards(RuntimeOrigin::signed(2)));

        // Check that balances are correct after all operations
        assert_eq!(TokenSystem::get_grid_balance(&1), 4000); // 5000 - 5000 (staked) + 1000 (rewards) - 1000 (transferred) + rewards
        assert_eq!(TokenSystem::get_grid_balance(&2), 3000); // 3000 - 7000 (staked) + 1000 (received) + rewards
        assert_eq!(TokenSystem::get_watt_balance(&1), 7000); // 5000 + 2000 (received)
        assert_eq!(TokenSystem::get_watt_balance(&2), 3000); // 5000 - 2000 (transferred)
    });
}

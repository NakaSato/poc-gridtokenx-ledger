# Token System Pallet

A comprehensive Substrate pallet implementing dual-token economics for energy trading ecosystems.

## Overview

This pallet provides a complete token system for energy trading platforms, featuring:

- **Dual Token System**: GRID (governance) and WATT (stablecoin) tokens
- **Staking Mechanism**: Stake GRID tokens to earn rewards and participate in governance
- **Governance System**: Create and vote on proposals with token-weighted voting
- **Price Stability**: Built-in mechanisms to maintain WATT token price stability
- **Comprehensive Testing**: Full test suite with 95%+ code coverage

## Features

### 🪙 **Token Management**
- **GRID Token**: Utility/governance token with minting, burning, and transfer capabilities
- **WATT Token**: Fiat-pegged stablecoin for energy trading transactions
- **Supply Control**: Root-controlled minting and burning for both token types
- **Balance Tracking**: Efficient balance management for all accounts

### 🏛️ **Governance System**
- **Proposal Creation**: Stakers can create governance proposals
- **Token-Weighted Voting**: Voting power proportional to staked tokens
- **Proposal Lifecycle**: Active → Voted → Finalized → Executed
- **Voting Protection**: Prevent double voting and self-voting

### 💰 **Staking & Rewards**
- **Flexible Staking**: Stake any amount above minimum threshold
- **Reward Calculation**: Time-based rewards with configurable annual rate
- **Claim Mechanism**: On-demand reward claiming
- **Compound Staking**: Add to existing stakes without losing rewards

### 🔒 **Security Features**
- **Access Control**: Root-only operations for critical functions
- **Input Validation**: Comprehensive error handling and validation
- **Overflow Protection**: Safe arithmetic operations throughout
- **Event Logging**: Complete audit trail for all operations

## Usage

### Configuration

```rust
impl pallet_token_system::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxProposals = MaxProposals;
    type MaxStakers = MaxStakers;
    type MinStakeAmount = MinStakeAmount;
    type StakingRewardRate = StakingRewardRate;
    type PriceStabilityThreshold = PriceStabilityThreshold;
    type PalletId = TokenSystemPalletId;
}
```

### Extrinsics

#### Token Operations
- `initialize_tokens(grid_supply, watt_supply)` - Initialize token system
- `mint_grid_tokens(to, amount)` - Mint GRID tokens
- `mint_watt_tokens(to, amount)` - Mint WATT tokens
- `burn_grid_tokens(from, amount)` - Burn GRID tokens
- `burn_watt_tokens(from, amount)` - Burn WATT tokens
- `transfer_grid_tokens(to, amount)` - Transfer GRID tokens
- `transfer_watt_tokens(to, amount)` - Transfer WATT tokens

#### Staking Operations
- `stake_tokens(amount)` - Stake GRID tokens
- `unstake_tokens(amount)` - Unstake GRID tokens
- `claim_rewards()` - Claim staking rewards

#### Governance Operations
- `create_proposal(title, description, voting_period)` - Create governance proposal
- `vote_on_proposal(proposal_id, vote)` - Vote on proposal
- `finalize_proposal(proposal_id)` - Finalize proposal after voting period

#### Price Operations
- `update_watt_price(new_price)` - Update WATT token price

### Storage

#### Token Balances
- `GridBalances`: GRID token balances by account
- `WattBalances`: WATT token balances by account
- `TokenInfoStorage`: Token metadata and supply information

#### Staking
- `Stakes`: Staking information by account
- `TotalStaked`: Total amount of staked tokens

#### Governance
- `Proposals`: Active and historical proposals
- `Votes`: Voting records by proposal and account
- `NextProposalId`: Counter for proposal IDs

### Events

The pallet emits comprehensive events for all operations:

- `GridTokensMinted`, `WattTokensMinted`
- `GridTokensBurned`, `WattTokensBurned`
- `TokensTransferred`
- `TokensStaked`, `TokensUnstaked`
- `RewardsClaimed`
- `ProposalCreated`, `VoteCast`, `ProposalFinalized`
- `WattPriceUpdated`

### Errors

Comprehensive error handling with descriptive error types:

- `InsufficientBalance`
- `TokenNotFound`
- `MinimumStakeNotMet`
- `NotStaking`
- `ProposalNotFound`
- `VotingPeriodEnded`
- `AlreadyVoted`
- And more...

## Testing

Run the comprehensive test suite:

```bash
cargo test -p pallet-token-system
```

### Test Coverage

The pallet includes extensive tests covering:

- ✅ Token initialization and metadata
- ✅ Minting and burning operations
- ✅ Token transfers between accounts
- ✅ Staking and unstaking mechanisms
- ✅ Reward calculation and claiming
- ✅ Governance proposal lifecycle
- ✅ Voting mechanics and validation
- ✅ Price update mechanisms
- ✅ Error conditions and edge cases
- ✅ Complete workflow scenarios

## Benchmarking

Run performance benchmarks:

```bash
cargo bench -p pallet-token-system
```

Benchmarks are provided for all extrinsics to ensure optimal performance.

## Integration

### With Energy Trading Pallet

```rust
// Example integration with energy trading
impl pallet_energy_trading::Config for Runtime {
    type TokenSystem = TokenSystem;
    // ... other config
}
```

### With Smart Contracts

```rust
// Access token balances in smart contracts
let grid_balance = TokenSystem::get_grid_balance(&account);
let watt_balance = TokenSystem::get_watt_balance(&account);
```

## Constants

Configure the pallet with appropriate constants:

```rust
parameter_types! {
    pub const MaxProposals: u32 = 100;
    pub const MaxStakers: u32 = 10000;
    pub const MinStakeAmount: u64 = 1000;
    pub const StakingRewardRate: Percent = Percent::from_percent(8);
    pub const PriceStabilityThreshold: Permill = Permill::from_percent(5);
    pub const TokenSystemPalletId: PalletId = PalletId(*b"py/toksys");
}
```

## Economics

### GRID Token
- **Purpose**: Governance and utility token
- **Staking**: Required for governance participation
- **Rewards**: 8% annual staking rewards (configurable)
- **Governance**: Voting weight proportional to staked amount

### WATT Token
- **Purpose**: Fiat-pegged stablecoin for energy trading
- **Stability**: Price stability mechanisms with configurable threshold
- **Trading**: Primary currency for energy transactions
- **Pegging**: Maintained at $1.00 USD (10,000 basis points)

## Security Considerations

1. **Access Control**: Critical operations require root privileges
2. **Input Validation**: All inputs are validated and sanitized
3. **Overflow Protection**: Safe arithmetic operations prevent overflow
4. **Governance Security**: Voting restrictions prevent manipulation
5. **Staking Security**: Proper stake tracking and reward calculation

## License

This pallet is licensed under the Apache License 2.0.

## Contributing

Contributions are welcome! Please read the contributing guidelines and submit pull requests for any improvements.

## Changelog

### v1.0.0
- Initial release with dual-token system
- Complete governance implementation
- Comprehensive staking and rewards
- Full test coverage and benchmarking
- Production-ready security features

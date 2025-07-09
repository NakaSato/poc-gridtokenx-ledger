# Token System Pallet

ระบบโทเค็นที่สมบูรณ์สำหรับระบบนิเวศการซื้อขายพลังงานแบบกระจายอำนาจ

## คุณสมบัติหลัก

### 🪙 ระบบดูอัลโทเค็น
- **GRID Token**: โทเค็นอรรถประโยชน์/ธรรมาภิบาลพร้อมความสามารถ staking
- **WATT Token**: โทเค็นเสถียรภาพที่ผูกกับเงินสำหรับการซื้อขายพลังงาน

### 🏛️ ระบบธรรมาภิบาล
- การสร้างข้อเสนอโดยผู้ถือ GRID tokens
- การลงคะแนนเสียงด้วยพลังการลงคะแนนจาก staked tokens
- กระบวนการขั้นสุดท้ายข้อเสนออัตโนมัติ

### 💰 ระบบ Staking
- Stake GRID tokens เพื่อรับรางวัล
- อัตราผลตอบแทนรายปี 8%
- การคำนวณรางวัลตามบล็อก

### 📊 การจัดการราคา
- การอัปเดตราคาโทเค็นแบบเรียลไทม์
- กลไกเสถียรภาพราคา

## ประเภทโทเค็น

### GRID Token
- **วัตถุประสงค์**: โทเค็นการใช้งานและธรรมาภิบาล
- **คุณสมบัติ**:
  - สามารถ stake เพื่อรับรางวัล
  - ใช้สำหรับการลงคะแนนเสียงในธรรมาภิบาล
  - จำเป็นสำหรับการเข้าถึงแพลตฟอร์ม
- **อุปทานเริ่มต้น**: 1,000,000,000 GRID

### WATT Token
- **วัตถุประสงค์**: โทเค็นเสถียรภาพสำหรับการซื้อขายพลังงาน
- **คุณสมบัติ**:
  - ผูกกับดอลลาร์สหรัฐ (1 WATT = $1)
  - ใช้สำหรับการชำระเงินซื้อขายพลังงาน
  - กลไก mint/burn สำหรับรักษาเสถียรภาพ
- **อุปทานเริ่มต้น**: 1,000,000,000 WATT

## API หลัก

### การจัดการโทเค็น
```rust
// Mint โทเค็น
system.mint_grid(&account, amount)?;
system.mint_watt(&account, amount)?;

// Burn โทเค็น
system.burn_grid(&account, amount)?;
system.burn_watt(&account, amount)?;

// Transfer โทเค็น
system.transfer_grid(&from, &to, amount)?;
system.transfer_watt(&from, &to, amount)?;

// ตรวจสอบยอดคงเหลือ
let grid_balance = system.grid_balance(&account);
let watt_balance = system.watt_balance(&account);
```

### Staking
```rust
// Stake GRID tokens
system.stake(&account, amount)?;

// Unstake GRID tokens
system.unstake(&account, amount)?;

// คำนวณรางวัล
let rewards = system.calculate_rewards(&account);

// เรียกรางวัล
system.claim_rewards(&account)?;

// ดูข้อมูล staking
let stake_info = system.get_stake_info(&account);
```

### ธรรมาภิบาล
```rust
// สร้างข้อเสนอ
let proposal_id = system.create_proposal(
    &proposer,
    "ชื่อข้อเสนอ".to_string(),
    "รายละเอียดข้อเสนอ".to_string()
)?;

// ลงคะแนนเสียง
system.vote(proposal_id, &voter, true)?; // true = สนับสนุน, false = คัดค้าน

// สรุปผลข้อเสนอ
system.finalize_proposal(proposal_id)?;

// ดูข้อมูลข้อเสนอ
let proposal = system.get_proposal(proposal_id);
```

### การจัดการราคา
```rust
// อัปเดตราคาโทเค็น
system.update_token_price(TokenType::Grid, new_price)?;
system.update_token_price(TokenType::Watt, new_price)?;

// ดูข้อมูลโทเค็น
let token_info = system.get_token_info(&TokenType::Grid);
```

## Event System

ระบบ Event สำหรับติดตามกิจกรรมต่างๆ:

```rust
pub enum Event {
    // การ mint/burn โทเค็น
    GridMinted { to: AccountId, amount: Balance },
    WattMinted { to: AccountId, amount: Balance },
    GridBurned { from: AccountId, amount: Balance },
    WattBurned { from: AccountId, amount: Balance },
    
    // การ transfer โทเค็น
    GridTransferred { from: AccountId, to: AccountId, amount: Balance },
    WattTransferred { from: AccountId, to: AccountId, amount: Balance },
    
    // Staking
    Staked { who: AccountId, amount: Balance },
    Unstaked { who: AccountId, amount: Balance },
    RewardsClaimed { who: AccountId, amount: Balance },
    
    // ธรรมาภิบาล
    ProposalCreated { id: ProposalId, proposer: AccountId, title: String },
    VoteCast { proposal_id: ProposalId, voter: AccountId, support: bool, power: Balance },
    ProposalFinalized { id: ProposalId, status: ProposalStatus },
    
    // การจัดการราคา
    PriceUpdated { token_type: TokenType, new_price: u32 },
}
```

## การกำหนดค่า

```rust
pub struct TokenSystemConfig {
    /// จำนวน stake ขั้นต่ำที่ต้องการ
    pub min_stake_amount: Balance, // ค่าเริ่มต้น: 1,000 GRID
    
    /// อัตราผลตอบแทน staking ต่อปี (ในหน่วย basis points)
    pub staking_reward_rate: u32, // ค่าเริ่มต้น: 800 (8%)
    
    /// ระยะเวลาการลงคะแนนเสียง (ในบล็อก)
    pub voting_period: BlockNumber, // ค่าเริ่มต้น: 100 บล็อก
    
    /// ยอด GRID ขั้นต่ำสำหรับการสร้างข้อเสนอ
    pub min_proposal_balance: Balance, // ค่าเริ่มต้น: 10,000 GRID
    
    /// อุปทานเริ่มต้นของ GRID token
    pub grid_initial_supply: Balance, // ค่าเริ่มต้น: 1,000,000,000 GRID
    
    /// อุปทานเริ่มต้นของ WATT token
    pub watt_initial_supply: Balance, // ค่าเริ่มต้น: 1,000,000,000 WATT
}
```

## การจัดการข้อผิดพลาด

```rust
pub enum TokenError {
    InsufficientBalance,        // ยอดเงินไม่เพียงพอ
    TokenNotFound,             // ไม่พบโทเค็น
    ProposalNotFound,          // ไม่พบข้อเสนอ
    ProposalNotActive,         // ข้อเสนอไม่ได้ใช้งานอยู่
    AlreadyVoted,              // ลงคะแนนเสียงแล้ว
    VotingPeriodEnded,         // ระยะเวลาการลงคะแนนสิ้นสุด
    CannotVoteOnOwnProposal,   // ไม่สามารถลงคะแนนข้อเสนอของตนเอง
    NotStaking,                // ไม่ได้ stake
    MinimumStakeNotMet,        // จำนวน stake ไม่ถึงขั้นต่ำ
    InvalidTokenType,          // ประเภทโทเค็นไม่ถูกต้อง
    NoRewardsToClaim,          // ไม่มีรางวัลให้เรียก
    Unauthorized,              // ไม่มีสิทธิ์
}
```

## การใช้งาน

### 1. การติดตั้ง
```toml
[dependencies]
pallet-token-system = { path = "pallets/token-system" }
```

### 2. การเริ่มต้นระบบ
```rust
use pallet_token_system::{TokenSystem, TokenSystemConfig};

// สร้างระบบด้วยค่าเริ่มต้น
let mut token_system = TokenSystem::default();

// หรือสร้างด้วยการกำหนดค่าเอง
let config = TokenSystemConfig {
    min_stake_amount: 5000,
    staking_reward_rate: 1000, // 10%
    voting_period: 200,
    min_proposal_balance: 20000,
    grid_initial_supply: 2_000_000_000,
    watt_initial_supply: 2_000_000_000,
};
let mut token_system = TokenSystem::new(config);
```

### 3. ตัวอย่างการใช้งาน
```rust
// ออกโทเค็นให้ผู้ใช้
token_system.mint_grid(&"alice".to_string(), 100000)?;
token_system.mint_watt(&"alice".to_string(), 50000)?;

// Stake โทเค็น
token_system.stake(&"alice".to_string(), 20000)?;

// สร้างข้อเสนอ
let proposal_id = token_system.create_proposal(
    &"alice".to_string(),
    "เพิ่มรางวัล staking".to_string(),
    "ข้อเสนอเพิ่มรางวัล staking เป็น 10%".to_string()
)?;

// เลื่อนเวลา
token_system.set_block(50);

// เรียกรางวัล
token_system.claim_rewards(&"alice".to_string())?;

// ดู Events
for event in token_system.get_events() {
    println!("{:?}", event);
}
```

## การทดสอบ

รันการทดสอบ:
```bash
cargo test --package pallet-token-system
```

การทดสอบครอบคลุม:
- ✅ การ mint/burn/transfer โทเค็น
- ✅ ระบบ staking และรางวัล
- ✅ การสร้างและลงคะแนนข้อเสนอ
- ✅ การสรุปผลข้อเสนอ
- ✅ การอัปเดตราคาโทเค็น
- ✅ การจัดการข้อผิดพลาด
- ✅ เวิร์กโฟลว์แบบครบวงจร

## ความปลอดภัย

- ✅ การตรวจสอบยอดคงเหลือก่อนทำธุรกรรม
- ✅ การป้องกันการลงคะแนนซ้ำ
- ✅ การป้องกันการลงคะแนนข้อเสนอของตนเอง
- ✅ การตรวจสอบสิทธิ์ในการสร้างข้อเสนอ
- ✅ การจัดการ overflow/underflow ที่ปลอดภัย

## การผสานรวม

Pallet นี้ถูกออกแบบให้ทำงานร่วมกับส่วนประกอบอื่นๆ ในระบบนิเวศการซื้อขายพลังงาน:

- **Blockchain Module**: สำหรับการจัดเก็บธุรกรรม
- **Energy Trading Module**: สำหรับการซื้อขายพลังงาน P2P
- **Smart Contracts**: สำหรับการจัดการสัญญาอัตโนมัติ

## อนาคต

- 🔄 การเชื่อมต่อกับ Substrate Framework
- 🌉 การสร้างสะพานข้าม chain
- 📱 การพัฒนา mobile wallet
- 🔐 การตรวจสอบความปลอดภัยเพิ่มเติม

## ใบอนุญาต

Apache 2.0

// ตัวอย่างการใช้งาน Token System Pallet
use pallet_token_system::{TokenSystem, TokenSystemConfig, TokenType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 ตัวอย่างการใช้งาน Token System Pallet สำหรับการซื้อขายพลังงาน");
    println!("================================================================");

    // 1. สร้างระบบโทเค็นใหม่
    let config = TokenSystemConfig {
        min_stake_amount: 5000,
        staking_reward_rate: 1000, // 10% ต่อปี
        voting_period: 150,
        min_proposal_balance: 15000,
        grid_initial_supply: 2_000_000_000,
        watt_initial_supply: 2_000_000_000,
    };
    
    let mut token_system = TokenSystem::new(config);
    
    println!("\n✅ สร้างระบบโทเค็นสำเร็จ!");
    println!("   - GRID Token อุปทานเริ่มต้น: {} tokens", token_system.total_supply(&TokenType::Grid));
    println!("   - WATT Token อุปทานเริ่มต้น: {} tokens", token_system.total_supply(&TokenType::Watt));

    // 2. ออกโทเค็นให้ผู้ใช้ (สำหรับการทดสอบ)
    println!("\n🪙 การออกโทเค็นให้ผู้ใช้");
    
    // Alice - โซลาร์เซลล์เจ้าของบ้าน
    token_system.mint_grid(&"alice".to_string(), 100000)?;
    token_system.mint_watt(&"alice".to_string(), 50000)?;
    
    // Bob - เจ้าของฟาร์มลม
    token_system.mint_grid(&"bob".to_string(), 75000)?;
    token_system.mint_watt(&"bob".to_string(), 30000)?;
    
    // Charlie - ผู้บริโภคพลังงาน
    token_system.mint_grid(&"charlie".to_string(), 25000)?;
    token_system.mint_watt(&"charlie".to_string(), 20000)?;
    
    println!("   Alice: {} GRID, {} WATT", 
             token_system.grid_balance(&"alice".to_string()),
             token_system.watt_balance(&"alice".to_string()));
    println!("   Bob: {} GRID, {} WATT", 
             token_system.grid_balance(&"bob".to_string()),
             token_system.watt_balance(&"bob".to_string()));
    println!("   Charlie: {} GRID, {} WATT", 
             token_system.grid_balance(&"charlie".to_string()),
             token_system.watt_balance(&"charlie".to_string()));

    // 3. Staking เพื่อเข้าร่วมธรรมาภิบาล
    println!("\n💰 การ Stake โทเค็น GRID");
    
    token_system.stake(&"alice".to_string(), 30000)?;
    token_system.stake(&"bob".to_string(), 20000)?;
    token_system.stake(&"charlie".to_string(), 10000)?;
    
    println!("   Alice stake: {} GRID", 
             token_system.get_stake_info(&"alice".to_string()).unwrap().amount);
    println!("   Bob stake: {} GRID", 
             token_system.get_stake_info(&"bob".to_string()).unwrap().amount);
    println!("   Charlie stake: {} GRID", 
             token_system.get_stake_info(&"charlie".to_string()).unwrap().amount);
    println!("   รวม staked: {} GRID", token_system.total_staked());

    // 4. การซื้อขายพลังงาน (ตัวอย่าง)
    println!("\n⚡ การซื้อขายพลังงานด้วย WATT tokens");
    
    // Alice ขายพลังงานจากโซลาร์เซลล์ให้ Charlie (10 kWh = 10,000 WATT)
    let energy_cost = 10000; // 10 kWh ที่ 1 WATT/kWh
    token_system.transfer_watt(&"charlie".to_string(), &"alice".to_string(), energy_cost)?;
    
    println!("   Charlie ซื้อพลังงาน 10 kWh จาก Alice ด้วย {} WATT", energy_cost);
    println!("   Alice ยอดคงเหลือ WATT: {}", token_system.watt_balance(&"alice".to_string()));
    println!("   Charlie ยอดคงเหลือ WATT: {}", token_system.watt_balance(&"charlie".to_string()));

    // 5. สร้างข้อเสนอธรรมาภิบาล
    println!("\n🏛️ การสร้างข้อเสนอธรรมาภิบาล");
    
    let proposal_id = token_system.create_proposal(
        &"alice".to_string(),
        "เพิ่มรางวัล Staking เป็น 12%".to_string(),
        "เพื่อส่งเสริมให้ผู้ใช้เข้าร่วม staking มากขึ้น เสนอให้เพิ่มอัตราดอกเบี้ยจาก 10% เป็น 12% ต่อปี".to_string()
    )?;
    
    println!("   สร้างข้อเสนอ ID: {}", proposal_id);
    let proposal = token_system.get_proposal(proposal_id).unwrap();
    println!("   หัวข้อ: {}", proposal.title);
    println!("   ผู้เสนอ: {}", proposal.proposer);
    println!("   สถานะ: {:?}", proposal.status);

    // 6. การลงคะแนนเสียง
    println!("\n🗳️ การลงคะแนนเสียง");
    
    // Bob และ Charlie ลงคะแนนสนับสนุน
    token_system.vote(proposal_id, &"bob".to_string(), true)?;
    token_system.vote(proposal_id, &"charlie".to_string(), true)?;
    
    let proposal = token_system.get_proposal(proposal_id).unwrap();
    println!("   คะแนนสนับสนุน: {} votes", proposal.votes_for);
    println!("   คะแนนคัดค้าน: {} votes", proposal.votes_against);

    // 7. เลื่อนเวลาและเรียกรางวัล
    println!("\n⏰ เลื่อนเวลา 50 บล็อก และเรียกรางวัล Staking");
    
    token_system.set_block(50);
    
    // คำนวณและเรียกรางวัลสำหรับ Alice
    let alice_rewards = token_system.calculate_rewards(&"alice".to_string());
    if alice_rewards > 0 {
        token_system.claim_rewards(&"alice".to_string())?;
        println!("   Alice ได้รับรางวัล: {} GRID", alice_rewards);
        println!("   Alice ยอดคงเหลือใหม่: {} GRID", token_system.grid_balance(&"alice".to_string()));
    }

    // 8. สรุปผลข้อเสนอ
    println!("\n📋 สรุปผลข้อเสนอ");
    
    token_system.set_block(200); // เลื่อนเวลาผ่านระยะเวลาลงคะแนน
    token_system.finalize_proposal(proposal_id)?;
    
    let proposal = token_system.get_proposal(proposal_id).unwrap();
    println!("   สถานะสุดท้าย: {:?}", proposal.status);

    // 9. อัปเดตราคาโทเค็น (กลไกเสถียรภาพ)
    println!("\n💱 การอัปเดตราคาโทเค็น");
    
    // ราคา WATT ลดลงเล็กน้อยเนื่องจากอุปทานพลังงานเพิ่มขึ้น
    token_system.update_token_price(TokenType::Watt, 9800)?; // ลดจาก 10000 เป็น 9800
    
    let watt_info = token_system.get_token_info(&TokenType::Watt).unwrap();
    println!("   ราคา WATT ใหม่: {} basis points (${:.2})", 
             watt_info.price, watt_info.price as f64 / 10000.0);

    // 10. สรุปสถิติ
    println!("\n📊 สรุปสถิติระบบ");
    println!("   จำนวน Events ทั้งหมด: {}", token_system.get_events().len());
    println!("   GRID tokens ที่ stake: {}", token_system.total_staked());
    println!("   GRID total supply: {}", token_system.total_supply(&TokenType::Grid));
    println!("   WATT total supply: {}", token_system.total_supply(&TokenType::Watt));

    // แสดง Events ล่าสุด
    println!("\n🎯 Events ล่าสุด:");
    for (i, event) in token_system.get_events().iter().rev().take(5).enumerate() {
        println!("   {}. {:?}", i + 1, event);
    }

    println!("\n✨ การทดสอบสำเร็จ! Token System Pallet ทำงานได้อย่างสมบูรณ์");
    println!("🌟 พร้อมสำหรับการใช้งานในระบบการซื้อขายพลังงานจริง!");

    Ok(())
}

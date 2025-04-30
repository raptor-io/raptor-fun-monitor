use {
    serde::{Deserialize, Serialize},
    solana_pubkey::Pubkey,
};

/// PumpfunInstruction enum defines all possible instruction and event types for the contract
#[derive(Debug, Clone)]
pub enum PumpfunInstruction {
    LaunchEvent(LaunchEvent),
    SwapEvent(SwapEvent),
    CompleteEvent(CompleteEvent),
    WithdrawEvent(WithdrawEvent),
    MigrateEvent(MigrateEvent),
    Unknown,
}

/// Token creation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchEvent {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub metadata: Pubkey,

    pub decimals: u8,
    pub token_supply: u64,

    pub reserve_lamport: u64,
    pub reserve_token: u64,
}

/// Swap transaction event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,

    pub amount_in: u64,
    pub direction: u8,
    pub minimum_receive_amount: u64,
    pub amount_out: u64,

    pub reserve_lamport: u64,
    pub reserve_token: u64,
}

/// Bonding curve completion event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
}

/// Withdrawal event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawEvent {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
}

/// Migration event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrateEvent {
    pub admin: Pubkey,
    pub token: Pubkey,
    pub bonding_curve: Pubkey,
    pub token_in: u64,
    pub sol_in: u64,
}

/// Implement Default trait for handling unknown instructions
impl Default for PumpfunInstruction {
    fn default() -> Self {
        PumpfunInstruction::Unknown
    }
} 
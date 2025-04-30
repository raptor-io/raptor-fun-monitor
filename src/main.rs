use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult,
        // Assuming InstructionProcessorInputType is correct now
        instruction::InstructionProcessorInputType,
        metrics::MetricsCollection, processor::Processor,
        // Remove unused datasource
        // datasource::solana::SolanaRpcDatasource, 
    },
    // Import RpcBlockSubscribe related types
    carbon_rpc_block_subscribe_datasource::{RpcBlockSubscribe, Filters as RpcBlockSubscribeFilters},
    // Import private types directly from solana_client
    solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter}, 
    solana_transaction_status::{UiTransactionEncoding, TransactionDetails}, 
    solana_sdk::native_token::LAMPORTS_PER_SOL,
    std::sync::Arc,
};

mod decoder;
mod types;

use decoder::PumpfunDecoder;
use types::PumpfunInstruction;

// Read Program ID from environment variable, with fallback to default
fn get_program_id() -> String {
    std::env::var("PROGRAM_ID").unwrap_or_else(|_| {
        "RapXd6RgnAesGtkbRUfo31PnwYkuqcv3PwqXfo6nRXc".to_string()
    })
}

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    // Load environment variables and initialize logging
    dotenv::dotenv().ok();
    env_logger::init();
    
    // Get WebSocket RPC endpoint from environment variables
    let rpc_ws_url = std::env::var("RPC_WS_URL")
        .expect("RPC_WS_URL environment variable must be set for block subscription");
    
    let program_id = get_program_id();
    
    log::info!("Starting Pumpfun monitoring program...");
    log::info!("Using WebSocket RPC endpoint: {}", rpc_ws_url);
    log::info!("Monitoring Program ID: {}", program_id);
    
    // Configure filters for RpcBlockSubscribe
    // TODO: RpcBlockSubscribeFilter::All is inefficient. Filter mentions later or use a different datasource.
    let block_filter = RpcBlockSubscribeFilter::All;
    let block_subscribe_config = RpcBlockSubscribeConfig {
        // Use CommitmentConfig::confirmed() instead of just CommitmentLevel
        commitment: Some(solana_sdk::commitment_config::CommitmentConfig::confirmed()), 
        encoding: Some(UiTransactionEncoding::Base64), // Ensure we get encoded data
        transaction_details: Some(TransactionDetails::Full), 
        show_rewards: Some(false),
        max_supported_transaction_version: Some(0), // Required for legacy transactions
    };

    let filters = RpcBlockSubscribeFilters::new(
        block_filter,
        Some(block_subscribe_config),
    );

    // Create RpcBlockSubscribe Datasource
    let datasource = RpcBlockSubscribe::new(rpc_ws_url, filters);


    // Build Carbon Pipeline
    log::info!("Building processing pipeline...");
    carbon_core::pipeline::Pipeline::builder()
        .datasource(datasource)
        // Revert to the original instruction method call
        .instruction(PumpfunDecoder, PumpfunInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}

// Implement instruction processor
pub struct PumpfunInstructionProcessor;

#[async_trait]
impl Processor for PumpfunInstructionProcessor {
    // Revert InputType to the original form
    type InputType = InstructionProcessorInputType<PumpfunInstruction>; 

    async fn process(
        &mut self,
        // Revert data parameter to the original form
        data: Self::InputType, 
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        // Extract instruction data as before
        let (metadata, decoded_instruction, _inner_instructions) = data; // Destructure input
        let pumpfun_instruction: PumpfunInstruction = decoded_instruction.data; 

        log::debug!("Processing instruction at slot: {}", metadata.transaction_metadata.slot);

        match pumpfun_instruction {
            PumpfunInstruction::LaunchEvent(launch_event) => {
                log::info!("New token creation event: {:#?}", launch_event);
                log::info!(
                    "Token details - Creator: {}, Token address: {}, Supply: {}, Decimals: {}",
                    launch_event.creator,
                    launch_event.mint,
                    launch_event.token_supply,
                    launch_event.decimals
                );
            }
            PumpfunInstruction::SwapEvent(swap_event) => {
                let swap_threshold = std::env::var("SWAP_THRESHOLD")
                    .map(|val| val.parse::<u64>().unwrap_or(10 * LAMPORTS_PER_SOL))
                    .unwrap_or(10 * LAMPORTS_PER_SOL);
                
                if swap_event.amount_in > swap_threshold {
                    log::info!("Large transaction: {:#?}\nTransaction Signature: {}", swap_event, metadata.transaction_metadata.signature);
                    let direction = if swap_event.direction == 0 { "buy" } else { "sell" };
                    log::info!(
                        "Transaction details - User: {}, Token: {}, Direction: {}, Input: {}, Output: {}",
                        swap_event.user,
                        swap_event.mint,
                        direction,
                        swap_event.amount_in,
                        swap_event.amount_out
                    );
                }
            }
            PumpfunInstruction::CompleteEvent(complete_event) => {
                log::info!("Bonding curve completion event: {:#?}\nTransaction Signature: {}", complete_event, metadata.transaction_metadata.signature);
                log::info!(
                    "Completion details - User: {}, Token: {}, Curve address: {}",
                    complete_event.user,
                    complete_event.mint,
                    complete_event.bonding_curve
                );
            }
            PumpfunInstruction::WithdrawEvent(withdraw_event) => {
                log::info!("Withdrawal event: {:#?}\nTransaction Signature: {}", withdraw_event, metadata.transaction_metadata.signature);
                log::info!(
                    "Withdrawal details - Token: {}, SOL withdrawn: {}, Token withdrawn: {}",
                    withdraw_event.mint,
                    withdraw_event.sol_amount,
                    withdraw_event.token_amount
                );
            }
            _ => {
                // Avoid logging entire instruction for potentially large data
                log::debug!("Unhandled instruction type for signature: {}", metadata.transaction_metadata.signature);
            }
        };

        Ok(())
    }
} 
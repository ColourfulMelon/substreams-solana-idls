use substreams_solana_idls::orca::whirlpool::events::{self, WhirlpoolEvent};
use substreams_solana_idls::orca::whirlpool::instructions;

#[test]
fn whirlpool_unknown() {
    assert!(instructions::unpack(&[0u8; 16]).is_err());
}
#[test]
fn whirlpool_too_short() {
    assert!(instructions::unpack(&[0u8; 4]).is_err());
}
#[test]
fn whirlpool_event_unknown_returns_variant() {
    // orca events return Ok(Unknown)
    assert!(matches!(events::parse_event(&[0u8; 24]).unwrap(), WhirlpoolEvent::Unknown));
}

#[test]
fn current_swap_v2_allows_observed_zero_padding_only() {
    use substreams_solana_idls::orca::whirlpool::instructions::{unpack, SwapV2Instruction, WhirlpoolInstruction, SWAP_V2};
    let args = SwapV2Instruction {
        amount: 21_099,
        other_amount_threshold: 0,
        sqrt_price_limit: 79_226_673_515_401_279_992_447_579_055,
        amount_specified_is_input: true,
        a_to_b: false,
        remaining_accounts_info: None,
    };
    let mut data = SWAP_V2.to_vec();
    data.extend_from_slice(&borsh::to_vec(&args).unwrap());
    data.extend_from_slice(&[0; 7]);
    assert!(matches!(unpack(&data), Ok(WhirlpoolInstruction::SwapV2(_))));
    *data.last_mut().unwrap() = 1;
    assert!(unpack(&data).is_err());
}

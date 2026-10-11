#![cfg(test)]
#![allow(deprecated)]
mod tests {
    use hex_literal::hex;
    use substreams_solana_idls::meteora::dlmm;

    #[test]
    fn unpack_dlmm_swap_event() {
        // https://solscan.io/tx/<unknown? maybe but not necessary>. Maybe not.
        let bytes = hex!("e445a52e51cb9a1d516ce3becdd00ac471739c6f45944a0e56b3a0f9b2399f421b0a82b56c5b87acc78023b8ed27cc7cf74ba51f6820a847117a8fad48c80ae298e20bf37ff35e7ae8c7b85272b5010701feffff01feffff1f660a0000000000702c000000000000016037000000000000c40200000000000000623d010000000000000000000000000000000000000000");
        match dlmm::anchor_cpi_event::unpack(&bytes).expect("decode event") {
            dlmm::anchor_cpi_event::MeteoraDlmmAnchorCpiEvent::Swap(event) => {
                assert_eq!(event.lb_pair.to_string(), "8dsKNwMDMh1Vfr4YevzRY9oDS7N29fRHpPXcoDmUVSBd", "lb_pair");
                assert_eq!(event.from.to_string(), "HeLbvj7KM5fkduCdiufTa5SmVtuuVjySPps7dnp2pDZG", "from");
                assert_eq!(event.start_bin_id, -511, "start_bin_id");
                assert_eq!(event.end_bin_id, -511, "end_bin_id");
                assert_eq!(event.amount_in, 681_503, "amount_in");
                assert_eq!(event.amount_out, 11_376, "amount_out");
                assert!(event.swap_for_y, "swap_for_y");
                assert_eq!(event.fee, 14_176, "fee");
                assert_eq!(event.protocol_fee, 708, "protocol_fee");
                assert_eq!(event.fee_bps, 20_800_000, "fee_bps");
                assert_eq!(event.host_fee, 0, "host_fee");
            }
            _ => panic!("Expected Swap event"),
        }
    }
}

#[test]
fn swap2_decodes_referral_transfer_hook_without_changing_older_variants() {
    use borsh::BorshDeserialize;
    use substreams_solana_idls::meteora::dlmm::instructions::{
        unpack, AccountsType, MeteoraDlmmInstruction, RemainingAccountsInfo, RemainingAccountsSlice, Swap2Instruction, SWAP2,
    };
    let variants = [
        AccountsType::TransferHookX,
        AccountsType::TransferHookY,
        AccountsType::TransferHookReward,
        AccountsType::TransferHookMultiReward(7),
        AccountsType::TransferHookReferral,
    ];
    for (index, value) in variants.iter().enumerate() {
        let bytes = borsh::to_vec(value).unwrap();
        assert_eq!(bytes[0], index as u8);
        assert_eq!(&AccountsType::try_from_slice(&bytes).unwrap(), value);
    }
    let mut data = SWAP2.to_vec();
    data.extend_from_slice(
        &borsh::to_vec(&Swap2Instruction {
            amount_in: 214_849,
            min_amount_out: 0,
            remaining_accounts_info: RemainingAccountsInfo {
                slices: vec![RemainingAccountsSlice {
                    accounts_type: AccountsType::TransferHookReferral,
                    length: 0,
                }],
            },
        })
        .unwrap(),
    );
    assert!(matches!(unpack(&data), Ok(MeteoraDlmmInstruction::Swap2(_))));
}

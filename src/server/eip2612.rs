use alloy::primitives::{Address, FixedBytes, PrimitiveSignature};
use alloy::primitives::{keccak256, B256, U256};

pub fn verify_eip2612_signature(
    permit_message: FixedBytes<32>,
    signature: PrimitiveSignature,
    signer: Address,
) -> bool {
    let recovered_address = signature.recover_address_from_msg(permit_message).unwrap_or(Address::ZERO);

    recovered_address == signer
}

pub fn get_permit_signature_fields(
    signature: PrimitiveSignature,
) -> (u8, [u8; 32], [u8; 32]) {
    let r: FixedBytes<32> = signature.r().into();
    let s: FixedBytes<32> = signature.s().into();

    (signature.v().into(), r.0, s.0)
}

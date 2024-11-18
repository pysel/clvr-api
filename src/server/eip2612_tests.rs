#[cfg(test)]
mod tests {
    use alloy::signers::k256::elliptic_curve::generic_array::GenericArray;
    use k256::SecretKey as K256SecretKey;

    use alloy::hex;
    use alloy::primitives::{Address, FixedBytes}; // Ensure to import necessary types
    use alloy::primitives::{keccak256, U256};
    use alloy::signers::local::PrivateKeySigner;
    use alloy::signers::{k256, Signer};
    use once_cell::sync::Lazy;

    use crate::server::eip2612::verify_eip2612_signature;
    const PRIV_KEY_SIGNER_MOCK: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    const PUB_KEY_SIGNER_MOCK_RAW: [u8; 64] = [164, 59, 102, 209, 234, 238, 3, 240, 125, 100, 146, 4, 145, 248, 179, 72, 122, 144, 245, 39, 242, 52, 44, 140, 172, 205, 85, 213, 6, 80, 132, 73, 108, 87, 212, 9, 214, 219, 6, 250, 239, 216, 160, 170, 17, 6, 172, 214, 149, 1, 19, 78, 17, 207, 116, 178, 233, 92, 129, 180, 81, 218, 52, 51];

    const CONSTANT_LEADING_BYTES: &[u8] = &[0x19, 0x01];
    const DOMAIN_NAME: &str = "CLVR";
    const DOMAIN_VERSION: &str = "1";
    const CHAIN_ID: Lazy<U256> = Lazy::new(|| U256::from(137));
    const VERIFYING_CONTRACT: Lazy<Address> = Lazy::new(|| Address::from_raw_public_key(&PUB_KEY_SIGNER_MOCK_RAW));

    const OTHER: Lazy<Address> = Lazy::new(|| Address::from_raw_public_key(&[0; 64]));
    const SIGNER: Lazy<Address> = Lazy::new(|| Address::from_raw_public_key(&PUB_KEY_SIGNER_MOCK_RAW));
    const VALUE: Lazy<U256> = Lazy::new(|| U256::from(1000000));
    const NONCE: Lazy<U256> = Lazy::new(|| U256::from(0));
    const DEADLINE: Lazy<U256> = Lazy::new(|| U256::from(1715999999));

    fn generate_domain_separator(
        name: &str,
        version: &str,
        chain_id: U256,
        verifying_contract: Address
    ) -> FixedBytes<32> {
        let domain_typehash = keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
        let name_hash = keccak256(name.as_bytes());
        let version_hash = keccak256(version.as_bytes());

        let chain_id_bytes: FixedBytes<32> = chain_id.into();
        
        let combined_bytes: Vec<u8> = [
            domain_typehash.as_slice(),
            name_hash.as_slice(),
            version_hash.as_slice(),
            chain_id_bytes.as_slice(),
            verifying_contract.as_slice(),
        ].concat();
        
        keccak256(&combined_bytes)
    }

    fn generate_permit_body(
        owner: Address,
        spender: Address,
        value: U256,
        nonce: U256,
        deadline: U256
    ) -> FixedBytes<32> {
        let body_typehash = keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");

        let combined_bytes: Vec<u8> = [
            body_typehash.as_slice(),
            owner.as_slice(),
            spender.as_slice(),
            value.to_string().as_bytes(),
            nonce.to_string().as_bytes(),
            deadline.to_string().as_bytes(),
        ].concat();

        keccak256(&combined_bytes)
    }

    fn generate_permit_message(
        domain_separator: FixedBytes<32>,
        permit_body: FixedBytes<32>
    ) -> FixedBytes<32> {
        keccak256(&[CONSTANT_LEADING_BYTES, domain_separator.as_slice(), permit_body.as_slice()].concat())
    }
    
    #[tokio::test]
    async fn test_verify_eip2612_signature() {
        let key_hex = hex::decode(PRIV_KEY_SIGNER_MOCK).unwrap();
        let secret_key = K256SecretKey::from_bytes(&GenericArray::clone_from_slice(&key_hex)).unwrap();
        
        let signer: PrivateKeySigner = secret_key.into();
        let signer: PrivateKeySigner = signer.with_chain_id(Some(137));

        // generate permit message to sign
        let domain_separator = generate_domain_separator(DOMAIN_NAME, DOMAIN_VERSION, *CHAIN_ID, *VERIFYING_CONTRACT);
        let permit_body = generate_permit_body(*OTHER, *SIGNER, *VALUE, *NONCE, *DEADLINE);
        let permit_message = generate_permit_message(domain_separator, permit_body);

        // generate a signature
        let signature = signer.sign_message(permit_message.as_slice()).await.unwrap();
        // println!("permit_message: {:?}", hex::encode(permit_message.as_slice()));
        // println!("signature: {:?}", hex::encode(signature.as_bytes()));

        let result = verify_eip2612_signature(permit_message, signature, signer.address());
        assert!(result, "Signature verification failed");

        let result = verify_eip2612_signature(permit_message, signature, *OTHER);
        assert!(!result, "Signature verification should fail");
    }
}
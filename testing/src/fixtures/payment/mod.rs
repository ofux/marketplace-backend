use crate::fixtures::payment::constants::TRANSACTION_HASHES;
use domain::{ContractAddress, PaymentId, PaymentRequestId, TransactionHash};
use std::str::FromStr;
use uuid::Uuid;

use self::constants::CONTRACT_ADDRESSES;

pub mod constants;
pub mod events;

pub fn payment_id() -> PaymentId {
	Uuid::from_str("abad1756-18ba-42e2-8cbf-83369cecfb38").unwrap().into()
}

pub fn payment_request_id() -> PaymentRequestId {
	Uuid::from_str("b5db0b56-ab3e-4bd1-b9a2-6a3d41f35b8f").unwrap().into()
}

pub fn recipient_address() -> ContractAddress {
	CONTRACT_ADDRESSES[0].parse().unwrap()
}

pub fn transaction_hash() -> TransactionHash {
	TRANSACTION_HASHES[0].parse().unwrap()
}

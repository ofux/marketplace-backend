use crate::e2e_tests::{
	contributors,
	starknet::{accounts::accounts, Account},
};
use rstest::*;
use starknet::core::types::FieldElement;

#[rstest]
#[tokio::test]
async fn contact_information(accounts: [Account; 10]) {
	let contributor_account = format!("{:#x}", accounts[0].address());

	contributors::contact_information::add(&contributor_account, Some(String::from("discord")))
		.await;
	let contact_info = contributors::contact_information::get(&contributor_account).await;
	assert_eq!(
		FieldElement::from_hex_be(&contact_info.contributor_id).unwrap(),
		FieldElement::from_hex_be(&contributor_account).unwrap()
	);
	assert_eq!(contact_info.discord_handle.unwrap(), "discord");
}
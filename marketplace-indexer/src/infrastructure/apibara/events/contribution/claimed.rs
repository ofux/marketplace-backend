use marketplace_domain::{ContributionEvent, ContributorId, Event, HexPrefixedString};
use starknet::core::{types::FieldElement, utils::get_selector_from_name};

use crate::infrastructure::apibara::events::{
	topics::{StarknetTopics, Topics},
	EventTranslator, FromEventError,
};

pub struct Claimed;

impl EventTranslator for Claimed {
	fn selector() -> FieldElement {
		get_selector_from_name("ContributionClaimed").unwrap()
	}

	fn to_domain_event(mut topics: Topics) -> Result<Event, FromEventError> {
		let contribution_id: HexPrefixedString = topics.pop_front_as()?;
		let contributor_id: ContributorId = topics.pop_front_as()?;

		Ok(Event::Contribution(ContributionEvent::Claimed {
			id: contribution_id.into(),
			contributor_id,
		}))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::infrastructure::apibara::proto::TopicValue;
	use rstest::*;

	#[fixture]
	fn apibara_event_data() -> Topics {
		vec![
			TopicValue {
				value: vec![
					0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
					0, 0, 0, 0, 0, 12,
				],
			},
			TopicValue {
				value: vec![
					0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
					0, 0, 0, 0, 0, 24,
				],
			},
			TopicValue {
				value: vec![
					0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
					0, 0, 0, 0, 0, 0,
				],
			},
		]
		.into()
	}

	#[rstest]
	fn selector() {
		assert_eq!(
			get_selector_from_name("ContributionClaimed").unwrap(),
			<Claimed as EventTranslator>::selector()
		);
	}

	#[rstest]
	fn create_event_from_apibara(apibara_event_data: Topics) {
		let result = <Claimed as EventTranslator>::to_domain_event(apibara_event_data);
		assert!(result.is_ok(), "{}", result.err().unwrap());
		assert_eq!(
			Event::Contribution(ContributionEvent::Claimed {
				id: 12.into(),
				contributor_id: ContributorId::from(24)
			},),
			result.unwrap()
		);
	}
}
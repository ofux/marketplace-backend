use super::*;
use crate::{events::Aggregate, HexPrefixedString, ParseHexPrefixedStringError};
use crypto_bigint::U256;
use marketplace_wrappers::HexStringWrapper;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
	#[error("Invalid state transition")]
	InvalidStateTransition,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, Default, HexStringWrapper)]
pub struct Id(HexPrefixedString);
pub struct Contribution;

impl Aggregate for Contribution {
	type Error = Error;
	type Event = Event;
	type Id = Id;
	type State = State;

	fn apply(state: Self::State, event: Self::Event) -> Result<Self::State, Self::Error> {
		match event {
			Event::Created {
				id: _,
				project_id,
				gate,
			} if state.status == Status::None => Ok(Self::State {
				project_id,
				gate,
				status: Status::Open,
				..state
			}),
			Event::Assigned {
				id: _,
				contributor_id,
			} if state.status == Status::Open => Ok(Self::State {
				status: Status::Assigned,
				contributor_id: Some(contributor_id),
				..state
			}),
			Event::Unassigned { id: _ } if state.status == Status::Assigned => Ok(Self::State {
				status: Status::Open,
				contributor_id: None,
				..state
			}),
			Event::Validated { id: _ } if state.status == Status::Assigned => Ok(Self::State {
				status: Status::Completed,
				..state
			}),
			_ => Err(Self::Error::InvalidStateTransition),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use rstest::*;

	#[fixture]
	fn no_contribution() -> State {
		State::default()
	}

	#[fixture]
	fn open_contribution() -> State {
		State {
			status: Status::Open,
			..Default::default()
		}
	}

	#[fixture]
	fn assigned_contribution() -> State {
		State {
			status: Status::Assigned,
			contributor_id: Some(Default::default()),
			..Default::default()
		}
	}

	#[fixture]
	fn completed_contribution() -> State {
		State {
			status: Status::Completed,
			contributor_id: Some(Default::default()),
			..Default::default()
		}
	}

	#[fixture]
	fn contribution_created_event() -> Event {
		Event::Created {
			id: Default::default(),
			project_id: Default::default(),
			gate: Default::default(),
		}
	}

	#[fixture]
	fn contribution_assigned_event() -> Event {
		Event::Assigned {
			id: Default::default(),
			contributor_id: Default::default(),
		}
	}

	#[fixture]
	fn contribution_unassigned_event() -> Event {
		Event::Unassigned {
			id: Default::default(),
		}
	}

	#[fixture]
	fn contribution_validated_event() -> Event {
		Event::Validated {
			id: Default::default(),
		}
	}

	fn apply(state: State, event: Event) -> Result<State, Error> {
		<Contribution as Aggregate>::apply(state, event)
	}

	#[rstest]
	fn create_contribution(no_contribution: State, contribution_created_event: Event) {
		let state = apply(no_contribution, contribution_created_event).unwrap();
		assert_eq!(Status::Open, state.status);
	}

	#[rstest]
	fn assign_contribution(open_contribution: State, contribution_assigned_event: Event) {
		let state = apply(open_contribution, contribution_assigned_event).unwrap();
		assert_eq!(Status::Assigned, state.status);
		assert!(state.contributor_id.is_some());
	}

	#[rstest]
	fn unassign_contribution(assigned_contribution: State, contribution_unassigned_event: Event) {
		let state = apply(assigned_contribution, contribution_unassigned_event).unwrap();
		assert_eq!(Status::Open, state.status);
		assert!(state.contributor_id.is_none());
	}

	#[rstest]
	fn validate_contribution(assigned_contribution: State, contribution_validated_event: Event) {
		let state = apply(assigned_contribution, contribution_validated_event).unwrap();
		assert_eq!(Status::Completed, state.status);
	}

	#[rstest]
	#[case(no_contribution(), contribution_assigned_event())]
	#[case(no_contribution(), contribution_unassigned_event())]
	#[case(no_contribution(), contribution_validated_event())]
	#[case(open_contribution(), contribution_created_event())]
	#[case(open_contribution(), contribution_unassigned_event())]
	#[case(open_contribution(), contribution_validated_event())]
	#[case(assigned_contribution(), contribution_created_event())]
	#[case(assigned_contribution(), contribution_assigned_event())]
	#[case(completed_contribution(), contribution_created_event())]
	#[case(completed_contribution(), contribution_assigned_event())]
	#[case(completed_contribution(), contribution_unassigned_event())]
	#[case(completed_contribution(), contribution_validated_event())]
	fn invalid_transition(#[case] state: State, #[case] event: Event) {
		assert_eq!(
			Error::InvalidStateTransition,
			apply(state, event).unwrap_err()
		);
	}
}

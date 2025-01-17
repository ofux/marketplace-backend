use crate::Event;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::{Debug, Display};

pub trait Aggregate: Send + Sync + Default + Sized {
	type Event: Serialize + DeserializeOwned + Debug + Display + Clone + Into<Event>;
	type Id: PartialEq + Display;
}

pub trait EventSourcable: Aggregate {
	fn apply_event(self, event: &Self::Event) -> Self;

	fn apply_events(self, events: &[Self::Event]) -> Self {
		events.iter().fold(self, Self::apply_event)
	}

	fn from_events(events: &[Self::Event]) -> Self {
		Self::apply_events(Default::default(), events)
	}
}

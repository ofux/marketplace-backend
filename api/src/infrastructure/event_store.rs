use crate::{
	database::{schema::events, Client},
	domain::{EventStore, EventStoreError},
};
use diesel::prelude::*;
use domain::{Aggregate, Payment, Project};
use log::error;
use serde_json::Value;

trait NamedAggregate: Aggregate {
	fn name() -> String;
}

impl NamedAggregate for Project {
	fn name() -> String {
		String::from("PROJECT")
	}
}

impl NamedAggregate for Payment {
	fn name() -> String {
		String::from("PAYMENT")
	}
}

impl<A: NamedAggregate> EventStore<A> for Client {
	fn list_by_id(&self, aggregate_id: &A::Id) -> Result<Vec<A::Event>, EventStoreError> {
		let connection = self.connection().map_err(|e| {
			error!("Failed to connect to database: {e}");
			EventStoreError::Connection(e.into())
		})?;

		let events = events::dsl::events
			.select(events::payload)
			.filter(events::aggregate_id.eq(aggregate_id.to_string()))
			.filter(events::aggregate_name.eq_all(A::name()))
			.order_by(events::timestamp)
			.then_order_by(events::index)
			.load::<Value>(&*connection)
			.map_err(|e| {
				error!(
					"Failed to retrieve {} events of aggregate {aggregate_id} from database: {e}",
					A::name()
				);
				EventStoreError::List(e.into())
			})?;

		deserialize_events::<A>(events)
	}

	fn list(&self) -> Result<Vec<A::Event>, EventStoreError> {
		let connection = self.connection().map_err(|e| {
			error!("Failed to connect to database: {e}");
			EventStoreError::Connection(e.into())
		})?;

		let events = events::dsl::events
			.select(events::payload)
			.filter(events::aggregate_name.eq_all(A::name()))
			.order_by(events::timestamp)
			.then_order_by(events::index)
			.load::<Value>(&*connection)
			.map_err(|e| {
				error!("Failed to retrieve {} events from database: {e}", A::name());
				EventStoreError::List(e.into())
			})?;

		deserialize_events::<A>(events)
	}
}

fn deserialize_events<A: Aggregate>(
	serialized_events: Vec<Value>,
) -> Result<Vec<A::Event>, EventStoreError> {
	serialized_events
		.iter()
		.map(|event_value| {
			serde_json::from_value::<A::Event>(event_value.clone()).map_err(|e| {
				error!("Failed to deserialize event {event_value}: {e}");
				e
			})
		})
		.collect::<Result<Vec<_>, _>>()
		.map_err(|e| EventStoreError::List(e.into()))
}

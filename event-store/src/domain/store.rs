use crate::Event;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Unable to connect to the event store")]
	Connection(#[source] anyhow::Error),
	#[error("Invalid event")]
	InvalidEvent(#[source] anyhow::Error),
	#[error("Unable to append event to the store")]
	Append(#[source] anyhow::Error),
}

pub trait Store: Send + Sync {
	fn append(&self, aggregate_id: &str, events: Vec<Event>) -> Result<(), Error>;
}
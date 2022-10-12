use super::Message;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Something happened at infrastructure level")]
	Infrastructure(#[from] anyhow::Error),
	#[error("Consumer did not acknowledge the message")]
	Nack,
	#[error("Failed while serializing message")]
	Serialize(#[from] serde_json::Error),
}

#[async_trait]
pub trait Publisher<M: Message> {
	async fn publish(&self, message: M) -> Result<(), Error>;
}
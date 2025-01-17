use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
	Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Display, From, Into,
)]
pub struct Id(Uuid);

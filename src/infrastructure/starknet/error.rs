use super::contracts::ContractError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Something happened during contract invocation")]
	Contract(#[from] ContractError),
	#[error("Mutex lock cannot be acquired")]
	Mutex(String),
}
use crate::domain::{EventListener, PaymentRequest, ProjectionRepository};
use anyhow::Result;
use async_trait::async_trait;
use domain::{Event, PaymentRequestEvent};
use std::sync::Arc;

pub struct Projector {
	repository: Arc<dyn ProjectionRepository<PaymentRequest>>,
}

impl Projector {
	pub fn new(repository: Arc<dyn ProjectionRepository<PaymentRequest>>) -> Self {
		Self { repository }
	}
}

#[async_trait]
impl EventListener for Projector {
	async fn on_event(&self, event: &Event) -> Result<()> {
		if let Event::PaymentRequest(event) = event {
			match event {
				PaymentRequestEvent::Created {
					id,
					project_id,
					requestor_id,
					recipient_id,
					amount_in_usd,
					reason,
				} => self.repository.insert(&PaymentRequest::new(
					(*id).into(),
					(*project_id).into(),
					(*requestor_id).into(),
					(*recipient_id).into(),
					*amount_in_usd as i64,
					reason.clone(),
				))?,
			}
		}
		Ok(())
	}
}

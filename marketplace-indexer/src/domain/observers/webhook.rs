use super::*;
use std::sync::Arc;

pub struct WebhookObserver {
	projector: Arc<WebhookProjector>,
}

impl WebhookObserver {
	pub fn new(projector: Arc<WebhookProjector>) -> Self {
		Self { projector }
	}
}

#[async_trait]
impl Observer for WebhookObserver {
	async fn on_new_event(&self, event: &ObservedEvent, _block_number: u64) {
		match &event.event {
			Event::Contribution(event) =>
				<WebhookProjector as Projector<WebhookContributionProjection>>::project(
					&self.projector,
					&event,
				)
				.await,
			Event::Project(event) =>
				<WebhookProjector as Projector<WebhookProjectProjection>>::project(
					&self.projector,
					&event,
				)
				.await,
		};
	}
}

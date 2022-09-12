use async_trait::async_trait;
use log::info;

use crate::{
	Contribution, ContributionEvent, ProjectAggregate, ProjectEvent, Projection, Projector,
};

pub struct WebhookProjector {}

impl WebhookProjector {
	pub fn new() -> WebhookProjector {
		WebhookProjector {}
	}
}

#[async_trait]
impl Projector<WebhookContributionProjection> for WebhookProjector {
	async fn project(&self, event: &ContributionEvent) {
		info!("WebhookContributionProjection {event}");
	}
}

#[async_trait]
impl Projector<WebhookProjectProjection> for WebhookProjector {
	async fn project(&self, event: &ProjectEvent) {
		info!("WebhookProjectProjection {event}");
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WebhookContributionProjection {}

impl Projection for WebhookContributionProjection {
	type A = Contribution;
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WebhookProjectProjection {}

impl Projection for WebhookProjectProjection {
	type A = ProjectAggregate;
}

use anyhow::Result;
use dotenv::dotenv;
use infrastructure::tracing::Tracer;

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();

	let _tracer = Tracer::init("event-queue-worker")?;
	event_listeners::main().await
}
